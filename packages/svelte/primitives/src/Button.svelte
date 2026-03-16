<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import Icon from "./Icon.svelte";
  import type { ButtonTone, ButtonVariant, ControlSize } from "./types";

  export let variant: ButtonVariant = "secondary";
  export let tone: ButtonTone = "default";
  export let size: ControlSize = "md";
  export let type: HTMLButtonElement["type"] = "button";
  export let isDisabled = false;
  export let isLoading = false;
  export let leadingIcon: string | null = null;
  export let trailingIcon: string | null = null;
  export let chevron = false;
  export let ariaLabel: string | null = null;
  export let describedBy: string | null = null;
  export let className = "";

  const dispatch = createEventDispatcher<{
    click: MouseEvent;
    focus: FocusEvent;
    blur: FocusEvent;
  }>();

  $: isUnavailable = isDisabled || isLoading;
  $: iconOnly = !$$slots.default;
</script>

<button
  {type}
  class={`button ${className}`.trim()}
  data-variant={variant}
  data-tone={tone !== "default" ? tone : undefined}
  data-size={size}
  data-icon-only={iconOnly || undefined}
  data-loading={isLoading}
  disabled={isUnavailable}
  aria-label={ariaLabel ?? undefined}
  aria-describedby={describedBy ?? undefined}
  aria-busy={isLoading ? "true" : undefined}
  on:click={(event) => dispatch("click", event)}
  on:focus={(event) => dispatch("focus", event)}
  on:blur={(event) => dispatch("blur", event)}
