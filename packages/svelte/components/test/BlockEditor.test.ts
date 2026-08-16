import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import BlockEditor from "../src/BlockEditor.svelte";
import type { BlockTypeDefinition, EditorBlock } from "../src/types";

const blockTypes: BlockTypeDefinition[] = [
  { type: "text", label: "Text", icon: "align-left" },
  { type: "callout", label: "Callout", icon: "info" },
];

const blocks: EditorBlock[] = [
  { id: "b1", type: "text", content: "First" },
  { id: "b2", type: "callout", content: "Second" },
];

describe("BlockEditor (svelte)", () => {
  it("labels the root and renders each block as a labelled group", () => {
    const { container } = render(BlockEditor, {
      props: { blocks, blockTypes, ariaLabel: "Page blocks" },
    });
    const root = container.querySelector(".poodle-block-editor") as HTMLElement;
    expect(root.getAttribute("aria-label")).toBe("Page blocks");

    const groups = [...container.querySelectorAll(".poodle-block-editor__block")];
    expect(groups.length).toBe(2);
    expect(groups[0].getAttribute("aria-label")).toBe("text block");
    expect(groups[0].dataset.type).toBe("text");
  });

  it("falls back to a textarea renderer when no block snippet is given", () => {
    const { container } = render(BlockEditor, { props: { blocks, blockTypes } });
    const textareas = [...container.querySelectorAll("textarea.poodle-block-editor__input")];
    expect(textareas.length).toBe(2);
    expect(textareas[0].value).toBe("First");
  });

  it("emits the updated blocks when the fallback textarea changes", async () => {
    const onChange = vi.fn();
    const { container } = render(BlockEditor, { props: { blocks, blockTypes, onChange } });
    const textarea = container.querySelector("textarea.poodle-block-editor__input") as HTMLTextAreaElement;
    await fireEvent.input(textarea, { target: { value: "Edited" } });
    expect(onChange).toHaveBeenCalledTimes(1);
    const next = onChange.mock.calls[0][0] as EditorBlock[];
    expect(next[0].content).toBe("Edited");
    expect(next[1]).toEqual(blocks[1]);
  });

  it("moves a block down and up with boundary guards", async () => {
    const onChange = vi.fn();
    const { container } = render(BlockEditor, { props: { blocks, blockTypes, onChange } });

    const moveUpFirst = [...container.querySelectorAll('button[aria-label="Move up"]')][0];
    expect(moveUpFirst.disabled).toBe(true);

    const moveDownFirst = [...container.querySelectorAll('button[aria-label="Move down"]')][0] as HTMLButtonElement;
    await fireEvent.click(moveDownFirst);
    expect(onChange).toHaveBeenCalledTimes(1);
    const afterMove = onChange.mock.calls[0][0] as EditorBlock[];
    expect(afterMove.map((block) => block.id)).toEqual(["b2", "b1"]);

    const moveDownLast = [...container.querySelectorAll('button[aria-label="Move down"]')][1];
    expect(moveDownLast.disabled).toBe(true);
  });

  it("removes a block and hides the remove control when only one remains", async () => {
    const onChange = vi.fn();
    const { container } = render(BlockEditor, { props: { blocks, blockTypes, onChange } });
    const removeButtons = [...container.querySelectorAll('button[aria-label="Remove block"]')];
    expect(removeButtons.length).toBe(2);

    await fireEvent.click(removeButtons[0]);
    expect(onChange).toHaveBeenCalledTimes(1);
    const afterRemove = onChange.mock.calls[0][0] as EditorBlock[];
    expect(afterRemove.map((block) => block.id)).toEqual(["b2"]);

    const single = render(BlockEditor, {
      props: { blocks: [{ id: "only", type: "text", content: "" }], blockTypes },
    });
    expect(single.container.querySelector('button[aria-label="Remove block"]')).toBeNull();
  });

  it("disables editing controls when disabled", () => {
    const { container } = render(BlockEditor, {
      props: { blocks, blockTypes, disabled: true },
    });
    const root = container.querySelector(".poodle-block-editor") as HTMLElement;
    expect(root.classList.contains("poodle-block-editor--disabled")).toBe(true);
    const textarea = container.querySelector("textarea") as HTMLTextAreaElement;
    expect(textarea.disabled).toBe(true);
  });

  it("hides reorder, add, and remove controls in single mode", () => {
    const { container } = render(BlockEditor, {
      props: { blocks, blockTypes, mode: "single" },
    });
    const grip = container.querySelector(".poodle-block-editor__drag-grip") as HTMLElement;
    expect(grip.hidden).toBe(true);
    expect(container.querySelector('button[aria-label="Move up"]')).toBeNull();
    expect(container.querySelector('button[aria-label="Remove block"]')).toBeNull();
  });
});