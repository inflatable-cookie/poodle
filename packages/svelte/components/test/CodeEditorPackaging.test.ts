import { existsSync, readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";

const SVELTE_ROOT = "packages/svelte/components/src/index.ts";
const REACT_ROOT = "packages/react/components/src/index.ts";

function read(path: string): string {
  return readFileSync(path, "utf8");
}

describe("CodeEditor packaging", () => {
  it("neither root barrel exports the editor", () => {
    for (const root of [SVELTE_ROOT, REACT_ROOT]) {
      const source = read(root);
      expect(source).not.toContain("CodeEditor");
      expect(source).not.toContain("code-editor");
      expect(source).not.toContain("./editor");
    }
  });

  it("both shells expose the dedicated editor entry re-exporting the component", () => {
    const svelte = read("packages/svelte/components/src/editor.ts");
    expect(svelte).toContain("CodeEditor.svelte");
    const react = read("packages/react/components/src/editor.ts");
    expect(react).toContain("./CodeEditor");
  });

  it("both manifests pin the exact CodeMirror set with no ranges", () => {
    const expected = [
      "@codemirror/commands",
      "@codemirror/lang-css",
      "@codemirror/lang-html",
      "@codemirror/lang-javascript",
      "@codemirror/lang-json",
      "@codemirror/lang-markdown",
      "@codemirror/lang-rust",
      "@codemirror/lang-yaml",
      "@codemirror/language",
      "@codemirror/legacy-modes",
      "@codemirror/search",
      "@codemirror/state",
      "@codemirror/view",
    ];
    for (const manifestPath of [
      "packages/svelte/components/package.json",
      "packages/react/components/package.json",
    ]) {
      const manifest = JSON.parse(read(manifestPath)) as { dependencies?: Record<string, string> };
      for (const name of expected) {
        const specifier = manifest.dependencies?.[name];
        expect(specifier, `${manifestPath} ${name}`).toMatch(/^\d+\.\d+\.\d+$/);
      }
      const raw = read(manifestPath);
      expect(raw).toContain('"./editor"');
    }
  });

  it("the editor entry surface never names the engine", () => {
    for (const entry of [
      "packages/svelte/components/src/editor.ts",
      "packages/react/components/src/editor.ts",
    ]) {
      expect(read(entry)).not.toContain("@codemirror");
    }
  });

  describe("compiled distribution", () => {
    const distFiles = [
      "packages/svelte/components/dist/index.client.js",
      "packages/svelte/components/dist/index.server.js",
      "packages/react/components/dist/index.js",
      "packages/svelte/components/dist/editor.client.js",
      "packages/svelte/components/dist/editor.server.js",
      "packages/react/components/dist/editor.js",
      "packages/svelte/components/dist/editor.d.ts",
      "packages/react/components/dist/editor.d.ts",
    ];
    const available = distFiles.every((file) => existsSync(file));

    it.skipIf(!available)("root bundles load no editor engine or language chunk", () => {
      for (const root of [
        "packages/svelte/components/dist/index.client.js",
        "packages/svelte/components/dist/index.server.js",
        "packages/react/components/dist/index.js",
      ]) {
        const bundle = read(root);
        expect(bundle).not.toContain("codemirror");
        expect(bundle).not.toContain("code-editor-engine");
      }
    });

    it.skipIf(!available)("editor bundles reach the engine only through external imports", () => {
      for (const editor of [
        "packages/svelte/components/dist/editor.client.js",
        "packages/react/components/dist/editor.js",
      ]) {
        expect(read(editor)).toContain("@codemirror/state");
      }
    });

    it.skipIf(!available)("editor declarations carry no engine types", () => {
      for (const declarations of [
        "packages/svelte/components/dist/editor.d.ts",
        "packages/react/components/dist/editor.d.ts",
      ]) {
        expect(read(declarations)).not.toContain("@codemirror");
      }
    });
  });
});
