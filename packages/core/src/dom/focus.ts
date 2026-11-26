/**
 * Focusable-element query. Extracted from
 * packages/svelte/components/src/internal.ts — behavior unchanged.
 */

const FOCUSABLE_SELECTOR = [
  "button:not([disabled])",
  "a[href]",
  "area[href]",
  "input:not([disabled])",
  "select:not([disabled])",
  "textarea:not([disabled])",
  '[contenteditable]:not([contenteditable="false"])',
  "summary",
  '[tabindex]:not([tabindex="-1"])',
].join(", ");

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

export function getFocusableElements(root: HTMLElement | null): HTMLElement[] {
  if (!root) {
    return [];
  }

  return Array.from(root.querySelectorAll<HTMLElement>(FOCUSABLE_SELECTOR)).filter(
    (element) => !element.hasAttribute("hidden") && element.getClientRects().length > 0,
  );
}
