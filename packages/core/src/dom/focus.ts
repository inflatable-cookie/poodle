/**
 * Focusable-element query. Extracted from
 * packages/svelte/components/src/internal.ts — behavior unchanged.
 */

const FOCUSABLE_SELECTOR =
  'button:not([disabled]), [href], input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])';

export function getFocusableElements(root: HTMLElement | null): HTMLElement[] {
  if (!root) {
    return [];
  }

  return Array.from(root.querySelectorAll<HTMLElement>(FOCUSABLE_SELECTOR)).filter(
    (element) => !element.hasAttribute("hidden") && element.offsetParent !== null,
  );
}
