<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import Icon from "./Icon.svelte";
  import {
    getUiPresentation,
    resolveSemanticControlSize,
    resolveSupportingVisualSize,
  } from "./presentation";
  import Spinner from "./Spinner.svelte";
  import type {
    ButtonTone,
    ButtonVariant,
    ControlDensity,
    ControlSize,
    IconProp,
    SemanticControlSizeRole,
  } from "./types";

  export let variant: ButtonVariant = "secondary";
  export let tone: ButtonTone = "default";
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;
  export let type: HTMLButtonElement["type"] = "button";
  export let form: string | null = null;
  export let formaction: string | null = null;
  export let formenctype:
    | "application/x-www-form-urlencoded"
    | "multipart/form-data"
    | "text/plain"
    | null = null;
  export let formmethod: "get" | "post" | "dialog" | null = null;
  export let formnovalidate = false;
  export let formtarget: "_self" | "_blank" | "_parent" | "_top" | string | null = null;
  export let disabled = false;
  export let loading = false;
  export let leadingIcon: IconProp | null = null;
  export let trailingIcon: IconProp | null = null;
  export let chevron = false;
  /** Toggle pressed state. When provided (non-null), button acts as a toggle with aria-pressed. */
  export let pressed: boolean | null = null;
  /** Initial pressed state for uncontrolled toggle mode. */
  export let defaultPressed = false;
  export let ariaLabel: string | null = null;
  export let ariaExpanded: boolean | null = null;
  export let describedBy: string | null = null;
  export let className = "";
  export let style: string | null = null;

  const dispatch = createEventDispatcher<{
    click: MouseEvent;
    focus: FocusEvent;
    blur: FocusEvent;
    pressedChange: { pressed: boolean };
  }>();

  let uncontrolledPressed = defaultPressed;
  $: isToggle = pressed !== null || defaultPressed;
  $: pressedControlled = pressed !== null;
  $: currentPressed = pressedControlled ? pressed === true : uncontrolledPressed;

  function handleClick(event: MouseEvent): void {
    if (isToggle) {
      const next = !currentPressed;
      if (!pressedControlled) {
        uncontrolledPressed = next;
      }
      dispatch("pressedChange", { pressed: next });
    }
    dispatch("click", event);
  }

  const uiPresentation = getUiPresentation();

  $: isUnavailable = disabled || loading;
  $: iconOnly = !$$slots.default;
  $: hasLeading = $$slots.leading || leadingIcon || loading;
  $: hasTrailing = $$slots.trailing || trailingIcon || chevron;
  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;
  $: resolvedIconSize = resolveSupportingVisualSize(resolvedSize);
</script>

<button
  {...$$restProps}
  {type}
  form={form ?? undefined}
  formaction={formaction ?? undefined}
  formenctype={formenctype ?? undefined}
  formmethod={formmethod ?? undefined}
  formnovalidate={formnovalidate || undefined}
  formtarget={formtarget ?? undefined}
  class={`button ${className}`.trim()}
  style={style ?? undefined}
  data-variant={variant}
  data-tone={tone !== "default" ? tone : undefined}
  data-size={resolvedSize}
  data-density={resolvedDensity}
  data-icon-only={iconOnly || undefined}
  data-has-leading={hasLeading || undefined}
  data-has-trailing={hasTrailing || undefined}
  data-loading={loading}
  data-pressed={isToggle ? currentPressed : undefined}
  disabled={isUnavailable}
  aria-label={ariaLabel ?? undefined}
  aria-pressed={isToggle ? (currentPressed ? "true" : "false") : undefined}
  aria-expanded={ariaExpanded === null ? undefined : ariaExpanded ? "true" : "false"}
  aria-describedby={describedBy ?? undefined}
  aria-busy={loading ? "true" : undefined}
  on:click={handleClick}
  on:focus={(event) => dispatch("focus", event)}
  on:blur={(event) => dispatch("blur", event)}
