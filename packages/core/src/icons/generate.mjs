/**
 * Generates individual tree-shakeable icon modules from lucide-static.
 *
 * Each icon becomes a standalone module that exports its SVG node data
 * as an `IconNodes` array — the same `[tag, attrs][]` format used by
 * `lucide-static/icon-nodes.json`.
 *
 * Usage: node generate.mjs
 */

import { readFileSync, writeFileSync, mkdirSync, rmSync } from "node:fs";
import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));

// Resolve lucide-static icon-nodes.json
const iconNodesPath = resolve(
  __dirname,
  "node_modules",
  "lucide-static",
  "icon-nodes.json",
);

let rawPath = iconNodesPath;

// Fallback: check hoisted node_modules
try {
  readFileSync(rawPath);
} catch {
  rawPath = resolve(__dirname, "..", "..", "..", "node_modules", "lucide-static", "icon-nodes.json");
  try {
    readFileSync(rawPath);
  } catch {
    // Try bun-style hoisting
    const { execSync } = await import("node:child_process");
    const resolved = execSync("node -e \"console.log(require.resolve('lucide-static/icon-nodes.json'))\"", {
      cwd: __dirname,
      encoding: "utf-8",
    }).trim();
    rawPath = resolved;
  }
}

const allNodes = JSON.parse(readFileSync(rawPath, "utf-8"));
const iconNames = Object.keys(allNodes).sort();

// Clean and recreate output directory
const iconsDir = resolve(__dirname, "icons");
rmSync(iconsDir, { recursive: true, force: true });
mkdirSync(iconsDir, { recursive: true });

/**
 * Convert kebab-case icon name to a valid camelCase JS identifier.
 *   "arrow-down" -> "arrowDown"
 *   "a-arrow-down" -> "aArrowDown"
 *   "circle-x" -> "circleX"
 */
function toCamelCase(kebab) {
  return kebab.replace(/-([a-z0-9])/g, (_, c) => c.toUpperCase());
}

/** JS reserved words that cannot be used as identifiers. */
const reservedWords = new Set([
  "break", "case", "catch", "class", "const", "continue", "debugger",
  "default", "delete", "do", "else", "enum", "export", "extends",
  "false", "finally", "for", "function", "if", "import", "in",
  "instanceof", "new", "null", "return", "super", "switch", "this",
  "throw", "true", "try", "typeof", "var", "void", "while", "with",
  "yield", "let", "static", "implements", "interface", "package",
  "private", "protected", "public", "await", "abstract", "boolean",
  "byte", "char", "double", "final", "float", "goto", "int", "long",
  "native", "short", "synchronized", "throws", "transient", "volatile",
]);

function toIdentifier(kebab) {
  const camel = toCamelCase(kebab);
  return reservedWords.has(camel) ? `${camel}Icon` : camel;
}

const indexExports = [];

for (const name of iconNames) {
  const nodes = allNodes[name];
  const ident = toIdentifier(name);
  const nodesJson = JSON.stringify(nodes);

  const content = `import type { IconNodes } from "../types";\n\nexport const ${ident}: IconNodes = ${nodesJson};\n`;

  writeFileSync(resolve(iconsDir, `${name}.ts`), content);
  indexExports.push(`export { ${ident} } from "./icons/${name}";`);
}

// Write barrel index
const indexContent = `// Auto-generated — do not edit. Run \`node generate.mjs\` to regenerate.
//
// ${iconNames.length} Lucide icons available as tree-shakeable exports.
// Each export is an IconNodes array: [tag, attrs][].

export type { IconNodes, IconNodeElement } from "./types";

${indexExports.join("\n")}
`;

writeFileSync(resolve(__dirname, "index.ts"), indexContent);

// Write types file
const typesContent = `/** A single SVG child element: [tagName, attributes]. */
export type IconNodeElement = [string, Record<string, string>];

/** The SVG node data for a single icon. */
export type IconNodes = IconNodeElement[];
`;

writeFileSync(resolve(__dirname, "types.ts"), typesContent);

console.log(`Generated ${iconNames.length} icon modules in src/icons/`);
console.log(`Generated src/index.ts barrel export`);
console.log(`Generated src/types.ts`);
