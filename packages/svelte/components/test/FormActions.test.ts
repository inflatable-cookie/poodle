import { fireEvent, render, waitFor } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import FormActionsHarness from "./FormActionsHarness.svelte";
import type { FormActionDangerItem } from "../src/types";

describe("FormActions (svelte)", () => {
  it("applies alignment and exposes the separation and border flags", () => {
    const { container } = render(FormActionsHarness, {
      props: { align: "between", showTopSeparation: false, showTopBorder: true },
    });
    const root = container.querySelector<HTMLElement>(".poodle-form-actions");
    expect(root?.getAttribute("data-align")).toBe("between");
    expect(root?.getAttribute("data-top-separation")).toBe("false");
    expect(root?.getAttribute("data-top-border")).toBe("true");

    const buttons = [...container.querySelectorAll("button")].map((el) => el.textContent);
    expect(buttons).toContain("Cancel");
    expect(buttons).toContain("Save changes");
  });

  it("keeps danger content inline when no overflow items are supplied", () => {
    const { container } = render(FormActionsHarness, { props: { showDanger: true } });
    const danger = container.querySelector<HTMLElement>(".poodle-form-actions__danger");
    expect(danger?.getAttribute("data-mode")).toBe("inline");
    expect(danger?.querySelector(".harness-danger")?.textContent).toBe("Delete");
    expect(container.querySelector(".poodle-form-actions__danger-menu")).toBeNull();
  });

  it("collapses danger into the overflow menu only when both snippet and items are present", () => {
    const dangerItems: FormActionDangerItem[] = [{ value: "delete", label: "Delete", onSelect: () => {} }];
    const both = render(FormActionsHarness, { props: { showDanger: true, dangerItems } });
    expect(both.container.querySelector<HTMLElement>(".poodle-form-actions__danger")?.getAttribute("data-mode")).toBe(
      "responsive",
    );
    const menu = both.container.querySelector<HTMLElement>(".poodle-form-actions__danger-menu");
    expect(menu?.getAttribute("data-visible")).toBe("responsive");
    expect(menu?.querySelector(".poodle-menu__trigger")).not.toBeNull();

    const itemsOnly = render(FormActionsHarness, { props: { dangerItems } });
    expect(itemsOnly.container.querySelector(".poodle-form-actions__danger")).toBeNull();
    const alwaysMenu = itemsOnly.container.querySelector<HTMLElement>(".poodle-form-actions__danger-menu");
    expect(alwaysMenu?.getAttribute("data-visible")).toBe("always");
  });

  it("routes overflow menu selection to the matching item callback", async () => {
    const onDelete = vi.fn();
    const onReset = vi.fn();
    const dangerItems: FormActionDangerItem[] = [
      { value: "delete", label: "Delete", onSelect: onDelete },
      { value: "reset", label: "Reset", onSelect: onReset },
    ];
    const { container } = render(FormActionsHarness, { props: { showDanger: true, dangerItems } });

    const trigger = container.querySelector<HTMLElement>(".poodle-menu__trigger") as HTMLElement;
    await fireEvent.click(trigger);
    const surface = await waitFor(() => {
      const el = document.querySelector<HTMLElement>(".poodle-menu-surface");
      expect(el).not.toBeNull();
      return el as HTMLElement;
    });
    expect(surface.getAttribute("role")).toBe("menu");

    const items = [...surface.querySelectorAll<HTMLElement>(".poodle-menu-surface__item")];
    expect(items.map((el) => el.textContent?.trim())).toEqual(["Delete", "Reset"]);

    await fireEvent.click(items[1]);
    expect(onReset).toHaveBeenCalledTimes(1);
    expect(onDelete).not.toHaveBeenCalled();
  });
});
