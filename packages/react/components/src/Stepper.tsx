import { useRef, useState } from "react";

import "@poodle/styles/stepper.css";

import { Icon } from "./Icon";
import { Spinner } from "./Spinner";
import { resolveSemanticControlSize, resolveSupportingVisualSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, Orientation, SemanticControlSizeRole, StepperStep } from "./types";

export interface StepperProps {
  steps?: StepperStep[];
  value?: string | null;
  defaultValue?: string | null;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  orientation?: Orientation;
  disabled?: boolean;
  ariaLabel?: string | null;
  rerunLabel?: string;
  onValueChange?: (value: string) => void;
  onRerun?: (value: string) => void;
}

/**
 * The status word appended to a step's accessible name.
 *
 * Status reaches the eye through colour and glyph and neither reaches a screen
 * reader. `pending` is omitted deliberately: it is the unremarkable case, and
 * announcing it on every unvisited step is noise.
 */
function statusSuffix(status: StepperStep["status"]): string {
  switch (status) {
    case "running":
      return ", running";
    case "complete":
      return ", complete";
    case "failed":
      return ", failed";
    default:
      return "";
  }
}

export function Stepper({
  steps = [],
  value,
  defaultValue = null,
  size = null,
  sizeRole = "control",
  density = null,
  orientation = "horizontal",
  disabled = false,
  ariaLabel = null,
  rerunLabel = "Re-run step",
  onValueChange,
  onRerun,
}: StepperProps) {
  const uiPresentation = useUiPresentation();
  // Falling back to the first step matters: a stepper with no current step
  // renders every row as "not here", which is never what a wizard means.
  const [uncontrolledValue, setUncontrolledValue] = useState<string | null>(
    defaultValue ?? steps[0]?.value ?? null,
  );
  const rootRef = useRef<HTMLElement | null>(null);

  const isControlled = value !== undefined;
  const currentValue = isControlled ? value : uncontrolledValue;
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const markerVisualSize = resolveSupportingVisualSize(resolvedSize);

  const isStepDisabled = (step: StepperStep) => disabled || step.isDisabled === true;

  function selectStep(step: StepperStep) {
    if (isStepDisabled(step)) return;
    if (!isControlled) setUncontrolledValue(step.value);
    onValueChange?.(step.value);
  }

  /** Move focus between triggers without selecting — see stepper.md §6. */
  function moveFocus(event: React.KeyboardEvent, index: number) {
    const enabled = steps.map((step, i) => ({ step, i })).filter(({ step }) => !isStepDisabled(step));
    if (enabled.length === 0) return;

    const position = enabled.findIndex(({ i }) => i === index);
    let target: number | undefined;

    // Arrows follow the axis the steps flow along (stepper.md §6).
    const nextKey = orientation === "vertical" ? "ArrowDown" : "ArrowRight";
    const prevKey = orientation === "vertical" ? "ArrowUp" : "ArrowLeft";
    if (event.key === nextKey) target = enabled[Math.min(position + 1, enabled.length - 1)]?.i;
    else if (event.key === prevKey) target = enabled[Math.max(position - 1, 0)]?.i;
    else if (event.key === "Home") target = enabled[0]?.i;
    else if (event.key === "End") target = enabled[enabled.length - 1]?.i;
    else return;

    event.preventDefault();
    if (target === undefined) return;
    rootRef.current
      ?.querySelectorAll<HTMLButtonElement>(".poodle-stepper__trigger")
      ?.[target]?.focus();
  }

  return (
    <nav
      ref={rootRef}
      className="poodle-stepper"
      data-size={resolvedSize}
      data-density={resolvedDensity}
      data-orientation={orientation}
      aria-label={ariaLabel ?? undefined}
    >
      <ol className="poodle-stepper__list">
        {steps.map((step, index) => (
          <li key={step.value} className="poodle-stepper__step" data-status={step.status}>
            <button
              type="button"
              className="poodle-stepper__trigger"
              disabled={isStepDisabled(step)}
              aria-current={currentValue === step.value ? "step" : undefined}
              aria-invalid={step.status === "failed" ? true : undefined}
              aria-label={`${step.label}${statusSuffix(step.status)}${step.description ? `. ${step.description}` : ""}`}
              onClick={() => selectStep(step)}
              onKeyDown={(event) => moveFocus(event, index)}
            >
              {/* Decorative: the status word is already in the accessible name,
                  so announcing the glyph too would read "tick, complete". */}
              <span className="poodle-stepper__marker" aria-hidden="true">
                {step.status === "running" ? (
                  <Spinner variant="ring" size={markerVisualSize} tone="current" />
                ) : step.status === "complete" ? (
                  <Icon name="check" size={markerVisualSize} />
                ) : step.status === "failed" ? (
                  <Icon name="x" size={markerVisualSize} />
                ) : (
                  index + 1
                )}
              </span>
              <span className="poodle-stepper__label">{step.label}</span>
            </button>

            {onRerun && step.status === "complete" ? (
              <button
                type="button"
                className="poodle-stepper__rerun"
                disabled={isStepDisabled(step)}
                aria-label={`${rerunLabel}: ${step.label}`}
                onClick={() => onRerun(step.value)}
              >
                <Icon name="refresh-cw" size={markerVisualSize} />
              </button>
            ) : null}
          </li>
        ))}
      </ol>
    </nav>
  );
}
