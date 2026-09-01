import { fireEvent, render } from "@testing-library/react";
import { readFileSync } from "node:fs";
import { describe, expect, it, vi } from "vitest";

import { MarkdownEditor } from "../src/MarkdownEditor";

// Vitest stubs CSS imports; inject the shared sheet before computed-style proof
// (same pattern as SettingsShell / AppHeader).
const markdownEditorCss = readFileSync(
  new URL("../../../core/src/styles/markdown-editor.css", `file://${import.meta.dirname}/`),
  "utf8",
);

function injectStyles(): void {
  const style = document.createElement("style");
  style.textContent = markdownEditorCss;
  document.head.appendChild(style);
}

function isZeroLength(value: string): boolean {
  return value === "0" || value === "0px";
}

describe("MarkdownEditor (react)", () => {
  it("renders only the textarea in edit mode", () => {
    const { container } = render(<MarkdownEditor mode="edit" />);
    expect(container.querySelector("textarea.poodle-md-editor__textarea")).not.toBeNull();
    expect(container.querySelector(".poodle-md-editor__preview")).toBeNull();
    const body = container.querySelector(".poodle-md-editor__body") as HTMLElement;
    expect(body.dataset.mode).toBe("edit");
  });

  it("renders only the preview pane in preview mode and disables the tools", () => {
    const { container } = render(<MarkdownEditor mode="preview" value="# Hello" />);
    expect(container.querySelector("textarea.poodle-md-editor__textarea")).toBeNull();
    expect(container.querySelector(".poodle-md-editor__preview")).not.toBeNull();
    const bold = container.querySelector('button[aria-label="Bold"]') as HTMLButtonElement;
    expect(bold.disabled).toBe(true);
  });

  it("renders both panes in split mode", () => {
    const { container } = render(<MarkdownEditor mode="split" value="**hi**" />);
    expect(container.querySelector("textarea.poodle-md-editor__textarea")).not.toBeNull();
    expect(container.querySelector(".poodle-md-editor__preview")).not.toBeNull();
  });

  it("renders the preview from the custom renderer", () => {
    const { container } = render(
      <MarkdownEditor
        mode="preview"
        value="# Hello"
        renderHtml={(markdown) => `<h1>${markdown.replace("# ", "")}</h1>`}
      />,
    );
    const preview = container.querySelector(".poodle-md-editor__preview") as HTMLElement;
    expect(preview.querySelector("h1")?.textContent).toBe("Hello");
  });

  it("shows the empty preview placeholder when there is no content", () => {
    const { container } = render(<MarkdownEditor mode="preview" value="" />);
    expect(container.querySelector(".poodle-md-editor__preview-empty")?.textContent).toBe(
      "Nothing to preview",
    );
  });

  it("reports value changes while the host owns the value", () => {
    const onValueChange = vi.fn();
    const { container } = render(<MarkdownEditor value="" onValueChange={onValueChange} />);
    const textarea = container.querySelector("textarea") as HTMLTextAreaElement;
    fireEvent.input(textarea, { target: { value: "hello" } });
    expect(onValueChange).toHaveBeenCalledWith("hello");
  });

  it("labels the textarea with the accessible label", () => {
    const { container } = render(<MarkdownEditor ariaLabel="Notes" />);
    const textarea = container.querySelector("textarea") as HTMLTextAreaElement;
    expect(textarea.getAttribute("aria-label")).toBe("Notes");
  });

  it("wraps selected text with markdown syntax from the Bold tool", () => {
    const { container } = render(<MarkdownEditor value="hello" />);
    const textarea = container.querySelector("textarea") as HTMLTextAreaElement;
    textarea.focus();
    textarea.setSelectionRange(0, 5);
    const bold = container.querySelector('button[aria-label="Bold"]') as HTMLButtonElement;
    fireEvent.click(bold);
    expect(textarea.value).toBe("**hello**");
  });

  it("keeps the preview pane as the vertical scroll owner in the shared stylesheet", () => {
    injectStyles();
    const { container } = render(<MarkdownEditor mode="preview" value="# Hello" />);
    const root = container.querySelector(".poodle-md-editor") as HTMLElement;
    const body = container.querySelector(".poodle-md-editor__body") as HTMLElement;
    const preview = container.querySelector(".poodle-md-editor__preview") as HTMLElement;
    const rootStyle = getComputedStyle(root);
    const bodyStyle = getComputedStyle(body);
    const previewStyle = getComputedStyle(preview);
    expect(rootStyle.display).toBe("flex");
    expect(rootStyle.flexDirection).toBe("column");
    expect(rootStyle.maxHeight).toBe("100%");
    expect(isZeroLength(bodyStyle.minHeight)).toBe(true);
    expect(previewStyle.overflowY).toBe("auto");
    expect(isZeroLength(previewStyle.minHeight)).toBe(true);
  });
});
