import { fireEvent, render } from "@testing-library/react";
import { useState } from "react";
import { describe, expect, it, vi } from "vitest";
import type {
  HistoryContinuation,
  HistoryPathPage,
} from "@inflatable-cookie/poodle-core";

import { getFocusableElements } from "@inflatable-cookie/poodle-core";
import { Dialog } from "../src/Dialog";
import { HistoryCenter } from "../src/HistoryCenter";
import { Select } from "../src/Select";
import type { SelectItems } from "../src/types";

/**
 * The real keyboard path for Select-in-a-trap (planner ruling on ab608031):
 * DOM focus stays on the trigger or input while the highlight moves by
 * `aria-activedescendant` — options are `tabIndex={-1}`, so Tab always
 * originates in-surface, where the container's own trap (HistoryCenter's
 * section, Dialog's surface) governs it. Driving that path — focus the
 * trigger, open by keyboard, move the highlight, press Tab — must close the
 * dropdown without changing the value while focus stays inside the trap.
 * No test here focuses a listbox option directly; that path is not
 * reachable by keyboard and the contracts do not define it.
 */

const options: SelectItems = [
  { value: "alpha", label: "Alpha" },
  { value: "beta", label: "Beta" },
];

function page(
  entries: HistoryPathPage["entries"],
  precedingContinuationCount = 1,
): HistoryPathPage {
  return { entries, offset: 0, precedingContinuationCount, truncatedBefore: false, truncatedAfter: false };
}

const continuation = (
  entryId: string,
  overrides: Partial<HistoryContinuation> = {},
): HistoryContinuation => ({
  entryId,
  label: entryId,
  preferred: false,
  entryCount: 2,
  branchId: `b-${entryId}`,
  branchName: null,
  ...overrides,
});

const twoForkPages = [
  page([
    { id: "c3", label: "Current draft", position: "current", continuationCount: 0 },
    { id: "c2", label: "Arranged intro", position: "past", continuationCount: 3 },
    { id: "c1", label: "Committed mix 1", position: "past", continuationCount: 1 },
  ]),
];

const twoForkContinuations: Record<string, HistoryContinuation[]> = {
  c2: [
    continuation("l1", { label: "Lead intro", branchName: "feature/lead" }),
    continuation("x1", { label: "Alt intro", preferred: true, branchName: "feature/alt", entryCount: 1 }),
  ],
};

const twoForkRuns: Record<string, HistoryPathPage[]> = {
  x1: [
    page([
      { id: "x2", label: "Alt mix", position: "past", continuationCount: 0 },
      { id: "x1", label: "Alt intro", position: "past", continuationCount: 1 },
    ]),
  ],
  l1: [
    page([
      { id: "l2", label: "Lead mix", position: "past", continuationCount: 0 },
      { id: "l1", label: "Lead intro", position: "past", continuationCount: 1 },
    ]),
  ],
};

/** Minimal host simulation: resolve both host operations synchronously. */
function HistoryHost() {
  const [continuationsResult, setContinuationsResult] = useState<{
    entryId: string;
    continuations: HistoryContinuation[];
  } | null>(null);
  const [runResult, setRunResult] = useState<{
    fromEntryId: string;
    pages: HistoryPathPage[];
  } | null>(null);
  return (
    <HistoryCenter
      pages={twoForkPages}
      continuationsResult={continuationsResult}
      runResult={runResult}
      defaultOpen
      onLoadContinuations={(entryId) =>
        setContinuationsResult({ entryId, continuations: twoForkContinuations[entryId] ?? [] })
      }
      onLoadContinuationRun={(fromEntryId) =>
        setRunResult({ fromEntryId, pages: twoForkRuns[fromEntryId] ?? [] })
      }
    />
  );
}

/**
 * A Tab keypress with real focus traversal. happy-dom performs no default
 * focus movement, so after an uninterrupted key the helper advances focus
 * to the next tabbable element in document order itself — the sequential
 * navigation the browser would run, minus `tabindex="-1"` and hidden
 * inputs, which native Tab skips. Returns whether any handler prevented
 * the key, in which case focus is left exactly where the handler put it.
 */
