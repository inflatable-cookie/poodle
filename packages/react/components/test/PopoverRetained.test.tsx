import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { Popover } from "../src";

/**
 * Two shipped Popover defects, retained from the rejected g14 conformance
 * pilot (g14.005, g14.007) as focused regressions. The pilot corpus is gone;
 * these are the claims worth keeping. The Svelte pair lives in
 * `packages/svelte/components/test/PopoverRetained.svelte.test.ts` — both
 * shells carried both defects.
 */
describe("Popover — retained regressions", () => {
  it("keeps a controlled open request inert while disabled", () => {
    render(
      <Popover open disabled trigger={<span>Open</span>}>
        <button type="button" data-testid="surface-action">
          Surface action
        </button>
      </Popover>,
    );

    expect(screen.queryByTestId("surface-action")).toBeNull();
  });

  it("renders the surface for a controlled open request when enabled", () => {
    render(
      <Popover open trigger={<span>Open</span>}>
        <button type="button" data-testid="surface-action">
          Surface action
        </button>
      </Popover>,
    );

    expect(screen.getByTestId("surface-action")).toBeTruthy();
  });

  it("restores focus to the interactive trigger, not its wrapper", () => {
    render(
      <Popover
        defaultOpen
        triggerIsInteractive
        trigger={
          <button type="button" data-testid="inner-trigger">
            Open
          </button>
        }
      >
        <button type="button" data-testid="surface-action">
          Surface action
        </button>
      </Popover>,
    );

    screen.getByTestId("surface-action").focus();
    fireEvent.keyDown(document, { key: "Escape" });

    expect(document.activeElement).toBe(screen.getByTestId("inner-trigger"));
  });
});
