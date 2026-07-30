/**
 * Anchored-overlay clipping and tracking.
 *
 * An overlay anchored to an inline trigger has two failure modes that no
 * z-index can fix: an ancestor with `overflow: hidden/auto/scroll` clips it,
 * and an ancestor that establishes a containing block (transform, filter,
 * contain, backdrop-filter, will-change) traps even `position: fixed`. The
 * library-wide answer is to portal the surface out to the theme root and
 * position it in viewport coordinates.
 *
 * That trade buys correct stacking and costs the automatic relationship with
 * the anchor: a portalled surface no longer moves when its trigger scrolls,
 * and it no longer disappears when the trigger scrolls out of a pane. This
 * module supplies both halves — `observeAnchorMovement` to reposition, and
 * `resolveClipRect` / `isAnchorClipped` to decide when the surface should hide
 * because its anchor is no longer visible.
 *
 * The geometry is pure and unit-tested; the listener wiring is a thin DOM
 * binding over it.
 */

export interface ClipRect {
  top: number;
  right: number;
  bottom: number;
  left: number;
}

export interface AnchorViewport {
  width: number;
  height: number;
}

/**
 * A point or region with no element behind it — a right-click position, a text
 * caret, a canvas hit. `contextElement` is the real element the point belongs
 * to, when there is one, so clipping and scroll tracking still work.
 */
export interface VirtualAnchor {
  getBoundingClientRect(): {
    top: number;
    right: number;
    bottom: number;
    left: number;
    width: number;
    height: number;
  };
  contextElement?: Element | null;
}

export type AnchorTarget = Element | VirtualAnchor;

/** The real element behind an anchor, if any — virtual anchors may have none. */
export function anchorElement(anchor: AnchorTarget | null | undefined): Element | null {
  if (!anchor) {
    return null;
  }

  return "nodeType" in anchor ? (anchor as Element) : (anchor.contextElement ?? null);
}

/** A zero-size anchor at a viewport point, for pointer-positioned overlays. */
export function pointAnchor(
  x: number,
  y: number,
  contextElement: Element | null = null,
): VirtualAnchor {
  return {
    contextElement,
    getBoundingClientRect: () => ({
      top: y,
      right: x,
      bottom: y,
      left: x,
      width: 0,
      height: 0,
    }),
  };
}

/** Overflow values that clip descendants. `clip` included for completeness. */
const CLIPPING_OVERFLOW = /(auto|scroll|overlay|hidden|clip)/;

/** Rect of the whole viewport — the outermost clipper. */
export function viewportClipRect(viewport: AnchorViewport): ClipRect {
  return { top: 0, right: viewport.width, bottom: viewport.height, left: 0 };
}

/** Intersection of two clip boxes. Empty results are allowed (and mean hidden). */
export function intersectClip(a: ClipRect, b: ClipRect): ClipRect {
  return {
    top: Math.max(a.top, b.top),
    right: Math.min(a.right, b.right),
    bottom: Math.min(a.bottom, b.bottom),
    left: Math.max(a.left, b.left),
  };
}

/**
 * True when the anchor has no visible area left inside its clip box — the
 * surface should hide rather than float over unrelated content.
 *
 * Touching edges count as clipped: a zero-height sliver is not something a
 * user can aim at, so an overlay hanging off it reads as detached.
 */
export function isAnchorClipped(anchor: ClipRect, clip: ClipRect): boolean {
  const visible = intersectClip(anchor, clip);
  return visible.right <= visible.left || visible.bottom <= visible.top;
}

/**
 * True when a virtual point anchor sits outside its clip box.
 *
 * Point anchors intentionally have no area, so the rectangle-intersection
 * rule above would classify every context-menu anchor as clipped. Edges are
 * excluded: an overlay anchored exactly outside a viewport/scroller boundary
 * should hide rather than detach from its invocation point.
 */
export function isPointAnchorClipped(anchor: ClipRect, clip: ClipRect): boolean {
  return (
    anchor.left <= clip.left ||
    anchor.left >= clip.right ||
    anchor.top <= clip.top ||
    anchor.top >= clip.bottom
  );
}

function isElement(node: Node | null): node is HTMLElement {
  return node !== null && node.nodeType === 1;
}

