import { fireEvent, render, screen } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import Harness from "./PopoverRetainedHarness.svelte";

/**
 * Two shipped Popover defects, retained from the rejected g14 conformance
 * pilot (g14.005, g14.007) as focused regressions. The pilot corpus is gone;
 * these are the claims worth keeping.
 */
describe("Popover — retained regressions", () => {
  it("keeps a controlled open request inert while disabled", async () => {
    // g14.005: a controlled `open: true` host with `disabled: true` rendered
    // the surface anyway, past the machine's own guard.
    render(Harness, { props: { open: true, disabled: true } });

    expect(screen.queryByTestId("surface-action")).toBeNull();
  });

  it("renders the surface for a controlled open request when enabled", async () => {
    render(Harness, { props: { open: true } });

    expect(screen.getByTestId("surface-action")).toBeTruthy();
  });

  it("restores focus to the interactive trigger, not its wrapper", async () => {
    // g14.007: with `triggerIsInteractive` the wrapper observes clicks without
    // becoming a button. Restoring focus to the wrapper left the operator on
    // something Enter could not activate.
    render(Harness, { props: { defaultOpen: true, triggerIsInteractive: true } });

    const surfaceAction = screen.getByTestId("surface-action");
    surfaceAction.focus();
    await fireEvent.keyDown(document, { key: "Escape" });

    expect(document.activeElement).toBe(screen.getByTestId("inner-trigger"));
  });
});
