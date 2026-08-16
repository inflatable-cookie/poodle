import { fireEvent, render, waitFor } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import EditableLabel from "../src/EditableLabel.svelte";

describe("EditableLabel (svelte)", () => {
  it("enters edit mode on double-click seeded with the value and cancels back on Escape", async () => {
    const onEditStart = vi.fn();
    const onCancel = vi.fn();
    const onCommit = vi.fn();
    const { container } = render(EditableLabel, {
      props: { value: "My project title", onEditStart, onCancel, onCommit },
    });
    const display = container.querySelector<HTMLButtonElement>(".poodle-editable-label__display") as HTMLButtonElement;
    expect(container.querySelector(".poodle-editable-label__text")?.textContent).toBe("My project title");

    await fireEvent.dblClick(display);
    await waitFor(() => {
      const input = container.querySelector<HTMLInputElement>(".poodle-editable-label__input");
      expect(input).not.toBeNull();
      expect(input?.value).toBe("My project title");
      expect(document.activeElement).toBe(input);
    });
    expect(onEditStart).toHaveBeenCalledTimes(1);

    const input = container.querySelector<HTMLInputElement>(".poodle-editable-label__input") as HTMLInputElement;
    await fireEvent.input(input, { target: { value: "Renamed" } });
    await fireEvent.keyDown(input, { key: "Escape" });

    expect(onCancel).toHaveBeenCalledTimes(1);
    expect(onCommit).not.toHaveBeenCalled();
    expect(container.querySelector(".poodle-editable-label__input")).toBeNull();
    expect(container.querySelector(".poodle-editable-label__text")?.textContent).toBe("My project title");
  });

  it("commits on Enter and on blur with the trimmed draft and previous value", async () => {
    const onCommit = vi.fn();
    const enter = render(EditableLabel, {
      props: { value: "Old title", onCommit },
    });
    const enterDisplay = enter.container.querySelector<HTMLButtonElement>(
      ".poodle-editable-label__display",
    ) as HTMLButtonElement;
    await fireEvent.dblClick(enterDisplay);
    await waitFor(() => {
      expect(enter.container.querySelector(".poodle-editable-label__input")).not.toBeNull();
    });
    const enterInput = enter.container.querySelector<HTMLInputElement>(".poodle-editable-label__input") as HTMLInputElement;
    await fireEvent.input(enterInput, { target: { value: "  New title  " } });
    await fireEvent.keyDown(enterInput, { key: "Enter" });
    expect(onCommit).toHaveBeenCalledWith({ value: "New title", previousValue: "Old title" });

    const blur = render(EditableLabel, {
      props: { value: "Old title", onCommit },
    });
    const blurDisplay = blur.container.querySelector<HTMLButtonElement>(
      ".poodle-editable-label__display",
    ) as HTMLButtonElement;
    await fireEvent.dblClick(blurDisplay);
    await waitFor(() => {
      expect(blur.container.querySelector(".poodle-editable-label__input")).not.toBeNull();
    });
    const blurInput = blur.container.querySelector<HTMLInputElement>(".poodle-editable-label__input") as HTMLInputElement;
    await fireEvent.input(blurInput, { target: { value: "Blurred" } });
    await fireEvent.blur(blurInput);
    expect(onCommit).toHaveBeenCalledWith({ value: "Blurred", previousValue: "Old title" });
    expect(blur.container.querySelector(".poodle-editable-label__input")).toBeNull();
  });

  it("activates via click, Enter, or Space in enterOrSpace mode", async () => {
    const click = render(EditableLabel, {
      props: { value: "Track 01", activationMode: "enterOrSpace" },
    });
    await fireEvent.click(click.container.querySelector(".poodle-editable-label__display") as HTMLElement);
    expect(click.container.querySelector(".poodle-editable-label__input")).not.toBeNull();

    const enter = render(EditableLabel, {
      props: { value: "Track 02", activationMode: "enterOrSpace" },
    });
    await fireEvent.keyDown(enter.container.querySelector(".poodle-editable-label__display") as HTMLElement, {
      key: "Enter",
    });
    expect(enter.container.querySelector(".poodle-editable-label__input")).not.toBeNull();

    const space = render(EditableLabel, {
      props: { value: "Track 03", activationMode: "enterOrSpace" },
    });
    await fireEvent.keyDown(space.container.querySelector(".poodle-editable-label__display") as HTMLElement, {
      key: " ",
    });
    expect(space.container.querySelector(".poodle-editable-label__input")).not.toBeNull();
  });

  it("stays in view mode under programmatic activation", async () => {
    const { container } = render(EditableLabel, {
      props: { value: "Static", activationMode: "programmatic" },
    });
    const display = container.querySelector<HTMLElement>(".poodle-editable-label__display") as HTMLElement;
    await fireEvent.click(display);
    await fireEvent.dblClick(display);
    await fireEvent.keyDown(display, { key: "Enter" });
    expect(container.querySelector(".poodle-editable-label__input")).toBeNull();
  });

  it("blocks edit entry while disabled", async () => {
    const onEditStart = vi.fn();
    const { container } = render(EditableLabel, {
      props: { value: "Read-only", disabled: true, onEditStart },
    });
    const display = container.querySelector<HTMLButtonElement>(".poodle-editable-label__display") as HTMLButtonElement;
    expect(display.disabled).toBe(true);

    await fireEvent.click(display);
    await fireEvent.dblClick(display);
    expect(container.querySelector(".poodle-editable-label__input")).toBeNull();
    expect(onEditStart).not.toHaveBeenCalled();
  });

  it("shows the empty state text when the value is empty", () => {
    const { container } = render(EditableLabel, {
      props: { value: "", emptyText: "Add a description…" },
    });
    const display = container.querySelector<HTMLElement>(".poodle-editable-label__display");
    expect(display?.classList.contains("poodle-editable-label__display--empty")).toBe(true);
    expect(display?.querySelector(".poodle-editable-label__text")?.textContent).toBe("Add a description…");
    expect(container.querySelector(".poodle-editable-label__icon")).toBeNull();

    const withIcon = render(EditableLabel, {
      props: { value: "", emptyText: "Add a description…", showEditIcon: true },
    });
    expect(withIcon.container.querySelector(".poodle-editable-label__icon")).not.toBeNull();
  });
});
