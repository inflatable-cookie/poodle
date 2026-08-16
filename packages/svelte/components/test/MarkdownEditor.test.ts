import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import MarkdownEditor from "../src/MarkdownEditor.svelte";

describe("MarkdownEditor (svelte)", () => {
  it("renders only the textarea in edit mode", () => {
    const { container } = render(MarkdownEditor, { props: { mode: "edit" } });
    expect(container.querySelector("textarea.poodle-md-editor__textarea")).not.toBeNull();
    expect(container.querySelector(".poodle-md-editor__preview")).toBeNull();
    const body = container.querySelector(".poodle-md-editor__body") as HTMLElement;
    expect(body.dataset.mode).toBe("edit");
  });

  it("renders only the preview pane in preview mode and disables the tools", () => {
    const { container } = render(MarkdownEditor, {
      props: { mode: "preview", value: "# Hello" },
    });
    expect(container.querySelector("textarea.poodle-md-editor__textarea")).toBeNull();
    expect(container.querySelector(".poodle-md-editor__preview")).not.toBeNull();
    const bold = container.querySelector('button[aria-label="Bold"]') as HTMLButtonElement;
    expect(bold.disabled).toBe(true);
  });

  it("renders both panes in split mode", () => {
    const { container } = render(MarkdownEditor, {
      props: { mode: "split", value: "**hi**" },
    });
    expect(container.querySelector("textarea.poodle-md-editor__textarea")).not.toBeNull();
    expect(container.querySelector(".poodle-md-editor__preview")).not.toBeNull();
  });

  it("renders the preview from the custom renderer", () => {
    const { container } = render(MarkdownEditor, {
      props: {
        mode: "preview",
        value: "# Hello",
        renderHtml: (markdown) => `<h1>${markdown.replace("# ", "")}</h1>`,
      },
    });
    const preview = container.querySelector(".poodle-md-editor__preview") as HTMLElement;
    expect(preview.querySelector("h1")?.textContent).toBe("Hello");
  });

  it("shows the empty preview placeholder when there is no content", () => {
    const { container } = render(MarkdownEditor, { props: { mode: "preview", value: "" } });
    expect(container.querySelector(".poodle-md-editor__preview-empty")?.textContent).toBe(
      "Nothing to preview",
    );
  });

  it("reports value changes while the host owns the value", async () => {
    const onValueChange = vi.fn();
    const { container } = render(MarkdownEditor, {
      props: { value: "", onValueChange },
    });
    const textarea = container.querySelector("textarea") as HTMLTextAreaElement;
    await fireEvent.input(textarea, { target: { value: "hello" } });
    expect(onValueChange).toHaveBeenCalledWith("hello");
  });

  it("labels the textarea with the accessible label", () => {
    const { container } = render(MarkdownEditor, { props: { ariaLabel: "Notes" } });
    const textarea = container.querySelector("textarea") as HTMLTextAreaElement;
    expect(textarea.getAttribute("aria-label")).toBe("Notes");
  });

  it("wraps selected text with markdown syntax from the Bold tool", async () => {
    const { container } = render(MarkdownEditor, { props: { value: "hello" } });
    const textarea = container.querySelector("textarea") as HTMLTextAreaElement;
    textarea.focus();
    textarea.setSelectionRange(0, 5);
    const bold = container.querySelector('button[aria-label="Bold"]') as HTMLButtonElement;
    await fireEvent.click(bold);
    expect(textarea.value).toBe("**hello**");
  });
});