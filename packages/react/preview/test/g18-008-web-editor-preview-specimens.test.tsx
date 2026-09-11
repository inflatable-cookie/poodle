import { readFileSync } from "node:fs";
import { join } from "node:path";
import { act, fireEvent, render, waitFor } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { allComponents } from "../src/gallery/registry";
import { specimenMap } from "../src/gallery/specimen-map";
import { CodeEditorSpecimen } from "../src/gallery/specimens/CodeEditorSpecimen";
import { RichTextEditorSpecimen } from "../src/gallery/specimens/RichTextEditorSpecimen";
import { RichTextRendererSpecimen } from "../src/gallery/specimens/RichTextRendererSpecimen";
import {
  CODE_TYPESCRIPT_SOURCE,
  RICH_TEXT_IMAGE_ALT,
  RICH_TEXT_STANDARD_DOCUMENT,
} from "../../../svelte/preview/src/specimens/web-editor-documents";

const PREVIEW_ROOT = join(import.meta.dirname, "..");
const EDITOR_SLUGS = ["code-editor", "rich-text-editor", "rich-text-renderer"] as const;

function press(target: Element, key: string): boolean {
  return target.dispatchEvent(
    new KeyboardEvent("keydown", { bubbles: true, cancelable: true, key }),
  );
}

describe("g18.008 web editor preview specimens (react)", () => {
  it("registers the three public editor surfaces with matching slugs", () => {
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
      ["CodeEditorSpecimen.tsx", "@inflatable-cookie/poodle-react/editor"],
      ["RichTextEditorSpecimen.tsx", "@inflatable-cookie/poodle-react/rich-text"],
      ["RichTextRendererSpecimen.tsx", "@inflatable-cookie/poodle-react/rich-text"],
    ] as const;
    for (const [file, entry] of files) {
      const source = readFileSync(join(PREVIEW_ROOT, "src/gallery/specimens", file), "utf8");
      expect(source).toContain(`from "${entry}"`);
      expect(source).not.toContain("from \"./code-editor-engine\"");
      expect(source).not.toContain("from \"./rich-text-engine\"");
    }
  });

  it("updates host-controlled CodeEditor state from a user edit", async () => {
    const { container } = render(<CodeEditorSpecimen />);
    const live = container.querySelector("[data-part='live-editor']");
    expect(live).not.toBeNull();
    await waitFor(() => {
      expect(container.querySelectorAll(".cm-content").length).toBe(4);
    });
    await act(async () => {
      press(live!.querySelector(".cm-content")!, "Enter");
    });
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
    const { container } = render(<RichTextEditorSpecimen />);
    const live = container.querySelector("[data-part='live-editor']");
    await waitFor(() => {
      expect(live?.querySelector(".ProseMirror")).not.toBeNull();
    });
    const before = container.querySelector("[data-part='host-document']")?.textContent ?? "";
    expect(before).toContain('"table"');
    const insert = live!.querySelector<HTMLButtonElement>('[data-command="insert-table"] button');
    expect(insert).not.toBeNull();
    fireEvent.click(insert!);
    await waitFor(() => {
      expect(container.querySelector("[data-part='host-document']")?.textContent).not.toBe(before);
    });
    const after = container.querySelector("[data-part='host-document']")?.textContent ?? "";
    expect([...after.matchAll(/"type":"table"/g)].length).toBeGreaterThan(
      [...before.matchAll(/"type":"table"/g)].length,
    );
  });

  it("keeps images off until the project opts in", async () => {
    const { container } = render(<RichTextEditorSpecimen />);
    const policy = container.querySelector("[data-part='image-policy-editor']");
    await waitFor(() => {
      expect(policy?.querySelector(".ProseMirror")).not.toBeNull();
    });
    expect(policy?.querySelector('[data-command="insert-image"] button')).toBeNull();
    expect(policy?.querySelector("img")).toBeNull();
    fireEvent.click(container.querySelector("[data-part='images-toggle']")!);
    await waitFor(() => {
      expect(
        container.querySelector("[data-part='image-policy-editor'] img")?.getAttribute("alt"),
      ).toBe(RICH_TEXT_IMAGE_ALT);
    });
    expect(
      container.querySelector("[data-part='image-policy-editor'] [data-command='insert-image'] button"),
    ).not.toBeNull();
  });

  it("renders the same representative document without editable state", async () => {
    const { container } = render(<RichTextRendererSpecimen />);
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

  it("keeps the caret through the live specimen's immediate controlled echo", async () => {
    // The specimen echoes each onChange straight back through `value`: the
    // exact host journey that moved the caret to the document end. Typing is
    // driven entirely through the DOM, the way a browser and IME commit do.
    const { container } = render(<RichTextEditorSpecimen />);
    const live = container.querySelector("[data-part='live-editor']");
    await waitFor(() => {
      expect(live?.querySelector(".ProseMirror")).not.toBeNull();
    });
    const surface = live!.querySelector<HTMLElement>(".ProseMirror")!;
    await act(async () => {
      surface.focus();
    });
    const readout = () => container.querySelector("[data-part='host-document']")?.textContent ?? "";
    let caret = 5;
    let inserted = "";
    for (const character of ["X", "Y", "Z"]) {
      await act(async () => {
        surface.dispatchEvent(new CompositionEvent("compositionstart", { bubbles: true }));
        const textNode = surface.querySelector("h1")?.firstChild as Text;
        const text = textNode.nodeValue ?? "";
        textNode.nodeValue = `${text.slice(0, caret)}${character}${text.slice(caret)}`;
        const domSelection = document.getSelection();
        if (!domSelection) throw new Error("no DOM selection available");
        const range = document.createRange();
        range.setStart(textNode, caret + 1);
        range.setEnd(textNode, caret + 1);
        domSelection.removeAllRanges();
        domSelection.addRange(range);
        surface.dispatchEvent(
          new CompositionEvent("compositionend", { data: character, bubbles: true }),
        );
        // Let ProseMirror's composition-end flush and the controlled echo
        // settle before the next composed character.
        await new Promise((resolve) => setTimeout(resolve, 80));
      });
      caret += 1;
      inserted += character;
      // React flushes controlled state at the act boundary, so the host
      // document and caret are asserted after each committed character.
      expect(document.getSelection()?.focusOffset).toBe(caret);
      expect(readout()).toContain(`Relea${inserted}se notes`);
    }
    await waitFor(() => {
      expect(readout()).toContain("ReleaXYZse notes");
    });
    expect(surface.querySelector("h1")?.textContent).toBe("ReleaXYZse notes");
  });
});
