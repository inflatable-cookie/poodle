import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { FormActions } from "../src/FormActions";
import type { FormActionDangerItem } from "../src/types";

describe("FormActions (react)", () => {
  it("applies alignment and exposes the separation and border flags", () => {
    const { container } = render(
      <FormActions align="between" showTopSeparation={false} showTopBorder>
        <button type="button">Cancel</button>
        <button type="button">Save changes</button>
      </FormActions>,
    );
    const root = container.querySelector<HTMLElement>(".poodle-form-actions");
    expect(root?.getAttribute("data-align")).toBe("between");
    expect(root?.getAttribute("data-top-separation")).toBe("false");
    expect(root?.getAttribute("data-top-border")).toBe("true");

    const buttons = [...container.querySelectorAll("button")].map((el) => el.textContent);
    expect(buttons).toContain("Cancel");
    expect(buttons).toContain("Save changes");
  });

  it("keeps danger content inline when no overflow items are supplied", () => {
    const { container } = render(
      <FormActions danger={<button type="button" className="harness-danger">Delete</button>}>
        <button type="button">Save changes</button>
      </FormActions>,
    );
    const danger = container.querySelector<HTMLElement>(".poodle-form-actions__danger");
    expect(danger?.getAttribute("data-mode")).toBe("inline");
    expect(danger?.querySelector(".harness-danger")?.textContent).toBe("Delete");
    expect(container.querySelector(".poodle-form-actions__danger-menu")).toBeNull();
  });

  it("collapses danger into the overflow menu only when both snippet and items are present", () => {
    const dangerItems: FormActionDangerItem[] = [{ value: "delete", label: "Delete", onSelect: () => {} }];
    const both = render(
      <FormActions
        danger={<button type="button">Delete</button>}
        dangerItems={dangerItems}
      >
        <button type="button">Save changes</button>
      </FormActions>,
    );
    expect(both.container.querySelector<HTMLElement>(".poodle-form-actions__danger")?.getAttribute("data-mode")).toBe(
      "responsive",
    );
    const menu = both.container.querySelector<HTMLElement>(".poodle-form-actions__danger-menu");
    expect(menu?.getAttribute("data-visible")).toBe("responsive");
    expect(menu?.querySelector(".poodle-menu__trigger")).not.toBeNull();

    const itemsOnly = render(
      <FormActions dangerItems={dangerItems}>
        <button type="button">Save changes</button>
      </FormActions>,
    );
    expect(itemsOnly.container.querySelector(".poodle-form-actions__danger")).toBeNull();
    const alwaysMenu = itemsOnly.container.querySelector<HTMLElement>(".poodle-form-actions__danger-menu");
    expect(alwaysMenu?.getAttribute("data-visible")).toBe("always");
  });

  it("routes overflow menu selection to the matching item callback", () => {
    const onDelete = vi.fn();
    const onReset = vi.fn();
    const dangerItems: FormActionDangerItem[] = [
      { value: "delete", label: "Delete", onSelect: onDelete },
      { value: "reset", label: "Reset", onSelect: onReset },
    ];
    const { container } = render(
      <FormActions
        danger={<button type="button">Delete</button>}
        dangerItems={dangerItems}
      >
        <button type="button">Save changes</button>
      </FormActions>,
    );

    const trigger = container.querySelector<HTMLElement>(".poodle-menu__trigger") as HTMLElement;
    fireEvent.click(trigger);

    const surface = document.querySelector<HTMLElement>(".poodle-menu-surface");
    expect(surface).not.toBeNull();
    expect(surface?.getAttribute("role")).toBe("menu");

    const items = [...surface.querySelectorAll<HTMLElement>(".poodle-menu-surface__item")];
    expect(items.map((el) => el.textContent)).toEqual(["Delete", "Reset"]);

    fireEvent.click(items[1]);
    expect(onReset).toHaveBeenCalledTimes(1);
    expect(onDelete).not.toHaveBeenCalled();
  });
});
