// Script to generate schemas from bindings.gen.ts using ts-to-zod
// We need to make a temporary file and extract "user-defined types" from bindings.gen.ts, as ts-to-zod does not support reading from stdin

import { readFileSync, writeFileSync, rmSync } from "fs";
import { join, dirname } from "path";
import { fileURLToPath } from 'url';
import { execSync } from "child_process";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const tempFile = join(__dirname, "temp-bindings.gen.ts");

// Parse out everything between "user-defined types" and "tauri-specta globals"
let bindingsGen = readFileSync(join(__dirname, "./src/bindings.gen.ts"), "utf-8");
bindingsGen = bindingsGen.split("/** user-defined types **/")[1].split("/** tauri-specta globals **/")[0];

writeFileSync(tempFile, bindingsGen);

// Generate the schemas
execSync(`pnpm exec ts-to-zod ./temp-bindings.gen.ts ./src/schemas.gen.ts`);

// Clean up
rmSync(tempFile);
