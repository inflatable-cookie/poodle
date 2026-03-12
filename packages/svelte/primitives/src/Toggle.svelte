<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import type { ControlSize, ToggleVariant } from "./types";

  export let isPressed: boolean | null = null;
  export let defaultPressed = false;
  export let variant: ToggleVariant = "ghost";
  export let size: ControlSize = "md";
  export let layout: "inline" | "stack" = "inline";
  export let isDisabled = false;
  export let ariaLabel: string | null = null;
  export let className = "";

  const dispatch = createEventDispatcher<{
    pressedChange: { pressed: boolean };
  }>();

  let uncontrolledPressed = defaultPressed;

  $: controlled = isPressed !== null;
  $: pressed = controlled ? isPressed === true : uncontrolledPressed;

  function toggle(): void {
    const nextPressed = !pressed;

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
  data-size={size}
  data-layout={layout}
  data-pressed={pressed}
  disabled={isDisabled}
  aria-label={ariaLabel ?? undefined}
  aria-pressed={pressed ? "true" : "false"}
  on:click={toggle}
>
  <slot />
</button>

<style>
  .toggle {
    --pug-toggle-fill: color-mix(in srgb, var(--pug-color-background-surface) 86%, transparent);
    --pug-toggle-border: color-mix(in srgb, var(--pug-color-border-subtle) 78%, transparent);
    --pug-toggle-text: var(--pug-color-text-primary);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: var(--pug-space-inline-sm);
    min-width: 2.25rem;
    height: var(--pug-size-control-height);
    padding: 0 var(--pug-space-control-x);
    border: 0.0625rem solid var(--pug-toggle-border);
    border-radius: var(--pug-radius-control);
    background: var(--pug-toggle-fill);
    color: var(--pug-toggle-text);
    cursor: pointer;
    font-family: var(--pug-typography-label-family);
    font-size: 0.75rem;
    font-weight: 600;
    line-height: 1;
  }

  .toggle[data-size="sm"] {
    height: calc(var(--pug-size-control-height) - 0.375rem);
    padding: 0 calc(var(--pug-space-control-x) - 0.125rem);
  }

  .toggle[data-size="lg"] {
    height: calc(var(--pug-size-control-height) + 0.375rem);
    padding: 0 calc(var(--pug-space-control-x) + 0.125rem);
  }

  .toggle[data-layout="stack"],
  .toggle[data-layout="stack"][data-size="sm"],
  .toggle[data-layout="stack"][data-size="lg"] {
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
    --pug-toggle-fill: color-mix(in srgb, var(--pug-color-accent-base) 18%, var(--pug-color-background-surface));
    --pug-toggle-border: color-mix(in srgb, var(--pug-color-accent-base) 38%, var(--pug-color-border-default));
  }

  .toggle[data-variant="secondary"] {
    --pug-toggle-fill: var(--pug-color-background-surface);
    --pug-toggle-border: var(--pug-color-border-default);
  }

  .toggle[data-pressed="true"] {
    background: var(--pug-color-accent-base);
    border-color: color-mix(in srgb, var(--pug-color-accent-base) 78%, black);
    color: var(--pug-color-text-inverse);
  }

  .toggle:focus-visible {
    outline: var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .toggle:disabled {
    cursor: not-allowed;
    opacity: var(--pug-state-opacity-disabled);
  }
</style>
