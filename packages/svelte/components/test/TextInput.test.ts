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

/**
 * g16.007: the portable command and suppression rules, asserted beside the
 * native mounted proof so the two cannot drift. Enter submits the current
 * value and Escape cancels — neither is allowed to edit the field on its own,
 * because a controlled host owns the value and only `valueChange` may move it.
 */
describe("TextInput (svelte) submit and cancel", () => {
  it("submits the current value on Enter without changing it", async () => {
    const submitted: string[] = [];
    const changed: string[] = [];
    const { container } = render(TextInput, {
      props: {
        id: "command",
        value: "kick",
        onSubmit: (value: string) => submitted.push(value),
        onValueChange: (value: string) => changed.push(value),
      },
    });

    await fireEvent.keyDown(container.querySelector("input")!, { key: "Enter" });

    expect(submitted).toEqual(["kick"]);
    expect(changed).toEqual([]);
    expect(container.querySelector("input")!.value).toBe("kick");
  });

  it("cancels on Escape without changing the value", async () => {
    const cancelled: string[] = [];
    const changed: string[] = [];
    const { container } = render(TextInput, {
      props: {
        id: "command",
        value: "kick",
        onCancel: () => cancelled.push("cancel"),
        onValueChange: (value: string) => changed.push(value),
      },
    });

    await fireEvent.keyDown(container.querySelector("input")!, { key: "Escape" });

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
describe("TextInput (svelte) disabled and read-only", () => {
  it("disables the control and renders no clear button", () => {
    const { container } = render(TextInput, {
      props: { id: "search", type: "search", value: "kick", disabled: true },
    });
    expect(container.querySelector("input")!.disabled).toBe(true);
    expect(container.querySelector(".poodle-text-input__clear")).toBeNull();
  });

  it("marks a read-only control readonly and renders no clear button", () => {
    const { container } = render(TextInput, {
      props: { id: "search", type: "search", value: "kick", readOnly: true },
    });
    const input = container.querySelector("input")!;
    expect(input.readOnly).toBe(true);
    expect(input.disabled).toBe(false);
    expect(container.querySelector(".poodle-text-input__clear")).toBeNull();
  });

  it("still submits from a read-only control", async () => {
    const submitted: string[] = [];
    const { container } = render(TextInput, {
      props: {
        id: "search",
        value: "kick",
        readOnly: true,
        onSubmit: (value: string) => submitted.push(value),
      },
    });

    await fireEvent.keyDown(container.querySelector("input")!, { key: "Enter" });

    expect(submitted).toEqual(["kick"]);
  });

  it("forwards maxLength to the native limit", () => {
    const { container } = render(TextInput, { props: { id: "t1", maxLength: 4 } });
    expect(container.querySelector("input")!.getAttribute("maxlength")).toBe("4");
  });
});
