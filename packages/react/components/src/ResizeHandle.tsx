import "@inflatable-cookie/poodle-core/styles/resize-handle.css";

import { useEffect, useRef, useState, type KeyboardEvent as ReactKeyboardEvent, type MouseEvent as ReactMouseEvent } from "react";

import { resizeAxisPosition, resizeDragDelta, resizeKeydownStep } from "@inflatable-cookie/poodle-core";

import type { SplitOrientation } from "./types";

export interface ResizeHandleProps {
  orientation?: SplitOrientation;
  disabled?: boolean;
  ariaLabel?: string | null;
  ariaValueNow?: number | null;
  ariaValueMin?: number;
  ariaValueMax?: number;
  onResizeStart?: ((position: number) => void) | null;
  onResizeMove?: ((delta: number) => void) | null;
  onResizeEnd?: ((position: number) => void) | null;
  onResizeStep?: ((delta: number) => void) | null;
}

export function ResizeHandle({
  orientation = "horizontal",
  disabled = false,
  ariaLabel = null,
  ariaValueNow = null,
  ariaValueMin = 0,
  ariaValueMax = 100,
  onResizeStart = null,
  onResizeMove = null,
  onResizeEnd = null,
  onResizeStep = null,
}: ResizeHandleProps) {
  const [isDragging, setIsDragging] = useState(false);
  const lastPosition = useRef(0);
  const cleanupRef = useRef<(() => void) | null>(null);

  function handlePointerDown(event: ReactMouseEvent): void {
    if (disabled) return;
    event.preventDefault();
    setIsDragging(true);
    lastPosition.current = resizeAxisPosition(orientation, event.clientX, event.clientY);
    onResizeStart?.(lastPosition.current);

    function handlePointerMove(moveEvent: MouseEvent): void {
      const move = resizeDragDelta(
        lastPosition.current,
        resizeAxisPosition(orientation, moveEvent.clientX, moveEvent.clientY),
      );
      lastPosition.current = move.position;
      onResizeMove?.(move.delta);
    }

    function handlePointerUp(upEvent: MouseEvent): void {
      const upPosition = resizeAxisPosition(orientation, upEvent.clientX, upEvent.clientY);
      lastPosition.current = upPosition;
      setIsDragging(false);
      onResizeEnd?.(upPosition);
      cleanupRef.current?.();
      cleanupRef.current = null;
    }

    window.addEventListener("mousemove", handlePointerMove);
    window.addEventListener("mouseup", handlePointerUp);
    cleanupRef.current = () => {
      window.removeEventListener("mousemove", handlePointerMove);
      window.removeEventListener("mouseup", handlePointerUp);
    };
  }

  function handleKeydown(event: ReactKeyboardEvent): void {
    if (disabled) return;

    const step = resizeKeydownStep(event.key, orientation);

    if (step === null) return;

    event.preventDefault();
    onResizeStep?.(step);
  }

  useEffect(() => {
    return () => {
      cleanupRef.current?.();
      cleanupRef.current = null;
    };
  }, []);

  return (
    <div
      className="poodle-resize-handle"
      data-orientation={orientation}
      data-disabled={disabled || undefined}
      data-dragging={isDragging || undefined}
      role="separator"
      aria-orientation={orientation}
      aria-label={ariaLabel ?? "Resize"}
      aria-valuenow={ariaValueNow ?? undefined}
      aria-valuemin={ariaValueMin}
      aria-valuemax={ariaValueMax}
      tabIndex={disabled ? -1 : 0}
      onMouseDown={handlePointerDown}
      onKeyDown={handleKeydown}
    >
      <span className="poodle-resize-handle__hit" aria-hidden="true" />
      <span className="poodle-resize-handle__line" aria-hidden="true" />
    </div>
  );
}
