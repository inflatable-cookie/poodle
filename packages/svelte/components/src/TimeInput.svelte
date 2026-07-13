<script lang="ts">
  import "@poodle/styles/time-input.css";
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

  $effect.pre(() => {
    if (!seededDefaultValue) {
      uncontrolledValue = defaultValue;
      seededDefaultValue = true;
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