/** True when the element's own overflow clips its descendants. */
export function clipsOverflow(element: HTMLElement): boolean {
  const style = getComputedStyle(element);
  return (
    CLIPPING_OVERFLOW.test(style.overflowY) ||
    CLIPPING_OVERFLOW.test(style.overflowX) ||
    CLIPPING_OVERFLOW.test(style.overflow)
  );
}

/** True when the element can scroll — the set worth listening to for movement. */
function scrollsOverflow(element: HTMLElement): boolean {
  const style = getComputedStyle(element);
  return /(auto|scroll|overlay)/.test(`${style.overflowY} ${style.overflowX} ${style.overflow}`);
}

/**
 * True when this element establishes a containing block for `position: fixed`
 * descendants.
 *
 * This is the set named at the top of the module: the only ancestors that can
 * trap — and therefore clip — a fixed descendant. Ordinary `overflow: hidden`
 * is not among them.
 */
export function createsFixedContainingBlock(element: HTMLElement): boolean {
  const style = getComputedStyle(element);
  const set = (value: string | undefined): boolean =>
    Boolean(value) && value !== "none";

  return (
    set(style.transform) ||
    set(style.perspective) ||
    set(style.filter) ||
    set((style as unknown as { backdropFilter?: string }).backdropFilter) ||
    /paint|layout|strict|content/.test(style.contain ?? "") ||
    /transform|perspective|filter|contain/.test(style.willChange ?? "")
  );
}

/** How a subtree escapes its ancestors' clipping. */
type Escape = "static" | "absolute" | "fixed";

/**
 * Read a position, defaulting to `static`.
 *
 * An unset `position` computes to `""` in some DOM implementations rather than
 * `"static"`, and comparing against `"static"` directly then reads every plain
 * element as positioned.
 */
function positionOf(element: HTMLElement): string {
  return getComputedStyle(element).position || "static";
}

function escapeOf(element: HTMLElement): Escape {
  const position = positionOf(element);
  if (position === "fixed") return "fixed";
  if (position === "absolute") return "absolute";
  return "static";
}

/**
 * Every ancestor that clips the element, innermost first.
 *
 * Overflow alone does not make an ancestor a clipper: it also has to be an
 * ancestor the element is *laid out inside*. A `position: fixed` element is
 * clipped only by an ancestor that establishes a containing block for it, and a
 * `position: absolute` element only by a positioned one. Treating every
 * overflow ancestor as a clipper hid surfaces that were on screen — a pane that
 * expands to `fixed; inset: 0` over the app reported its trigger as clipped by
 * the smaller region its DOM ancestors occupy, so the popover opened invisibly.
 *
 * The escape is tracked while climbing rather than read once off the anchor,
 * because the anchor is usually static and it is an *ancestor* pane that is
 * fixed. Reading only the anchor's own position would miss exactly that case.
 */
export function collectClipAncestors(element: Element | null): HTMLElement[] {
  const ancestors: HTMLElement[] = [];
  if (!isElement(element)) return ancestors;

  let escape = escapeOf(element);
  let node = element.parentElement;

  while (isElement(node) && node !== document.documentElement) {
    const positioned = positionOf(node) !== "static";
    const holdsFixed = createsFixedContainingBlock(node);

    // Whether this ancestor is one the element is laid out inside at all.
    const contains =
      escape === "fixed" ? holdsFixed : escape === "absolute" ? positioned || holdsFixed : true;

    if (contains) {
      if (clipsOverflow(node)) {
        ancestors.push(node);
      }
      // The containing block is reached, so the escape is spent — and this
      // element's own position governs how it escapes in turn.
      escape = escapeOf(node);
    }

    node = node.parentElement;
  }

  return ancestors;
}

/** Every scrollable ancestor of the element, innermost first. */
export function collectScrollParents(element: Element | null): HTMLElement[] {
  const parents: HTMLElement[] = [];
  let node = element?.parentElement ?? null;

  while (isElement(node) && node !== document.documentElement) {
    if (scrollsOverflow(node)) {
      parents.push(node);
    }
    node = node.parentElement;
  }

  return parents;
}

/**
 * The box the element is actually visible within: the viewport intersected
 * with every clipping ancestor. Used to decide whether an anchor is still on
 * screen, not to place the surface — a portalled surface is placed against the
 * viewport alone.
 */
