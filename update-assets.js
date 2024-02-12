import { readdirSync, existsSync, readFileSync, copyFileSync, statSync, mkdirSync } from 'fs';
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

// function syncFolder(source, destination) {
//     // Sync all files in source to destination
//     readdirSync(source).forEach(file => {
//         syncFile(join(source, file), join(destination, file));
//     });
// }

function resursiveSyncFolder(source, destination) {
    // Sync all files in source to destination
    readdirSync(source).forEach(file => {
        const sourcePath = join(source, file);
        const destinationPath = join(destination, file);

        // Check if the file is a directory
        if (statSync(sourcePath).isDirectory()) {
            console.log(`Syncing folder ${destinationPath}`);

            // Create the directory if it doesn't exist
            if (!existsSync(destinationPath)) {
                return // Ignore the folder
            }

            // Sync the folder
            resursiveSyncFolder(sourcePath, destinationPath);
        } else {
            syncFile(sourcePath, destinationPath);
        }
    })
}

resursiveSyncFolder("../StarRailRes/image/character_preview/", "./public/hsr/image/character_preview/");
resursiveSyncFolder("../StarRailRes/image/light_cone_preview/", "./public/hsr/image/light_cone_preview/");
resursiveSyncFolder("../StarRailRes/icon/", "./public/hsr/icon/");
