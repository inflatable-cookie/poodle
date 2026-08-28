import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import Select from "../src/Select.svelte";
import type { SelectItems } from "../src/types";

const options: SelectItems = [
  { value: "alpha", label: "Alpha" },
  { value: "beta", label: "Beta" },
];

describe("Select (svelte) dismissOnOutsideInteract", () => {
  const triggerOf = (container: HTMLElement) =>
    container.querySelector(".poodle-select__trigger") as HTMLButtonElement;

  // The listbox is portalled to the theme root, so `aria-controls` is the link
  // back, matching the other anchored overlay tests.
  const listboxOf = (container: HTMLElement) =>
    document.getElementById(triggerOf(container).getAttribute("aria-controls") ?? "") as HTMLElement;

  it("dismisses the listbox on outside mousedown by default", async () => {
    const { container } = render(Select, { props: { options, native: false } });
    await fireEvent.click(triggerOf(container));
    expect(listboxOf(container)).not.toBeNull();

    await fireEvent.mouseDown(document.body);
    expect(listboxOf(container)).toBeNull();
  });

  it("keeps the listbox open on outside mousedown when dismissOnOutsideInteract=false", async () => {
    const { container } = render(Select, {
      props: { options, native: false, dismissOnOutsideInteract: false },
    });
    await fireEvent.click(triggerOf(container));
    expect(listboxOf(container)).not.toBeNull();

    await fireEvent.mouseDown(document.body);
    expect(listboxOf(container)).not.toBeNull();
  });
});

describe("Select (svelte) ghost variant", () => {
  it("keeps the chevron indicator on the non-searchable trigger", () => {
    const { container } = render(Select, {
      props: { options, native: false, variant: "ghost" },
    });

    // b031: ghost drops the border and the fill, not the dropdown signal.
    expect(container.querySelector(".poodle-select__indicator-button")).not.toBeNull();
  });

  it("stamps data-variant on the native root", () => {
    const { container } = render(Select, {
      props: { options, variant: "ghost" },
    });

    const root = container.querySelector(".poodle-select");
    expect(root?.classList.contains("poodle-select--custom")).toBe(false);
    expect(root?.getAttribute("data-variant")).toBe("ghost");
  });
});

/**
 * g14.007 retained regression. An option identified only by
 * `${listboxId}-option-${index}` cannot be addressed stably by anything — a
 * consumer's test, an automation script, or a parity harness. `Tabs` already
 * exposed `data-value`; `Select` and `Menu` now match it.
 */
describe("Select (svelte) option identity", () => {
  it("addresses every option by its value", async () => {
    const { container } = render(Select, { props: { options, native: false } });
    await fireEvent.click(container.querySelector(".poodle-select__trigger") as HTMLElement);

    const values = [...document.querySelectorAll('[role="option"]')].map((el) =>
      el.getAttribute("data-value"),
    );
    expect(values).toEqual(["alpha", "beta"]);
  });
});