export function resolveClipRect(element: Element | null, viewport: AnchorViewport): ClipRect {
  let clip = viewportClipRect(viewport);

  for (const ancestor of collectClipAncestors(element)) {
    const rect = ancestor.getBoundingClientRect();
    clip = intersectClip(clip, {
      top: rect.top,
      right: rect.right,
      bottom: rect.bottom,
      left: rect.left,
    });
  }

  return clip;
}

export interface AnchorObservation {
  /** Stop listening. Safe to call more than once. */
  (): void;
}

/**
 * Call `onUpdate` whenever the anchor could have moved or resized: any
 * scrollable ancestor scrolling, the window scrolling or resizing, or either
 * element changing size. Returns the teardown.
 *
 * Scroll listeners are passive and capture-phase so they fire for ancestors
 * that stop propagation.
 */
export function observeAnchorMovement(
  anchor: AnchorTarget | null,
  surface: Element | null,
  onUpdate: () => void,
): AnchorObservation {
  if (typeof window === "undefined" || !anchor) {
    return () => {};
  }

  const element = anchorElement(anchor);
  const scrollTargets: Array<Element | Window> = [...collectScrollParents(element), window];

  for (const target of scrollTargets) {
    target.addEventListener("scroll", onUpdate, { passive: true, capture: true });
  }
  window.addEventListener("resize", onUpdate, { passive: true });

  let resizeObserver: ResizeObserver | null = null;
  if (typeof ResizeObserver !== "undefined") {
    resizeObserver = new ResizeObserver(onUpdate);
    if (element) {
      resizeObserver.observe(element);
    }
    if (surface) {
      resizeObserver.observe(surface);
    }
  }

  let disposed = false;
  return () => {
    if (disposed) {
      return;
    }
    disposed = true;

    for (const target of scrollTargets) {
      target.removeEventListener("scroll", onUpdate, { capture: true });
    }
    window.removeEventListener("resize", onUpdate);
    resizeObserver?.disconnect();
  };
}

/**
 * Portal target for an overlay opened from `origin`: the explicit theme root
 * when the host declares one, otherwise the nearest themed ancestor, never
 * `<html>`, falling back to `<body>`. Mounting inside the theme scope keeps
 * token inheritance intact — an overlay under a bare `<body>` would resolve
 * against whatever theme the document root happens to carry.
 */
/**
 * The z-index an anchored surface needs to clear whatever it was opened from.
 *
 * Portalling to the theme root fixes clipping but creates a layering problem:
 * the surface leaves the subtree it belongs to, so a popover opened *inside* a
 * dialog is no longer a descendant of it and has only its own token to argue
 * with. `overlay.z.menu` is 400 and `overlay.z.dialog` is 800, so the popover
 * renders behind the dialog it came from — useless, and exactly the report.
 *
 * Raising the menu token above the dialog token would only move the bug: a
 * page-level menu would then float over an unrelated modal.
 *
 * So the layer is read from the DOM at the moment of opening. The trigger is
 * still inside the dialog, so walking its ancestors finds the dialog's stacking
 * context and its z-index; the surface takes one above the highest it finds.
 * That composes to any depth — a popover in a dialog in a drawer — without any
 * context plumbing, and leaves a surface opened from ordinary page content on
 * its own token.
 */
export function resolveLayerZIndex(origin: Element | null, ownZIndex: number): number {
  let highest = 0;
  let node: Element | null = origin;

  while (node && node !== document.body) {
    const value = Number.parseInt(getComputedStyle(node).zIndex, 10);
    // `auto` parses to NaN, which is the common case and means the element
    // creates no stacking context of its own.
    if (Number.isFinite(value) && value > highest) {
      highest = value;
    }
    node = node.parentElement;
  }

  return highest >= ownZIndex ? highest + 1 : ownZIndex;
}

export function resolvePortalTarget(origin: Element | null): HTMLElement {
  const parent = origin?.parentElement ?? null;
  const explicit = parent?.closest("[data-poodle-theme-root]") as HTMLElement | null;
  const themed = parent?.closest("[data-theme]") as HTMLElement | null;

  if (explicit) {
    return explicit;
  }

  return themed && themed !== document.documentElement ? themed : document.body;
}
