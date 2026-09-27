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
        trigger={(state) => (
          <button
            type="button"
            data-testid="inner-trigger"
            aria-expanded={state.expanded}
            aria-controls={state.controls ?? undefined}
            disabled={state.disabled}
          >
            Open
          </button>
        )}
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

describe("Popover — opt-in focus trap", () => {
  function trapped(extra?: { trapFocus?: boolean }) {
    return (
      <Popover
        defaultOpen
        trapFocus={extra?.trapFocus}
        triggerIsInteractive
        trigger={(state) => (
          <button
            type="button"
            data-testid="inner-trigger"
            aria-expanded={state.expanded}
            aria-controls={state.controls ?? undefined}
            disabled={state.disabled}
          >
            Open
          </button>
        )}
      >
        <button type="button" data-testid="surface-action">
          Surface action
        </button>
        {extra?.trapFocus === true ? (
          <button type="button" data-testid="surface-next">
            Next action
          </button>
        ) : null}
      </Popover>
    );
  }

  it("does not trap Tab when trapFocus is unset", () => {
    render(trapped());

    const action = screen.getByTestId("surface-action");
    action.focus();
    const event = new KeyboardEvent("keydown", { key: "Tab", bubbles: true, cancelable: true });
    action.dispatchEvent(event);

    expect(event.defaultPrevented).toBe(false);
  });

  it("cycles Tab from last to first and Shift+Tab from first to last", () => {
    render(trapped({ trapFocus: true }));

    const first = screen.getByTestId("surface-action");
    const last = screen.getByTestId("surface-next");

    last.focus();
    fireEvent.keyDown(last, { key: "Tab" });
    expect(document.activeElement).toBe(first);

    first.focus();
    fireEvent.keyDown(first, { key: "Tab", shiftKey: true });
    expect(document.activeElement).toBe(last);
  });

  it("still restores trigger focus on Escape when the trap is on", () => {
    render(trapped({ trapFocus: true }));

    screen.getByTestId("surface-action").focus();
    fireEvent.keyDown(document, { key: "Escape" });

    expect(screen.queryByTestId("surface-action")).toBeNull();
    expect(document.activeElement).toBe(screen.getByTestId("inner-trigger"));
  });

  it("still dismisses on outside pointerdown when the trap is on", () => {
    render(trapped({ trapFocus: true }));

    fireEvent.mouseDown(document.body);
    expect(screen.queryByTestId("surface-action")).toBeNull();
    expect(document.activeElement).toBe(screen.getByTestId("inner-trigger"));
  });
});
