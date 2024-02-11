import { readdirSync, existsSync, readFileSync, copyFileSync } from 'fs';
import { join, dirname, basename } from "path";
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

function syncFile(source, destination) {
    // Copy the file from source to destination
    // Unless the file is already there and has the same content

    // Check if the file exists
    if (existsSync(destination)) {
        // Check if the file is the same
        if (readFileSync(source).equals(readFileSync(destination))) {
            console.log(`File ${destination} is already up to date`);
            return;
        }
    }

    // Copy the file
    copyFileSync(source, destination);
    console.log(`File ${destination} updated`);
}

if (!existsSync("../StarRailRes")) {
    console.error("StarRailRes not found");
    process.exit(1);
}

const jsonAssets = "../StarRailRes/index_new/en/"
const jsonAssetsDestination = "./src-tauri/src/data/"

// Sync all json assets in destination
readdirSync(jsonAssetsDestination).forEach(file => {
    if (file.endsWith(".json")) {
        const name = basename(file);
        syncFile(join(jsonAssets, name), join(jsonAssetsDestination, name));
    }
});

const characterPreviewAssets = "../StarRailRes/image/character_preview/"
const characterPreviewAssetsDestination = "./public/hsr/image/character_preview/"

readdirSync(characterPreviewAssets).forEach(file => {
    const name = basename(file);
    syncFile(join(characterPreviewAssets, name), join(characterPreviewAssetsDestination, name));
});

const lightConePreviewAssets = "../StarRailRes/image/light_cone_preview/"
const lightConePreviewAssetsDestination = "./public/hsr/image/light_cone_preview/"

readdirSync(lightConePreviewAssets).forEach(file => {
    const name = basename(file);
    syncFile(join(lightConePreviewAssets, name), join(lightConePreviewAssetsDestination, name));
});
