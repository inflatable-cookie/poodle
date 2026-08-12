import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { FilterBuilder } from "../src/FilterBuilder";
import type { FilterFieldDefinition } from "../src/types";

const fields: FilterFieldDefinition[] = [
  { key: "status", label: "Status", kind: "select", options: [{ value: "open", label: "Open" }] },
];

describe("FilterBuilder (react) dismissOnOutsideInteract", () => {
  const triggerOf = (container: HTMLElement) =>
    container.querySelector(".poodle-filter-builder__trigger") as HTMLButtonElement;

  // The surface is portalled to the theme root; `aria-controls` is the link
  // back, matching the other anchored overlay tests.
  const surfaceOf = (container: HTMLElement) =>
    document.getElementById(
      triggerOf(container).getAttribute("aria-controls") ?? "",
    ) as HTMLElement;

  it("dismisses the surface on outside mousedown by default", async () => {
    const { container } = render(<FilterBuilder fields={fields} />);
    await fireEvent.click(triggerOf(container));
    expect(surfaceOf(container)).not.toBeNull();

    await fireEvent.mouseDown(document.body);
    expect(surfaceOf(container)).toBeNull();
  });

  it("keeps the surface open on outside mousedown when dismissOnOutsideInteract=false", async () => {
    const { container } = render(
      <FilterBuilder fields={fields} dismissOnOutsideInteract={false} />,
    );
    await fireEvent.click(triggerOf(container));
    expect(surfaceOf(container)).not.toBeNull();

    await fireEvent.mouseDown(document.body);
    expect(surfaceOf(container)).not.toBeNull();
  });

  it("renders no hardcoded chevron on the opener trigger", () => {
    // b031: the ▾ workaround is gone; the dropdown affordance lives in the
    // ghost Select primitive, not in a per-component character.
    const { container } = render(<FilterBuilder fields={fields} />);

    expect(container.querySelector(".poodle-filter-builder__chevron")).toBeNull();
  });
});
