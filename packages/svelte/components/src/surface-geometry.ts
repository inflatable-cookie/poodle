import {
  createInstanceId,
  observeOverlaySurfaceGeometry,
  type OverlayPlacement,
  type OverlaySurfaceGeometryChangeHandler,
} from "@inflatable-cookie/poodle-core";

export interface SurfaceGeometryOptions {
  onSurfaceGeometryChange?: OverlaySurfaceGeometryChangeHandler | null;
  placement?: OverlayPlacement | null;
}

/** Observe a private in-flow surface such as a nested submenu flyout. */
export function surfaceGeometry(node: HTMLElement, options: SurfaceGeometryOptions) {
  let current = options;
  const observer = observeOverlaySurfaceGeometry(
    node,
    createInstanceId("overlay-surface"),
    {
      onChange: current.onSurfaceGeometryChange,
      placement: current.placement,
    },
  );

  return {
    update(next: SurfaceGeometryOptions) {
      current = next;
      observer.update({
        onChange: current.onSurfaceGeometryChange,
        placement: current.placement,
      });
    },
    destroy() {
      observer.destroy();
    },
  };
}
