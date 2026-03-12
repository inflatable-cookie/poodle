<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import type { ButtonVariant, ControlSize } from "./types";

  export let variant: ButtonVariant = "ghost";
  export let size: ControlSize = "md";
  export let icon: string;
  export let ariaLabel: string;
  export let isDisabled = false;
  export let isLoading = false;
  export let isPressed: boolean | null = null;
  export let describedBy: string | null = null;
  export let type: HTMLButtonElement["type"] = "button";

  const dispatch = createEventDispatcher<{
    click: MouseEvent;
    focus: FocusEvent;
    blur: FocusEvent;
  }>();

  $: isUnavailable = isDisabled || isLoading;
</script>

<button
  {type}
  class="icon-button"
  data-variant={variant}
  data-size={size}
  data-loading={isLoading}
  data-pressed={isPressed === true}
  disabled={isUnavailable}
  aria-label={ariaLabel}
  aria-describedby={describedBy ?? undefined}
  aria-busy={isLoading ? "true" : undefined}
  aria-pressed={isPressed === null ? undefined : isPressed ? "true" : "false"}
  on:click={(event) => dispatch("click", event)}
  on:focus={(event) => dispatch("focus", event)}
  on:blur={(event) => dispatch("blur", event)}
>
  {#if isLoading}
    <span class="icon-button__spinner" aria-hidden="true"></span>
  {:else}
    <span class="icon-button__glyph" aria-hidden="true">
      <slot>{icon}</slot>
    </span>
  {/if}
</button>

<style>
  .icon-button {
    --pug-icon-button-fill: color-mix(in srgb, var(--pug-color-background-surface) 58%, transparent);
    --pug-icon-button-fill-hover: color-mix(
      in srgb,
      var(--pug-icon-button-fill) 76%,
      var(--pug-color-background-elevated)
    );
    --pug-icon-button-fill-active: color-mix(
      in srgb,
      var(--pug-icon-button-fill) 64%,
      var(--pug-color-background-elevated)
    );
    --pug-icon-button-border: color-mix(in srgb, var(--pug-color-border-subtle) 76%, transparent);
    --pug-icon-button-text: var(--pug-color-text-primary);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: var(--pug-size-control-height);
    height: var(--pug-size-control-height);
    padding: 0;
    border: 0.0625rem solid var(--pug-icon-button-border);
    border-radius: var(--pug-treatment-interactive-solid-radius, var(--pug-radius-control));
    background: var(--pug-icon-button-fill);
    box-shadow: inset 0 0.0625rem 0 color-mix(in srgb, white 8%, transparent);
    color: var(--pug-icon-button-text);
    cursor: pointer;
    transition:
      background var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard),
      border-color var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard),
      box-shadow var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard),
      transform var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard);
  }

  .icon-button[data-size="sm"] {
    width: calc(var(--pug-size-control-height) - 0.375rem);
    height: calc(var(--pug-size-control-height) - 0.375rem);
  }

  .icon-button[data-size="lg"] {
    width: calc(var(--pug-size-control-height) + 0.375rem);
    height: calc(var(--pug-size-control-height) + 0.375rem);
  }

  .icon-button[data-variant="primary"] {
    --pug-icon-button-fill: var(--pug-color-accent-base);
    --pug-icon-button-border: color-mix(in srgb, var(--pug-color-accent-base) 84%, black);
    --pug-icon-button-text: var(--pug-color-text-inverse);
  }

  .icon-button[data-variant="secondary"] {
    --pug-icon-button-fill: var(--pug-color-background-surface);
    --pug-icon-button-border: var(--pug-color-border-default);
  }

  .icon-button[data-variant="danger"] {
    --pug-icon-button-fill: color-mix(in srgb, var(--pug-color-status-danger) 16%, var(--pug-color-background-surface));
    --pug-icon-button-border: color-mix(in srgb, var(--pug-color-status-danger) 46%, var(--pug-color-border-default));
  }

  .icon-button[data-pressed="true"] {
    background: color-mix(in srgb, var(--pug-color-accent-base) 20%, var(--pug-icon-button-fill));
    border-color: color-mix(in srgb, var(--pug-color-accent-base) 56%, var(--pug-icon-button-border));
    box-shadow:
      inset 0 0.0625rem 0 color-mix(in srgb, white 12%, transparent),
      inset 0 0 0 0.0625rem color-mix(in srgb, var(--pug-color-accent-base) 18%, transparent);
  }

  .icon-button:hover:not(:disabled) {
    background: var(--pug-icon-button-fill-hover);
    border-color: color-mix(in srgb, var(--pug-icon-button-border) 74%, var(--pug-color-text-primary));
  }

  .icon-button:active:not(:disabled) {
    background: var(--pug-icon-button-fill-active);
    transform: translateY(0.03125rem);
  }

  .icon-button:focus-visible {
    outline: var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .icon-button:disabled {
    cursor: not-allowed;
    opacity: var(--pug-state-opacity-disabled);
  }

  .icon-button__glyph,
  .icon-button__spinner {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 0.875rem;
    height: 0.875rem;
    font-family: var(--pug-typography-code-family);
    font-size: 0.8125rem;
    line-height: 1;
  }

  .icon-button__spinner {
    border: 0.125rem solid color-mix(in srgb, currentColor 24%, transparent);
    border-top-color: currentColor;
    border-radius: 999px;
    animation: icon-button-spinner 0.8s linear infinite;
  }

  @keyframes icon-button-spinner {
    to {
      transform: rotate(360deg);
    }
  }
</style>
