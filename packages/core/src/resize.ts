/**
 * Resize-handle behavior machinery.
 * Contract: docs/contracts/components/resize-handle.md, "Behavior Machine".
 *
 * Pure axis/step math for drag and keyboard resizing; window listener
 * plumbing stays adapter-side (drag-gesture listeners are legitimate
 * adapter effects).
 */

export type ResizeOrientation = "horizontal" | "vertical";

/** Position along the resize axis for a pointer event's coordinates. */
export function resizeAxisPosition(orientation: ResizeOrientation, clientX: number, clientY: number): number {
  return orientation === "horizontal" ? clientX : clientY;
}

/** Drag delta between the last tracked position and the current one. */
export function resizeDragDelta(lastPosition: number, position: number): { delta: number; position: number } {
  return { delta: position - lastPosition, position };
}

/**
 * Keyboard resize step: arrows along the axis step by ±8px; Home/End emit
 * saturating deltas the container clamps. Returns null for unhandled keys.
 */
export function resizeKeydownStep(key: string, orientation: ResizeOrientation): number | null {
  const prevKey = orientation === "horizontal" ? "ArrowLeft" : "ArrowUp";
  const nextKey = orientation === "horizontal" ? "ArrowRight" : "ArrowDown";

  if (key === prevKey) {
    return -8;
  }

  if (key === nextKey) {
    return 8;
  }

  if (key === "Home") {
    return -9999;
  }

  if (key === "End") {
    return 9999;
  }

  return null;
}
