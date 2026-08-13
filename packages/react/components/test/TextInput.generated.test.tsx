import { act, fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { TextInput } from "../src/TextInput";
import { textInputDefinition } from "../src/generated/text-input";

// Card 048 R2: the DOM reads the generated artifact — the data-* attribute
// names, the part class names, and the TXT-16 padding custom properties
// come from text_input.rs via `text-input-ts`, never from hand-written
// literals in this component. A rename in the definition moves the DOM;
// `effigy ir:check` gates drift in the artifact.
//
// The IME and selection tests are the milestone's acceptance lines: a
// composition sequence must not fire intermediate `onValueChange`, and the
// DOM still owns selection — the definition declares the boundary, it does
// not implement it (R2).

function attributeName(id: string): string {
  const attribute = textInputDefinition.attributes.find((entry) => entry.id === id);
  if (!attribute) throw new Error(`definition lacks attribute '${id}'`);
  return attribute.name;
}

function partClass(id: string): string {
  const part = textInputDefinition.parts.find((entry) => entry.id === id);
  if (!part) throw new Error(`definition lacks part '${id}'`);
  return part.className;
}

// Part classes can be space-joined (base + modifier, e.g.
// "poodle-text-input__affix poodle-text-input__affix--prefix"); the
// modifier token is the unique selector for that part.
function partSelector(id: string): string {
  const classes = partClass(id).split(" ");
  return `.${classes[classes.length - 1]}`;
}

function stylePropName(id: string): string {
  const prop = textInputDefinition.styleProps.find((entry) => entry.id === id);
  if (!prop) throw new Error(`definition lacks style prop '${id}'`);
  return prop.name;
}

describe("TextInput (react) — generated definition drives the DOM", () => {
  it("emits the data attributes under the definition's names and values", () => {
    const { container } = render(<TextInput id="t1" validationState="invalid" />);
    const el = container.querySelector(partSelector("root")) as HTMLElement;

    // The definition's attribute entries are what the DOM carries — the
    // names in this test came from the artifact itself.
    expect(attributeName("validation-state")).toBe("data-validation-state");
    expect(attributeName("size")).toBe("data-size");
    expect(attributeName("density")).toBe("data-density");
    expect(attributeName("type")).toBe("data-type");

    expect(el.getAttribute(attributeName("validation-state"))).toBe("invalid");
    expect(el.getAttribute(attributeName("size"))).toBe("md");
    expect(el.getAttribute(attributeName("density"))).toBe("default");
    expect(el.getAttribute(attributeName("type"))).toBe("text");
  });

  it("renders the anatomy under the definition's part classes", () => {
    const { container } = render(<TextInput id="t1" prefix="$" suffix="/mo" showCharCount maxLength={10} />);
    const root = container.querySelector(partSelector("root")) as HTMLElement;

    expect(root.querySelector(partSelector("field"))).not.toBeNull();
    expect(root.querySelector(partSelector("input-control"))).not.toBeNull();
    expect(root.querySelector(partSelector("prefix"))).not.toBeNull();
    expect(root.querySelector(partSelector("suffix"))).not.toBeNull();
    expect(root.querySelector(partSelector("char-count"))).not.toBeNull();
  });

  it("conditionally renders the clear button, validation indicator and char count", () => {
    const { container } = render(<TextInput id="t1" type="search" defaultValue="query" validationState="invalid" />);
    const root = container.querySelector(partSelector("root")) as HTMLElement;

    // Search mode with a value: the clear button renders.
    expect(root.querySelector(partSelector("clear-button"))).not.toBeNull();
    // A non-none validation state: the indicator renders.
    expect(root.querySelector(partSelector("validation-indicator"))).not.toBeNull();
    // showCharCount false: no char count.
    expect(root.querySelector(partSelector("char-count"))).toBeNull();

    const plain = render(<TextInput id="t2" />);
    const plainRoot = plain.container.querySelector(partSelector("root")) as HTMLElement;
    expect(plainRoot.querySelector(partSelector("clear-button"))).toBeNull();
    expect(plainRoot.querySelector(partSelector("validation-indicator"))).toBeNull();
  });

  it("emits the three shared TXT-16 style props under the definition's names", () => {
    const { container } = render(<TextInput id="t1" type="search" defaultValue="q" />);
    const root = container.querySelector(partSelector("root")) as HTMLElement;
    const style = root.getAttribute("style") ?? "";

    // React emits the three shared padding reservations (the two overlay
    // insets are Svelte-only emissions — CSS fallbacks cover React).
    expect(style).toContain(stylePropName("control-padding-start"));
    expect(style).toContain(stylePropName("control-padding-end"));
    expect(style).toContain(stylePropName("multiline-padding-end"));
  });

  it("does not fire intermediate onValueChange during IME composition", async () => {
    const onValueChange = vi.fn();
    const { container } = render(<TextInput id="t1" onValueChange={onValueChange} />);
    const input = container.querySelector("input") as HTMLInputElement;

    await act(async () => {
      input.dispatchEvent(new Event("compositionstart", { bubbles: true }));
    });
    // Each composition update fires a native input event with
    // isComposing true — none of them may reach the value path. The value
    // travels through the event (browser-faithful; React's value tracker
    // would swallow a pre-set value).
    fireEvent.input(input, { target: { value: "こ" }, isComposing: true, data: "こ" });
    fireEvent.input(input, { target: { value: "こん" }, isComposing: true, data: "ん" });
    expect(onValueChange).not.toHaveBeenCalled();

    // The composition commits once on compositionend (React's onChange is
    // value-diff-based, so the final committed input event is the last
    // buffer the tracker already saw — the end handler is the commit
    // point, per the card's no-intermediate-fire acceptance line).
    fireEvent.input(input, { target: { value: "こん" }, isComposing: false, data: "こん" });
    await act(async () => {
      input.dispatchEvent(new Event("compositionend", { bubbles: true }));
    });
    expect(onValueChange).toHaveBeenCalledTimes(1);
    expect(onValueChange).toHaveBeenLastCalledWith("こん");
  });

  it("leaves selection to the DOM — the component carries no selection state", async () => {
    const { container, rerender } = render(<TextInput id="t1" defaultValue="hello" placeholder="a" />);
    const input = container.querySelector("input") as HTMLInputElement;
    const root = container.querySelector(partSelector("root")) as HTMLElement;

    // The root carries no selection-related attribute and the control has
    // no selection handler — the DOM owns selection (R2, T §6). The
    // selectionStart/selectionEnd/isFocused props are Rust-targets-only
    // (T §3); the web components have no such surface at all.
    for (const attribute of Array.from(root.attributes)) {
      expect(attribute.name.toLowerCase()).not.toContain("selection");
    }

    // A re-render must not rewrite the DOM's selection — the component has
    // no selection state to restore (the Rust targets own the caret; the
    // web leaves it to the browser).
    input.focus();
    input.setSelectionRange(1, 3);
    expect(input.selectionStart).toBe(1);
    await rerender(<TextInput id="t1" defaultValue="hello" placeholder="b" />);
    expect(input.selectionStart).toBe(1);
    expect(input.selectionEnd).toBe(3);
  });
});
