import { existsSync, readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";

const SVELTE_ROOT = "packages/svelte/components/src/index.ts";
const REACT_ROOT = "packages/react/components/src/index.ts";

function read(path: string): string {
  return readFileSync(path, "utf8");
}

const RICH_TEXT_SOURCES = [
  "packages/svelte/components/src/rich-text-engine.ts",
  "packages/react/components/src/rich-text-engine.ts",
  "packages/svelte/components/src/rich-text.ts",
  "packages/react/components/src/rich-text.ts",
];

const TIPTAP_SPECIFIERS = [
  "@tiptap/core",
  "@tiptap/pm",
  "@tiptap/extensions",
  "@tiptap/extension-blockquote",
  "@tiptap/extension-bold",
  "@tiptap/extension-code",
  "@tiptap/extension-code-block",
  "@tiptap/extension-document",
  "@tiptap/extension-hard-break",
  "@tiptap/extension-heading",
  "@tiptap/extension-horizontal-rule",
  "@tiptap/extension-image",
  "@tiptap/extension-italic",
  "@tiptap/extension-link",
  "@tiptap/extension-list",
  "@tiptap/extension-paragraph",
  "@tiptap/extension-strike",
  "@tiptap/extension-table",
  "@tiptap/extension-text",
];

describe("RichText packaging", () => {
  it("neither root barrel nor markdown/editor entries export the rich-text pair", () => {
    for (const root of [
      SVELTE_ROOT,
      REACT_ROOT,
      "packages/svelte/components/src/markdown.ts",
      "packages/react/components/src/markdown.ts",
      "packages/svelte/components/src/editor.ts",
      "packages/react/components/src/editor.ts",
    ]) {
      const source = read(root);
      expect(source).not.toContain("RichTextEditor");
      expect(source).not.toContain("RichTextRenderer");
      expect(source).not.toContain("rich-text");
      expect(source).not.toContain("./rich-text");
    }
  });

  it("both shells expose the dedicated rich-text entry re-exporting the pair", () => {
    const svelte = read("packages/svelte/components/src/rich-text.ts");
    expect(svelte).toContain("RichTextEditor.svelte");
    expect(svelte).toContain("RichTextRenderer.svelte");
    const react = read("packages/react/components/src/rich-text.ts");
    expect(react).toContain("./RichTextEditor");
    expect(react).toContain("./RichTextRenderer");
  });

  it("both manifests pin the exact TipTap set with no ranges", () => {
    for (const manifestPath of [
      "packages/svelte/components/package.json",
      "packages/react/components/package.json",
    ]) {
      const manifest = JSON.parse(read(manifestPath)) as { dependencies?: Record<string, string> };
      for (const name of TIPTAP_SPECIFIERS) {
        const specifier = manifest.dependencies?.[name];
        expect(specifier, `${manifestPath} ${name}`).toMatch(/^\d+\.\d+\.\d+$/);
      }
      const raw = read(manifestPath);
      expect(raw).toContain('"./rich-text"');
      // The engine set is separate from the editor set: no cross-contamination.
      expect(manifest.dependencies?.["@codemirror/state"]).toMatch(/^\d+\.\d+\.\d+$/);
    }
  });

  it("the rich-text entry surface never names the engine", () => {
    for (const entry of [
      "packages/svelte/components/src/rich-text.ts",
      "packages/react/components/src/rich-text.ts",
    ]) {
      expect(read(entry)).not.toContain("@tiptap");
    }
  });

  it("the engine surface never exposes engine objects", () => {
    for (const source of RICH_TEXT_SOURCES) {
      const text = read(source);
      // Engine types stay private: the public engine surface returns
      // commands, snapshots, and documents only.
      expect(text).not.toMatch(/export (function|const) .*Editor( |:|\()/);
      expect(text).not.toMatch(/import .* from "prosemirror-/);
    }
  });

  it("the shared core stays engine-free", () => {
    const core = read("packages/core/src/rich-text.ts");
    expect(core).not.toContain("@tiptap");
    expect(core).not.toContain("prosemirror");
  });

  describe("compiled distribution", () => {
    const distFiles = [
      "packages/svelte/components/dist/index.client.js",
      "packages/svelte/components/dist/index.server.js",
      "packages/react/components/dist/index.js",
      "packages/svelte/components/dist/rich-text.client.js",
      "packages/svelte/components/dist/rich-text.server.js",
      "packages/react/components/dist/rich-text.js",
      "packages/svelte/components/dist/rich-text.d.ts",
      "packages/react/components/dist/rich-text.d.ts",
    ];
    const available = distFiles.every((file) => existsSync(file));

    it.skipIf(!available)("root bundles load no rich-text engine or TipTap module", () => {
      for (const root of [
        "packages/svelte/components/dist/index.client.js",
        "packages/svelte/components/dist/index.server.js",
        "packages/react/components/dist/index.js",
      ]) {
        const bundle = read(root);
        expect(bundle).not.toContain("tiptap");
        expect(bundle).not.toContain("rich-text-engine");
      }
    });

    it.skipIf(!available)("editor and markdown entries stay engine-free", () => {
      for (const entry of [
        "packages/svelte/components/dist/editor.client.js",
        "packages/svelte/components/dist/editor.server.js",
        "packages/react/components/dist/editor.js",
        "packages/svelte/components/dist/markdown.client.js",
        "packages/svelte/components/dist/markdown.server.js",
        "packages/react/components/dist/markdown.js",
      ]) {
        expect(read(entry)).not.toContain("tiptap");
      }
    });

    it.skipIf(!available)("rich-text bundles reach the engine only through external imports", () => {
      for (const entry of [
        "packages/svelte/components/dist/rich-text.client.js",
        "packages/react/components/dist/rich-text.js",
      ]) {
        expect(read(entry)).toContain("@tiptap/core");
      }
    });

    it.skipIf(!available)("rich-text declarations carry no engine types", () => {
      for (const declarations of [
        "packages/svelte/components/dist/rich-text.d.ts",
        "packages/react/components/dist/rich-text.d.ts",
      ]) {
        expect(read(declarations)).not.toContain("tiptap");
      }
    });
  });
});
