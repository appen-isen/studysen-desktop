#!/usr/bin/env zx

import { question, $ } from "zx";
import fs from "fs";
import path from "path";

// On récupère le chemin vers l'app Expo à exporter
let defaultPath = "../isen-orbit";
let appPath = await question(`Chemin vers l'app Expo à exporter ? (par défaut: ${defaultPath}): `);
appPath = appPath.trim() || defaultPath;

console.log("⏳ Export en cours...");
await $`cd ${appPath} && npx expo export --platform web`.pipe(process.stdout);

console.log("⏳ Copie des fichiers...");
await $`cp -r ${appPath}/dist/* ./src/`.pipe(process.stdout);

console.log("✅ Build web terminé et copié dans src/");

// On applique les correctifs pour ajouter un fichier preload dans index.html
const indexPath = path.join("src", "index.html");
let indexContent = fs.readFileSync(indexPath, "utf8");

// Ajoute la ligne juste avant le premier <script ...> existant, sinon avant </body>
const preloadScript = `<script src="preload.ts" defer></script>`;
if (!indexContent.includes(preloadScript)) {
    const scriptTagRegex = /<script\s+src=["'][^"']*["'][^>]*>/i;
    // Insère avant le premier <script>
    indexContent = indexContent.replace(
        scriptTagRegex,
        `${preloadScript}\n$&`
    );
    fs.writeFileSync(indexPath, indexContent, "utf8");
    console.log("✅ Script preload.ts ajouté à index.html");
}

//On applique le correctif pour changer l'URL de l'API dans le dossier _expo/static/js/web
const webJsDir = path.join("src", "_expo", "static", "js", "web");
const jsFile = fs.readdirSync(webJsDir)[0];
const webJsPath = path.join(webJsDir, jsFile);
let webJsContent = fs.readFileSync(webJsPath, "utf8");
webJsContent = webJsContent.replace(/https:\/\/web\.isen-ouest\.fr\/webAurion/g, "http://localhost:11689/");