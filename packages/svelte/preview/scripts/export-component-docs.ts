import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

import { componentDocsMap } from "../src/component-docs";

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const previewDir = path.resolve(scriptDir, "..");
const artifactPath = path.join(previewDir, "artifacts", "component-docs.json");

fs.mkdirSync(path.dirname(artifactPath), { recursive: true });
fs.writeFileSync(artifactPath, `${JSON.stringify(componentDocsMap, null, 2)}\n`);

console.log(`Wrote ${artifactPath}`);
