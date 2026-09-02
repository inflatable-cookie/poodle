import { createRef } from "react";
import { act, fireEvent, render, waitFor } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { EditableLabel, type EditableLabelHandle } from "../src/EditableLabel";

describe("EditableLabel (react)", () => {
  it("enters edit mode on double-click seeded with the value and cancels back on Escape", () => {
    const onEditStart = vi.fn();
    const onCancel = vi.fn();
    const onCommit = vi.fn();
    const { container } = render(
      <EditableLabel value="My project title" onEditStart={onEditStart} onCancel={onCancel} onCommit={onCommit} />,
    );
    const display = container.querySelector<HTMLButtonElement>(".poodle-editable-label__display") as HTMLButtonElement;
    expect(display.textContent).toBe("My project title");

    fireEvent.dblClick(display);
    const input = container.querySelector<HTMLInputElement>(".poodle-editable-label__input");
    expect(input).not.toBeNull();
    expect(input?.value).toBe("My project title");
    expect(document.activeElement).toBe(input);
    expect(onEditStart).toHaveBeenCalledTimes(1);

    fireEvent.change(input as HTMLInputElement, { target: { value: "Renamed" } });
    fireEvent.keyDown(input as HTMLInputElement, { key: "Escape" });

    expect(onCancel).toHaveBeenCalledTimes(1);
    expect(onCommit).not.toHaveBeenCalled();
    expect(container.querySelector(".poodle-editable-label__input")).toBeNull();
    expect(container.querySelector(".poodle-editable-label__display")?.textContent).toBe("My project title");
  });

  it("commits on Enter and on blur with the trimmed draft and previous value", async () => {
    const onEnterCommit = vi.fn();
    const enter = render(<EditableLabel value="Old title" onCommit={onEnterCommit} />);
    const enterDisplay = enter.container.querySelector<HTMLButtonElement>(
      ".poodle-editable-label__display",
    ) as HTMLButtonElement;
    fireEvent.dblClick(enterDisplay);
    const enterInput = enter.container.querySelector<HTMLInputElement>(".poodle-editable-label__input") as HTMLInputElement;
    fireEvent.change(enterInput, { target: { value: "  New title  " } });
    fireEvent.keyDown(enterInput, { key: "Enter" });
    expect(onEnterCommit).toHaveBeenCalledWith({ value: "New title", previousValue: "Old title" });

    const onBlurCommit = vi.fn();
    const blur = render(<EditableLabel value="Old title" onCommit={onBlurCommit} />);
    const blurDisplay = blur.container.querySelector<HTMLButtonElement>(
      ".poodle-editable-label__display",
    ) as HTMLButtonElement;
    fireEvent.dblClick(blurDisplay);
    const blurInput = blur.container.querySelector<HTMLInputElement>(".poodle-editable-label__input") as HTMLInputElement;
    fireEvent.change(blurInput, { target: { value: "Blurred" } });
    fireEvent.blur(blurInput);
    await waitFor(() => {
      expect(onBlurCommit).toHaveBeenCalledWith({ value: "Blurred", previousValue: "Old title" });
    });
    expect(blur.container.querySelector(".poodle-editable-label__input")).toBeNull();
  });

  it("activates via click, Enter, or Space in enterOrSpace mode", () => {
    const click = render(<EditableLabel value="Track 01" activationMode="enterOrSpace" />);
    fireEvent.click(click.container.querySelector(".poodle-editable-label__display") as HTMLElement);
    expect(click.container.querySelector(".poodle-editable-label__input")).not.toBeNull();
    click.unmount();

    const enter = render(<EditableLabel value="Track 02" activationMode="enterOrSpace" />);
    fireEvent.keyDown(enter.container.querySelector(".poodle-editable-label__display") as HTMLElement, {
      key: "Enter",
    });
    expect(enter.container.querySelector(".poodle-editable-label__input")).not.toBeNull();
    enter.unmount();

    const space = render(<EditableLabel value="Track 03" activationMode="enterOrSpace" />);
    fireEvent.keyDown(space.container.querySelector(".poodle-editable-label__display") as HTMLElement, {
      key: " ",
    });
    expect(space.container.querySelector(".poodle-editable-label__input")).not.toBeNull();
    space.unmount();
  });

  it("stays in view mode under programmatic activation", () => {
    const { container } = render(<EditableLabel value="Static" activationMode="programmatic" />);
    const display = container.querySelector<HTMLElement>(".poodle-editable-label__display") as HTMLElement;
    fireEvent.click(display);
    fireEvent.dblClick(display);
    fireEvent.keyDown(display, { key: "Enter" });
    expect(container.querySelector(".poodle-editable-label__input")).toBeNull();
  });

  it("blocks edit entry while disabled", () => {
    const onEditStart = vi.fn();
    const { container } = render(<EditableLabel value="Read-only" disabled onEditStart={onEditStart} />);
    const display = container.querySelector<HTMLButtonElement>(".poodle-editable-label__display") as HTMLButtonElement;
    expect(display.disabled).toBe(true);

    fireEvent.click(display);
    fireEvent.dblClick(display);
    expect(container.querySelector(".poodle-editable-label__input")).toBeNull();
    expect(onEditStart).not.toHaveBeenCalled();
  });

  it("shows the empty state text when the value is empty", () => {
    const { container } = render(<EditableLabel value="" emptyText="Add a description…" />);
    const display = container.querySelector<HTMLElement>(".poodle-editable-label__display");
    expect(display?.classList.contains("poodle-editable-label__display--empty")).toBe(true);
    expect(display?.querySelector(".poodle-editable-label__text")?.textContent).toBe("Add a description…");
    expect(container.querySelector(".poodle-editable-label__icon")).toBeNull();

    const withIcon = render(<EditableLabel value="" emptyText="Add a description…" showEditIcon />);
    expect(withIcon.container.querySelector(".poodle-editable-label__icon")).not.toBeNull();
  });

  it("enters edit mode from Enter or Space in doubleClick mode", () => {
    const enter = render(<EditableLabel value="Kick" />);
    fireEvent.keyDown(enter.container.querySelector(".poodle-editable-label__display") as HTMLElement, {
      key: "Enter",
    });
    expect(enter.container.querySelector(".poodle-editable-label__input")).not.toBeNull();
    enter.unmount();

    const space = render(<EditableLabel value="Kick" />);
    fireEvent.keyDown(space.container.querySelector(".poodle-editable-label__display") as HTMLElement, {
      key: " ",
    });
    expect(space.container.querySelector(".poodle-editable-label__input")).not.toBeNull();
    space.unmount();
  });

  it("stays in view on a single click in the default doubleClick mode", () => {
    const { container } = render(<EditableLabel value="Kick" />);
    fireEvent.click(container.querySelector(".poodle-editable-label__display") as HTMLElement);
    expect(container.querySelector(".poodle-editable-label__input")).toBeNull();
  });

  it("exposes focus, startEditing, and cancelEditing through EditableLabelHandle", async () => {
    const onEditStart = vi.fn();
    const onCancel = vi.fn();
    const onCommit = vi.fn();
    const ref = createRef<EditableLabelHandle>();
    const { container } = render(
      <EditableLabel ref={ref} value="Kick" onEditStart={onEditStart} onCancel={onCancel} onCommit={onCommit} />,
    );

    act(() => {
      ref.current?.focus();
    });
    expect(document.activeElement).toBe(container.querySelector(".poodle-editable-label__display"));

    act(() => {
      ref.current?.startEditing();
    });
    await waitFor(() => {
      expect(container.querySelector(".poodle-editable-label__input")).not.toBeNull();
    });
    expect(onEditStart).toHaveBeenCalledTimes(1);
    await waitFor(() => {
      expect(document.activeElement).toBe(container.querySelector(".poodle-editable-label__input"));
    });

    act(() => {
      ref.current?.cancelEditing();
    });
    await waitFor(() => {
      expect(container.querySelector(".poodle-editable-label__input")).toBeNull();
    });
    expect(onCancel).toHaveBeenCalledTimes(1);
    expect(onCommit).not.toHaveBeenCalled();
    await waitFor(() => {
      expect(document.activeElement).toBe(container.querySelector(".poodle-editable-label__display"));
    });
  });

  it("startEditing ignores programmatic activation and is inert when disabled", async () => {
    const programmatic = createRef<EditableLabelHandle>();
    const { container } = render(
      <EditableLabel ref={programmatic} value="Kick" activationMode="programmatic" />,
    );
    act(() => {
      programmatic.current?.startEditing();
    });
    await waitFor(() => {
      expect(container.querySelector(".poodle-editable-label__input")).not.toBeNull();
    });

    const blocked = createRef<EditableLabelHandle>();
    const disabled = render(<EditableLabel ref={blocked} value="Kick" disabled />);
    act(() => {
      blocked.current?.startEditing();
    });
    expect(disabled.container.querySelector(".poodle-editable-label__input")).toBeNull();
  });

  it("names display and editor from the visible value when ariaLabel is omitted", () => {
    const { container } = render(<EditableLabel value="Kick" />);
    expect(container.querySelector(".poodle-editable-label__display")?.getAttribute("aria-label")).toBe("Kick");
    fireEvent.dblClick(container.querySelector(".poodle-editable-label__display") as HTMLElement);
    expect(container.querySelector(".poodle-editable-label__input")?.getAttribute("aria-label")).toBe("Kick");
  });

  it("clamps maxLength to Unicode scalar values, not UTF-16 units", () => {
    const { container } = render(<EditableLabel value="" maxLength={1} />);
    fireEvent.dblClick(container.querySelector(".poodle-editable-label__display") as HTMLElement);
    const input = container.querySelector<HTMLInputElement>(".poodle-editable-label__input") as HTMLInputElement;
    expect(input.hasAttribute("maxLength") || input.hasAttribute("maxlength")).toBe(false);
    fireEvent.change(input, { target: { value: "𝄞" } });
    expect(input.value).toBe("𝄞");
    fireEvent.change(input, { target: { value: "𝄞A" } });
    expect(input.value).toBe("𝄞");
  });

  it("commits portable NEL/BOM trim and ignores a later unmount blur", async () => {
    const onCommit = vi.fn();
    const onCancel = vi.fn();
    const { container, unmount } = render(
      <EditableLabel value="Kick" onCommit={onCommit} onCancel={onCancel} />,
    );
    fireEvent.dblClick(container.querySelector(".poodle-editable-label__display") as HTMLElement);
    const input = container.querySelector<HTMLInputElement>(".poodle-editable-label__input") as HTMLInputElement;
    fireEvent.change(input, { target: { value: "\u0085Take\uFEFF" } });
    fireEvent.keyDown(input, { key: "Enter" });
    expect(onCommit).toHaveBeenCalledWith({ value: "Take", previousValue: "Kick" });
    unmount();
    await Promise.resolve();
    expect(onCommit).toHaveBeenCalledTimes(1);
    expect(onCancel).not.toHaveBeenCalled();
  });

  it("emits neither commit nor cancel when unmounted while editing", async () => {
    const onCommit = vi.fn();
    const onCancel = vi.fn();
    const { container, unmount } = render(
      <EditableLabel value="Kick" onCommit={onCommit} onCancel={onCancel} />,
    );
    fireEvent.dblClick(container.querySelector(".poodle-editable-label__display") as HTMLElement);
    fireEvent.change(container.querySelector(".poodle-editable-label__input") as HTMLInputElement, {
      target: { value: "Kicks" },
    });
    unmount();
    await Promise.resolve();
    expect(onCommit).not.toHaveBeenCalled();
    expect(onCancel).not.toHaveBeenCalled();
  });

  it("commits once on window blur without restoring display focus", async () => {
    const onCommit = vi.fn();
    const { container } = render(<EditableLabel value="Kick" onCommit={onCommit} />);
    fireEvent.dblClick(container.querySelector(".poodle-editable-label__display") as HTMLElement);
    fireEvent.change(container.querySelector(".poodle-editable-label__input") as HTMLInputElement, {
      target: { value: "Kicks" },
    });
    window.dispatchEvent(new Event("blur"));
    await waitFor(() => {
      expect(onCommit).toHaveBeenCalledWith({ value: "Kicks", previousValue: "Kick" });
    });
    expect(document.activeElement).not.toBe(container.querySelector(".poodle-editable-label__display"));
  });

  it("returns to view on a new committed value without committing", async () => {
    const onCommit = vi.fn();
    const onCancel = vi.fn();
    const { container, rerender } = render(
      <EditableLabel value="Kick" onCommit={onCommit} onCancel={onCancel} />,
    );
    fireEvent.dblClick(container.querySelector(".poodle-editable-label__display") as HTMLElement);
    fireEvent.change(container.querySelector(".poodle-editable-label__input") as HTMLInputElement, {
      target: { value: "Kicks" },
    });

    rerender(<EditableLabel value="Kick" onCommit={onCommit} onCancel={onCancel} />);
    expect(container.querySelector(".poodle-editable-label__input")).not.toBeNull();
    expect(onCommit).not.toHaveBeenCalled();
    expect(onCancel).not.toHaveBeenCalled();

    rerender(<EditableLabel value="Snare" onCommit={onCommit} onCancel={onCancel} />);
    await waitFor(() => {
      expect(container.querySelector(".poodle-editable-label__input")).toBeNull();
    });
    expect(onCommit).not.toHaveBeenCalled();
    expect(onCancel).not.toHaveBeenCalled();
    expect(container.querySelector(".poodle-editable-label__display")?.textContent).toBe("Snare");
  });

  it("cancels without committing when disabled during an edit", async () => {
    const onCommit = vi.fn();
    const onCancel = vi.fn();
    const { container, rerender } = render(
      <EditableLabel value="Kick" onCommit={onCommit} onCancel={onCancel} />,
    );
    fireEvent.dblClick(container.querySelector(".poodle-editable-label__display") as HTMLElement);
    rerender(<EditableLabel value="Kick" disabled onCommit={onCommit} onCancel={onCancel} />);
    await waitFor(() => {
      expect(container.querySelector(".poodle-editable-label__input")).toBeNull();
    });
    expect(onCancel).toHaveBeenCalledTimes(1);
    expect(onCommit).not.toHaveBeenCalled();
  });
});
