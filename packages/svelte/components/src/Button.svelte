<script lang="ts">
  import type { Snippet } from "svelte";

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

  interface Props {
    variant?: ButtonVariant;
    tone?: ButtonTone;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    type?: HTMLButtonElement["type"];
    form?: string | null;
    formaction?: string | null;
    formenctype?:
      | "application/x-www-form-urlencoded"
      | "multipart/form-data"
      | "text/plain"
      | null;
    formmethod?: "get" | "post" | "dialog" | null;
    formnovalidate?: boolean;
    formtarget?: "_self" | "_blank" | "_parent" | "_top" | string | null;
    disabled?: boolean;
    loading?: boolean;
    leadingIcon?: IconProp | null;
    trailingIcon?: IconProp | null;
    chevron?: boolean;
    pressed?: boolean | null;
    defaultPressed?: boolean;
    ariaLabel?: string | null;
    ariaExpanded?: boolean | null;
    describedBy?: string | null;
    className?: string;
    style?: string | null;
    onClick?: ((event: MouseEvent) => void) | null;
    onFocus?: ((event: FocusEvent) => void) | null;
    onBlur?: ((event: FocusEvent) => void) | null;
    onPressedChange?: ((pressed: boolean) => void) | null;
    children?: Snippet<[]>;
    leading?: Snippet<[]>;
    trailing?: Snippet<[]>;
    [key: string]: unknown;
  }

  let {
    variant = "secondary",
    tone = "default",
    size = null,
    sizeRole = "control",
    density = null,
    type = "button",
    form = null,
    formaction = null,
    formenctype = null,
    formmethod = null,
    formnovalidate = false,
    formtarget = null,
    disabled = false,
    loading = false,
    leadingIcon = null,
    trailingIcon = null,
    chevron = false,
    pressed = $bindable<boolean | null>(null),
    defaultPressed = false,
    ariaLabel = null,
    ariaExpanded = null,
    describedBy = null,
    className = "",
    style = null,
    onClick = null,
    onFocus = null,
    onBlur = null,
    onPressedChange = null,
    children,
    leading,
    trailing,
    ...restProps
  }: Props = $props();

  const uiPresentation = getUiPresentation();

  let seededDefaultPressed = $state(false);
  let uncontrolledPressed = $state(false);

  $effect.pre(() => {
    if (!seededDefaultPressed && pressed === null) {
      uncontrolledPressed = defaultPressed;
      seededDefaultPressed = true;
    }
  });

  const isToggle = $derived(pressed !== null || defaultPressed);
  const pressedControlled = $derived(pressed !== null);
  const currentPressed = $derived(pressedControlled ? pressed === true : uncontrolledPressed);
  const isUnavailable = $derived(disabled || loading);
  const iconOnly = $derived(!children);
  const hasLeading = $derived(Boolean(leading) || Boolean(leadingIcon) || loading);
  const hasTrailing = $derived(Boolean(trailing) || Boolean(trailingIcon) || chevron);
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const resolvedIconSize = $derived(resolveSupportingVisualSize(resolvedSize));

  function handleClick(event: MouseEvent): void {
    if (isToggle) {
      const next = !currentPressed;
      if (pressedControlled) {
        pressed = next;
      } else {
        uncontrolledPressed = next;
      }
      onPressedChange?.(next);
    }
    onClick?.(event);
  }
</script>

<button
  {...restProps}
  {type}
  form={form ?? undefined}
  formaction={formaction ?? undefined}
  formenctype={formenctype ?? undefined}
  formmethod={formmethod ?? undefined}
  formnovalidate={formnovalidate || undefined}
  formtarget={formtarget ?? undefined}
  class={`poodle-button ${className}`.trim()}
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
  onclick={handleClick}
  onfocus={(event) => {
    onFocus?.(event);
  }}
  onblur={(event) => {
    onBlur?.(event);
  }}
