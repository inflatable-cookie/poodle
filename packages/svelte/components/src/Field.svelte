<script lang="ts">
  import "@poodle/styles/field.css";
  import type { Snippet } from "svelte";
  import { default as Icon } from "./Icon.svelte";
  import { default as Popover } from "./Popover.svelte";
  import { default as UiPresentationProvider } from "./UiPresentationProvider.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole, ValidationState } from "./types";

  interface FieldControlProps {
    describedBy: string | null;
    descriptionId: string | null;
    errorId: string | null;
    messageId: string | null;
    validationState: ValidationState;
  }

  interface Props {
    id: string;
    label: string;
    description?: string | null;
    hint?: string | null;
    error?: string | null;
    pendingMessage?: string | null;
    validationState?: ValidationState;
    required?: boolean;
    optionalLabel?: string | null;
    span?: number | "full" | null;
    gridArea?: string | null;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    control?: Snippet<[FieldControlProps]>;
    children?: Snippet;
  }

  let {
    id,
    label,
    description = null,
    hint = null,
    error = null,
    pendingMessage = null,
    validationState = "none",
    required = false,
    optionalLabel = null,
    span = null,
    gridArea = null,
    size = null,
    sizeRole = "control",
    density = null,
    control,
    children,
  }: Props = $props();

  const uiPresentation = getUiPresentation();

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const infoText = $derived(description ?? hint);
  const descriptionId = $derived(description ? `${id}-description` : null);
  const errorId = $derived(error ? `${id}-error` : null);
  const pendingId = $derived(pendingMessage ? `${id}-pending` : null);
  const messageId = $derived(
    validationState === "invalid" && errorId
      ? errorId
      : validationState === "pending" && pendingId
        ? pendingId
        : null,
  );
  // aria-describedby = description first, then the active validation message
  // (mirrors the Rust FieldSpec.described_by).
  const describedBy = $derived(
    [descriptionId, messageId].filter(Boolean).join(" ") || null,
  );
  const fieldStyle = $derived(
    [
      span ? (span === "full" ? "grid-column: 1 / -1" : `grid-column: span ${span}`) : "",
      gridArea ? `grid-area: ${gridArea}` : "",
    ]
      .filter(Boolean)
      .join("; ") || undefined,
  );
</script>

<div
  class="poodle-field"
  data-size={resolvedSize}
  data-density={resolvedDensity}
  data-validation-state={validationState}
  style={fieldStyle}
>
  <div class="poodle-field__header">
    <div class="poodle-field__label-row">
      <label class="poodle-field__label" for={id}>
        {label}
        {#if required}
          <span class="poodle-field__required" aria-hidden="true">*</span>
        {/if}
      </label>
      {#if infoText}
        <Popover placement="top" offset={6} ariaLabel="Field description">
          {#snippet trigger()}
            <span class="poodle-field__info-trigger-wrap">
              <span class="poodle-field__info-icon" aria-label="More information">
                <Icon name="info" />
              </span>
            </span>
          {/snippet}
          <p class="poodle-field__info-content">{infoText}</p>
        </Popover>
      {/if}
    </div>
    {#if !required && optionalLabel}
      <span class="poodle-field__optional">{optionalLabel}</span>
    {/if}
  </div>

  <div class="poodle-field__control">
    <UiPresentationProvider sizeScale={resolvedSize} density={resolvedDensity}>
      {#if control}
        {@render control({
          describedBy,
          descriptionId,
          errorId,
          messageId,
          validationState,
        })}
      {:else}
        {@render children?.()}
      {/if}
    </UiPresentationProvider>
  </div>

  {#if description}
    <span id={descriptionId} class="poodle-field__sr-description">{description}</span>
  {/if}

  {#if validationState === "invalid" && error}
    <p class="poodle-field__message poodle-field__message--error" id={errorId} aria-live="polite">
      {error}
    </p>
  {:else if validationState === "pending" && pendingMessage}
    <p class="poodle-field__message poodle-field__message--pending" id={pendingId} aria-live="polite">
      {pendingMessage}
    </p>
  {/if}
</div>

