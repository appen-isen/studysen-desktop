use std::str::FromStr;
use std::sync::Mutex;
use actix_cors::Cors;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use actix_web::http::StatusCode;
use tauri::{AppHandle, http};
use http::Method;
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};
use tauri_plugin_http::reqwest::Client;

struct TauriAppState {
    app: Mutex<AppHandle>,
}

async fn proxy(
    req: HttpRequest,
    body: web::Bytes,
    client: web::Data<Client>,
) -> actix_web::Result<HttpResponse> {
    // On génère l'URL de la requête
    let uri = format!(
        "https://web.isen-ouest.fr/webAurion{}{}",
        req.uri().path(),
        req.uri()
            .query()
            .map_or(String::new(), |q| format!("?{}", q))
    );
    // La méthode de la requête
    let method_str = req.method().as_str();
    let method = Method::from_str(method_str).unwrap_or(Method::GET);

    // On crée la requête avec reqwest
    let mut request_builder = client
        .request(method, &uri)
        .body(body.to_vec());

    println!("Body: {:?}", body);

    // Copie les en-têtes de la requête d'origine
    let headers: Vec<(String, String)> = req
        .headers()
        .iter()
        .map(|(header_name, header_value)| {
            (
                header_name.as_str().to_string(),
                header_value.to_str().unwrap_or("").to_string(),
            )
        })
        .collect();
    for (header_name, header_value) in headers {
        println!("Added header to request: {}: {}", header_name, header_value);
        request_builder = request_builder.header(header_name, header_value);
    }

    // Ajout de headers supplémentaires
    request_builder = request_builder.header("Host", "web.isen-ouest.fr");
    request_builder = request_builder.header("Origin", "https://web.isen-ouest.fr");
    request_builder = request_builder.header("Referer", "https://web.isen-ouest.fr/webAurion/");

    println!("Request body: {:?}", request_builder);
    // On envoie la requête
    let response = request_builder
        .send()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    // On génère la réponse depuis la réponse de reqwest
    let status = response.status().as_u16();
    let mut actix_response = HttpResponse::build(StatusCode::from_u16(status).unwrap_or(StatusCode::OK));

    // On copie les headers depuis la réponse de reqwest
    for (header_name, header_value) in response.headers().iter() {
        let new_header_name = header_name.as_str();
        let new_header_value = header_value.to_str().unwrap_or("");
        println!("Added header to response: {}: {}", new_header_name, new_header_value);
        actix_response.append_header((new_header_name, new_header_value));
    }

    // On lit la réponse en bytes
    let bytes = response
        .bytes()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    println!("Response body: {:?}", bytes);
    Ok(actix_response.body(bytes))
}

#[actix_web::main]
pub async fn init(app: AppHandle) -> std::io::Result<()> {
    let tauri_app = web::Data::new(TauriAppState {
        app: Mutex::new(app),
    });
    let tauri_app_clone = tauri_app.clone();
    let client = Client::new();

    let port: u16 = 11689;
    println!("Listening on port {port}...");
    // On démarre le serveur HTTP
    match HttpServer::new(move || {
        let tauri_app_instance = tauri_app_clone.clone();
        App::new()
            .wrap(Cors::permissive())
            .app_data(tauri_app_instance)
            .app_data(web::Data::new(client.clone()))
            .default_service(web::to(proxy))
    })
        .bind(("127.0.0.1", port))
    {
        Ok(server) => server.run().await?,
        Err(e) => {
            // Si le port est déjà utilisé, on affiche un message d'erreur
            println!("Error starting server: {}", e);
            tauri_app
                .app
                .lock()
                .unwrap()
                .dialog()
                .message(format!(
                    "Le port {port} est déjà utilisé, veuillez le fermer."
                ))
                .kind(MessageDialogKind::Error)
                .title("Erreur")
                .blocking_show();
            return Err(e);
        }
    }
    Ok(())
}
