/**
 * Direct adapter manifests must name exactly their `RenderComponent` impls.
 *
 * These manifests are runtime introspection for the legacy direct-adapter
 * path. They are deliberately independent per backend, but neither may claim a
 * component it cannot render or omit one it implements.
 */
import { readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";

type Adapter = {
  name: string;
  sourceRoot: string;
};

const ADAPTERS: Adapter[] = [
  {
    name: "GPUI",
    sourceRoot: new URL("../../adapter/src", import.meta.url).pathname,
  },
  {
    name: "Jetstream",
    sourceRoot: new URL("../../../jetstream/adapter/src", import.meta.url).pathname,
  },
];

function rustFiles(root: string): string[] {
  return readdirSync(root).flatMap((entry) => {
    const path = join(root, entry);
    if (statSync(path).isDirectory()) return rustFiles(path);
    return path.endsWith(".rs") ? [path] : [];
  });
}

function implementationNames(adapter: Adapter): string[] {
  const source = rustFiles(adapter.sourceRoot)
    .map((path) => readFileSync(path, "utf8"))
    .join("\n");
  return [
    ...source.matchAll(
      /impl\s+RenderComponent<([A-Za-z0-9_]+Spec)>\s+for\s+[A-Za-z0-9_]+Adapter/g,
    ),
  ].map((match) => match[1]);
}

function manifestNames(adapter: Adapter): string[] {
  const source = readFileSync(join(adapter.sourceRoot, "lib.rs"), "utf8");
  return [...source.matchAll(/const SUPPORTED_[A-Z_]+:\s*&\[&str\]\s*=\s*&\[(.*?)\];/gs)]
    .flatMap((block) => [...block[1].matchAll(/"([A-Za-z0-9_]+Spec)"/g)])
    .map((match) => match[1]);
}

function duplicates(names: string[]): string[] {
  return [...new Set(names.filter((name, index) => names.indexOf(name) !== index))].sort();
}

let failed = false;

for (const adapter of ADAPTERS) {
  const implementations = implementationNames(adapter);
  const manifest = manifestNames(adapter);
  const implementationSet = new Set(implementations);
  const manifestSet = new Set(manifest);
  const missing = [...implementationSet].filter((name) => !manifestSet.has(name)).sort();
  const phantom = [...manifestSet].filter((name) => !implementationSet.has(name)).sort();
  const repeated = duplicates(manifest);

  if (missing.length || phantom.length || repeated.length) {
    failed = true;
    console.error(`${adapter.name} adapter manifest drift:`);
    if (missing.length) console.error(`  missing: ${missing.join(", ")}`);
    if (phantom.length) console.error(`  no implementation: ${phantom.join(", ")}`);
    if (repeated.length) console.error(`  duplicate: ${repeated.join(", ")}`);
    continue;
  }

  console.log(
    `${adapter.name} adapter manifest matches ${implementations.length} direct RenderComponent implementations.`,
  );
}

if (failed) process.exit(1);
