import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { Select } from "../src/Select";
import type { SelectItems } from "../src/types";

const options: SelectItems = [
  { value: "alpha", label: "Alpha" },
  { value: "beta", label: "Beta" },
];

describe("Select (react) dismissOnOutsideInteract", () => {
  const triggerOf = (container: HTMLElement) =>
    container.querySelector(".poodle-select__trigger") as HTMLButtonElement;

  // The listbox is portalled to the theme root, so `aria-controls` is the link
  // back, matching the other anchored overlay tests.
  const listboxOf = (container: HTMLElement) =>
    document.getElementById(triggerOf(container).getAttribute("aria-controls") ?? "") as HTMLElement;

  it("dismisses the listbox on outside mousedown by default", async () => {
    const { container } = render(<Select options={options} native={false} />);
    await fireEvent.click(triggerOf(container));
    expect(listboxOf(container)).not.toBeNull();

    await fireEvent.mouseDown(document.body);
    expect(listboxOf(container)).toBeNull();
  });

  it("keeps the listbox open on outside mousedown when dismissOnOutsideInteract=false", async () => {
    const { container } = render(
      <Select options={options} native={false} dismissOnOutsideInteract={false} />,
    );
    await fireEvent.click(triggerOf(container));
    expect(listboxOf(container)).not.toBeNull();

    await fireEvent.mouseDown(document.body);
    expect(listboxOf(container)).not.toBeNull();
  });
});

describe("Select (react) ghost variant", () => {
  it("keeps the chevron indicator on the non-searchable trigger", () => {
    const { container } = render(<Select options={options} native={false} variant="ghost" />);

    // b031: ghost drops the border and the fill, not the dropdown signal.
    expect(container.querySelector(".poodle-select__indicator-button")).not.toBeNull();
  });
});
