import { createRef } from "react";
import { render } from "@testing-library/react";
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
