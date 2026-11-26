/**
 * Focusable-element query. Extracted from
 * packages/svelte/components/src/internal.ts — behavior unchanged.
 */

const NON_EDITABLE_FOCUSABLE_SELECTOR = [
  "button:not([disabled])",
  "a[href]",
  "area[href]",
  "input:not([disabled])",
  "select:not([disabled])",
  "textarea:not([disabled])",
  "summary",
  '[tabindex]:not([tabindex="-1"])',
].join(", ");

const FOCUSABLE_SELECTOR = `${NON_EDITABLE_FOCUSABLE_SELECTOR}, [contenteditable]:not([contenteditable="false" i])`;

/**
 * Tab-key focus trap for modal surfaces: wraps focus from last→first and
 * first→last; when focus sits on the surface itself or outside the focusable
 * list, treats that as a boundary and wraps in; with no focusable children,
 * focus pins to the surface itself. Call from the surface's keydown handler;
 * non-Tab keys are ignored.
 */
export function trapFocusKeydown(surface: HTMLElement | null, event: KeyboardEvent): void {
  if (event.key !== "Tab" || !surface) {
    return;
  }

  const focusable = getFocusableElements(surface);

  if (focusable.length === 0) {
    event.preventDefault();
    surface.focus();
    return;
  }

  const first = focusable[0];
  const last = focusable[focusable.length - 1];
  const active = surface.ownerDocument.activeElement as HTMLElement | null;
  const activeIndex = active ? focusable.indexOf(active) : -1;

  if (event.shiftKey) {
    if (activeIndex <= 0) {
      event.preventDefault();
      last?.focus();
    }
    return;
  }

  if (activeIndex === -1 || activeIndex === focusable.length - 1) {
    event.preventDefault();
    first?.focus();
  }
}

function isNestedContentEditable(element: HTMLElement): boolean {
  const value = element.getAttribute("contenteditable");
  if (value === null || value.toLowerCase() === "false") {
    return false;
  }

  const ancestor = element.parentElement?.closest("[contenteditable]");
  if (!ancestor) {
    return false;
  }

  const ancestorValue = ancestor.getAttribute("contenteditable");
  return ancestorValue !== null && ancestorValue.toLowerCase() !== "false";
}

function isTabbable(element: HTMLElement): boolean {
  if (element.hasAttribute("hidden") || element.getClientRects().length === 0) {
    return false;
  }

  if (element.tagName === "INPUT" && (element as HTMLInputElement).type === "hidden") {
    return false;
  }

  const tabindex = element.getAttribute("tabindex");
  if (tabindex !== null) {
    const parsed = Number.parseInt(tabindex, 10);
    if (!Number.isNaN(parsed) && parsed < 0) {
      return false;
    }
  }

  if (isNestedContentEditable(element) && !element.matches(NON_EDITABLE_FOCUSABLE_SELECTOR)) {
    return false;
  }

  return element.closest("[inert]") === null;
}

export function getFocusableElements(root: HTMLElement | null): HTMLElement[] {
  if (!root) {
    return [];
  }

  return Array.from(root.querySelectorAll<HTMLElement>(FOCUSABLE_SELECTOR)).filter(isTabbable);
}
