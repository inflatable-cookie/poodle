import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { EditableLabel } from "../src/EditableLabel";

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

  it("commits on Enter and on blur with the trimmed draft and previous value", () => {
    const onCommit = vi.fn();
    const enter = render(<EditableLabel value="Old title" onCommit={onCommit} />);
    const enterDisplay = enter.container.querySelector<HTMLButtonElement>(
      ".poodle-editable-label__display",
    ) as HTMLButtonElement;
    fireEvent.dblClick(enterDisplay);
    const enterInput = enter.container.querySelector<HTMLInputElement>(".poodle-editable-label__input") as HTMLInputElement;
    fireEvent.change(enterInput, { target: { value: "  New title  " } });
    fireEvent.keyDown(enterInput, { key: "Enter" });
    expect(onCommit).toHaveBeenCalledWith({ value: "New title", previousValue: "Old title" });

    const blur = render(<EditableLabel value="Old title" onCommit={onCommit} />);
    const blurDisplay = blur.container.querySelector<HTMLButtonElement>(
      ".poodle-editable-label__display",
    ) as HTMLButtonElement;
    fireEvent.dblClick(blurDisplay);
    const blurInput = blur.container.querySelector<HTMLInputElement>(".poodle-editable-label__input") as HTMLInputElement;
    fireEvent.change(blurInput, { target: { value: "Blurred" } });
    fireEvent.blur(blurInput);
    expect(onCommit).toHaveBeenCalledWith({ value: "Blurred", previousValue: "Old title" });
    expect(blur.container.querySelector(".poodle-editable-label__input")).toBeNull();
  });

  it("activates via click, Enter, or Space in enterOrSpace mode", () => {
    const click = render(<EditableLabel value="Track 01" activationMode="enterOrSpace" />);
    fireEvent.click(click.container.querySelector(".poodle-editable-label__display") as HTMLElement);
    expect(click.container.querySelector(".poodle-editable-label__input")).not.toBeNull();

    const enter = render(<EditableLabel value="Track 02" activationMode="enterOrSpace" />);
    fireEvent.keyDown(enter.container.querySelector(".poodle-editable-label__display") as HTMLElement, {
      key: "Enter",
    });
    expect(enter.container.querySelector(".poodle-editable-label__input")).not.toBeNull();

    const space = render(<EditableLabel value="Track 03" activationMode="enterOrSpace" />);
    fireEvent.keyDown(space.container.querySelector(".poodle-editable-label__display") as HTMLElement, {
      key: " ",
    });
    expect(space.container.querySelector(".poodle-editable-label__input")).not.toBeNull();
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
});
