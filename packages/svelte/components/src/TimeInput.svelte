<script lang="ts">
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

  $effect.pre(() => {
    if (uncontrolledValue === null) {
      uncontrolledValue = defaultValue;
    }
  });

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const isControlled = $derived(value !== undefined);
  const currentValue = $derived((isControlled ? value : uncontrolledValue) ?? "");

  function handleInput(event: Event): void {
    const nextValue = (event.currentTarget as HTMLInputElement).value || null;

    if (!isControlled) {
      uncontrolledValue = nextValue;
    } else {
      value = nextValue;
    }

    onValueChange?.(nextValue);
  }
</script>

<input
  id={id ?? undefined}
  class="poodle-time-input"
  data-size={resolvedSize}
  data-density={resolvedDensity}
  type="time"
  value={currentValue}
  {min}
  {max}
  {step}
  disabled={disabled}
  aria-label={ariaLabel ?? undefined}
  aria-describedby={describedBy ?? undefined}
  oninput={handleInput}
/>

<style>
  .poodle-time-input {
    min-height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-space-control-x);
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-control);
    background: var(--poodle-color-background-surface);
    color: var(--poodle-color-text-primary);
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-typography-body-size);
    line-height: var(--poodle-typography-body-lineHeight);
  }

  .poodle-time-input:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .poodle-time-input:disabled {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  /* Size variants */
  .poodle-time-input[data-size="xs"] { min-height: 1.5rem; font-size: 0.75rem; }
  .poodle-time-input[data-size="sm"] { min-height: 1.75rem; font-size: 0.8125rem; }
  .poodle-time-input[data-size="lg"] { min-height: 2.75rem; font-size: 0.9375rem; }
  .poodle-time-input[data-size="xl"] { min-height: 3.25rem; font-size: 1rem; }

  /* Density variants */
  .poodle-time-input[data-density="compact"] { padding: 0 calc(var(--poodle-space-control-x) - 0.125rem); }
  .poodle-time-input[data-density="comfortable"] { padding: 0 calc(var(--poodle-space-control-x) + 0.125rem); }
</style>
