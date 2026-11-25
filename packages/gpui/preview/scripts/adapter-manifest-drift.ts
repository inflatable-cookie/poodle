/**
 * GPUI's legacy direct adapter manifest must match its `RenderComponent` impls.
 */
import { readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";

const sourceRoot = new URL("../../adapter/src", import.meta.url).pathname;

function rustFiles(root: string): string[] {
  return readdirSync(root).flatMap((entry) => {
    const path = join(root, entry);
    if (statSync(path).isDirectory()) return rustFiles(path);
    return path.endsWith(".rs") ? [path] : [];
  });
}

function implementationNames(): string[] {
  const source = rustFiles(sourceRoot)
    .map((path) => readFileSync(path, "utf8"))
    .join("\n");
  return [
    ...source.matchAll(
      /impl\s+RenderComponent<([A-Za-z0-9_]+Spec)>\s+for\s+GpuiAdapter\b/g,
    ),
  ].map((match) => match[1]);
}

function manifestNames(): string[] {
  const source = readFileSync(join(sourceRoot, "lib.rs"), "utf8");
  return [...source.matchAll(/const SUPPORTED_[A-Z_]+:\s*&\[&str\]\s*=\s*&\[(.*?)\];/gs)]
    .flatMap((block) => [...block[1].matchAll(/"([A-Za-z0-9_]+Spec)"/g)])
    .map((match) => match[1]);
}

function duplicates(names: string[]): string[] {
  return [...new Set(names.filter((name, index) => names.indexOf(name) !== index))].sort();
}

const implementations = implementationNames();
const manifest = manifestNames();
const implementationSet = new Set(implementations);
const manifestSet = new Set(manifest);
const missing = [...implementationSet].filter((name) => !manifestSet.has(name)).sort();
const phantom = [...manifestSet].filter((name) => !implementationSet.has(name)).sort();
const repeated = duplicates(manifest);

if (missing.length || phantom.length || repeated.length) {
  console.error("GPUI adapter manifest drift:");
  if (missing.length) console.error(`  missing: ${missing.join(", ")}`);
  if (phantom.length) console.error(`  no implementation: ${phantom.join(", ")}`);
  if (repeated.length) console.error(`  duplicate: ${repeated.join(", ")}`);
  process.exit(1);
}

console.log(
  `GPUI adapter manifest matches ${implementations.length} direct RenderComponent implementations.`,
);
