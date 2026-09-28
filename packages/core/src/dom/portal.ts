/**
 * Portalled-surface registry.
 *
 * Anchored overlay surfaces leave their trigger's subtree — the Svelte
 * `anchored` action and the React `AnchoredSurface` move them to the theme
 * root so no clipping ancestor can cut them off. The move costs the free DOM
 * containment the subtree used to give: a popover can no longer tell by
 * ancestry that a listbox opened inside it still belongs to it.
 *
 * Each portalled surface therefore records the anchor element it is
 * positioned against. Popover resolves its portalled descendants through
 * this registry instead of the DOM tree, so Tab from a portal returns to
 * the popover for any portalled child — Select listboxes today, any
 * anchored overlay tomorrow — without naming the child.
 *
 * Web-only DOM bookkeeping; the entries are element references held while
 * the surface is mounted and released on unmount.
 */

const portalAnchors = new Map<HTMLElement, Element | null>();

/**
 * Record a surface that was just portalled out of its subtree, against the
 * anchor element it tracks. Re-registering the same node replaces the
 * anchor; anchors move when the caller's options change while mounted.
 */
export function registerPortalledSurface(node: HTMLElement, anchor: Element | null): void {
  portalAnchors.set(node, anchor);
}

/** Release a portalled surface. Call on unmount and on portal teardown. */
export function unregisterPortalledSurface(node: HTMLElement): void {
  portalAnchors.delete(node);
}

/** The anchor element a portalled surface was recorded against, if any. */
export function portalAnchorOf(node: HTMLElement): Element | null {
  return portalAnchors.get(node) ?? null;
}

/**
 * Every live portalled surface whose anchor sits inside one of the
 * containers — directly, or transitively through another portalled surface
 * that does. A Select listbox opened from inside a Popover surface reports
 * the Select root as its anchor, and that root is DOM-inside the (portalled)
 * surface element, so one containment walk finds the whole chain whatever
 * the portal depth.
 */
export function portalledDescendantsOf(
  ...containers: Array<Element | null | undefined>
): HTMLElement[] {
  const roots = containers.filter((container): container is Element => container != null);

  if (roots.length === 0 || portalAnchors.size === 0) {
    return [];
  }

  const inside = (element: Element | null): boolean =>
    element !== null && roots.some((root) => root.contains(element));

  const found = new Set<HTMLElement>();
  let grew = true;

  while (grew) {
    grew = false;

    for (const [node, anchor] of portalAnchors) {
      if (found.has(node)) {
        continue;
      }

      // The anchor is DOM-inside one of the containers, or DOM-inside an
      // already-found portalled surface (a submenu anchored at a row of a
      // menu that is itself portalled). The fixpoint loop resolves chains
      // of any depth in registration order, independent of effect order.
      const anchoredInsideFound =
        anchor !== null && [...found].some((surface) => surface.contains(anchor));

      if (inside(anchor) || anchoredInsideFound) {
        found.add(node);
        grew = true;
      }
    }
  }

  return [...found];
}
