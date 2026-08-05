import type { OverlayPlacement, RectLike } from "../position.ts";

export interface OverlayViewportRect {
  x: number;
  y: number;
  width: number;
  height: number;
  top: number;
  right: number;
  bottom: number;
  left: number;
}

export interface OverlaySurfaceGeometry {
  surfaceId: string;
  rect: OverlayViewportRect;
  placement: OverlayPlacement | null;
  visible: boolean;
}

export type OverlaySurfaceGeometryChange =
  | { type: "upsert"; surface: OverlaySurfaceGeometry }
  | { type: "remove"; surfaceId: string };

export type OverlaySurfaceGeometryChangeHandler = (
  change: OverlaySurfaceGeometryChange,
) => void;

export interface OverlaySurfaceGeometryObserverOptions {
  onChange?: OverlaySurfaceGeometryChangeHandler | null;
  placement?: OverlayPlacement | null;
  /** Anchored primitives report explicitly after their first positioning. */
  reportInitial?: boolean;
}

export interface OverlaySurfaceGeometryObserver {
  update(options: OverlaySurfaceGeometryObserverOptions): void;
  report(): void;
  destroy(): void;
}

export function copyOverlayViewportRect(rect: RectLike & { x?: number; y?: number }): OverlayViewportRect {
  return {
    x: Number.isFinite(rect.x) ? rect.x! : rect.left,
    y: Number.isFinite(rect.y) ? rect.y! : rect.top,
    width: rect.width,
    height: rect.height,
    top: rect.top,
    right: rect.right,
    bottom: rect.bottom,
    left: rect.left,
  };
}

/**
 * Observe one private web surface without exposing its element to consumers.
 * The callback receives copied viewport numbers and an opaque caller-owned id.
 */
export function observeOverlaySurfaceGeometry(
  node: HTMLElement,
  surfaceId: string,
  options: OverlaySurfaceGeometryObserverOptions,
): OverlaySurfaceGeometryObserver {
  let current = options;
  let last: OverlaySurfaceGeometry | null = null;
  let frame: number | null = null;
  let resizeObserver: ResizeObserver | null = null;
  let mutationObserver: MutationObserver | null = null;
  let listening = false;
  let destroyed = false;

  const schedule = (): void => {
    if (destroyed || !current.onChange || frame !== null) {
      return;
    }

    if (typeof requestAnimationFrame === "undefined") {
      report();
      return;
    }

    frame = requestAnimationFrame(() => {
      frame = null;
      report();
    });
  };

  const attach = (): void => {
    if (listening || !current.onChange || typeof window === "undefined") {
      return;
    }

    listening = true;
    window.addEventListener("scroll", schedule, { passive: true, capture: true });
    window.addEventListener("resize", schedule, { passive: true });

    if (typeof ResizeObserver !== "undefined") {
      resizeObserver = new ResizeObserver(schedule);
      resizeObserver.observe(node);
    }

    if (typeof MutationObserver !== "undefined") {
      mutationObserver = new MutationObserver(schedule);
      mutationObserver.observe(node, {
        attributes: true,
        attributeFilter: ["class", "style", "data-anchor-hidden", "data-placement"],
      });
    }
  };

  const detach = (): void => {
    if (!listening || typeof window === "undefined") {
      return;
    }

    listening = false;
    window.removeEventListener("scroll", schedule, { capture: true });
    window.removeEventListener("resize", schedule);
    resizeObserver?.disconnect();
    resizeObserver = null;
    mutationObserver?.disconnect();
    mutationObserver = null;

    if (frame !== null && typeof cancelAnimationFrame !== "undefined") {
      cancelAnimationFrame(frame);
      frame = null;
    }
  };

  const remove = (handler = current.onChange): void => {
    if (!last || !handler) {
      return;
    }

    handler({ type: "remove", surfaceId });
    last = null;
  };

  const report = (): void => {
    if (destroyed || !current.onChange) {
      return;
    }

    const rect = copyOverlayViewportRect(node.getBoundingClientRect());
    const next: OverlaySurfaceGeometry = {
      surfaceId,
      rect,
      placement: current.placement ?? null,
      visible:
        node.dataset.anchorHidden !== "true" &&
        rect.width > 0 &&
        rect.height > 0,
    };

    if (last && equalOverlaySurfaceGeometry(last, next)) {
      return;
    }

    last = next;
    current.onChange({ type: "upsert", surface: next });
  };

  attach();
  if (current.reportInitial !== false) {
    schedule();
  }

  return {
    update(next) {
      if (destroyed) {
        return;
      }

      const previousHandler = current.onChange;
      const handlerChanged = previousHandler !== next.onChange;
      current = next;

      if (handlerChanged) {
        remove(previousHandler);
        detach();
        attach();
      }

      schedule();
    },
    report,
    destroy() {
      if (destroyed) {
        return;
      }

      remove();
      destroyed = true;
      detach();
    },
  };
}

export function equalOverlaySurfaceGeometry(
  left: OverlaySurfaceGeometry,
  right: OverlaySurfaceGeometry,
): boolean {
  return (
    left.surfaceId === right.surfaceId &&
    left.placement === right.placement &&
    left.visible === right.visible &&
    left.rect.x === right.rect.x &&
    left.rect.y === right.rect.y &&
    left.rect.width === right.rect.width &&
    left.rect.height === right.rect.height &&
    left.rect.top === right.rect.top &&
    left.rect.right === right.rect.right &&
    left.rect.bottom === right.rect.bottom &&
    left.rect.left === right.rect.left
  );
}
