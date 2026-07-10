/**
 * Focusable-element query. Extracted from
 * packages/svelte/components/src/internal.ts — behavior unchanged.
 */

const FOCUSABLE_SELECTOR =
  'button:not([disabled]), [href], input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])';

/**
 * Tab-key focus trap for modal surfaces: wraps focus from last→first and
 * first→last; with no focusable children, focus pins to the surface itself.
 * Call from the surface's keydown handler; non-Tab keys are ignored.
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

  if (event.shiftKey && document.activeElement === first) {
    event.preventDefault();
    last?.focus();
  }

  if (!event.shiftKey && document.activeElement === last) {
    event.preventDefault();
    first?.focus();
  }
}

export function getFocusableElements(root: HTMLElement | null): HTMLElement[] {
  if (!root) {
    return [];
  }

  return Array.from(root.querySelectorAll<HTMLElement>(FOCUSABLE_SELECTOR)).filter(
    (element) => !element.hasAttribute("hidden") && element.offsetParent !== null,
  );
}
