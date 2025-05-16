#!/usr/bin/env zx

import { question, $ } from "zx";
import fs from "fs";

const packageJSON = "package.json";
const tauriConf = "src-tauri/tauri.conf.json";
const cargoToml = "src-tauri/Cargo.toml";
const latestJSON = "latest.json";

//On regarde si TAURI_SIGNING_PRIVATE_KEY est défini
if (!process.env.TAURI_SIGNING_PRIVATE_KEY) {
    console.error("❌ La variable d'environnement TAURI_SIGNING_PRIVATE_KEY n'est pas définie.");
    console.error("❌ Veuillez la définir avant de continuer. (Voir README.md)");
    process.exit(1);
}

// Nouvelle version à prendre
let newVersion = await question(`Nouvelle version ? (ex: 1.1.0) (par défaut, version incrémentée: ): `);
newVersion = newVersion.trim() || "";
if (newVersion === "") {
    //On convertit le fichier package.json en objet JS
    let packageData = JSON.parse(fs.readFileSync(packageJSON, "utf8"));
    //On incrémente la version
    const version = packageData.version.split(".");
    version[2] = parseInt(version[2]) + 1;
    newVersion = version.join(".");
    console.log(`✅ Version incrémentée automatiquement: ${newVersion}`);
}
else if (!/^\d+\.\d+\.\d+$/.test(newVersion.trim())) {
    console.error("❌ Format de version invalide. Utilisez le format x.y.z (ex: 1.1.0)");
    process.exit(1);
}

console.log("⏳ Mise à jour de la version dans les fichiers...");

// On met la version dans les fichiers nécessaires
const filesToUpdate = [packageJSON, tauriConf, latestJSON, cargoToml];
filesToUpdate.forEach((file) => {
    let data = fs.readFileSync(file, "utf8");
    if (file === packageJSON || file === tauriConf || file === latestJSON) {
        // On met à jour la version dans package.json et tauri.conf.json
        data = JSON.parse(data);
        data.version = newVersion;
        data = JSON.stringify(data, null, 4);
    }
    else {
        // On met à jour la version dans Cargo.toml
        data = data.replace(/version = "\d+\.\d+\.\d+"/, `version = "${newVersion}"`);
    }
    fs.writeFileSync(file, data, "utf8");
    console.log(`✅ Version mise à jour dans ${file}`);
});

console.log("✅ Pré-update terminée !");
