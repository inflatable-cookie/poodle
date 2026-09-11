import { existsSync } from "node:fs";
import { describe, expect, it } from "vitest";

import { createCodeEditorLanguageRegistry as createSvelteRegistry } from "@inflatable-cookie/poodle-svelte/editor/codemirror";
import { createCodeEditorLanguageRegistry as createReactRegistry } from "@inflatable-cookie/poodle-react/editor/codemirror";

/**
 * g18.012: language support is consumer-owned.
 *
 * This fixture runs inside the isolated packed-install consumer. That consumer
 * declares exactly one chosen grammar package (`@codemirror/lang-json`), so:
 *   - both shipped adapter subpaths must construct working registries,
 *   - the chosen grammar must be installed and must load through the adapter,
 *   - every unselected grammar must be absent from the install graph — it is
 *     not a Poodle dependency, so nothing pulls it in.
 */
describe("packed editor language registry (g18.012)", () => {
  it("constructs registries through both shipped adapter subpaths", async () => {
    for (const create of [createSvelteRegistry, createReactRegistry]) {
      const registry = create({
        json: () => import("@codemirror/lang-json").then((module) => module.json()),
      });
      expect(registry.has("json")).toBe(true);
      expect(registry.has("cobol")).toBe(false);
      const support = await registry.load("json");
      expect(support).toBeTruthy();
      expect(typeof (support as { extension?: unknown }).extension).toBe("object");
    }
  });

  it("refuses unknown ids and malformed registries from the installed packages", async () => {
    const registry = createSvelteRegistry({});
    await expect(registry.load("cobol")).rejects.toThrow(/unsupported language/);
    expect(() => createReactRegistry({ "plain-text": () => Promise.reject(new Error("x")) })).toThrow(
      /built in/,
    );
  });

  it("the consumer's chosen grammar is installed; unselected grammars are absent", async () => {
    // Chosen: the consumer manifest declares exactly this grammar package.
    expect(existsSync("node_modules/@codemirror/lang-json/package.json")).toBe(true);
    // Chosen grammar's substrate reaches the install through its own deps.
    expect(existsSync("node_modules/@codemirror/language/package.json")).toBe(true);
    // Unselected: Poodle no longer depends on any grammar package, so these
    // must not enter the isolated install graph.
    for (const unselected of [
      "@codemirror/lang-css",
      "@codemirror/lang-html",
      "@codemirror/lang-javascript",
      "@codemirror/lang-markdown",
      "@codemirror/lang-rust",
      "@codemirror/lang-yaml",
      "@codemirror/legacy-modes",
    ]) {
      expect(existsSync(`node_modules/${unselected}/package.json`)).toBe(false);
      await expect(import(/* @vite-ignore */ unselected)).rejects.toThrow();
    }
  });
});
