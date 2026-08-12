import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import OrderBy from "../src/OrderBy.svelte";
import type { SortField } from "../src/types";

const fields: SortField[] = [
  { value: "name", label: "Name" },
  { value: "date", label: "Date" },
];

describe("OrderBy (svelte) dismissOnOutsideInteract", () => {
  const triggerOf = (container: HTMLElement) =>
    container.querySelector(".poodle-order-by__trigger") as HTMLButtonElement;

  // The surface is portalled to the theme root; `aria-controls` is the link
  // back, matching the other anchored overlay tests.
  const surfaceOf = (container: HTMLElement) =>
    document.getElementById(
      triggerOf(container).getAttribute("aria-controls") ?? "",
    ) as HTMLElement | null;

  it("dismisses the surface on outside mousedown by default", async () => {
    const { container } = render(OrderBy, { props: { fields } });
    await fireEvent.click(triggerOf(container));
    expect(surfaceOf(container)).not.toBeNull();

    await fireEvent.mouseDown(document.body);
    expect(surfaceOf(container)).toBeNull();
  });

  it("keeps the surface open on outside mousedown when dismissOnOutsideInteract=false", async () => {
    const { container } = render(OrderBy, {
      props: { fields, dismissOnOutsideInteract: false },
    });
    await fireEvent.click(triggerOf(container));
    expect(surfaceOf(container)).not.toBeNull();

    await fireEvent.mouseDown(document.body);
    expect(surfaceOf(container)).not.toBeNull();
  });

  it("renders no hardcoded chevron on the summary trigger", () => {
    // b031: the ▾ workaround is gone; the dropdown affordance lives in the
    // ghost Select primitive, not in a per-component character.
    const { container } = render(OrderBy, { props: { fields } });

    expect(container.querySelector(".poodle-order-by__chevron")).toBeNull();
  });
});
