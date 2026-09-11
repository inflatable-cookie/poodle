import { readFileSync } from "node:fs";
import { join } from "node:path";
import { EditorState } from "@codemirror/state";
import { syntaxTree } from "@codemirror/language";
import { fireEvent, render, waitFor } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import { createCodeEditorLanguageRegistry } from "@inflatable-cookie/poodle-svelte/editor/codemirror";
import { CodeEditor } from "@inflatable-cookie/poodle-svelte/editor";
import CodeEditorSpecimen from "../src/specimens/CodeEditorSpecimen.svelte";

const PREVIEW_ROOT = join(import.meta.dirname, "..");

function click(target: Element): void {
  target.dispatchEvent(new MouseEvent("click", { bubbles: true, cancelable: true }));
}

/**
 * g18.012: language support is consumer-owned. The specimen registers exactly
 * two consumer-selected languages plus built-in plain text, and this preview
 * package — not Poodle — installs the grammar packages those loaders name.
 */
describe("g18.012 CodeEditor extensible language registry (svelte)", () => {
  it("the specimen imports the adapter subpath and names its own grammars", () => {
    const source = readFileSync(
      join(PREVIEW_ROOT, "src/specimens/CodeEditorSpecimen.svelte"),
      "utf8",
    );
    expect(source).toContain('from "@inflatable-cookie/poodle-svelte/editor/codemirror"');
    expect(source).toContain('@codemirror/lang-javascript');
    expect(source).toContain('@codemirror/lang-json');
    expect(source).not.toContain("code-editor-languages");
  });

  it("switches plain text / typescript / json live without losing the editor or value", async () => {
    const { container } = render(CodeEditorSpecimen);
    const config = () => container.querySelector<HTMLElement>("[data-part='config-editor']");
    const editor = () => config()?.querySelector(".cm-editor") ?? null;
    const lines = () =>
      [...(config()?.querySelectorAll(".cm-line") ?? [])].map((line) => line.textContent ?? "");

    await waitFor(() => {
      expect(editor()).not.toBeNull();
    });
    const mounted = editor();
    expect(lines().join("\n")).toContain("export function distance");

    // TypeScript -> JSON: a different consumer-selected grammar, same editor.
    click(container.querySelector<HTMLButtonElement>("[data-part='language-json']")!);
    await waitFor(() => {
      expect(lines().join("\n")).toContain('"editor"');
    });
    expect(editor()).toBe(mounted);

    // JSON -> plain text: built in, no registry entry, same editor and value.
    click(container.querySelector<HTMLButtonElement>("[data-part='language-plain-text']")!);
    await waitFor(() => {
      expect(lines().join("\n")).toContain("export function distance");
    });
    expect(editor()).toBe(mounted);
    expect(
      container.querySelector<HTMLButtonElement>("[data-part='language-plain-text']")
        ?.getAttribute("aria-pressed"),
    ).toBe("true");
  });

  it("parses a consumer-selected grammar for real through the registry", async () => {
    let loads = 0;
    const registry = createCodeEditorLanguageRegistry({
      json: () => {
        loads += 1;
        return import("@codemirror/lang-json").then((module) => module.json());
      },
    });
    expect(loads).toBe(0);
    const support = await registry.load("json");
    expect(loads).toBe(1);
    const state = EditorState.create({
      doc: '{"ok": true}',
      extensions: [support as never],
    });
    const names: string[] = [];
    const cursor = syntaxTree(state).cursor();
    do {
      names.push(cursor.name);
    } while (cursor.next());
    expect(names).toContain("JsonText");
    // The memoized load is reused on re-selection.
    await registry.load("json");
    expect(loads).toBe(1);
  });

  it("a missing id and a failed load refuse instead of falling back", async () => {
    const registry = createCodeEditorLanguageRegistry({
      broken: () => Promise.reject(new Error("grammar exploded")),
    });
    await expect(registry.load("cobol")).rejects.toThrow(/unsupported language "cobol"/);
    await expect(registry.load("broken")).rejects.toThrow("grammar exploded");

    // Mounted refusal: the component throws rather than silently presenting
    // plain text for a language the host asked for.
    let threw = false;
    try {
      render(CodeEditor, {
        props: { value: "one", language: "cobol", languageRegistry: registry },
      });
    } catch (error) {
      threw = /unsupported language "cobol"/.test(String(error));
    }
    expect(threw).toBe(true);
  });
});
