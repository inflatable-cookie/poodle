import { render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import FieldSetHarness from "./FieldSetHarness.svelte";

describe("FieldSet (svelte)", () => {
  it("groups content in a fieldset with legend and description", () => {
    const { container } = render(FieldSetHarness, {
      props: { legend: "Billing address", description: "Used for invoices." },
    });
    const root = container.querySelector<HTMLElement>("fieldset.poodle-fieldset");
    expect(root).not.toBeNull();
    expect(container.querySelector(".poodle-fieldset__legend")?.textContent).toBe("Billing address");
    expect(container.querySelector(".poodle-fieldset__description")?.textContent).toBe("Used for invoices.");

    const legendless = render(FieldSetHarness, {});
    expect(legendless.container.querySelector(".poodle-fieldset__legend")).toBeNull();
    expect(legendless.container.querySelector(".poodle-fieldset__description")).toBeNull();
  });

  it("builds the field grid from columns and the space scale gap", () => {
    const { container } = render(FieldSetHarness, { props: { columns: 2, gap: "lg" } });
    const fields = container.querySelector<HTMLElement>(".poodle-fieldset__fields");
    expect(fields?.getAttribute("style")).toContain("grid-template-columns: repeat(2, minmax(0, 1fr))");
    expect(fields?.getAttribute("style")).toContain("column-gap: var(--poodle-space-panel-x)");
    expect(fields?.getAttribute("style")).toContain("row-gap: calc(var(--poodle-space-panel-x) + 0.5rem)");
  });

  it("spans the parent grid with a number or full width", () => {
    const spanned = render(FieldSetHarness, { props: { span: 3 } });
    expect(spanned.container.querySelector<HTMLElement>("fieldset.poodle-fieldset")?.getAttribute("style")).toContain(
      "grid-column: span 3",
    );

    const full = render(FieldSetHarness, { props: { span: "full" } });
    expect(full.container.querySelector<HTMLElement>("fieldset.poodle-fieldset")?.getAttribute("style")).toContain(
      "grid-column: 1 / -1",
    );
  });

  it("renders slotted field content inside the grid", () => {
    const { container } = render(FieldSetHarness, {});
    const fields = container.querySelector<HTMLElement>(".poodle-fieldset__fields");
    expect(fields?.querySelector(".harness-field-a")?.textContent).toBe("Field A");
    expect(fields?.querySelector(".harness-field-b")?.textContent).toBe("Field B");
  });
});
