#!/usr/bin/env zx

import { question, $ } from "zx";
import fs from "fs";

const latestJSON = "latest.json";

//On transforme le fichier latest.json en objet JS
let latestData = JSON.parse(fs.readFileSync(latestJSON, "utf8"));

let notes = await question(`Message pour la mise à jour (une seule ligne): `);
latestData.notes = notes.trim() || "";
latestData.pub_date = new Date().toISOString().replace(/\.\d{3}Z$/, "Z");

// On va chercher la signature dans src-tauri/target/release/bundle/nsis/
const path = `src-tauri/target/release/bundle/nsis/ISEN-Orbit_${latestData.version}_x64-setup.exe.sig`;
let signature = "";
if (fs.existsSync(path)) {
    signature = fs.readFileSync(path, "utf8").trim();
} else {
    console.error(`❌ Signature du NSIS non trouvée: ${path}`);
    process.exit(1);
}

latestData.platforms["windows-x86_64"] = {
    signature: signature,
    url: `https://github.com/appen-isen/isen-orbit-desktop/releases/download/v${latestData.version}/ISEN-Orbit_${latestData.version}_x64-setup.exe`
}

fs.writeFileSync(latestJSON, JSON.stringify(latestData, null, 4), "utf8");

console.log("✅ Post-update terminée !");
console.log("Il faut maintenant créer une nouvelle release sur GitHub avec le tag v" + latestData.version);
console.log("Et mettre dans cette release les fichiers exécutables suivants :");
console.log(" - src-tauri/target/bundle/nsis/ISEN-Orbit_" + latestData.version + "_x64-setup.exe");
console.log(" - latest.json");
