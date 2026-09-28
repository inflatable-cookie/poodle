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

const DEFAULT_AS_SVELTE =
  /export\s*\{\s*default as\s+(\w+)\s*\}\s*from\s+"\.\/[^"\n]+\.svelte"/g;
const ALIAS_EXPORT = /export\s*\{\s*[A-Za-z_$][\w$]*\s+as\s+(\w+)\s*\}/g;

export function parseSveltePublicComponentNames(source: string): string[] {
  const names = new Set<string>();
  for (const match of source.matchAll(DEFAULT_AS_SVELTE)) names.add(match[1]);
  for (const match of source.matchAll(ALIAS_EXPORT)) names.add(match[1]);
  return [...names].sort();
}

export function sveltePublicComponentNames(root = repoRoot): Set<string> {
  const names = new Set<string>();
  for (const barrel of SVELTE_PUBLIC_BARRELS) {
    const source = readFileSync(join(root, barrel), "utf8");
    for (const name of parseSveltePublicComponentNames(source)) names.add(name);
  }
  return names;
}