function tabbables(): HTMLElement[] {
  return getFocusableElements(document.body).filter(
    (element) =>
      element.getAttribute("tabindex") !== "-1" &&
      !(element instanceof HTMLInputElement && element.type === "hidden"),
  );
}

function pressTab(target: HTMLElement): boolean {
  const prevented = fireEvent.keyDown(target, { key: "Tab" }) === false;
  if (!prevented) {
    const order = tabbables();
    const next = order[order.indexOf(document.activeElement as HTMLElement) + 1] ?? null;
    if (next) {
      next.focus();
    } else {
      (document.activeElement as HTMLElement)?.blur();
    }
  }
  return prevented;
}

function rowByEntry(entryId: string): HTMLElement {
  const row = document.querySelector(
    `[data-row-kind="entry"][data-row-entry="${entryId}"]`,
  ) as HTMLElement;
  if (!row) throw new Error(`no entry row for ${entryId}`);
  return row;
}

describe("Select keyboard Tab in a trap (react)", () => {
  it("closes the HistoryCenter picker on Tab with the pick unchanged and focus in the section", () => {
    render(<HistoryHost />);

    // Two forks at c2: the picker row carries an enabled Select.
    fireEvent.click(
      rowByEntry("c2").querySelector('[data-part="fork-disclosure"]') as HTMLElement,
    );
    const picker = document.querySelector('[data-part="picker"]') as HTMLElement;
    const trigger = picker.querySelector(".poodle-select__trigger") as HTMLButtonElement;
    expect(trigger.disabled).toBe(false);
    const pickedBefore = trigger.textContent;

    // The real keyboard path: focus stays on the trigger throughout.
    // Native .focus(): fireEvent.focus dispatches an event without moving
    // happy-dom focus.
    trigger.focus();
    fireEvent.keyDown(trigger, { key: "Enter" });
    // Enter activates a button through a click in browsers.
    fireEvent.click(trigger);
    expect(document.querySelector('[role="listbox"]')).not.toBeNull();

    fireEvent.keyDown(trigger, { key: "ArrowDown" });
    expect(
      document.querySelector('[role="listbox"] [data-highlighted="true"]'),
    ).not.toBeNull();

    // Non-modal pass-through: the container's own trap governs the key, the
    // dropdown closes without committing the highlight, and DOM focus never
    // left the trigger inside the section.
    const prevented = pressTab(trigger);
    expect(prevented).toBe(false);
    expect(document.querySelector('[role="listbox"]')).toBeNull();
    expect(trigger.textContent).toBe(pickedBefore);
    const landed = document.activeElement as HTMLElement;
    expect(landed.classList.contains("poodle-menu__trigger")).toBe(true);
    expect(
      (document.querySelector(".poodle-history-center") as HTMLElement).contains(landed),
    ).toBe(true);
  });

  it("closes a Dialog Select on Tab with the value unchanged and focus in the dialog", () => {
    const onValueChange = vi.fn();
    const { container } = render(
      <div data-poodle-theme-root>
        <Dialog open title="Pick one">
          <Select options={options} native={false} ariaLabel="Pick" onValueChange={onValueChange} />
          <button type="button" data-testid="after-action">
            After action
          </button>
        </Dialog>
      </div>,
    );

    const trigger = container.querySelector(".poodle-select__trigger") as HTMLButtonElement;

    trigger.focus();
    fireEvent.keyDown(trigger, { key: "Enter" });
    // Enter activates a button through a click in browsers.
    fireEvent.click(trigger);
    expect(document.querySelector('[role="listbox"]')).not.toBeNull();

    fireEvent.keyDown(trigger, { key: "ArrowDown" });
    expect(
      document.querySelector('[role="listbox"] [data-highlighted="true"]'),
    ).not.toBeNull();

    const prevented = pressTab(trigger);
    expect(prevented).toBe(false);
    expect(document.querySelector('[role="listbox"]')).toBeNull();
    expect(onValueChange).not.toHaveBeenCalled();
    const landed = document.activeElement as HTMLElement;
    expect(landed.getAttribute("data-testid")).toBe("after-action");
    expect(
      (document.querySelector(".poodle-dialog__surface") as HTMLElement).contains(landed),
    ).toBe(true);
  });
});
