<script module lang="ts">
  import { buttonDefinition } from "../../preview/src/generated/button";

  // The definition owns the rendered vocabulary (card 041 R2): the
  // anatomy's DOM classes and the eleven data-* attribute names. A rename
  // in packages/codegen/src/models/button.rs moves the DOM here with no
  // hand edit; `effigy ir:check` gates drift in the artifact.
  const parts = new Map<string, string>(buttonDefinition.parts.map((part) => [part.id, part.className]));
  const attributes = new Map<string, string>(buttonDefinition.attributes.map((attribute) => [attribute.id, attribute.name]));

  function partClass(id: string): string {
    const className = parts.get(id);
    if (!className) throw new Error(`Button definition has no part '${id}'`);
    return className;
  }

  function attributeName(id: string): string {
    const name = attributes.get(id);
    if (!name) throw new Error(`Button definition has no attribute '${id}'`);
    return name;
  }

  const rootClass = partClass("root");
  const spinnerClass = partClass("spinner");
  const iconClass = partClass("leading-icon");
  const labelClass = partClass("label");
  const chevronClass = partClass("chevron");

  const dataVariant = attributeName("variant");
  const dataTone = attributeName("tone");
  const dataSize = attributeName("size");
  const dataDensity = attributeName("density");
  const dataIconOnly = attributeName("icon-only");
  const dataHasLeading = attributeName("has-leading");
  const dataHasTrailing = attributeName("has-trailing");
  const dataTruncate = attributeName("truncate");
  const dataFit = attributeName("fit");
  const dataLoading = attributeName("loading");
  const dataPressed = attributeName("pressed");
</script>

<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/button.css";
  import type { Snippet } from "svelte";

  import { default as Icon } from "./Icon.svelte";
  import {
    getUiPresentation,
    resolveSemanticControlSize,
    resolveSupportingVisualSize,
  } from "./presentation";
  import { default as Spinner } from "./Spinner.svelte";
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

  // The eleven data-* attributes, emitted from the definition's attribute
  // names (R2). The value derivation stays here — it is the runtime's
  // projection (CROSS-14) — but the names come from button.rs.
  const dataAttributes = $derived({
    [dataVariant]: variant,
    [dataTone]: tone !== "default" ? tone : undefined,
    [dataSize]: resolvedSize,
    [dataDensity]: resolvedDensity,
    [dataIconOnly]: iconOnly || undefined,
    [dataHasLeading]: hasLeading || undefined,
    [dataHasTrailing]: hasTrailing || undefined,
    [dataTruncate]: truncate || undefined,
    [dataFit]: fit !== "default" ? fit : undefined,
    [dataLoading]: loading,
    [dataPressed]: isToggle ? currentPressed : undefined,
  });

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
  class={`${rootClass} ${className}`.trim()}
  style={resolvedStyle || undefined}
  {...dataAttributes}
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
    <span class={spinnerClass} aria-hidden="true">
      <Spinner variant="ring" size={resolvedIconSize} tone="current" />
    </span>
  {/if}

  {#if leading || leadingIcon}
    <span class={iconClass} aria-hidden="true">
      {#if leading}
        {@render leading()}
      {:else if leadingIcon}
        <Icon icon={leadingIcon} size={resolvedIconSize} />
      {/if}
    </span>
  {/if}

  {#if children}
    <span class={labelClass}>
      {@render children()}
    </span>
  {/if}

  {#if trailing || trailingIcon}
    <span class={iconClass} aria-hidden="true">
      {#if trailing}
        {@render trailing()}
      {:else if trailingIcon}
        <Icon icon={trailingIcon} size={resolvedIconSize} />
      {/if}
    </span>
  {/if}

  {#if chevron}
    <span class={chevronClass} aria-hidden="true">
      <Icon name="chevron-down" size={resolvedIconSize} />
    </span>
  {/if}
</button>

