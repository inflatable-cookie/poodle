import { fireEvent, render, waitFor } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import CodeEditorSpecimen from "../src/specimens/CodeEditorSpecimen.svelte";

/**
 * g18.016: the Configuration specimen's line-numbers toggle must reconfigure
 * the mounted CodeEditor live. Button state and visible gutter must agree
 * after every click, with no remount and no lost editor state.
 */
describe("g18.016 CodeEditor specimen line-number toggle (svelte)", () => {
  function configEditor(container: HTMLElement): HTMLElement {
    const editor = container.querySelector<HTMLElement>("[data-part='config-editor']");
    if (!editor) throw new Error("config editor did not mount");
    return editor;
  }

  function lineGutter(container: HTMLElement): Element | null {
    return configEditor(container).querySelector(".cm-gutters .cm-lineNumbers");
  }

  function toggle(container: HTMLElement): HTMLButtonElement {
    const button = container.querySelector<HTMLButtonElement>(
      "[data-part='line-numbers-toggle']",
    );
    if (!button) throw new Error("line-numbers toggle did not mount");
    return button;
  }

  async function mounted() {
    const { container } = render(CodeEditorSpecimen);
    await waitFor(() => {
      expect(lineGutter(container)).not.toBeNull();
    });
    return container;
  }

  it("the toggle button and the visible gutter agree after each click", async () => {
    const container = await mounted();
    expect(toggle(container).getAttribute("aria-pressed")).toBe("true");

    await fireEvent.click(toggle(container));
    await waitFor(() => {
      expect(lineGutter(container)).toBeNull();
    });
    expect(toggle(container).getAttribute("aria-pressed")).toBe("false");

    await fireEvent.click(toggle(container));
    await waitFor(() => {
      expect(lineGutter(container)).not.toBeNull();
    });
    expect(toggle(container).getAttribute("aria-pressed")).toBe("true");
  });

  it("reconfiguration keeps the mounted editor and its document", async () => {
    const container = await mounted();
    const editor = configEditor(container).querySelector(".cm-editor");
    expect(editor).not.toBeNull();
    await fireEvent.click(toggle(container));
    await waitFor(() => {
      expect(lineGutter(container)).toBeNull();
    });
    expect(configEditor(container).querySelector(".cm-editor")).toBe(editor);
    await fireEvent.click(toggle(container));
    await waitFor(() => {
      expect(lineGutter(container)).not.toBeNull();
    });
    expect(configEditor(container).querySelector(".cm-editor")).toBe(editor);
    const lines = [...configEditor(container).querySelectorAll(".cm-line")].map(
      (line) => line.textContent ?? "",
    );
    expect(lines.join("\n")).toContain("export function distance");
  });
});
