use std::sync::Mutex;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use tauri::http::HeaderMap;
use tauri::AppHandle;
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};
use tauri_plugin_http::reqwest;
use tauri_plugin_http::reqwest::Client;

struct TauriAppState {
    app: Mutex<AppHandle>,
}

const CLIENT: Client = reqwest::Client::new();

async fn proxy(
    req: HttpRequest,
    body: web::Bytes,
    client: web::Data<Client>,
) -> actix_web::Result<HttpResponse> {
    // Construct the target URL
    let uri = format!(
        "https://web.isen-ouest.fr/webAurion{}{}",
        req.uri().path(),
        req.uri()
            .query()
            .map_or(String::new(), |q| format!("?{}", q))
    );

    // Build the reqwest request
    let mut request_builder = client
        .request(req.method().clone().parse(), &uri)
        .body(body.to_vec());

    // Copy headers from the original request
    for (header_name, header_value) in req.headers().iter() {
        request_builder = request_builder.header(header_name, header_value);
    }

    // Send the request and await the response
    let response = request_builder
        .send()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    // Build the Actix Web HttpResponse from the reqwest response
    let mut actix_response = HttpResponse::build(response.status());

    // Copy headers from the reqwest response
    for (header_name, header_value) in response.headers().iter() {
        actix_response.append_header((header_name.clone(), header_value.clone()));
    }

    // Read the response body
    let bytes = response
        .bytes()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

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
    match (HttpServer::new(move || {
        let tauri_app_instance = tauri_app_clone.clone();
        App::new()
            .app_data(tauri_app_instance)
            .app_data(web::Data::new(client.clone()))
            .default_service(web::to(proxy))
    })
    .bind(("127.0.0.1", port)))
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
