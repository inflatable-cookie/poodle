import { describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

import {
  parseSveltePublicComponentNames,
  sveltePublicComponentNames,
} from "./svelte-public-components";

describe("svelte public component names", () => {
  test("live barrels include public components and exclude internals", () => {
    const names = sveltePublicComponentNames();
    expect(names.has("Button")).toBe(true);
    expect(names.has("MarkdownRenderer")).toBe(true);
    expect(names.has("CodeEditor")).toBe(true);
    expect(names.has("RichTextEditor")).toBe(true);
    expect(names.has("DragDropProvider")).toBe(true);
    expect(names.has("MenuSurface")).toBe(false);
  });

  test("a planted top-level .svelte file is not public unless a barrel exports it", () => {
    const planted = parseSveltePublicComponentNames(
      'export { default as Button } from "./Button.svelte";\n',
    );
    expect(planted).toEqual(["Button"]);
    expect(planted).not.toContain("MenuSurface");
    expect(planted).not.toContain("PlantedInternal");
  });

  test("an alias re-export counts as a public component", () => {
    expect(
      parseSveltePublicComponentNames(
        'import Planted from "./Planted.svelte";\nexport { Planted as PlantedPublic };\n',
      ),
    ).toEqual(["PlantedPublic"]);
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
    const names = sveltePublicComponentNames(root);
    expect(names.has("Button")).toBe(true);
    expect(names.has("PlantedInternal")).toBe(false);
  });
});
