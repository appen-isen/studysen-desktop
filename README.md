# ISEN Orbit Desktop

L'application ISEN Orbit est un portage de l'application mobile ISEN Orbit sur Windows, Linux et MacOS.

## Développement

1. Tout d'abord, il faut installer les prérequis pour le développement de l'application.
   https://v2.tauri.app/start/prerequisites/ (ne pas faire la partie sur l'application mobile car on utilise déjà React Native pour ça)

2. Ensuite, il faut installer les dépendances du projet avec la commande suivante :

    ```bash
    npm install
    ```

3. Ensuite, vous pouvez lancer le site Web Expo avec la commande suivante dans le repértoire de l'application mobile ISEN Orbit :

    ```bash
    npm run web
    ```

    Cela va lancer le serveur de développement Expo sur le port 8081.

4. Démarragez le projet avec la commande suivante :

    ```bash
    npm run tauri dev
    ```
