import "@poodle/styles/anchored-surface.css";
import {
  anchorElement,
  isAnchorClipped,
  isPointAnchorClipped,
  observeAnchorMovement,
  resolveClipRect,
  resolveOverlayPosition,
  resolvePortalTarget,
  type AnchorTarget,
} from "@poodle/headless";

import type { OverlayPlacement } from "./types";

export interface AnchoredOptions {
  /**
   * What the surface is positioned against: usually the trigger element, or a
   * virtual anchor (`pointAnchor`) for pointer-positioned overlays.
   */
  anchor: AnchorTarget | null;
  placement?: OverlayPlacement;
  offset?: number;
  /** Size the surface to the anchor's width (listbox-style pickers). */
  matchWidth?: boolean;
  /** Floor the surface's width at the anchor's, letting content grow past it. */
  minWidth?: boolean;
  /**
   * Reports the placement actually used after collision resolution. Supplying
   * it hands the consumer ownership of `data-placement` too — several
   * components publish a coarser value there (`top` / `above`) than the
   * resolver's, and two writers on one attribute would fight.
   */
  onPlacement?: ((placement: OverlayPlacement) => void) | null;
}

/**
 * Portal an anchored overlay surface out of the layout and position it in
 * viewport coordinates.
 *
 * Anchoring a surface with `position: absolute` inside its trigger's subtree
 * is only correct while no ancestor clips or transforms — in a real app pane,
 * scroller or animated card, one always does, and the surface is cut off no
 * matter its z-index. So the surface leaves the subtree: it mounts at the
 * theme root, takes `position: fixed`, and is placed by the shared
 * collision-aware resolver.
 *
 * The action then owns everything that relationship used to give for free:
 * it repositions on scroll and resize, and sets `data-anchor-hidden="true"`
 * when the anchor scrolls out of its own clipping ancestor, so a surface
 * never floats detached over unrelated content.
 *
 * Callers keep the surface element and its classes — this only moves and
 * places it. They must also widen their dismiss layer's `contains` to cover
 * the surface (see `layerContains`), since it is no longer a descendant of
 * the trigger.
 */
export function anchored(node: HTMLElement, options: AnchoredOptions) {
  if (typeof document === "undefined") {
    return { update() {}, destroy() {} };
  }

  let current = options;
  let reportedPlacement: OverlayPlacement | null = null;
  let target: HTMLElement | null = null;
  let stopObserving: (() => void) | null = null;

  function reposition(): void {
    const anchor = current.anchor;

    if (!anchor || !target) {
      return;
    }

    const anchorRect = anchor.getBoundingClientRect();
    const viewport = { width: window.innerWidth, height: window.innerHeight };

    const clipRect = resolveClipRect(anchorElement(anchor), viewport);
    const clipped =
      "nodeType" in anchor
        ? isAnchorClipped(anchorRect, clipRect)
        : isPointAnchorClipped(anchorRect, clipRect);
    if (clipped) {
      node.dataset.anchorHidden = "true";
      return;
    }

    delete node.dataset.anchorHidden;

    if (current.matchWidth) {
      node.style.width = `${anchorRect.width}px`;
    } else if (current.minWidth) {
      node.style.minWidth = `${anchorRect.width}px`;
    }

    const position = resolveOverlayPosition(
      anchorRect,
      node.getBoundingClientRect(),
      current.placement ?? "bottom-start",
      viewport,
      current.offset ?? 8,
    );

    // Round to whole pixels: a surface parked on a fractional device-pixel
    // phase rasterises its text blurrier than the same content in flow.
    node.style.top = `${Math.round(position.top)}px`;
    node.style.left = `${Math.round(position.left)}px`;

    if (position.placement === reportedPlacement) {
      return;
    }

    reportedPlacement = position.placement;

    if (current.onPlacement) {
      current.onPlacement(position.placement);
    } else {
      node.dataset.placement = position.placement;
    }
  }

  /**
   * Portal on the first tick that has an anchor — not necessarily mount.
   *
   * A surface rendered open on first paint (`defaultOpen`) mounts in the same
   * flush as its trigger, before `bind:this` has assigned the anchor. Bailing
   * out then would leave it in flow forever, which is exactly the bug this
   * action exists to fix. A surface that never gets an anchor — a nested
   * submenu flyout, positioned against its own row inside an already-portalled
   * parent — correctly stays where it is.
   */
  function attach(): void {
    if (target || !current.anchor) {
      return;
    }

    target = resolvePortalTarget(anchorElement(current.anchor) ?? node);
    node.dataset.poodleAnchored = "true";
    target.appendChild(node);

    reposition();
    stopObserving = observeAnchorMovement(current.anchor, node, reposition);
  }

  attach();

  return {
    update(next: AnchoredOptions) {
      const anchorChanged = next.anchor !== current.anchor;
      current = next;

      if (!target) {
        attach();
        return;
      }

      if (anchorChanged) {
        stopObserving?.();
        stopObserving = observeAnchorMovement(current.anchor, node, reposition);
      }

      reposition();
    },
    destroy() {
      stopObserving?.();

      if (target && node.parentNode === target) {
        target.removeChild(node);
      }
    },
  };
}
