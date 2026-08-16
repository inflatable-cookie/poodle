import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import TextInput from "../src/TextInput.svelte";

// Focus parity with the React implementation (see TextInput.test.tsx): the
// `autofocus` prop is a web-native attribute forwarded to both the input and
// the textarea branches (falsy -> attribute absent), and the exported `focus()`
// moves focus to the underlying control. Svelte renders the attribute and the
// browser focuses; React's `autoFocus` focuses on mount instead — same
// browser-visible result, asserted per mechanism in each suite.
describe("TextInput (svelte)", () => {
  it("renders autofocus on the input when true", () => {
    const { container } = render(TextInput, { props: { id: "t1", autofocus: true } });
    expect(container.querySelector("input")!.hasAttribute("autofocus")).toBe(true);
  });

  it("omits autofocus on the input when false", () => {
    const { container } = render(TextInput, { props: { id: "t1", autofocus: false } });
    expect(container.querySelector("input")!.hasAttribute("autofocus")).toBe(false);
  });

  it("renders autofocus on the textarea in multiline mode when true", () => {
    const { container } = render(TextInput, { props: { id: "t1", type: "multiline", autofocus: true } });
    expect(container.querySelector("textarea")!.hasAttribute("autofocus")).toBe(true);
  });

  it("omits autofocus on the textarea in multiline mode when false", () => {
    const { container } = render(TextInput, { props: { id: "t1", type: "multiline", autofocus: false } });
    expect(container.querySelector("textarea")!.hasAttribute("autofocus")).toBe(false);
  });

  it("focus() moves focus to the underlying control", () => {
    const { component, container } = render(TextInput, { props: { id: "t1" } });
    const input = container.querySelector("input")!;
    (component as unknown as { focus: () => void }).focus();
    expect(document.activeElement).toBe(input);
  });
});

/**
 * g14.006 retained regression: clearing a search field is two portable
 * signals in one order — the value change first, then the clear command. A
 * host that reads only `clear` and a host that reads only `valueChange` both
 * have to see the field empty.
 */
describe("TextInput (svelte) search clear", () => {
  it("emits valueChange with the empty value before clear", async () => {
    const order: string[] = [];
    const { container } = render(TextInput, {
      props: {
        id: "search",
        type: "search",
        value: "kick",
        onValueChange: (value: string) => order.push(`valueChange:${value}`),
        onClear: () => order.push("clear"),
      },
    });

    await fireEvent.click(container.querySelector(".poodle-text-input__clear") as HTMLElement);

    expect(order).toEqual(["valueChange:", "clear"]);
  });
});
