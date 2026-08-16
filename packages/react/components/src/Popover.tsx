import { useEffect, useRef, useState, type CSSProperties, type ReactNode } from "react";
import {
  createInstanceId,
  getFocusableElements,
  layerContains,
  popoverParts,
  popoverTransition,
  registerDismissLayer,
  type PopoverContext,
  type PopoverEvent,
  type OverlaySurfaceGeometryChangeHandler,
} from "@inflatable-cookie/poodle-core";

import "@inflatable-cookie/poodle-core/styles/popover.css";

import { AnchoredSurface } from "./AnchoredSurface";
import { reactifyPart } from "./parts";
import type { OverlayPlacement, PopoverInitialFocus } from "./types";

/**
 * The contracted Popover surface. `trigger` and the content take React nodes;
 * `triggerIsInteractive` and `onSurfaceGeometryChange` are documented web-only
 * extensions kept beside this adapter.
 *
 * Contract: `docs/contracts/components/popover.md`. The Svelte pair is
 * `packages/svelte/components/src/Popover.svelte`; the Rust counterpart is
 * `poodle_specs::PopoverSpec`.
 */
export interface PopoverProps {
  open?: boolean | null;
  defaultOpen?: boolean;
  placement?: OverlayPlacement;
  offset?: number;
  dismissOnOutsideInteract?: boolean;
  initialFocus?: PopoverInitialFocus;
  ariaLabel?: string | null;
  block?: boolean;
  disabled?: boolean;
  surfaceWidth?: "content" | "trigger";
  surfaceMinWidth?: string | null;
  surfaceMaxWidth?: string | null;
  triggerIsInteractive?: boolean;
  onOpenChange?: (open: boolean) => void;
  onSurfaceGeometryChange?: OverlaySurfaceGeometryChangeHandler;
  trigger?: ReactNode;
  children?: ReactNode;
}

export function Popover({
  open = null,
  defaultOpen = false,
  placement = "bottom-start",
  offset = 8,
  dismissOnOutsideInteract = true,
  initialFocus = "first-focusable",
  ariaLabel = null,
  block = false,
  triggerIsInteractive = false,
  disabled = false,
  surfaceWidth = "content",
  surfaceMinWidth = null,
  surfaceMaxWidth = null,
  onOpenChange,
  onSurfaceGeometryChange,
  trigger,
  children,
}: PopoverProps) {
  const popoverId = useRef<string | null>(null);
  if (popoverId.current === null) popoverId.current = createInstanceId("popover");

  // The root is state, not a ref: the portalled surface has to re-render once
  // it exists so it can be positioned against it.
  const [rootElement, setRootElement] = useState<HTMLDivElement | null>(null);
  const triggerRef = useRef<HTMLDivElement | null>(null);
  const surfaceRef = useRef<HTMLDivElement | null>(null);
  const [uncontrolledOpen, setUncontrolledOpen] = useState(defaultOpen);
  const [resolvedPlacement, setResolvedPlacement] = useState<OverlayPlacement>(placement);
  const previousOpen = useRef(false);

  const isControlled = open !== null;
  // Disabled blocks open in every direction (contract §3): a controlled
  // `open` request while disabled stays inert — the machine's own guard,
  // mirrored here so the visible state can never disagree with it.
  const isOpen = !disabled && (isControlled ? open === true : uncontrolledOpen);

  const machineContext: PopoverContext = { disabled, dismissOnOutsideInteract, initialFocus };

  // initial focus on open (after the surface renders)
  useEffect(() => {
    if (isOpen && !previousOpen.current && surfaceRef.current) {
      if (initialFocus === "content") {
        surfaceRef.current.focus();
      } else if (initialFocus === "first-focusable") {
        getFocusableElements(surfaceRef.current)[0]?.focus();
      }
    }
    previousOpen.current = isOpen;
  }, [isOpen, initialFocus]);

  const sendRef = useRef<(event: PopoverEvent) => void>(() => {});
  sendRef.current = (event: PopoverEvent) => {
    const result = popoverTransition(isOpen ? "open" : "closed", machineContext, event);
    for (const effect of result.effects) {
      switch (effect.type) {
        case "emitOpenChange":
          if (!isControlled) setUncontrolledOpen(effect.open);
          onOpenChange?.(effect.open);
          break;
        case "restoreTriggerFocus": {
          // With `triggerIsInteractive` the wrapper observes clicks without
          // becoming a button, so the operable control is the one the caller
          // composed inside it. Restoring focus to the wrapper would land it
          // on something that cannot be activated — the operator would press
          // Enter and nothing would happen.
          const target = triggerIsInteractive
            ? triggerRef.current?.querySelector<HTMLElement>(
                'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])',
              ) ?? triggerRef.current
            : triggerRef.current;
          target?.focus();
          break;
        }
        case "focusOnOpen":
          // handled by the isOpen effect above
          break;
      }
    }
  };

  useEffect(() => {
    if (isOpen) return;
    setResolvedPlacement(placement);
  }, [isOpen, placement]);

  useEffect(() => {
    if (!isOpen) return;
    return registerDismissLayer({
      // The surface is portalled out of the root, so both are "inside".
      contains: (target) => layerContains(target as Node, rootElement, surfaceRef.current),
      dismissOnOutsideInteract,
      onDismiss: (reason) =>
        sendRef.current(reason === "escape" ? { type: "ESCAPE" } : { type: "OUTSIDE_INTERACT" }),
      // Parenthood and stack order derive from real layer containment: the
      // layer's `contains` covers both the root and the portalled surface,
      // so a nested popover's root is contained by its host's layer no
      // matter where the surfaces were portalled to.
      hostElement: rootElement,
    });
  }, [isOpen, dismissOnOutsideInteract, rootElement]);

  const parts = popoverParts(isOpen ? "open" : "closed", machineContext, {
    surfaceId: popoverId.current,
    ariaLabel,
    block,
    triggerIsInteractive,
    placement: resolvedPlacement,
    surfaceWidth,
  });

  const surfaceStyle: CSSProperties & Record<string, string> = {
    ...(surfaceMinWidth ? { "--poodle-popover-surface-min-width": surfaceMinWidth } : null),
    ...(surfaceMaxWidth ? { "--poodle-popover-surface-max-width": surfaceMaxWidth } : null),
  };

  return (
    <div {...reactifyPart(parts.root)} className="poodle-popover" ref={setRootElement}>
      <div
        ref={triggerRef}
        {...reactifyPart(parts.trigger)}
        className="poodle-popover__trigger"
        onClick={() => sendRef.current({ type: "TOGGLE" })}
        onKeyDown={triggerIsInteractive ? undefined : (event) => {
          if (disabled) return;
          if (event.key === "Enter" || event.key === " ") {
            event.preventDefault();
            sendRef.current({ type: "TOGGLE" });
          }
        }}
      >
        {trigger}
      </div>

      {isOpen ? (
        <AnchoredSurface
          ref={surfaceRef}
          anchor={rootElement}
          placement={placement}
          offset={offset}
          matchWidth={surfaceWidth === "trigger"}
          onPlacement={setResolvedPlacement}
          onSurfaceGeometryChange={onSurfaceGeometryChange}
          {...reactifyPart(parts.surface)}
          className="poodle-popover__surface"
          style={surfaceStyle}
        >
          {children}
        </AnchoredSurface>
      ) : null}
    </div>
  );
}
