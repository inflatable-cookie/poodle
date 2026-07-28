import {
  createElement,
  forwardRef,
  useCallback,
  useLayoutEffect,
  useMemo,
  useRef,
  useState,
  type HTMLAttributes,
} from "react";
import { createPortal } from "react-dom";

import "@poodle/styles/anchored-surface.css";
import {
  anchorElement,
  isAnchorClipped,
  observeAnchorMovement,
  resolveClipRect,
  resolveOverlayPosition,
  resolveLayerZIndex,
  resolvePortalTarget,
  type AnchorTarget,
} from "@poodle/headless";

import type { OverlayPlacement } from "./types";

export interface AnchoredSurfaceProps extends HTMLAttributes<HTMLElement> {
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
  /** Element to render. Tooltips are inline spans; everything else is a div. */
  tag?: "div" | "span";
}

/**
 * Portalled, viewport-positioned overlay surface — the React mirror of the
 * Svelte `anchored` action. See `packages/svelte/components/src/anchored.ts`
 * for the reasoning: an anchored surface inside its trigger's subtree is
 * clipped by any scrolling or transformed ancestor, so it mounts at the theme
 * root instead and tracks its anchor explicitly.
 *
 * Every other prop is spread onto the surface element, so callers keep their
 * own classes, roles and ARIA. They must also widen their dismiss layer's
 * `contains` to cover the surface (see `layerContains`), since it is no
 * longer a descendant of the trigger.
 */
export const AnchoredSurface = forwardRef<HTMLElement, AnchoredSurfaceProps>(
  function AnchoredSurface(
    {
      anchor,
      placement = "bottom-start",
      offset = 8,
      matchWidth = false,
      minWidth = false,
      onPlacement = null,
      tag = "div",
      style,
      children,
      ...rest
    },
    forwardedRef,
  ) {
    const [surface, setSurface] = useState<HTMLElement | null>(null);
    const reported = useRef<OverlayPlacement | null>(null);

    const target = useMemo(
      () =>
        anchor && typeof document !== "undefined"
          ? resolvePortalTarget(anchorElement(anchor))
          : null,
      [anchor],
    );

    const attachRef = useCallback(
      (node: HTMLElement | null) => {
        setSurface(node);

        if (typeof forwardedRef === "function") {
          forwardedRef(node);
        } else if (forwardedRef) {
          forwardedRef.current = node;
        }
      },
      [forwardedRef],
    );

    // Read the layer from the trigger, which is still inside whatever opened
    // this surface. Portalling to the theme root escapes clipping ancestors but
    // also leaves the dialog's stacking context, so a popover opened from a
    // modal would otherwise argue its own token (menu, 400) against the
    // dialog's (800) and lose. See resolveLayerZIndex.
    useLayoutEffect(() => {
      if (!anchor || !surface) return;
      const own = Number.parseInt(getComputedStyle(surface).zIndex, 10);
      surface.style.zIndex = String(
        resolveLayerZIndex(anchorElement(anchor), Number.isFinite(own) ? own : 0),
      );
    }, [anchor, surface]);

    useLayoutEffect(() => {
      if (!anchor || !surface) return;

      const reposition = (): void => {
        const anchorRect = anchor.getBoundingClientRect();
        const viewport = { width: window.innerWidth, height: window.innerHeight };

        if (isAnchorClipped(anchorRect, resolveClipRect(anchorElement(anchor), viewport))) {
          surface.dataset.anchorHidden = "true";
          return;
        }

        delete surface.dataset.anchorHidden;

        if (matchWidth) {
          surface.style.width = `${anchorRect.width}px`;
        } else if (minWidth) {
          surface.style.minWidth = `${anchorRect.width}px`;
        }

        const position = resolveOverlayPosition(
          anchorRect,
          surface.getBoundingClientRect(),
          placement,
          viewport,
          offset,
        );

        // Round to whole pixels: a surface parked on a fractional device-pixel
        // phase rasterises its text blurrier than the same content in flow.
        surface.style.top = `${Math.round(position.top)}px`;
        surface.style.left = `${Math.round(position.left)}px`;

        if (position.placement === reported.current) {
          return;
        }

        reported.current = position.placement;

        if (onPlacement) {
          onPlacement(position.placement);
        } else {
          surface.dataset.placement = position.placement;
        }
      };

      reposition();
      return observeAnchorMovement(anchor, surface, reposition);
    }, [anchor, surface, placement, offset, matchWidth, minWidth, onPlacement]);

    if (!target) {
      return null;
    }

    return createPortal(
      createElement(
        tag,
        { ref: attachRef, "data-poodle-anchored": "true", style, ...rest },
        children,
      ),
      target,
    );
  },
);
