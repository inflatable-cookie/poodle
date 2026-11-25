import { useLayoutEffect, useRef, useState, type ReactNode } from "react";
import { createPortal } from "react-dom";
import { resolvePortalTarget } from "@inflatable-cookie/poodle-core";

/**
 * Theme-aware portal (mirrors the Svelte `portal` action): mounts overlay
 * content into the explicit `[data-poodle-theme-root]` when present,
 * otherwise the nearest theme or style ancestor, never directly under
 * <html>; falls back to <body>. A zero-size marker span anchors the lookup
 * at the component's position in the tree.
 */
export function ThemePortal({ children }: { children: ReactNode }) {
  const markerRef = useRef<HTMLSpanElement | null>(null);
  const [target, setTarget] = useState<HTMLElement | null>(null);

  useLayoutEffect(() => {
    const marker = markerRef.current;
    if (!marker) return;
    setTarget(resolvePortalTarget(marker));
  }, []);

  return (
    <>
      <span ref={markerRef} style={{ display: "none" }} aria-hidden="true" />
      {target ? createPortal(children, target) : null}
    </>
  );
}
