import { readFileSync } from "node:fs";
import { join } from "node:path";
import { fireEvent, render, waitFor } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import { allComponents, webOnlyComponents } from "../src/component-registry";
import { specimenMap } from "../src/specimens/registry";
import CodeEditorSpecimen from "../src/specimens/CodeEditorSpecimen.svelte";
import RichTextEditorSpecimen from "../src/specimens/RichTextEditorSpecimen.svelte";
import RichTextRendererSpecimen from "../src/specimens/RichTextRendererSpecimen.svelte";
import {
  CODE_TYPESCRIPT_SOURCE,
  RICH_TEXT_IMAGE_ALT,
  RICH_TEXT_STANDARD_DOCUMENT,
} from "../src/specimens/web-editor-documents";

const PREVIEW_ROOT = join(import.meta.dirname, "..");
const EDITOR_SLUGS = ["code-editor", "rich-text-editor", "rich-text-renderer"] as const;

function press(target: Element, key: string): boolean {
  return target.dispatchEvent(
    new KeyboardEvent("keydown", { bubbles: true, cancelable: true, key }),
  );
}

describe("g18.008 web editor preview specimens (svelte)", () => {
  it("registers the three public editor surfaces as web-only catalogue entries", () => {
    expect(webOnlyComponents.map((component) => component.slug)).toEqual([
      "meter-surface",
      ...EDITOR_SLUGS,
    ]);
    for (const slug of EDITOR_SLUGS) {
      const entry = allComponents.find((component) => component.slug === slug);
      expect(entry, slug).toBeDefined();
      expect(entry?.hasSpecimen).toBe(true);
      expect(specimenMap[slug]).toBeDefined();
      expect(`#components/${slug}`).toMatch(/^#components\/[a-z0-9-]+$/);
    }
  });

  it("mounts the public editor and rich-text subpath exports", () => {
    const files = [
      ["CodeEditorSpecimen.svelte", "@inflatable-cookie/poodle-svelte/editor"],
      ["RichTextEditorSpecimen.svelte", "@inflatable-cookie/poodle-svelte/rich-text"],
      ["RichTextRendererSpecimen.svelte", "@inflatable-cookie/poodle-svelte/rich-text"],
    ] as const;
    for (const [file, entry] of files) {
      const source = readFileSync(join(PREVIEW_ROOT, "src/specimens", file), "utf8");
      expect(source).toContain(`from "${entry}"`);
      expect(source).not.toContain("from \"./code-editor-engine\"");
      expect(source).not.toContain("from \"./rich-text-engine\"");
    }
  });

  it("updates host-controlled CodeEditor state from a user edit", async () => {
    const { container } = render(CodeEditorSpecimen);
    const live = container.querySelector("[data-part='live-editor']");
    expect(live).not.toBeNull();
    await waitFor(() => {
      expect(live?.querySelector(".cm-content")).not.toBeNull();
    });
    const content = live!.querySelector(".cm-content")!;
    press(content, "Enter");
    await waitFor(() => {
      expect(container.querySelector("[data-part='host-value']")?.textContent).not.toBe(
        CODE_TYPESCRIPT_SOURCE,
      );
    });
    expect(container.querySelector("[data-part='host-value']")?.textContent).toContain(
      "export function distance",
    );
    expect(container.querySelector("[data-part='read-only-editor'] .cm-content")).not.toBeNull();
    expect(
      container.querySelector("[data-part='diagnostics-editor'] [data-poodle-diagnostic='type-mismatch']"),
    ).not.toBeNull();
  });

  it("updates host-controlled ProseMirror JSON from a table action", async () => {
    const { container } = render(RichTextEditorSpecimen);
    const live = container.querySelector("[data-part='live-editor']");
    await waitFor(() => {
      expect(live?.querySelector(".ProseMirror")).not.toBeNull();
    });
    const before = container.querySelector("[data-part='host-document']")?.textContent ?? "";
    expect(before).toContain('"table"');
    const insert = live!.querySelector<HTMLButtonElement>('button[data-command="insert-table"]');
    expect(insert).not.toBeNull();
    await fireEvent.click(insert!);
    await waitFor(() => {
      expect(container.querySelector("[data-part='host-document']")?.textContent).not.toBe(before);
    });
    const after = container.querySelector("[data-part='host-document']")?.textContent ?? "";
    expect([...after.matchAll(/"type":"table"/g)].length).toBeGreaterThan(
      [...before.matchAll(/"type":"table"/g)].length,
    );
  });

  it("keeps images off until the project opts in", async () => {
    const { container } = render(RichTextEditorSpecimen);
    const policy = container.querySelector("[data-part='image-policy-editor']");
    await waitFor(() => {
      expect(policy?.querySelector(".ProseMirror")).not.toBeNull();
    });
    expect(policy?.querySelector('button[data-command="insert-image"]')).toBeNull();
    expect(policy?.querySelector("img")).toBeNull();
    await fireEvent.click(container.querySelector("[data-part='images-toggle']")!);
    await waitFor(() => {
      expect(
        container.querySelector("[data-part='image-policy-editor'] img")?.getAttribute("alt"),
      ).toBe(RICH_TEXT_IMAGE_ALT);
    });
    expect(
      container.querySelector("[data-part='image-policy-editor'] button[data-command='insert-image']"),
    ).not.toBeNull();
  });

  it("renders the same representative document without editable state", async () => {
    const { container } = render(RichTextRendererSpecimen);
    await waitFor(() => {
      expect(container.querySelector("[data-part='standard-renderer'] table")).not.toBeNull();
    });
    expect(container.querySelector("[contenteditable]")).toBeNull();
    expect(container.querySelector(".ProseMirror")).toBeNull();
    expect(container.querySelector("[data-part='standard-renderer'] h1")?.textContent).toBe(
      "Release notes",
    );
    expect(container.querySelector("[data-part='standard-renderer'] table th")?.textContent).toBe(
      "Surface",
    );
    expect(container.querySelector("[data-part='image-renderer'] img")?.getAttribute("alt")).toBe(
      RICH_TEXT_IMAGE_ALT,
    );
    expect(JSON.stringify(RICH_TEXT_STANDARD_DOCUMENT)).toContain("Release notes");
  });
});
