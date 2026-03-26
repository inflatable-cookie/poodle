<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { ControlDensity, ControlSize, SemanticControlSizeRole, ToggleVariant } from "./types";

  export let pressed: boolean | null = null;
  export let defaultPressed = false;
  export let variant: ToggleVariant = "ghost";
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;
  export let layout: "inline" | "stack" = "inline";
  export let disabled = false;
  export let ariaLabel: string | null = null;
  export let className = "";

  const dispatch = createEventDispatcher<{
    pressedChange: { pressed: boolean };
  }>();

  let uncontrolledPressed = defaultPressed;
  const uiPresentation = getUiPresentation();

  $: controlled = pressed !== null;
  $: currentPressed = controlled ? pressed === true : uncontrolledPressed;
  $: resolvedSize = size ?? resolveSemanticControlSize(uiPresentation?.sizeScale ?? "md", sizeRole);
  $: resolvedDensity = density ?? uiPresentation?.density ?? "default";

  function toggle(): void {
    const nextPressed = !currentPressed;

    if (!controlled) {
      uncontrolledPressed = nextPressed;
    }

    dispatch("pressedChange", { pressed: nextPressed });
  }
</script>

<button
  type="button"
  class={`toggle ${className}`.trim()}
  data-variant={variant}
  data-size={resolvedSize}
  data-density={resolvedDensity}
  data-layout={layout}
  data-pressed={currentPressed}
  disabled={disabled}
  aria-label={ariaLabel ?? undefined}
  aria-pressed={currentPressed ? "true" : "false"}
  on:click={toggle}
>
  <slot />
</button>

<style>
  .toggle {
    --poodle-toggle-fill: var(
      --poodle-treatment-interactive-fill,
      color-mix(in srgb, var(--poodle-color-background-surface) 86%, transparent)
    );
    --poodle-toggle-border: var(
      --poodle-treatment-interactive-border,
      color-mix(in srgb, var(--poodle-color-border-subtle) 78%, transparent)
    );
    --poodle-toggle-text: var(--poodle-color-text-primary);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: var(--poodle-space-inline-sm);
    min-width: 2.25rem;
    height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-space-control-x);
    border: 0.0625rem solid var(--poodle-toggle-border);
    border-radius: var(--poodle-treatment-interactive-radius, var(--poodle-radius-control));
    background: var(--poodle-toggle-fill);
    box-shadow: var(--poodle-treatment-interactive-shadow, none);
    color: var(--poodle-toggle-text);
    cursor: pointer;
    font-family: var(--poodle-typography-label-family);
    font-size: 0.75rem;
    font-weight: 600;
    line-height: 1;
  }

  .toggle[data-size="sm"] {
    height: calc(var(--poodle-size-control-height) - 0.375rem);
    padding: 0 calc(var(--poodle-space-control-x) - 0.125rem);
  }

  .toggle[data-size="xs"] {
    height: calc(var(--poodle-size-control-height) - 0.5rem);
    padding: 0 calc(var(--poodle-space-control-x) - 0.1875rem);
    font-size: 0.6875rem;
  }

  .toggle[data-size="lg"] {
    height: calc(var(--poodle-size-control-height) + 0.375rem);
    padding: 0 calc(var(--poodle-space-control-x) + 0.125rem);
  }

  .toggle[data-size="xl"] {
    height: calc(var(--poodle-size-control-height) + 0.5rem);
    padding: 0 calc(var(--poodle-space-control-x) + 0.1875rem);
    font-size: 0.8125rem;
  }

  .toggle[data-layout="stack"],
  .toggle[data-layout="stack"][data-size="xs"],
  .toggle[data-layout="stack"][data-size="sm"],
  .toggle[data-layout="stack"][data-size="lg"],
  .toggle[data-layout="stack"][data-size="xl"] {
    display: grid;
    width: 100%;
    min-width: 0;
    height: auto;
    padding: 0;
    justify-content: stretch;
    justify-items: stretch;
    align-content: start;
    text-align: left;
    line-height: 1.3;
  }

  .toggle[data-variant="primary"] {
    --poodle-toggle-fill: color-mix(in srgb, var(--poodle-color-accent-base) 18%, var(--poodle-color-background-surface));
    --poodle-toggle-border: color-mix(in srgb, var(--poodle-color-accent-base) 38%, var(--poodle-color-border-default));
  }

  .toggle[data-variant="secondary"] {
    --poodle-toggle-fill: var(--poodle-color-background-surface);
    --poodle-toggle-border: var(--poodle-color-border-default);
  }

  .toggle[data-pressed="true"] {
    background:
      linear-gradient(
        color-mix(in srgb, var(--poodle-color-accent-base) 82%, transparent),
        color-mix(in srgb, var(--poodle-color-accent-base) 82%, transparent)
      ),
      var(
        --poodle-treatment-interactive-fill,
        var(--poodle-color-accent-base)
      );
    border-color: var(
      --poodle-treatment-interactive-border-active,
      color-mix(in srgb, var(--poodle-color-accent-base) 78%, black)
    );
    color: var(--poodle-color-text-inverse);
  }

  .toggle:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .toggle:disabled {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  /* Density variants */
  .toggle[data-density="compact"] { padding: 0 calc(var(--poodle-space-control-x) - 0.125rem); }
  .toggle[data-density="comfortable"] { padding: 0 calc(var(--poodle-space-control-x) + 0.125rem); }
</style>
