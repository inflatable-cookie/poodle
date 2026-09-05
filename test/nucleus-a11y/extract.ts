// g16.111 — Svelte DOM accessibility extractor.
//
// Reads the mounted document through the accessible-name algorithm and
// computed roles (dom-accessibility-api) and the ARIA attributes the
// contract names. Nothing is read from component source. Actions replay
// through DOM events; the browser default of focusing a control on pointer
// down is applied explicitly because happy-dom does not run it.

import { fireEvent } from "@testing-library/svelte";
import { computeAccessibleName, getRole, isInaccessible } from "dom-accessibility-api";
import { tick } from "svelte";

import type { A1Action, A1Scenario, A1Target, SnapshotNode } from "./contract";

const SKIPPED_ROLES = new Set(["generic", "presentation", "none"]);
const NATIVELY_FOCUSABLE = new Set(["BUTTON", "INPUT", "SELECT", "TEXTAREA", "SUMMARY"]);

const DOM_KEYS: Record<string, string> = {
  right: "ArrowRight",
  left: "ArrowLeft",
  up: "ArrowUp",
  down: "ArrowDown",
  enter: "Enter",
  space: " ",
  escape: "Escape",
  home: "Home",
  end: "End",
  tab: "Tab",
};

function accessibleElements(): HTMLElement[] {
  const out: HTMLElement[] = [];
  for (const element of Array.from(document.body.querySelectorAll<HTMLElement>("*"))) {
    if (isInaccessible(element)) continue;
    const role = getRole(element);
    if (role === null || SKIPPED_ROLES.has(role)) continue;
    out.push(element);
  }
  return out;
}

function trimmedName(element: HTMLElement): string | null {
  const name = computeAccessibleName(element).trim();
  return name.length > 0 ? name : null;
}

function isDisabled(element: HTMLElement): boolean {
  return (
    (element as HTMLButtonElement).disabled === true ||
    element.getAttribute("aria-disabled") === "true"
  );
}

function tristate(value: string | null): boolean | "mixed" | null {
  if (value === "true") return true;
  if (value === "false") return false;
  if (value === "mixed") return "mixed";
  return null;
}

function declaredState(element: HTMLElement, state: string): boolean | "mixed" | null {
  switch (state) {
    case "checked": {
      const aria = element.getAttribute("aria-checked");
      if (aria !== null) return tristate(aria);
      if (element.tagName === "INPUT" && ((element as HTMLInputElement).type === "checkbox" || (element as HTMLInputElement).type === "radio")) {
        const input = element as HTMLInputElement;
        return input.indeterminate ? "mixed" : input.checked;
      }
      return null;
    }
    case "expanded":
      return tristate(element.getAttribute("aria-expanded"));
    case "selected": {
      const aria = element.getAttribute("aria-selected");
      if (aria !== null) return tristate(aria);
      const checked = element.getAttribute("aria-checked");
      if (checked !== null) return tristate(checked);
      if (element instanceof HTMLOptionElement) return element.selected;
      return null;
    }
    case "disabled":
      return isDisabled(element);
    case "invalid":
      return tristate(element.getAttribute("aria-invalid"));
    case "busy":
      return tristate(element.getAttribute("aria-busy"));
    default:
      throw new Error(`A1 scenario declares an unknown state ${state}`);
  }
}

function isSequentialTabStop(element: HTMLElement): boolean {
  if (isDisabled(element)) return false;
  const tabindex = element.getAttribute("tabindex");
  if (tabindex !== null) return Number(tabindex) >= 0;
  if (element instanceof HTMLInputElement && element.type === "hidden") return false;
  if (element instanceof HTMLAnchorElement) return element.hasAttribute("href");
  return NATIVELY_FOCUSABLE.has(element.tagName);
}

function collapsedText(text: string): string | null {
  const collapsed = text.split(/\s+/).filter(Boolean).join(" ");
  return collapsed.length > 0 ? collapsed : null;
}

function valueText(element: HTMLElement, role: string): string | null {
  const declared = element.getAttribute("aria-valuetext");
  if (declared !== null) return collapsedText(declared);
  if (role === "combobox" || role === "textbox") {
    if (element instanceof HTMLSelectElement) return collapsedText(element.selectedOptions[0]?.label ?? "");
    if (element instanceof HTMLInputElement || element instanceof HTMLTextAreaElement) {
      return collapsedText(element.value);
    }
    return collapsedText(element.textContent ?? "");
  }
  return null;
}

function resolveTargets(element: HTMLElement, attribute: string, elements: HTMLElement[]): number[] {
  const reference = element.getAttribute(attribute);
  if (reference === null) return [];
  return reference
    .split(/\s+/)
    .filter(Boolean)
    .map((id) => {
      const target = document.getElementById(id);
      return target === null ? -1 : elements.indexOf(target as HTMLElement);
    });
}

function numberOrNull(value: string | null): number | null {
  if (value === null) return null;
  const parsed = Number(value);
  return Number.isFinite(parsed) ? parsed : null;
}

export function extractSnapshotNodes(scenario: A1Scenario): SnapshotNode[] {
  const elements = accessibleElements();
  let focusOrder = 0;
  return elements.map((element) => {
    const role = getRole(element) as string;
    const states: SnapshotNode["states"] = {};
    for (const state of scenario.declared_states) states[state] = declaredState(element, state);
    const tabStop = isSequentialTabStop(element);
    const order = tabStop ? focusOrder++ : null;
    return {
      role,
      name: trimmedName(element),
      value: numberOrNull(element.getAttribute("aria-valuenow")),
      value_text: valueText(element, role),
      states,
      relationships: {
        controls: resolveTargets(element, "aria-controls", elements),
        labelled_by: resolveTargets(element, "aria-labelledby", elements),
        described_by: resolveTargets(element, "aria-describedby", elements),
      },
      level: numberOrNull(element.getAttribute("aria-level") ?? element.getAttribute("data-level")),
      orientation: element.getAttribute("aria-orientation"),
      focus_order: order,
      focused: document.activeElement === element,
    };
  });
}

function resolveTarget(target: A1Target): HTMLElement {
  const match = accessibleElements().find(
    (element) =>
      (target.role === undefined || getRole(element) === target.role) &&
      (target.name === undefined || trimmedName(element) === target.name),
  );
  if (match === undefined) throw new Error(`no mounted element matches A1 target ${JSON.stringify(target)}`);
  return match;
}

/// Let Svelte flush and the requestAnimationFrame shim (a macrotask in the
/// shared vitest setup) run twice.
export async function settle(): Promise<void> {
  await tick();
  await new Promise((resolve) => setTimeout(resolve, 0));
  await tick();
  await new Promise((resolve) => setTimeout(resolve, 0));
}

export async function replayActions(actions: A1Action[]): Promise<void> {
  for (const action of actions) {
    const element = resolveTarget(action.target);
    if (action.type === "pointer_activate") {
      await fireEvent.pointerDown(element, { button: 0, pointerId: 1 });
      await fireEvent.mouseDown(element, { button: 0 });
      if (!isDisabled(element)) element.focus();
      await fireEvent.pointerUp(element, { button: 0, pointerId: 1 });
      await fireEvent.mouseUp(element, { button: 0 });
      element.click();
    } else {
      const key = DOM_KEYS[action.key];
      if (key === undefined) throw new Error(`A1 scenario uses an unmapped key ${action.key}`);
      element.focus();
      await fireEvent.keyDown(element, { key });
      await fireEvent.keyUp(element, { key });
    }
    await settle();
  }
}
