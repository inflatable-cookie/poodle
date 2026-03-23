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

  .toggle[data-size="lg"] {
    height: calc(var(--poodle-size-control-height) + 0.375rem);
    padding: 0 calc(var(--poodle-space-control-x) + 0.125rem);
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
</style>
