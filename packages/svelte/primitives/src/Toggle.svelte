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
    --flint-toggle-fill: var(
      --flint-treatment-interactive-fill,
      color-mix(in srgb, var(--flint-color-background-surface) 86%, transparent)
    );
    --flint-toggle-border: var(
      --flint-treatment-interactive-border,
      color-mix(in srgb, var(--flint-color-border-subtle) 78%, transparent)
    );
    --flint-toggle-text: var(--flint-color-text-primary);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: var(--flint-space-inline-sm);
    min-width: 2.25rem;
    height: var(--flint-size-control-height);
    padding: 0 var(--flint-space-control-x);
    border: 0.0625rem solid var(--flint-toggle-border);
    border-radius: var(--flint-treatment-interactive-radius, var(--flint-radius-control));
    background: var(--flint-toggle-fill);
    box-shadow: var(--flint-treatment-interactive-shadow, none);
    color: var(--flint-toggle-text);
    cursor: pointer;
    font-family: var(--flint-typography-label-family);
    font-size: 0.75rem;
    font-weight: 600;
    line-height: 1;
  }

  .toggle[data-size="sm"] {
    height: calc(var(--flint-size-control-height) - 0.375rem);
    padding: 0 calc(var(--flint-space-control-x) - 0.125rem);
  }

  .toggle[data-size="lg"] {
    height: calc(var(--flint-size-control-height) + 0.375rem);
    padding: 0 calc(var(--flint-space-control-x) + 0.125rem);
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
    --flint-toggle-fill: color-mix(in srgb, var(--flint-color-accent-base) 18%, var(--flint-color-background-surface));
    --flint-toggle-border: color-mix(in srgb, var(--flint-color-accent-base) 38%, var(--flint-color-border-default));
  }

  .toggle[data-variant="secondary"] {
    --flint-toggle-fill: var(--flint-color-background-surface);
    --flint-toggle-border: var(--flint-color-border-default);
  }

  .toggle[data-pressed="true"] {
    background:
      linear-gradient(
        color-mix(in srgb, var(--flint-color-accent-base) 82%, transparent),
        color-mix(in srgb, var(--flint-color-accent-base) 82%, transparent)
      ),
      var(
        --flint-treatment-interactive-fill,
        var(--flint-color-accent-base)
      );
    border-color: var(
      --flint-treatment-interactive-border-active,
      color-mix(in srgb, var(--flint-color-accent-base) 78%, black)
    );
    color: var(--flint-color-text-inverse);
  }

  .toggle:focus-visible {
    outline: var(--flint-border-width-focus) solid var(--flint-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .toggle:disabled {
    cursor: not-allowed;
    opacity: var(--flint-state-opacity-disabled);
  }
</style>
