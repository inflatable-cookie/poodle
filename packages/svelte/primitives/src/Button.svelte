<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import Icon from "./Icon.svelte";
  import type { ButtonTone, ButtonVariant, ControlSize, IconProp } from "./types";

  export let variant: ButtonVariant = "secondary";
  export let tone: ButtonTone = "default";
  export let size: ControlSize = "md";
  export let type: HTMLButtonElement["type"] = "button";
  export let isDisabled = false;
  export let isLoading = false;
  export let leadingIcon: IconProp | null = null;
  export let trailingIcon: IconProp | null = null;
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
  $: hasLeading = $$slots.leading || leadingIcon || isLoading;
  $: hasTrailing = $$slots.trailing || trailingIcon || chevron;
</script>

<button
  {type}
  class={`button ${className}`.trim()}
  data-variant={variant}
  data-tone={tone !== "default" ? tone : undefined}
  data-size={size}
  data-icon-only={iconOnly || undefined}
  data-has-leading={hasLeading || undefined}
  data-has-trailing={hasTrailing || undefined}
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
        <Icon icon={leadingIcon} size="sm" />
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
        <Icon icon={trailingIcon} size="sm" />
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
    --poodle-button-fill: var(
      --poodle-treatment-interactive-fill,
      var(--poodle-color-background-surface)
    );
    --poodle-button-fill-hover: var(
      --poodle-treatment-interactive-fill-active,
      color-mix(in srgb, var(--poodle-color-background-surface) 84%, var(--poodle-color-background-elevated))
    );
    --poodle-button-fill-active: color-mix(
      in srgb,
      var(--poodle-color-background-surface) 72%,
      var(--poodle-color-background-elevated)
    );
    --poodle-button-border: var(
      --poodle-treatment-interactive-border,
      var(--poodle-color-border-default)
    );
    --poodle-button-text: var(--poodle-color-text-primary);
    --poodle-button-shadow: var(
      --poodle-treatment-interactive-shadow,
      inset 0 0.0625rem 0 color-mix(in srgb, white 8%, transparent)
    );
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 0.375rem;
    min-width: 5rem;
    height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-space-control-x);
    border: 0.0625rem solid var(--poodle-button-border);
    border-radius: var(--poodle-treatment-interactive-radius, var(--poodle-radius-control));
    background: var(--poodle-button-fill);
    box-shadow: var(--poodle-button-shadow);
    color: var(--poodle-button-text);
    cursor: pointer;
    font-family: var(--poodle-typography-label-family);
    font-size: var(--poodle-typography-label-size);
    font-weight: var(--poodle-typography-label-weight);
    letter-spacing: 0.01em;
    line-height: 1;
    text-decoration: none;
    transition:
      background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      border-color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      box-shadow var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      transform var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .button[data-size="sm"] {
    min-width: 4.25rem;
    height: calc(var(--poodle-size-control-height) - 0.375rem);
    padding: 0 calc(var(--poodle-space-control-x) - 0.125rem);
    font-size: 0.75rem;
  }

  .button[data-size="lg"] {
    min-width: 5.75rem;
    height: calc(var(--poodle-size-control-height) + 0.375rem);
    padding: 0 calc(var(--poodle-space-control-x) + 0.125rem);
    font-size: 0.875rem;
  }

  /* Icon padding adjustment: reduce padding on icon side by 0.125rem */
  .button[data-has-leading] {
    padding-left: calc(var(--poodle-space-control-x) - 0.125rem);
  }

  .button[data-has-trailing] {
    padding-right: calc(var(--poodle-space-control-x) - 0.125rem);
  }

  .button[data-has-leading][data-size="sm"] {
    padding-left: calc(var(--poodle-space-control-x) - 0.25rem);
  }

  .button[data-has-trailing][data-size="sm"] {
    padding-right: calc(var(--poodle-space-control-x) - 0.25rem);
  }

  .button[data-has-leading][data-size="lg"] {
    padding-left: var(--poodle-space-control-x);
  }

  .button[data-has-trailing][data-size="lg"] {
    padding-right: var(--poodle-space-control-x);
  }

  /* Icon-only: square, no min-width */
  .button[data-icon-only] {
    min-width: 0;
    padding: 0;
    width: var(--poodle-size-control-height);
  }

  .button[data-icon-only][data-size="sm"] {
    width: calc(var(--poodle-size-control-height) - 0.375rem);
  }

  .button[data-icon-only][data-size="lg"] {
    width: calc(var(--poodle-size-control-height) + 0.375rem);
  }

  .button[data-variant="primary"] {
    --poodle-button-fill: var(
      --poodle-treatment-interactive-primary-fill,
      var(--poodle-color-accent-base)
    );
    --poodle-button-fill-hover: var(
      --poodle-treatment-interactive-primary-fill-hover,
      color-mix(in srgb, white 12%, var(--poodle-color-accent-base))
    );
    --poodle-button-border: var(
      --poodle-treatment-interactive-primary-border,
      color-mix(in srgb, var(--poodle-color-accent-base) 84%, black)
    );
    --poodle-button-text: var(
      --poodle-treatment-interactive-primary-text,
      var(--poodle-color-text-inverse)
    );
    --poodle-button-shadow: var(
      --poodle-treatment-interactive-primary-shadow,
      inset 0 0.0625rem 0 color-mix(in srgb, white 14%, transparent),
      0 0.375rem 1.125rem color-mix(in srgb, black 18%, transparent)
    );
  }

  .button[data-variant="ghost"] {
    --poodle-button-fill: transparent;
    --poodle-button-border: transparent;
    --poodle-button-shadow: none;
  }

  .button[data-tone="danger"] {
    --poodle-button-fill: color-mix(in srgb, var(--poodle-color-status-danger) 16%, var(--poodle-color-background-surface));
    --poodle-button-border: color-mix(in srgb, var(--poodle-color-status-danger) 46%, var(--poodle-color-border-default));
    --poodle-button-text: var(--poodle-color-text-primary);
  }

  .button[data-variant="primary"][data-tone="danger"] {
    --poodle-button-fill: var(--poodle-color-status-danger);
    --poodle-button-border: color-mix(in srgb, var(--poodle-color-status-danger) 84%, black);
    --poodle-button-text: var(--poodle-color-text-inverse);
    --poodle-button-shadow:
      inset 0 0.0625rem 0 color-mix(in srgb, white 14%, transparent),
      0 0.375rem 1.125rem color-mix(in srgb, black 18%, transparent);
  }

  .button[data-variant="ghost"][data-tone="danger"] {
    --poodle-button-fill: transparent;
    --poodle-button-border: transparent;
    --poodle-button-text: var(--poodle-color-status-danger);
    --poodle-button-shadow: none;
  }

  .button:hover:not(:disabled) {
    background: var(--poodle-button-fill-hover);
    border-color: var(
      --poodle-treatment-interactive-border-active,
      color-mix(in srgb, var(--poodle-button-border) 78%, var(--poodle-color-text-primary))
    );
    box-shadow: var(
      --poodle-treatment-interactive-shadow-active,
      var(--poodle-button-shadow)
    );
  }

  .button:active:not(:disabled) {
    background: var(--poodle-button-fill-active);
    transform: translateY(0.03125rem);
  }

  .button:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .button:disabled {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  .button__label {
    min-width: 0;
    white-space: nowrap;
  }

  .button__icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: var(--poodle-size-icon-md);
    height: var(--poodle-size-icon-md);
    font-family: var(--poodle-typography-code-family);
    font-size: 0.875rem;
    line-height: 1;
  }

  .button__chevron {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    opacity: 0.5;
    margin-left: calc(var(--poodle-space-inline-sm) * -0.25);
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