>
  {#if isLoading}
    <span class="button__spinner" aria-hidden="true"></span>
  {/if}

  {#if $$slots.leading || leadingIcon}
    <span class="button__icon" aria-hidden="true">
      {#if $$slots.leading}
        <slot name="leading" />
      {:else if leadingIcon}
        <Icon name={leadingIcon} size="sm" />
      {/if}
    </span>
  {/if}

  {#if $$slots.default}
    <span class="button__label">
      <slot />
    </span>
  {/if}

  {#if $$slots.trailing || trailingIcon}
    <span class="button__icon" aria-hidden="true">
      {#if $$slots.trailing}
        <slot name="trailing" />
      {:else if trailingIcon}
        <Icon name={trailingIcon} size="sm" />
      {/if}
    </span>
  {/if}

  {#if chevron}
    <span class="button__chevron" aria-hidden="true">
      <Icon name="chevron-down" size="sm" />
    </span>
  {/if}
</button>

<style>
  .button {
    --pug-button-fill: var(--pug-color-background-surface);
    --pug-button-fill-hover: color-mix(
      in srgb,
      var(--pug-button-fill) 84%,
      var(--pug-color-background-elevated)
    );
    --pug-button-fill-active: color-mix(
      in srgb,
      var(--pug-button-fill) 72%,
      var(--pug-color-background-elevated)
    );
    --pug-button-border: var(--pug-color-border-default);
    --pug-button-text: var(--pug-color-text-primary);
    --pug-button-shadow: inset 0 0.0625rem 0 color-mix(in srgb, white 8%, transparent);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: var(--pug-space-inline-sm);
    min-width: 5rem;
    height: var(--pug-size-control-height);
    padding: 0 var(--pug-space-control-x);
    border: 0.0625rem solid var(--pug-button-border);
    border-radius: var(--pug-treatment-interactive-solid-radius, var(--pug-radius-control));
    background: var(--pug-button-fill);
    box-shadow: var(--pug-button-shadow);
    color: var(--pug-button-text);
    cursor: pointer;
    font-family: var(--pug-typography-label-family);
    font-size: var(--pug-typography-label-size);
    font-weight: var(--pug-typography-label-weight);
    letter-spacing: 0.01em;
    line-height: 1;
    text-decoration: none;
    transition:
      background var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard),
      border-color var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard),
      color var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard),
      box-shadow var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard),
      transform var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard);
  }

  .button[data-size="sm"] {
    min-width: 4.25rem;
    height: calc(var(--pug-size-control-height) - 0.375rem);
    padding: 0 calc(var(--pug-space-control-x) - 0.125rem);
    font-size: 0.75rem;
  }

  .button[data-size="lg"] {
    min-width: 5.75rem;
    height: calc(var(--pug-size-control-height) + 0.375rem);
    padding: 0 calc(var(--pug-space-control-x) + 0.125rem);
    font-size: 0.875rem;
  }

  /* Icon-only: square, no min-width */
  .button[data-icon-only] {
    min-width: 0;
    padding: 0;
    width: var(--pug-size-control-height);
  }

  .button[data-icon-only][data-size="sm"] {
    width: calc(var(--pug-size-control-height) - 0.375rem);
  }

  .button[data-icon-only][data-size="lg"] {
    width: calc(var(--pug-size-control-height) + 0.375rem);
  }

  .button[data-variant="primary"] {
    --pug-button-fill: var(--pug-color-accent-base);
    --pug-button-border: color-mix(in srgb, var(--pug-color-accent-base) 84%, black);
    --pug-button-text: var(--pug-color-text-inverse);
    --pug-button-shadow:
      inset 0 0.0625rem 0 color-mix(in srgb, white 14%, transparent),
      0 0.375rem 1.125rem color-mix(in srgb, black 18%, transparent);
  }

  .button[data-variant="ghost"] {
    --pug-button-fill: transparent;
    --pug-button-border: transparent;
    --pug-button-shadow: none;
  }

  .button[data-tone="danger"] {
    --pug-button-fill: color-mix(in srgb, var(--pug-color-status-danger) 16%, var(--pug-color-background-surface));
    --pug-button-border: color-mix(in srgb, var(--pug-color-status-danger) 46%, var(--pug-color-border-default));
    --pug-button-text: var(--pug-color-text-primary);
  }

  .button[data-variant="primary"][data-tone="danger"] {
    --pug-button-fill: var(--pug-color-status-danger);
    --pug-button-border: color-mix(in srgb, var(--pug-color-status-danger) 84%, black);
    --pug-button-text: var(--pug-color-text-inverse);
    --pug-button-shadow:
      inset 0 0.0625rem 0 color-mix(in srgb, white 14%, transparent),
      0 0.375rem 1.125rem color-mix(in srgb, black 18%, transparent);
  }

  .button[data-variant="ghost"][data-tone="danger"] {
    --pug-button-fill: transparent;
    --pug-button-border: transparent;
    --pug-button-text: var(--pug-color-status-danger);
    --pug-button-shadow: none;
  }

  .button:hover:not(:disabled) {
    background: var(--pug-button-fill-hover);
    border-color: color-mix(in srgb, var(--pug-button-border) 78%, var(--pug-color-text-primary));
  }

  .button:active:not(:disabled) {
    background: var(--pug-button-fill-active);
    transform: translateY(0.03125rem);
  }

  .button:focus-visible {
    outline: var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .button:disabled {
    cursor: not-allowed;
    opacity: var(--pug-state-opacity-disabled);
  }

  .button__label {
    min-width: 0;
    white-space: nowrap;
  }

  .button__icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: var(--pug-size-icon-md);
    height: var(--pug-size-icon-md);
    font-family: var(--pug-typography-code-family);
    font-size: 0.875rem;
    line-height: 1;
  }

  .button__chevron {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    opacity: 0.5;
    margin-left: calc(var(--pug-space-inline-sm) * -0.25);
  }

  .button__spinner {
    width: 0.75rem;
    height: 0.75rem;
    border: 0.125rem solid color-mix(in srgb, currentColor 24%, transparent);
    border-top-color: currentColor;
    border-radius: 999px;
    animation: button-spinner 0.8s linear infinite;
  }

  @keyframes button-spinner {
    to {
      transform: rotate(360deg);
    }
  }
</style>
