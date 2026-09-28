import { describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

import {
  parseSveltePublicComponentExports,
  selectPublicSvelteEntries,
  sveltePublicComponentSources,
} from "./svelte-public-components";

describe("svelte public component sources", () => {
  test("live barrels cover public components and exclude internals", () => {
    const sources = sveltePublicComponentSources();
    expect(sources.has("Button")).toBe(true);
    expect(sources.has("MarkdownRenderer")).toBe(true);
    expect(sources.has("CodeEditor")).toBe(true);
    expect(sources.has("RichTextEditor")).toBe(true);
    expect(sources.has("DragDropProvider")).toBe(true);
    expect(sources.has("MenuSurface")).toBe(false);
  });

  test("a planted top-level .svelte file is not public unless a barrel exports it", () => {
    const planted = parseSveltePublicComponentExports(
      'export { default as Button } from "./Button.svelte";\n',
    );
    expect(planted).toEqual(new Map([["Button", "Button"]]));
    expect(planted.has("MenuSurface")).toBe(false);
    expect(planted.has("PlantedInternal")).toBe(false);
  });

  test("an alias re-export keeps the implementation file public under its export name", () => {
    const planted = parseSveltePublicComponentExports(
      'import Planted from "./Planted.svelte";\nexport { Planted as PlantedPublic };\n',
    );
    expect(planted).toEqual(new Map([["PlantedPublic", "Planted"]]));
  });

  test("a source-module alias re-export maps the export to its file", () => {
    const planted = parseSveltePublicComponentExports(
      'export { Renamed as PlantedPublic } from "./Planted.svelte";\nexport { default } from "./Solo.svelte";\n',
    );
    expect(planted).toEqual(
      new Map([
        ["PlantedPublic", "Planted"],
        ["Solo", "Solo"],
      ]),
    );
  });

  test("selection keeps a module backed by an alias export and drops internals", () => {
    const planted = parseSveltePublicComponentExports(
      'import Planted from "./Planted.svelte";\nexport { Planted as PlantedPublic };\n',
    );
    const selected = selectPublicSvelteEntries(
      { "./src/Planted.svelte": "planted-module", "./src/MenuSurface.svelte": "internal-module" },
      new Set(planted.values()),
    );
    expect(selected).toEqual([["Planted", "planted-module"]]);
  });

  test("a planted barrel directory is the authority, not a filename glob", () => {
    const root = mkdtempSync(join(tmpdir(), "poodle-svelte-public-"));
    mkdirSync(join(root, "packages/svelte/components/src"), { recursive: true });
    writeFileSync(
      join(root, "packages/svelte/components/src/index.ts"),
      'export { default as Button } from "./Button.svelte";\n',
    );
    for (const barrel of ["markdown.ts", "rich-text.ts", "editor.ts", "drag-drop.ts"]) {
      writeFileSync(join(root, "packages/svelte/components/src", barrel), "");
    }
    writeFileSync(join(root, "packages/svelte/components/src/PlantedInternal.svelte"), "<div></div>\n");
    const sources = sveltePublicComponentSources(root);
    expect(sources.has("Button")).toBe(true);
    expect(sources.has("PlantedInternal")).toBe(false);
  });
});
