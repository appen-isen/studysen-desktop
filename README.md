# Studysen Desktop

L'application Studysen est un portage de l'application mobile Studysen sur Windows, Linux et MacOS.

## Développement

1. Tout d'abord, il faut installer les prérequis pour le développement de l'application.
   https://v2.tauri.app/start/prerequisites/ (ne pas faire la partie sur l'application mobile car on utilise déjà React Native pour ça)

2. Ensuite, il faut installer les dépendances du projet avec la commande suivante :

    ```bash
    npm install
    ```

3. Ensuite, vous pouvez lancer le site Web Expo avec la commande suivante dans le repértoire de l'application mobile Studysen (autre repository que celui-ci sur GitHub) :

    ```bash
    npm run web
    ```

    Cela va lancer le serveur de développement Expo sur le port 8081.

4. Démarragez le projet avec la commande suivante :

    ```bash
    npm run tauri dev
    ```

## Build

1. Vous pouvez builder l'application avec la commande suivante, la clé privée de signature doit être renseignée dans la variable d'environnement `TAURI_SIGNING_PRIVATE_KEY` (vous pouvez la trouver dans le drive dédié aux clés) :

    - **Pour Windows** (dans PowerShell) :

    ```powershell
    $env:TAURI_SIGNING_PRIVATE_KEY="CHEMIN VERS CLÉ PRIVÉE"
    npm run build
    ```

    - **Pour Linux/MacOS** :

    ```bash
    export TAURI_SIGNING_PRIVATE_KEY="CHEMIN VERS CLÉ PRIVÉE"
    npm run build
    ```

2. Maintenant, il suffit de faire un commit et un push sur GitHub car le numéro de version de l'application à été incrémenté automatiquement dans les fichiers de configuration.

3. Puis, il faut créer une release sur GitHub avec le tag `vX.X.X` (X.X.X étant la version de l'application) et uploader les fichiers `src-tauri/target/release/bundle/nsis/Studysen_0.1.0_x64-setup.exe` et `latest.json` dans la release.
