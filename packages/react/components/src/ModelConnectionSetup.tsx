import "@inflatable-cookie/poodle-core/styles/model-connection.css";

import { useRef, useState, type ReactNode } from "react";

import {
  modelConnectionSetupCanContinue,
  modelConnectionSetupCanSubmit,
  modelConnectionSetupTransition,
  type ModelConnectionOption,
  type ModelConnectionPickerState,
  type ModelConnectionSetupStage,
} from "@inflatable-cookie/poodle-core";

import { Button } from "./Button";
import { Callout } from "./Callout";
import { Icon } from "./Icon";
import { ModelConnectionPicker } from "./ModelConnectionPicker";
import { Spinner } from "./Spinner";

export interface ModelConnectionSetupLeadingProps {
  option: ModelConnectionOption;
}

export interface ModelConnectionSetupConfigurationProps {
  option: ModelConnectionOption;
  isPending: boolean;
}

export interface ModelConnectionSetupProps {
  stage?: ModelConnectionSetupStage | undefined;
  defaultStage?: ModelConnectionSetupStage;
  options?: ModelConnectionOption[];
  value?: string | null | undefined;
  defaultValue?: string | null;
  query?: string | undefined;
  pickerState?: ModelConnectionPickerState;
  title?: string;
  description?: string | null;
  canSubmit?: boolean;
  isPending?: boolean;
  pendingLabel?: string;
  error?: string | null;
  success?: string | null;
  continueLabel?: string;
  submitLabel?: string;
  backLabel?: string;
  cancelLabel?: string;
  ariaLabel?: string | null;
  onStageChange?: ((stage: ModelConnectionSetupStage) => void) | null;
  onValueChange?: ((id: string) => void) | null;
  onQueryChange?: ((query: string) => void) | null;
  onSubmit?: ((id: string) => void) | null;
  onCancel?: (() => void) | null;
  leading?: (props: ModelConnectionSetupLeadingProps) => ReactNode;
  configuration?: (props: ModelConnectionSetupConfigurationProps) => ReactNode;
  configureAside?: (props: ModelConnectionSetupLeadingProps) => ReactNode;
}