>
  {#if loading}
    <span class="poodle-button__spinner" aria-hidden="true">
      <Spinner variant="ring" size={resolvedIconSize} tone="current" />
    </span>
  {/if}

  {#if leading || leadingIcon}
    <span class="poodle-button__icon" aria-hidden="true">
      {#if leading}
        {@render leading()}
      {:else if leadingIcon}
        <Icon icon={leadingIcon} size={resolvedIconSize} />
      {/if}
    </span>
  {/if}

  {#if children}
    <span class="poodle-button__label">
      {@render children()}
    </span>
  {/if}

  {#if trailing || trailingIcon}
    <span class="poodle-button__icon" aria-hidden="true">
      {#if trailing}
        {@render trailing()}
      {:else if trailingIcon}
        <Icon icon={trailingIcon} size={resolvedIconSize} />
      {/if}
    </span>
  {/if}

  {#if chevron}
    <span class="poodle-button__chevron" aria-hidden="true">
      <Icon name="chevron-down" size={resolvedIconSize} />
    </span>
  {/if}
</button>

<style>
  .poodle-button {
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

  .poodle-button[data-size="xs"] {
    min-width: 3.75rem;
    height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-space-control-x);
    font-size: 0.6875rem;
  }

  .poodle-button[data-size="sm"] {
    min-width: 4.25rem;
    height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-space-control-x);
    font-size: 0.75rem;
  }

  .poodle-button[data-size="lg"] {
    min-width: 5.75rem;
    height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-space-control-x);
    font-size: 0.875rem;
  }

  .poodle-button[data-size="xl"] {
    min-width: 6.5rem;
    height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-space-control-x);
    font-size: 0.9375rem;
  }

  /* Icon padding adjustment: reduce padding on icon side by 0.125rem */
  .poodle-button[data-has-leading] {
    padding-left: calc(var(--poodle-space-control-x) - 0.125rem);
  }

  .poodle-button[data-has-trailing] {
    padding-right: calc(var(--poodle-space-control-x) - 0.125rem);
  }

  .poodle-button[data-has-leading][data-size="xs"] {
    padding-left: calc(var(--poodle-space-control-x) - 0.1875rem);
  }

  .poodle-button[data-has-trailing][data-size="xs"] {
    padding-right: calc(var(--poodle-space-control-x) - 0.1875rem);
  }

  .poodle-button[data-has-leading][data-size="sm"] {
    padding-left: calc(var(--poodle-space-control-x) - 0.25rem);
  }

  .poodle-button[data-has-trailing][data-size="sm"] {
    padding-right: calc(var(--poodle-space-control-x) - 0.25rem);
  }

  .poodle-button[data-has-leading][data-size="lg"] {
    padding-left: var(--poodle-space-control-x);
  }

  .poodle-button[data-has-trailing][data-size="lg"] {
    padding-right: var(--poodle-space-control-x);
  }

  .poodle-button[data-has-leading][data-size="xl"] {
    padding-left: calc(var(--poodle-space-control-x) + 0.0625rem);
  }

  .poodle-button[data-has-trailing][data-size="xl"] {
    padding-right: calc(var(--poodle-space-control-x) + 0.0625rem);
  }

  /* Icon-only: square, no min-width */
  .poodle-button[data-icon-only] {
    min-width: 0;
    padding: 0;
    width: var(--poodle-size-control-height);
  }

  .poodle-button[data-icon-only][data-size="xs"],
  .poodle-button[data-icon-only][data-size="sm"],
  .poodle-button[data-icon-only][data-size="lg"],
  .poodle-button[data-icon-only][data-size="xl"] {
    width: var(--poodle-size-control-height);
  }

  .poodle-button[data-variant="primary"] {
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

  .poodle-button[data-variant="ghost"] {
    --poodle-button-fill: transparent;
    --poodle-button-border: transparent;
    --poodle-button-shadow: none;
  }

  .poodle-button[data-tone="danger"] {
    --poodle-button-fill: color-mix(in srgb, var(--poodle-color-status-danger) 16%, var(--poodle-surface, var(--poodle-color-background-surface)));
    --poodle-button-fill-hover: color-mix(in srgb, var(--poodle-color-status-danger) 24%, var(--poodle-surface, var(--poodle-color-background-surface)));
    --poodle-button-fill-active: color-mix(in srgb, var(--poodle-color-status-danger) 32%, var(--poodle-surface, var(--poodle-color-background-surface)));
    --poodle-button-border: var(--poodle-color-border-default);
    --poodle-button-border-hover: color-mix(in srgb, var(--poodle-color-status-danger) 62%, var(--poodle-color-border-default));
    --poodle-button-text: var(--poodle-color-text-primary);
  }

  .poodle-button[data-variant="primary"][data-tone="danger"] {
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

  .poodle-button[data-variant="ghost"][data-tone="danger"] {
    --poodle-button-fill: transparent;
    --poodle-button-fill-hover: color-mix(in srgb, var(--poodle-color-status-danger) 12%, transparent);
    --poodle-button-fill-active: color-mix(in srgb, var(--poodle-color-status-danger) 18%, transparent);
    --poodle-button-border: transparent;
    --poodle-button-border-hover: color-mix(in srgb, var(--poodle-color-status-danger) 28%, transparent);
    --poodle-button-text: var(--poodle-color-status-danger);
    --poodle-button-shadow: none;
  }

  .poodle-button:hover:not(:disabled) {
    background: var(--poodle-button-fill-hover);
    border-color: var(--poodle-button-border-hover);
    box-shadow: var(
      --poodle-treatment-interactive-shadow-active,
      var(--poodle-button-shadow)
    );
  }

  .poodle-button:active:not(:disabled) {
    background: var(--poodle-button-fill-active);
    transform: translateY(0.03125rem);
  }

  .poodle-button:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .poodle-button:disabled {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  /* Pressed/toggle state — only applies accent treatment to non-primary variants.
     Primary is already accent-colored, so pressed is purely semantic (aria-pressed). */
  .poodle-button[data-pressed="true"]:not([data-variant="primary"]) {
    --poodle-button-fill: var(--poodle-color-accent-base);
    --poodle-button-fill-hover: color-mix(in srgb, white 12%, var(--poodle-color-accent-base));
    --poodle-button-fill-active: color-mix(in srgb, var(--poodle-color-accent-base) 88%, black);
    --poodle-button-border: color-mix(in srgb, var(--poodle-color-accent-base) 85%, black);
    --poodle-button-text: var(--poodle-color-text-inverse);
    --poodle-button-shadow: none;
  }

  .poodle-button__label {
    min-width: 0;
    white-space: nowrap;
  }

  .poodle-button__icon,
  .poodle-button__spinner {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: var(--poodle-size-icon-md);
    height: var(--poodle-size-icon-md);
    font-family: var(--poodle-typography-code-family);
    font-size: 0.875rem;
    line-height: 1;
  }

  .poodle-button__chevron {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    opacity: 0.5;
    margin-left: calc(var(--poodle-space-inline-sm) * -0.25);
  }

  .poodle-button__spinner :global(.poodle-spinner) {
    width: 0.75rem;
    height: 0.75rem;
  }

  /* Density variants */
  .poodle-button[data-density="compact"] {
    padding: 0 calc(var(--poodle-space-control-x) - 0.125rem);
    gap: 0.25rem;
  }

  .poodle-button[data-density="comfortable"] {
    padding: 0 calc(var(--poodle-space-control-x) + 0.125rem);
    gap: var(--poodle-space-inline-md);
  }
</style>
