import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

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
