<script lang="ts">
  import "@poodle/styles/button.css";
  import type { Snippet } from "svelte";

  import { default as Icon } from "./Icon.svelte";
  import {
    getUiPresentation,
    resolveSemanticControlSize,
    resolveSupportingVisualSize,
  } from "./presentation.ts";
  import { default as Spinner } from "./Spinner.svelte";
  import type {
    ButtonTone,
    ButtonVariant,
    ControlDensity,
    ControlSize,
    IconProp,
    SemanticControlSizeRole,
  } from "./types.ts";

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
    truncate?: boolean;
    fit?: "default" | "content";
    maxWidth?: string | null;
    pressed?: boolean | null;
    defaultPressed?: boolean | null;
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
    truncate = false,
    fit = "default",
    maxWidth = null,
    pressed = $bindable<boolean | null>(null),
    defaultPressed = null,
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
      uncontrolledPressed = defaultPressed === true;
      seededDefaultPressed = true;
    }
  });

  const isToggle = $derived(pressed !== null || defaultPressed !== null);
  const pressedControlled = $derived(pressed !== null);
  const currentPressed = $derived(pressedControlled ? pressed === true : uncontrolledPressed);
  const isUnavailable = $derived(disabled || loading);
  const iconOnly = $derived(!children);
  const hasLeading = $derived(Boolean(leading) || Boolean(leadingIcon) || loading);
  const hasTrailing = $derived(Boolean(trailing) || Boolean(trailingIcon) || chevron);
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const resolvedIconSize = $derived(resolveSupportingVisualSize(resolvedSize));
  const resolvedStyle = $derived(
    [
      style,
      maxWidth ? `max-width: ${maxWidth}` : null,
    ]
      .filter(Boolean)
      .join("; ")
  );

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
  style={resolvedStyle || undefined}
  data-variant={variant}
  data-tone={tone !== "default" ? tone : undefined}
  data-size={resolvedSize}
  data-density={resolvedDensity}
  data-icon-only={iconOnly || undefined}
  data-has-leading={hasLeading || undefined}
  data-has-trailing={hasTrailing || undefined}
  data-truncate={truncate || undefined}
  data-fit={fit !== "default" ? fit : undefined}
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

