<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/time-input.css";
  import {
    timeInputContext,
    timeInputInvalid,
    timeInputTransition,
    type TimeInputDraft,
  } from "@inflatable-cookie/poodle-core";

  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  interface Props {
    id?: string | null;
    value?: string | null | undefined;
    defaultValue?: string | null;
    min?: string | null;
    max?: string | null;
    step?: number;
    disabled?: boolean;
    ariaLabel?: string | null;
    describedBy?: string | null;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onValueChange?: ((value: string | null) => void) | undefined;
  }

  let {
    id = null,
    value = $bindable<string | null | undefined>(undefined),
    defaultValue = null,
    min = null,
    max = null,
    step = 60,
    disabled = false,
    ariaLabel = null,
    describedBy = null,
    size = null,
    sizeRole = "control",
    density = null,
    onValueChange = undefined,
  }: Props = $props();
  const uiPresentation = getUiPresentation();

  let uncontrolledValue = $state<string | null>(null);
  let seededDefaultValue = $state(false);
  let localDraft = $state<TimeInputDraft | null>(null);
  let nativeDraftText = $state<string | null>(null);
  let lastControlledValue = $state<string | null | undefined>(undefined);

  $effect.pre(() => {
    if (!seededDefaultValue) {
      uncontrolledValue = defaultValue;
      seededDefaultValue = true;
    }
  });

  $effect(() => {
    if (value === undefined || value === lastControlledValue) {
      return;
    }

    lastControlledValue = value;
    localDraft = null;
    nativeDraftText = null;
  });

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const isControlled = $derived(value !== undefined);
  const committed = $derived((isControlled ? value : uncontrolledValue) ?? null);
  const machineContext = $derived(
    timeInputContext({
      committed,
      defaultValue,
      draft: localDraft,
      min,
      max,
      step,
      disabled,
    }),
  );
  const displayValue = $derived(nativeDraftText ?? committed ?? "");
  const invalid = $derived(timeInputInvalid(machineContext));

  function commitEmitted(next: string | null): void {
    if (!isControlled) {
      uncontrolledValue = next;
    } else {
      value = next;
      lastControlledValue = next;
    }

    onValueChange?.(next);
  }

  function handleInput(event: Event): void {
    const input = event.currentTarget as HTMLInputElement;
    const text = input.value;

    // Native incomplete drafts report `value === ""` with `validity.badInput`.
    // A deliberate clear reports empty without badInput.
    if (text === "" && input.validity.badInput) {
      localDraft = localDraft ?? { hour: "", minute: "", second: "" };
      nativeDraftText = "";
      return;
    }

    const result = timeInputTransition(machineContext, { type: "COMMIT_TEXT", text });
    localDraft = result.context.draft;
    nativeDraftText = result.context.draft === null ? null : text;

    for (const effect of result.effects) {
      commitEmitted(effect.value);
    }
  }

  function revertDraft(type: "BLUR" | "ESCAPE"): void {
    if (localDraft === null) {
      return;
    }

    const result = timeInputTransition(machineContext, { type });
    localDraft = result.context.draft;
    nativeDraftText = null;
  }

  function handleBlur(): void {
    revertDraft("BLUR");
  }

  function handleKeydown(event: KeyboardEvent): void {
    if (event.key !== "Escape") {
      return;
    }

    event.preventDefault();
    revertDraft("ESCAPE");
  }
</script>

<input
  id={id ?? undefined}
  class="poodle-time-input"
  data-size={resolvedSize}
  data-density={resolvedDensity}
  type="time"
  value={displayValue}
  {min}
  {max}
  {step}
  disabled={disabled}
  aria-invalid={invalid ? "true" : undefined}
  aria-label={ariaLabel ?? undefined}
  aria-describedby={describedBy ?? undefined}
  oninput={handleInput}
  onblur={handleBlur}
  onkeydown={handleKeydown}
/>

