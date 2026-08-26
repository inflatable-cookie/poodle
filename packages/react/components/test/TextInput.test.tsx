import { createRef } from "react";
import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { TextInput, type TextInputHandle } from "../src/TextInput";

// Focus parity with the Svelte implementation (see TextInput.test.ts): the
// `autofocus` prop is a web-native attribute on both branches (falsy -> nothing
// rendered). Svelte renders the `autofocus` attribute and the browser focuses;
// React's canonical `autoFocus` focuses the control on mount instead (the
// attribute is emitted in react-dom/server markup). Same browser-visible
// result, so the truthy side asserts the mount focus and the falsy side
// asserts no attribute and no focus. `focus()` — exposed through
// useImperativeHandle per the MenuSurface precedent — moves focus to the
// underlying control.
describe("TextInput (react)", () => {
  it("focuses the input on mount when autofocus is true", () => {
    const { container } = render(<TextInput id="t1" autofocus />);
    expect(document.activeElement).toBe(container.querySelector("input"));
  });

  it("omits autofocus on the input when false", () => {
    const { container } = render(<TextInput id="t1" autofocus={false} />);
    const input = container.querySelector("input")!;
    expect(input.hasAttribute("autofocus")).toBe(false);
    expect(document.activeElement).not.toBe(input);
  });

  it("focuses the textarea on mount in multiline mode when autofocus is true", () => {
    const { container } = render(<TextInput id="t1" type="multiline" autofocus />);
    expect(document.activeElement).toBe(container.querySelector("textarea"));
  });

  it("omits autofocus on the textarea in multiline mode when false", () => {
    const { container } = render(<TextInput id="t1" type="multiline" autofocus={false} />);
    const textarea = container.querySelector("textarea")!;
    expect(textarea.hasAttribute("autofocus")).toBe(false);
    expect(document.activeElement).not.toBe(textarea);
  });

  it("focus() via ref moves focus to the underlying control", () => {
    const ref = createRef<TextInputHandle>();
    const { container } = render(<TextInput ref={ref} id="t1" />);
    const input = container.querySelector("input")!;
    ref.current?.focus();
    expect(document.activeElement).toBe(input);
  });
});

/**
 * g14.006 retained regression: clearing a search field is two portable
 * signals in one order — the value change first, then the clear command. A
 * host that reads only `clear` and a host that reads only `valueChange` both
 * have to see the field empty.
 */
describe("TextInput (react) search clear", () => {
  it("emits valueChange with the empty value before clear", () => {
    const order: string[] = [];
    const { container } = render(
      <TextInput
        id="search"
        type="search"
        value="kick"
        onValueChange={(value) => order.push(`valueChange:${value}`)}
        onClear={() => order.push("clear")}
      />,
    );

    fireEvent.click(container.querySelector(".poodle-text-input__clear") as HTMLElement);

    expect(order).toEqual(["valueChange:", "clear"]);
  });
});

/**
 * g16.007: the portable command and suppression rules, asserted beside the
 * native mounted proof so the two cannot drift. Enter submits the current
 * value and Escape cancels — neither is allowed to edit the field on its own,
 * because a controlled host owns the value and only `valueChange` may move it.
 */
describe("TextInput (react) submit and cancel", () => {
  it("submits the current value on Enter without changing it", () => {
    const submitted: string[] = [];
    const changed: string[] = [];
    const { container } = render(
      <TextInput
        id="command"
        value="kick"
        onSubmit={(value) => submitted.push(value)}
        onValueChange={(value) => changed.push(value)}
      />,
    );

    fireEvent.keyDown(container.querySelector("input")!, { key: "Enter" });

    expect(submitted).toEqual(["kick"]);
    expect(changed).toEqual([]);
    expect(container.querySelector("input")!.value).toBe("kick");
  });

  it("cancels on Escape without changing the value", () => {
    const cancelled: string[] = [];
    const changed: string[] = [];
    const { container } = render(
      <TextInput
        id="command"
        value="kick"
        onCancel={() => cancelled.push("cancel")}
        onValueChange={(value) => changed.push(value)}
      />,
    );

    fireEvent.keyDown(container.querySelector("input")!, { key: "Escape" });

    expect(cancelled).toEqual(["cancel"]);
    expect(changed).toEqual([]);
    expect(container.querySelector("input")!.value).toBe("kick");
  });
});

/**
 * g16.007: disabled is inert; read-only stays a real field that can be read
 * from and commanded but never edited. The clear control is the one part of a
 * search field a pointer can reach, so its absence is what "cannot mutate"
 * looks like from outside.
 */
describe("TextInput (react) disabled and read-only", () => {
  it("disables the control and renders no clear button", () => {
    const { container } = render(<TextInput id="search" type="search" value="kick" disabled />);
    expect(container.querySelector("input")!.disabled).toBe(true);
    expect(container.querySelector(".poodle-text-input__clear")).toBeNull();
  });

  it("marks a read-only control readonly and renders no clear button", () => {
    const { container } = render(<TextInput id="search" type="search" value="kick" readOnly />);
    const input = container.querySelector("input")!;
    expect(input.readOnly).toBe(true);
    expect(input.disabled).toBe(false);
    expect(container.querySelector(".poodle-text-input__clear")).toBeNull();
  });

  it("still submits from a read-only control", () => {
    const submitted: string[] = [];
    const { container } = render(
      <TextInput id="search" value="kick" readOnly onSubmit={(value) => submitted.push(value)} />,
    );

    fireEvent.keyDown(container.querySelector("input")!, { key: "Enter" });

    expect(submitted).toEqual(["kick"]);
  });

  it("forwards maxLength to the native limit", () => {
    const { container } = render(<TextInput id="t1" maxLength={4} />);
    expect(container.querySelector("input")!.getAttribute("maxlength")).toBe("4");
  });
});