export function ModelConnectionSetup({
  stage = undefined,
  defaultStage = "choose",
  options = [],
  value = undefined,
  defaultValue = null,
  query = undefined,
  pickerState = "ready",
  title = "Add model connection",
  description = null,
  canSubmit = false,
  isPending = false,
  pendingLabel = "Checking connection",
  error = null,
  success = null,
  continueLabel = "Continue",
  submitLabel = "Add connection",
  backLabel = "Back",
  cancelLabel = "Cancel",
  ariaLabel = null,
  onStageChange = null,
  onValueChange = null,
  onQueryChange = null,
  onSubmit = null,
  onCancel = null,
  leading,
  configuration,
  configureAside,
}: ModelConnectionSetupProps) {
  const [uncontrolledStage, setUncontrolledStage] = useState<ModelConnectionSetupStage>(defaultStage);
  const [uncontrolledValue, setUncontrolledValue] = useState<string | null>(defaultValue);
  const [uncontrolledQuery, setUncontrolledQuery] = useState("");

  const headingRef = useRef<HTMLHeadingElement | null>(null);
  const pickerRootRef = useRef<HTMLDivElement | null>(null);

  const isStageControlled = stage !== undefined;
  const isValueControlled = value !== undefined;
  const isQueryControlled = query !== undefined;
  const currentStage = isStageControlled ? (stage as ModelConnectionSetupStage) : uncontrolledStage;
  const currentValue = isValueControlled ? (value ?? null) : uncontrolledValue;
  const currentQuery = isQueryControlled ? (query ?? "") : uncontrolledQuery;
  const selected = options.find((option) => option.id === currentValue);
  const canContinue = modelConnectionSetupCanContinue({
    value: currentValue,
    options,
    isPending,
  });
  const canAdd = modelConnectionSetupCanSubmit({
    stage: currentStage,
    value: currentValue,
    options,
    canSubmit,
    isPending,
  });

  function focusHeading(): void {
    queueMicrotask(() => headingRef.current?.focus());
  }

  function focusSelectedOption(): void {
    queueMicrotask(() => {
      if (!currentValue) return;
      pickerRootRef.current
        ?.querySelector<HTMLElement>(
          `[data-model-connection-option="${CSS.escape(currentValue)}"]`,
        )
        ?.focus();
    });
  }

  function run(
    event:
      | { type: "SELECT"; id: string }
      | { type: "SET_QUERY"; query: string }
      | { type: "CONTINUE" }
      | { type: "BACK" }
      | { type: "SUBMIT" }
      | { type: "CANCEL" },
  ): void {
    const result = modelConnectionSetupTransition(
      {
        stage: currentStage,
        value: currentValue,
        query: currentQuery,
        options,
        canSubmit,
        isPending,
      },
      event,
    );

    if (!isStageControlled && result.context.stage !== currentStage) {
      setUncontrolledStage(result.context.stage);
    }
    if (!isValueControlled && result.context.value !== currentValue) {
      setUncontrolledValue(result.context.value);
    }
    if (!isQueryControlled && result.context.query !== currentQuery) {
      setUncontrolledQuery(result.context.query);
    }

    for (const effect of result.effects) {
      switch (effect.type) {
        case "emitStageChange":
          onStageChange?.(effect.stage);
          if (effect.stage === "configure") focusHeading();
          if (effect.stage === "choose") focusSelectedOption();
          break;
        case "emitValueChange":
          onValueChange?.(effect.id);
          break;
        case "emitQueryChange":
          onQueryChange?.(effect.query);
          break;
        case "emitSubmit":
          onSubmit?.(effect.id);
          break;
        case "emitCancel":
          onCancel?.();
          break;
      }
    }
  }

  return (
    <section
      className="poodle-model-connection-setup"
      data-stage={currentStage}
      data-pending={isPending ? "true" : "false"}
      aria-label={ariaLabel ?? title}
      aria-busy={isPending ? "true" : undefined}
    >
      <div className="poodle-model-connection-setup__header">
        <h2
          ref={headingRef}
          className="poodle-model-connection-setup__title"
          tabIndex={-1}
        >
          {title}
        </h2>
        {description ? (
          <p className="poodle-model-connection-setup__description">{description}</p>
        ) : null}
      </div>

      <div className="poodle-model-connection-setup__body">
        {currentStage === "choose" ? (
          <div ref={pickerRootRef}>
            <ModelConnectionPicker
              options={options}
              value={currentValue}
              query={currentQuery}
              state={pickerState}
              isDisabled={isPending}
              onValueChange={(id) => run({ type: "SELECT", id })}
              onQueryChange={(next) => run({ type: "SET_QUERY", query: next })}
              leading={leading}
            />
          </div>
        ) : selected ? (
          <>
            <div className="poodle-model-connection-setup__selected">
              <span className="poodle-model-connection-picker__leading" aria-hidden="true">
                {leading ? leading({ option: selected }) : <Icon name="package" />}
              </span>
              <div>
                <p className="poodle-model-connection-picker__provider">{selected.providerLabel}</p>
                {selected.routeLabel ? (
                  <p className="poodle-model-connection-setup__route">{selected.routeLabel}</p>
                ) : null}
              </div>
            </div>

            {error ? <Callout tone="danger" message={error} announceMode="assertive" /> : null}
            {success ? <Callout tone="success" message={success} announceMode="polite" /> : null}

            <div className="poodle-model-connection-setup__configuration">
              {configuration ? configuration({ option: selected, isPending }) : null}
              {configureAside ? (
                <div className="poodle-model-connection-setup__aside">
                  {configureAside({ option: selected })}
                </div>
              ) : null}
            </div>

            {isPending ? (
              <p className="poodle-model-connection-setup__pending" role="status" aria-live="polite">
                <Spinner variant="grid" tone="accent" />
                {pendingLabel}
              </p>
            ) : null}
          </>
        ) : null}
      </div>

      <div className="poodle-model-connection-setup__actions">
        {currentStage === "choose" ? (
          <>
            <Button variant="ghost" disabled={isPending} onClick={() => run({ type: "CANCEL" })}>
              {cancelLabel}
            </Button>
            <Button disabled={!canContinue} onClick={() => run({ type: "CONTINUE" })}>
              {continueLabel}
            </Button>
          </>
        ) : (
          <>
            <Button variant="ghost" disabled={isPending} onClick={() => run({ type: "BACK" })}>
              {backLabel}
            </Button>
            <Button variant="ghost" disabled={isPending} onClick={() => run({ type: "CANCEL" })}>
              {cancelLabel}
            </Button>
            <Button disabled={!canAdd} onClick={() => run({ type: "SUBMIT" })}>
              {submitLabel}
            </Button>
          </>
        )}
      </div>
    </section>
  );
}
