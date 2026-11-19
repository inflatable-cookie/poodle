import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

import { buildSkillReference } from "./skill-reference";

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(scriptDir, "../../../..");
const referencePath = path.join(repoRoot, "skills", "poodle", "references", "reference.md");

fs.mkdirSync(path.dirname(referencePath), { recursive: true });
fs.writeFileSync(referencePath, buildSkillReference());

console.log(`Wrote ${referencePath}`);
