#!/usr/bin/env zx

import { question, $ } from "zx";

// On récupère le chemin vers l'app Expo à exporter
let defaultPath = "../studysen";
let appPath = await question(`Chemin vers l'app Expo à exporter ? (par défaut: ${defaultPath}): `);
appPath = appPath.trim() || defaultPath;

console.log("⏳ Export en cours...");
await $`cd ${appPath} && npx expo export --platform web`.pipe(process.stdout);

console.log("⏳ Copie des fichiers...");
await $`mkdir -p ./dist/`.pipe(process.stdout);
await $`cp -r ${appPath}/dist/* ./dist/`.pipe(process.stdout);

console.log("✅ Build web terminé et copié dans dist/");