>
  {#if loading}
    <span class="button__spinner" aria-hidden="true">
      <Spinner variant="ring" size={resolvedIconSize} tone="current" />
    </span>
  {/if}

  {#if $$slots.leading || leadingIcon}
    <span class="button__icon" aria-hidden="true">
      {#if $$slots.leading}
        <slot name="leading" />
      {:else if leadingIcon}
        <Icon icon={leadingIcon} size={resolvedIconSize} />
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
        <Icon icon={trailingIcon} size={resolvedIconSize} />
      {/if}
    </span>
  {/if}

  {#if chevron}
    <span class="button__chevron" aria-hidden="true">
      <Icon name="chevron-down" size={resolvedIconSize} />
    </span>
  {/if}
</button>

<style>
  .button {
    --poodle-button-fill: var(
      --poodle-treatment-interactive-fill,
      color-mix(in srgb, var(--poodle-surface, var(--poodle-color-background-surface)) 88%, var(--poodle-color-text-primary))
    );
    --poodle-button-fill-hover: var(
      --poodle-treatment-interactive-fill-active,
      color-mix(in srgb, var(--poodle-surface, var(--poodle-color-background-surface)) 80%, var(--poodle-color-text-primary))
    );
    --poodle-button-fill-active: color-mix(
      in srgb,
      var(--poodle-surface, var(--poodle-color-background-surface)) 84%,
      var(--poodle-color-text-primary)
    );
    --poodle-button-border: var(
      --poodle-treatment-interactive-border,
      var(--poodle-color-border-default)
    );
    --poodle-button-border-hover: var(
      --poodle-treatment-interactive-border-active,
      color-mix(in srgb, var(--poodle-button-border) 78%, var(--poodle-color-text-primary))
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

  .button[data-size="xs"] {
    min-width: 3.75rem;
    height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-space-control-x);
    font-size: 0.6875rem;
  }

  .button[data-size="sm"] {
    min-width: 4.25rem;
    height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-space-control-x);
    font-size: 0.75rem;
  }

  .button[data-size="lg"] {
    min-width: 5.75rem;
    height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-space-control-x);
    font-size: 0.875rem;
  }

  .button[data-size="xl"] {
    min-width: 6.5rem;
    height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-space-control-x);
    font-size: 0.9375rem;
  }

  /* Icon padding adjustment: reduce padding on icon side by 0.125rem */
  .button[data-has-leading] {
    padding-left: calc(var(--poodle-space-control-x) - 0.125rem);
  }

  .button[data-has-trailing] {
    padding-right: calc(var(--poodle-space-control-x) - 0.125rem);
  }

  .button[data-has-leading][data-size="xs"] {
    padding-left: calc(var(--poodle-space-control-x) - 0.1875rem);
  }

  .button[data-has-trailing][data-size="xs"] {
    padding-right: calc(var(--poodle-space-control-x) - 0.1875rem);
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

  .button[data-has-leading][data-size="xl"] {
    padding-left: calc(var(--poodle-space-control-x) + 0.0625rem);
  }

  .button[data-has-trailing][data-size="xl"] {
    padding-right: calc(var(--poodle-space-control-x) + 0.0625rem);
  }

  /* Icon-only: square, no min-width */
  .button[data-icon-only] {
    min-width: 0;
    padding: 0;
    width: var(--poodle-size-control-height);
  }

  .button[data-icon-only][data-size="xs"] {
    width: calc(var(--poodle-size-control-height) - 0.5rem);
  }

  .button[data-icon-only][data-size="sm"] {
    width: calc(var(--poodle-size-control-height) - 0.375rem);
  }

  .button[data-icon-only][data-size="lg"] {
    width: calc(var(--poodle-size-control-height) + 0.375rem);
  }

  .button[data-icon-only][data-size="xl"] {
    width: calc(var(--poodle-size-control-height) + 0.5rem);
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
    --poodle-button-fill-active: color-mix(in srgb, var(--poodle-color-accent-base) 88%, black);
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
    --poodle-button-fill: color-mix(in srgb, var(--poodle-color-status-danger) 16%, var(--poodle-surface, var(--poodle-color-background-surface)));
    --poodle-button-fill-hover: color-mix(in srgb, var(--poodle-color-status-danger) 24%, var(--poodle-surface, var(--poodle-color-background-surface)));
    --poodle-button-fill-active: color-mix(in srgb, var(--poodle-color-status-danger) 32%, var(--poodle-surface, var(--poodle-color-background-surface)));
    --poodle-button-border: var(--poodle-color-border-default);
    --poodle-button-border-hover: color-mix(in srgb, var(--poodle-color-status-danger) 62%, var(--poodle-color-border-default));
    --poodle-button-text: var(--poodle-color-text-primary);
  }

  .button[data-variant="primary"][data-tone="danger"] {
    --poodle-button-fill: var(--poodle-color-status-danger);
    --poodle-button-fill-hover: color-mix(in srgb, white 12%, var(--poodle-color-status-danger));
    --poodle-button-fill-active: color-mix(in srgb, var(--poodle-color-status-danger) 88%, black);
    --poodle-button-border: color-mix(in srgb, var(--poodle-color-status-danger) 84%, black);
    --poodle-button-border-hover: color-mix(in srgb, var(--poodle-color-status-danger) 72%, black);
    --poodle-button-text: var(--poodle-color-text-inverse);
    --poodle-button-shadow:
      inset 0 0.0625rem 0 color-mix(in srgb, white 14%, transparent),
      0 0.375rem 1.125rem color-mix(in srgb, black 18%, transparent);
  }

  .button[data-variant="ghost"][data-tone="danger"] {
    --poodle-button-fill: transparent;
    --poodle-button-fill-hover: color-mix(in srgb, var(--poodle-color-status-danger) 12%, transparent);
    --poodle-button-fill-active: color-mix(in srgb, var(--poodle-color-status-danger) 18%, transparent);
    --poodle-button-border: transparent;
    --poodle-button-border-hover: color-mix(in srgb, var(--poodle-color-status-danger) 28%, transparent);
    --poodle-button-text: var(--poodle-color-status-danger);
    --poodle-button-shadow: none;
  }

  .button:hover:not(:disabled) {
    background: var(--poodle-button-fill-hover);
    border-color: var(--poodle-button-border-hover);
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

  /* Pressed/toggle state — only applies accent treatment to non-primary variants.
     Primary is already accent-colored, so pressed is purely semantic (aria-pressed). */
  .button[data-pressed="true"]:not([data-variant="primary"]) {
    --poodle-button-fill: var(--poodle-color-accent-base);
    --poodle-button-fill-hover: color-mix(in srgb, white 12%, var(--poodle-color-accent-base));
    --poodle-button-fill-active: color-mix(in srgb, var(--poodle-color-accent-base) 88%, black);
    --poodle-button-border: color-mix(in srgb, var(--poodle-color-accent-base) 85%, black);
    --poodle-button-text: var(--poodle-color-text-inverse);
    --poodle-button-shadow: none;
  }

  .button__label {
    min-width: 0;
    white-space: nowrap;
  }

  .button__icon,
  .button__spinner {
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

  .button__spinner :global(.spinner) {
    width: 0.75rem;
    height: 0.75rem;
  }

  /* Density variants */
  .button[data-density="compact"] {
    padding: 0 calc(var(--poodle-space-control-x) - 0.125rem);
    gap: 0.25rem;
  }

  .button[data-density="comfortable"] {
    padding: 0 calc(var(--poodle-space-control-x) + 0.125rem);
    gap: var(--poodle-space-inline-md);
  }
</style>
