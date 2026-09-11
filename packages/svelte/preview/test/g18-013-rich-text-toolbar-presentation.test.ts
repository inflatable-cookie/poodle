import { render, waitFor } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import RichTextEditorSpecimen from "../src/specimens/RichTextEditorSpecimen.svelte";

const PLAIN_EDITORS = ["live-editor", "subset-editor", "disabled-editor"] as const;

describe("g18.013 rich text toolbar presentation (svelte specimen)", () => {
  it("renders the automatic toolbar as grouped Poodle icon-button controls", async () => {
    const { container } = render(RichTextEditorSpecimen);
    const live = container.querySelector("[data-part='live-editor']");
    await waitFor(() => {
      expect(live?.querySelector('[data-command="bold"] button')).not.toBeNull();
    });
    // Controls read as controls: real Poodle icon buttons with accessible
    // names, never a sentence of label-like buttons. Table-context commands
    // start disabled outside a table — truthful availability.
    for (const wrapper of live!.querySelectorAll("[data-command]")) {
      const command = wrapper.getAttribute("data-command") ?? "";
      if (command === "heading-select") {
        // g18.020: heading levels are one Poodle Select, not icon buttons.
        expect(wrapper.querySelector("button.poodle-select__trigger")).not.toBeNull();
        expect(wrapper.querySelector("button.poodle-icon-button")).toBeNull();
        continue;
      }
      const button = wrapper.querySelector("button.poodle-icon-button");
      expect(button, command).not.toBeNull();
      expect(button?.getAttribute("aria-label")).toBeTruthy();
    }
    // No separate heading buttons remain anywhere in the toolbar.
    for (const level of [1, 2, 3, 4, 5, 6]) {
      expect(live!.querySelector(`[data-command="heading-${level}"]`)).toBeNull();
    }
    const boldButton = live!.querySelector<HTMLButtonElement>('[data-command="bold"] button');
    expect(boldButton?.getAttribute("disabled")).toBeNull();
    const addRow = live!.querySelector<HTMLButtonElement>('[data-command="add-row"] button');
    expect(addRow?.getAttribute("disabled")).not.toBeNull();
    const groups = live!.querySelectorAll(".poodle-rich-text-editor__group");
    expect(groups.length).toBeGreaterThan(1);
    for (const group of groups) {
      expect(group.getAttribute("role")).toBe("group");
      expect(group.getAttribute("aria-label")).toBeTruthy();
    }
  });

  it("exposes an explicit toolbar subset that renders and operates exactly", async () => {
    const { container } = render(RichTextEditorSpecimen);
    const subset = container.querySelector("[data-part='subset-editor']");
    await waitFor(() => {
      expect(subset?.querySelector('[data-command="bold"] button')).not.toBeNull();
    });
    const commands = [...subset!.querySelectorAll("[data-command]")].map((wrapper) =>
      wrapper.getAttribute("data-command"),
    );
    expect(commands).toEqual(["bold", "italic", "link"]);
    // Operating the collapsed selection stores the mark: the pressed state
    // turns truthful even though no document change is emitted yet.
    const bold = subset!.querySelector<HTMLButtonElement>('[data-command="bold"] button');
    expect(bold?.getAttribute("aria-pressed")).toBe("false");
    bold?.click();
    await waitFor(() => {
      expect(bold?.getAttribute("aria-pressed")).toBe("true");
    });
  });

  it("shows the disabled editor removed from interaction with truthful controls", async () => {
    const { container } = render(RichTextEditorSpecimen);
    const disabled = container.querySelector("[data-part='disabled-editor']");
    await waitFor(() => {
      expect(disabled?.querySelector(".ProseMirror")).not.toBeNull();
    });
    expect(disabled?.querySelector(".poodle-rich-text-editor")?.getAttribute("data-disabled")).toBe(
      "true",
    );
    const controls = disabled!.querySelectorAll("button.poodle-icon-button");
    expect(controls.length).toBeGreaterThan(10);
    for (const button of controls) expect(button.getAttribute("disabled")).not.toBeNull();
  });

  it("keeps every frame's editor mounted with the toolbar contract intact", async () => {
    const { container } = render(RichTextEditorSpecimen);
    for (const part of PLAIN_EDITORS) {
      const frame = container.querySelector(`[data-part='${part}']`);
      await waitFor(() => {
        expect(frame?.querySelector(".ProseMirror")).not.toBeNull();
      });
      expect(frame?.querySelector(".poodle-rich-text-editor__toolbar")?.getAttribute("role")).toBe(
        "toolbar",
      );
    }
  });
});
