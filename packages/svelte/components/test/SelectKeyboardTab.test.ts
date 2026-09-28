import { cleanup, fireEvent, render } from "@testing-library/svelte";
import { afterEach, describe, expect, it, vi } from "vitest";
import type {
  HistoryContinuation,
  HistoryPathPage,
} from "@inflatable-cookie/poodle-core";
import { getFocusableElements } from "@inflatable-cookie/poodle-core";

import HistoryCenterHostHarness from "./HistoryCenterHostHarness.svelte";
import SelectInDialogHarness from "./SelectInDialogHarness.svelte";

/**
 * The real keyboard path for Select-in-a-trap (planner ruling on ab608031):
 * DOM focus stays on the trigger or input while the highlight moves by
 * `aria-activedescendant` — options are `tabindex="-1"`, so Tab always
 * originates in-surface, where the container's own trap (HistoryCenter's
 * section, Dialog's surface) governs it. Driving that path — focus the
 * trigger, open by keyboard, move the highlight, press Tab — must close the
 * dropdown without changing the value while focus stays inside the trap.
 * No test here focuses a listbox option directly; that path is not
 * reachable by keyboard and the contracts do not define it.
 */

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

function rowByEntry(entryId: string): HTMLElement {
  const row = document.querySelector(
    `[data-row-kind="entry"][data-row-entry="${entryId}"]`,
  ) as HTMLElement;
  if (row === null) {
    throw new Error(`no entry row for ${entryId}`);
  }
  return row;
}

function openListbox(): HTMLElement {
  const listbox = document.querySelector('[role="listbox"]') as HTMLElement;
  if (!listbox) throw new Error("listbox did not open");
  return listbox;
}

// The helpers query `document` globally; isolate like HistoryCenter.test.ts.
afterEach(cleanup);

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

async function pressTab(target: HTMLElement): Promise<boolean> {
  const prevented = (await fireEvent.keyDown(target, { key: "Tab" })) === false;
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

describe("Select keyboard Tab in a trap (svelte)", () => {
  it("closes the HistoryCenter picker on Tab with the pick unchanged and focus in the section", async () => {
    render(HistoryCenterHostHarness, {
      props: {
        pages: twoForkPages,
        continuationsByEntry: twoForkContinuations,
        runsByFork: twoForkRuns,
        defaultOpen: true,
      },
    });

    // Two forks at c2: the picker row carries an enabled Select.
    await fireEvent.click(
      rowByEntry("c2").querySelector('[data-part="fork-disclosure"]') as HTMLElement,
    );
    const picker = document.querySelector('[data-part="picker"]') as HTMLElement;
    const trigger = picker.querySelector(".poodle-select__trigger") as HTMLButtonElement;
    expect(trigger.disabled).toBe(false);
    const pickedBefore = trigger.textContent;

    // The real keyboard path: focus stays on the trigger throughout. Native
    // .focus(): fireEvent.focus dispatches an event without moving
    // happy-dom focus.
    trigger.focus();
    await fireEvent.keyDown(trigger, { key: "Enter" });
    // Enter activates a button through a click in browsers.
    await fireEvent.click(trigger);
    expect(document.querySelector('[role="listbox"]')).not.toBeNull();

    await fireEvent.keyDown(trigger, { key: "ArrowDown" });
    expect(
      document.querySelector('[role="listbox"] [data-highlighted="true"]'),
    ).not.toBeNull();

    // The section trap passes a mid-list Tab through, so traversal runs: the
    // dropdown closes without committing the highlight, and real document
    // order carries focus to the picker's own actions menu — still inside
    // the section, never out to the page.
    const prevented = await pressTab(trigger);
    expect(prevented).toBe(false);
    expect(document.querySelector('[role="listbox"]')).toBeNull();
    expect(trigger.textContent).toBe(pickedBefore);
    const landed = document.activeElement as HTMLElement;
    expect(landed.classList.contains("poodle-menu__trigger")).toBe(true);
    expect(
      (document.querySelector(".poodle-history-center") as HTMLElement).contains(landed),
    ).toBe(true);
  });

  it("closes a Dialog Select on Tab with the value unchanged and focus in the dialog", async () => {
    const onValueChange = vi.fn();
    render(SelectInDialogHarness, { props: { onValueChange } });

    const trigger = document.querySelector(".poodle-select__trigger") as HTMLButtonElement;

    trigger.focus();
    await fireEvent.keyDown(trigger, { key: "Enter" });
    // Enter activates a button through a click in browsers.
    await fireEvent.click(trigger);
    openListbox();

    await fireEvent.keyDown(trigger, { key: "ArrowDown" });
    expect(
      document.querySelector('[role="listbox"] [data-highlighted="true"]'),
    ).not.toBeNull();

    // Real traversal carries focus to the trailing action — still inside
    // the dialog surface — while the dropdown closes without committing.
    const prevented = await pressTab(trigger);
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
