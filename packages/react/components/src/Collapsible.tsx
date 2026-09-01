import { useEffect, useId, useRef, useState, type ReactNode } from "react";
import { disclosureTransition } from "@inflatable-cookie/poodle-core";

import "@inflatable-cookie/poodle-core/styles/collapsible.css";

import { Icon } from "./Icon";
import { useClippedHeightMotion } from "./disclosure-motion";
import { useMotionPolicy, useMotionReady } from "./motion-policy";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

export interface CollapsibleProps {
  open?: boolean;
  defaultOpen?: boolean;
  title?: string | null;
  description?: string | null;
  disabled?: boolean;
  highlighted?: boolean;
  ariaLabel?: string | null;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  onOpenChange?: (open: boolean) => void;
  trigger?: (props: { isOpen: boolean }) => ReactNode;
  children?: ReactNode;
}

export function Collapsible({
  open,
  defaultOpen = false,
  title = null,
  description = null,
  disabled = false,
  highlighted = false,
  ariaLabel = null,
  size = null,
  sizeRole = "control",
  density = null,
  onOpenChange,
  trigger,
  children,
}: CollapsibleProps) {
  const uiPresentation = useUiPresentation();
  const motionPolicy = useMotionPolicy();
  const motionReady = useMotionReady();
  const collapsibleId = useId();
  const [uncontrolledOpen, setUncontrolledOpen] = useState(defaultOpen);
  const [closing, setClosing] = useState(false);
  const wasOpen = useRef(defaultOpen);

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const isControlled = open !== undefined;
  const isOpen = isControlled ? open === true : uncontrolledOpen;
  const wasOpenAtRender = wasOpen.current;
  const contentRef = useClippedHeightMotion({
    owner: `collapsible-${collapsibleId}`,
    open: isOpen,
    policy: motionPolicy,
    ready: motionReady,
    onCloseFinished: () => setClosing(false),
  });
  const keepContent = isOpen || closing || (wasOpenAtRender && motionReady);

  useEffect(() => {
    if (isOpen) {
      setClosing(false);
    } else if (wasOpen.current && motionReady) {
      setClosing(true);
    } else {
      setClosing(false);
    }
    wasOpen.current = isOpen;
  }, [isOpen, motionReady]);

  function toggle(): void {
    const result = disclosureTransition({ open: isOpen, disabled }, { type: "TOGGLE" });
    for (const effect of result.effects) {
      if (effect.type === "emitOpenChange") {
        if (motionReady && isOpen && !effect.open) {
          setClosing(true);
        } else if (effect.open) {
          setClosing(false);
        }
        if (!isControlled) setUncontrolledOpen(effect.open);
        onOpenChange?.(effect.open);
      }
    }
  }

  return (
    <section
      className="poodle-collapsible"
      data-open={isOpen}
      data-disabled={disabled}
      data-highlighted={highlighted}
      data-size={resolvedSize}
      data-density={resolvedDensity}
      data-motion-ready={motionReady}
    >
      <button
        type="button"
        className="poodle-collapsible__trigger"
        id={`${collapsibleId}-trigger`}
        disabled={disabled}
        aria-expanded={isOpen}
        aria-controls={`${collapsibleId}-content`}
        aria-label={title ? undefined : (ariaLabel ?? undefined)}
        onClick={toggle}
      >
        <span className="poodle-collapsible__heading">
          {trigger ? (
            trigger({ isOpen })
          ) : (
            <>
              {title ? <span className="poodle-collapsible__title">{title}</span> : null}
              {description ? <span className="poodle-collapsible__description">{description}</span> : null}
            </>
          )}
        </span>
        <span className="poodle-collapsible__indicator" aria-hidden="true">
          <Icon name="chevron-down" />
        </span>
      </button>

      <div
        className="poodle-collapsible__content-clip"
        ref={contentRef}
      >
        <div
          className="poodle-collapsible__content"
          id={`${collapsibleId}-content`}
          role="region"
          aria-labelledby={`${collapsibleId}-trigger`}
          hidden={!keepContent}
          inert={!isOpen}
          aria-hidden={!isOpen}
        >
          {children}
        </div>
      </div>
    </section>
  );
}
