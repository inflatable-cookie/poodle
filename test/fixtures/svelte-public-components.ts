import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = join(dirname(fileURLToPath(import.meta.url)), "../..");

/** Package barrels that re-export public Svelte components. */
export const SVELTE_PUBLIC_BARRELS = [
  "packages/svelte/components/src/index.ts",
  "packages/svelte/components/src/markdown.ts",
  "packages/svelte/components/src/rich-text.ts",
  "packages/svelte/components/src/editor.ts",
  "packages/svelte/components/src/drag-drop.ts",
] as const;

/**
 * Map every public component export to the `.svelte` module that implements
 * it. The public name and the implementation file can differ
 * (`import Planted from "./Planted.svelte"; export { Planted as PlantedPublic };`),
 * so suites must select modules by implementation file, never by public name.
 *
 * Handles the barrel shapes in use: `export { default as Name } from "./X.svelte"`,
 * `export { Binding as Name } from "./X.svelte"`, `export { default } from "./X.svelte"`,
 * and alias exports of an imported `.svelte` binding.
 */
export function parseSveltePublicComponentExports(source: string): Map<string, string> {
  const imported = new Map<string, string>();
  for (const match of source.matchAll(
    /import\s+([A-Za-z_$][\w$]*)\s+from\s*"\.\/([^"\n]+\.svelte)"/g,
  )) {
    imported.set(match[1], match[2].replace(/\.svelte$/, ""));
  }

  const exports = new Map<string, string>();
  const add = (name: string, file: string) => {
    if (!exports.has(name)) exports.set(name, file);
  };

  for (const match of source.matchAll(
    /export\s*\{([^}]*)\}\s*from\s*"\.\/([^"\n]+\.svelte)"/g,
  )) {
    const file = match[2].replace(/\.svelte$/, "");
    for (const raw of match[1].split(",")) {
      const specifier = raw.trim();
      if (!specifier || specifier.startsWith("type ")) continue;
      const alias = specifier.match(/^(?:default|[A-Za-z_$][\w$]*)\s+as\s+([A-Za-z_$][\w$]*)$/);
      if (alias) add(alias[1], file);
      else if (specifier === "default") add(file, file);
      else add(specifier, file);
    }
  }

  for (const match of source.matchAll(/export\s*\{([^}]*)\}\s*;/g)) {
    for (const raw of match[1].split(",")) {
      const specifier = raw.trim();
      if (!specifier || specifier.startsWith("type ")) continue;
      const alias = specifier.match(/^[A-Za-z_$][\w$]*\s+as\s+([A-Za-z_$][\w$]*)$/);
      const local = alias ? specifier.slice(0, specifier.indexOf(" as")) : specifier;
      const file = imported.get(local);
      if (file !== undefined) add(alias ? alias[1] : local, file);
    }
  }

  return exports;
}

/**
 * Implementation files (module names without `.svelte`) that back at least one
 * public export. A module here is public surface even when only an export
 * alias names it.
 */
export function sveltePublicComponentSources(root = repoRoot): Set<string> {
  const sources = new Set<string>();
  for (const barrel of SVELTE_PUBLIC_BARRELS) {
    for (const file of parseSveltePublicComponentExports(readFileSync(join(root, barrel), "utf8")).values()) {
      sources.add(file);
    }
  }
  return sources;
}

/**
 * The shared selection step for the smoke, a11y and parity sweeps: a glob
 * module is public exactly when it backs at least one public export.
 */
export function selectPublicSvelteEntries<T>(
  modules: Record<string, T>,
  publicSources: Set<string>,
): Array<[string, T]> {
  return Object.entries(modules)
    .map(([file, mod]) => [file.split("/").pop()!.replace(/\.svelte$/, ""), mod] as const)
    .filter(([name]) => publicSources.has(name));
}
