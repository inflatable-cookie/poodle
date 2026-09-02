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

  it("enters edit mode from Enter or Space in doubleClick mode", async () => {
    const enter = render(EditableLabel, { props: { value: "Kick" } });
    await fireEvent.keyDown(enter.container.querySelector(".poodle-editable-label__display") as HTMLElement, {
      key: "Enter",
    });
    expect(enter.container.querySelector(".poodle-editable-label__input")).not.toBeNull();

    const space = render(EditableLabel, { props: { value: "Kick" } });
    await fireEvent.keyDown(space.container.querySelector(".poodle-editable-label__display") as HTMLElement, {
      key: " ",
    });
    expect(space.container.querySelector(".poodle-editable-label__input")).not.toBeNull();
  });

  it("stays in view on a single click in the default doubleClick mode", async () => {
    const { container } = render(EditableLabel, { props: { value: "Kick" } });
    await fireEvent.click(container.querySelector(".poodle-editable-label__display") as HTMLElement);
    expect(container.querySelector(".poodle-editable-label__input")).toBeNull();
  });

  it("exposes focus, startEditing, and cancelEditing on the instance", async () => {
    const onEditStart = vi.fn();
    const onCancel = vi.fn();
    const onCommit = vi.fn();
    const { component, container } = render(EditableLabel, {
      props: { value: "Kick", onEditStart, onCancel, onCommit },
    });
    const methods = component as unknown as {
      focus: () => void;
      startEditing: () => void;
      cancelEditing: () => void;
    };

    methods.focus();
    expect(document.activeElement).toBe(container.querySelector(".poodle-editable-label__display"));

    methods.startEditing();
    await waitFor(() => {
      expect(container.querySelector(".poodle-editable-label__input")).not.toBeNull();
    });
    expect(onEditStart).toHaveBeenCalledTimes(1);
    expect(document.activeElement).toBe(container.querySelector(".poodle-editable-label__input"));

    methods.cancelEditing();
    await waitFor(() => {
      expect(container.querySelector(".poodle-editable-label__input")).toBeNull();
    });
    expect(onCancel).toHaveBeenCalledTimes(1);
    expect(onCommit).not.toHaveBeenCalled();
    expect(document.activeElement).toBe(container.querySelector(".poodle-editable-label__display"));
  });

  it("startEditing ignores programmatic activation and is inert when disabled", async () => {
    const programmatic = render(EditableLabel, {
      props: { value: "Kick", activationMode: "programmatic" },
    });
    (programmatic.component as unknown as { startEditing: () => void }).startEditing();
    await waitFor(() => {
      expect(programmatic.container.querySelector(".poodle-editable-label__input")).not.toBeNull();
    });

    const blocked = render(EditableLabel, { props: { value: "Kick", disabled: true } });
    (blocked.component as unknown as { startEditing: () => void }).startEditing();
    expect(blocked.container.querySelector(".poodle-editable-label__input")).toBeNull();
  });

  it("names display and editor from the visible value when ariaLabel is omitted", async () => {
    const { container } = render(EditableLabel, { props: { value: "Kick" } });
    expect(container.querySelector(".poodle-editable-label__display")?.getAttribute("aria-label")).toBe("Kick");
    await fireEvent.dblClick(container.querySelector(".poodle-editable-label__display") as HTMLElement);
    await waitFor(() => {
      expect(container.querySelector(".poodle-editable-label__input")?.getAttribute("aria-label")).toBe("Kick");
    });
  });

  it("clamps maxLength to Unicode scalar values, not UTF-16 units", async () => {
    const { container } = render(EditableLabel, { props: { value: "", maxLength: 1 } });
    await fireEvent.dblClick(container.querySelector(".poodle-editable-label__display") as HTMLElement);
    await waitFor(() => {
      expect(container.querySelector(".poodle-editable-label__input")).not.toBeNull();
    });
    const input = container.querySelector<HTMLInputElement>(".poodle-editable-label__input") as HTMLInputElement;
    expect(input.hasAttribute("maxlength")).toBe(false);
    await fireEvent.input(input, { target: { value: "𝄞" } });
    expect(input.value).toBe("𝄞");
    await fireEvent.input(input, { target: { value: "𝄞A" } });
    expect(input.value).toBe("𝄞");
  });

  it("commits portable NEL/BOM trim and ignores a later unmount blur", async () => {
    const onCommit = vi.fn();
    const onCancel = vi.fn();
    const { container, unmount } = render(EditableLabel, {
      props: { value: "Kick", onCommit, onCancel },
    });
    await fireEvent.dblClick(container.querySelector(".poodle-editable-label__display") as HTMLElement);
    await waitFor(() => {
      expect(container.querySelector(".poodle-editable-label__input")).not.toBeNull();
    });
    const input = container.querySelector<HTMLInputElement>(".poodle-editable-label__input") as HTMLInputElement;
    await fireEvent.input(input, { target: { value: "\u0085Take\uFEFF" } });
    await fireEvent.keyDown(input, { key: "Enter" });
    expect(onCommit).toHaveBeenCalledWith({ value: "Take", previousValue: "Kick" });
    unmount();
    await Promise.resolve();
    expect(onCommit).toHaveBeenCalledTimes(1);
    expect(onCancel).not.toHaveBeenCalled();
  });

  it("emits neither commit nor cancel when unmounted while editing", async () => {
    const onCommit = vi.fn();
    const onCancel = vi.fn();
    const { container, unmount } = render(EditableLabel, {
      props: { value: "Kick", onCommit, onCancel },
    });
    await fireEvent.dblClick(container.querySelector(".poodle-editable-label__display") as HTMLElement);
    await waitFor(() => {
      expect(container.querySelector(".poodle-editable-label__input")).not.toBeNull();
    });
    await fireEvent.input(container.querySelector(".poodle-editable-label__input") as HTMLInputElement, {
      target: { value: "Kicks" },
    });
    unmount();
    await Promise.resolve();
    expect(onCommit).not.toHaveBeenCalled();
    expect(onCancel).not.toHaveBeenCalled();
  });

  it("commits once on window blur without restoring display focus", async () => {
    const onCommit = vi.fn();
    const { container } = render(EditableLabel, { props: { value: "Kick", onCommit } });
    const display = container.querySelector<HTMLButtonElement>(".poodle-editable-label__display") as HTMLButtonElement;
    await fireEvent.dblClick(display);
    await waitFor(() => {
      expect(container.querySelector(".poodle-editable-label__input")).not.toBeNull();
    });
    await fireEvent.input(container.querySelector(".poodle-editable-label__input") as HTMLInputElement, {
      target: { value: "Kicks" },
    });
    window.dispatchEvent(new Event("blur"));
    await waitFor(() => {
      expect(onCommit).toHaveBeenCalledWith({ value: "Kicks", previousValue: "Kick" });
    });
    expect(document.activeElement).not.toBe(container.querySelector(".poodle-editable-label__display"));
  });

  it("restores display focus on Enter and Escape, not on Tab blur", async () => {
    const onEnterCommit = vi.fn();
    const enter = render(EditableLabel, { props: { value: "Kick", onCommit: onEnterCommit } });
    await fireEvent.dblClick(enter.container.querySelector(".poodle-editable-label__display") as HTMLElement);
    await waitFor(() => {
      expect(enter.container.querySelector(".poodle-editable-label__input")).not.toBeNull();
    });
    await fireEvent.keyDown(enter.container.querySelector(".poodle-editable-label__input") as HTMLInputElement, {
      key: "Enter",
    });
    await waitFor(() => {
      expect(onEnterCommit).toHaveBeenCalledWith({ value: "Kick", previousValue: "Kick" });
      expect(document.activeElement).toBe(enter.container.querySelector(".poodle-editable-label__display"));
    });
    enter.unmount();

    const onCancel = vi.fn();
    const escape = render(EditableLabel, { props: { value: "Kick", onCancel } });
    await fireEvent.dblClick(escape.container.querySelector(".poodle-editable-label__display") as HTMLElement);
    await waitFor(() => {
      expect(escape.container.querySelector(".poodle-editable-label__input")).not.toBeNull();
    });
    await fireEvent.input(escape.container.querySelector(".poodle-editable-label__input") as HTMLInputElement, {
      target: { value: "Kicks" },
    });
    await fireEvent.keyDown(escape.container.querySelector(".poodle-editable-label__input") as HTMLInputElement, {
      key: "Escape",
    });
    await waitFor(() => {
      expect(onCancel).toHaveBeenCalledTimes(1);
      expect(document.activeElement).toBe(escape.container.querySelector(".poodle-editable-label__display"));
    });
    escape.unmount();

    const onTabCommit = vi.fn();
    const tab = render(EditableLabel, { props: { value: "Kick", onCommit: onTabCommit } });
    await fireEvent.dblClick(tab.container.querySelector(".poodle-editable-label__display") as HTMLElement);
    await waitFor(() => {
      expect(tab.container.querySelector(".poodle-editable-label__input")).not.toBeNull();
    });
    const tabInput = tab.container.querySelector(".poodle-editable-label__input") as HTMLInputElement;
    await fireEvent.keyDown(tabInput, { key: "Tab" });
    await fireEvent.blur(tabInput);
    await waitFor(() => {
      expect(onTabCommit).toHaveBeenCalledWith({ value: "Kick", previousValue: "Kick" });
    });
    expect(document.activeElement).not.toBe(tab.container.querySelector(".poodle-editable-label__display"));
    tab.unmount();
  });

  it("returns to view on a new committed value without committing", async () => {
    const onCommit = vi.fn();
    const onCancel = vi.fn();
    const { container, rerender } = render(EditableLabel, {
      props: { value: "Kick", onCommit, onCancel },
    });
    await fireEvent.dblClick(container.querySelector(".poodle-editable-label__display") as HTMLElement);
    await waitFor(() => {
      expect(container.querySelector(".poodle-editable-label__input")).not.toBeNull();
    });
    await fireEvent.input(container.querySelector(".poodle-editable-label__input") as HTMLInputElement, {
      target: { value: "Kicks" },
    });

    await rerender({ value: "Kick", onCommit, onCancel });
    expect(container.querySelector(".poodle-editable-label__input")).not.toBeNull();
    expect(onCommit).not.toHaveBeenCalled();
    expect(onCancel).not.toHaveBeenCalled();

    await rerender({ value: "Snare", onCommit, onCancel });
    await waitFor(() => {
      expect(container.querySelector(".poodle-editable-label__input")).toBeNull();
    });
    expect(onCommit).not.toHaveBeenCalled();
    expect(onCancel).not.toHaveBeenCalled();
    expect(container.querySelector(".poodle-editable-label__text")?.textContent).toBe("Snare");
  });

  it("cancels without committing when disabled during an edit", async () => {
    const onCommit = vi.fn();
    const onCancel = vi.fn();
    const { container, rerender } = render(EditableLabel, {
      props: { value: "Kick", onCommit, onCancel },
    });
    await fireEvent.dblClick(container.querySelector(".poodle-editable-label__display") as HTMLElement);
    await waitFor(() => {
      expect(container.querySelector(".poodle-editable-label__input")).not.toBeNull();
    });
    await rerender({ value: "Kick", disabled: true, onCommit, onCancel });
    await waitFor(() => {
      expect(container.querySelector(".poodle-editable-label__input")).toBeNull();
    });
    expect(onCancel).toHaveBeenCalledTimes(1);
    expect(onCommit).not.toHaveBeenCalled();
  });
});