describe("Select (svelte) semantic machine", () => {
  const triggerOf = (container: HTMLElement) =>
    container.querySelector(".poodle-select__trigger") as HTMLButtonElement;
  const inputOf = (container: HTMLElement) =>
    container.querySelector(".poodle-select__input") as HTMLInputElement;
  const listboxOf = (container: HTMLElement) =>
    document.querySelector('[role="listbox"]') as HTMLElement | null;

  it("reports query edits without committing a value", async () => {
    const onValueChange = vi.fn();
    const onQueryChange = vi.fn();
    const onOpenChange = vi.fn();
    const { container } = render(Select, {
      props: {
        options,
        searchable: true,
        freeform: true,
        native: false,
        onValueChange,
        onQueryChange,
        onOpenChange,
      },
    });

    await fireEvent.input(inputOf(container), { target: { value: "al" } });

    expect(onQueryChange.mock.calls.map((call) => call[0])).toEqual(["al"]);
    expect(onValueChange).not.toHaveBeenCalled();
    expect(onOpenChange.mock.calls.map((call) => call[0])).toEqual([true]);
  });

  it("commits a highlighted option on Enter and reports value after query", async () => {
    const onValueChange = vi.fn();
    const onQueryChange = vi.fn();
    const onOpenChange = vi.fn();
    const { container } = render(Select, {
      props: {
        options,
        searchable: true,
        freeform: true,
        native: false,
        onValueChange,
        onQueryChange,
        onOpenChange,
      },
    });

    await fireEvent.input(inputOf(container), { target: { value: "be" } });
    await fireEvent.keyDown(inputOf(container), { key: "Enter" });

    expect(onQueryChange.mock.calls.map((call) => call[0])).toEqual(["be", "Beta"]);
    expect(onValueChange.mock.calls.map((call) => call[0])).toEqual(["beta"]);
    expect(onOpenChange.mock.calls.map((call) => call[0])).toEqual([true, false]);
  });

  it("commits freeform text on Enter when no option is highlighted", async () => {
    const onValueChange = vi.fn();
    const { container } = render(Select, {
      props: { options, searchable: true, freeform: true, native: false, onValueChange },
    });

    await fireEvent.input(inputOf(container), { target: { value: "kiwi" } });
    await fireEvent.keyDown(inputOf(container), { key: "Enter" });

    expect(onValueChange.mock.calls.map((call) => call[0])).toEqual(["kiwi"]);
  });

  it("does not commit freeform text on Tab or Escape", async () => {
    const onValueChange = vi.fn();
    const { container } = render(Select, {
      props: { options, searchable: true, freeform: true, native: false, onValueChange },
    });

    await fireEvent.input(inputOf(container), { target: { value: "kiwi" } });
    await fireEvent.keyDown(inputOf(container), { key: "Escape" });
    expect(onValueChange).not.toHaveBeenCalled();
    expect(listboxOf(container)).toBeNull();

    await fireEvent.input(inputOf(container), { target: { value: "kiwi" } });
    await fireEvent.keyDown(inputOf(container), { key: "Tab" });
    expect(onValueChange).not.toHaveBeenCalled();
  });

  it("commits freeform text on control blur when no option is highlighted", async () => {
    const onValueChange = vi.fn();
    const { container } = render(Select, {
      props: { options, searchable: true, freeform: true, native: false, onValueChange },
    });

    await fireEvent.input(inputOf(container), { target: { value: "kiwi" } });
    await fireEvent.focusOut(container.querySelector(".poodle-select") as HTMLElement, {
      relatedTarget: document.body,
    });

    expect(onValueChange.mock.calls.map((call) => call[0])).toEqual(["kiwi"]);
  });

  it("does not commit a draft query before an option click", async () => {
    const onValueChange = vi.fn();
    const { container } = render(Select, {
      props: { options, searchable: true, freeform: true, native: false, onValueChange },
    });

    await fireEvent.input(inputOf(container), { target: { value: "al" } });
    const option = document.querySelector('[data-value="alpha"]') as HTMLElement;
    await fireEvent.mouseDown(option);
    await fireEvent.click(option);

    expect(onValueChange.mock.calls.map((call) => call[0])).toEqual(["alpha"]);
  });

  it("clamps arrow movement and skips a disabled option", async () => {
    const { container } = render(Select, {
      props: {
        options: [
          { value: "alpha", label: "Alpha" },
          { value: "skip", label: "Skip", disabled: true },
          { value: "beta", label: "Beta" },
        ],
        native: false,
      },
    });

    await fireEvent.click(triggerOf(container));
    const trigger = triggerOf(container);
    await fireEvent.keyDown(trigger, { key: "ArrowDown" });
    await fireEvent.keyDown(trigger, { key: "ArrowDown" });

    const highlighted = [...document.querySelectorAll('[role="option"]')].map((el) =>
      el.getAttribute("data-highlighted"),
    );
    expect(highlighted).toEqual(["false", "false", "true"]);
  });

  it("does not emit a second toggle from clear", async () => {
    const onOpenChange = vi.fn();
    const onValueChange = vi.fn();
    const { container } = render(Select, {
      props: {
        options,
        native: false,
        clearable: true,
        value: "alpha",
        onOpenChange,
        onValueChange,
      },
    });

    await fireEvent.click(triggerOf(container));
    onOpenChange.mockClear();
    await fireEvent.click(container.querySelector(".poodle-select__clear") as HTMLElement);

    expect(onValueChange.mock.calls.map((call) => call[0])).toEqual([""]);
    expect(onOpenChange).not.toHaveBeenCalled();
    expect(listboxOf(container)).not.toBeNull();
  });
});
