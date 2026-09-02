import { readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";

import { INTERNAL_SVELTE_NAMES } from "./shell-contract";

const internal = new Set<string>(INTERNAL_SVELTE_NAMES);

export function generateSvelteComponentDeclarations(packageRoot: string): void {
  const src = join(packageRoot, "src");
  const dist = join(packageRoot, "dist");
  const files = readdirSync(src).filter((name) => name.endsWith(".svelte"));
  if (files.length === 0) {
    throw new Error("no Svelte components found for declaration emit");
  }
  for (const file of files) {
    const stem = file.slice(0, -".svelte".length);
    const wildcardDeclaration = join(dist, `${file}.d.ts`);
    if (internal.has(stem)) {
      rmSync(wildcardDeclaration, { force: true });
      writeFileSync(
        join(dist, `${stem}.d.ts`),
        [
          `import type { Component } from "svelte";`,
          `declare const ${stem}: Component;`,
          `export default ${stem};`,
          "",
        ].join("\n"),
      );
      continue;
    }
    writeFileSync(
      wildcardDeclaration,
      [
        `import type { Component } from "svelte";`,
        `declare const ${stem}: Component;`,
        `export default ${stem};`,
        "",
      ].join("\n"),
    );
  }
  for (const name of readdirSync(dist).filter((entry) => entry.endsWith(".d.ts"))) {
    const path = join(dist, name);
    const source = readFileSync(path, "utf8");
    let next = source;
    for (const stem of INTERNAL_SVELTE_NAMES) {
      next = next.replaceAll(`./${stem}.svelte`, `./${stem}.js`);
    }
    if (next !== source) writeFileSync(path, next);
  }
}
