<script lang="ts">
  import type { Snippet } from "svelte";
  import Icon from "./Icon.svelte";
  import Popover from "./Popover.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole, ValidationState } from "./types";

  interface FieldControlProps {
    describedBy: string | null;
    descriptionId: null;
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
  const errorId = $derived(error ? `${id}-error` : null);
  const pendingId = $derived(pendingMessage ? `${id}-pending` : null);
  const messageId = $derived(
    validationState === "invalid" && errorId
      ? errorId
      : validationState === "pending" && pendingId
        ? pendingId
        : null,
  );
  const describedBy = $derived(messageId ?? null);
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
    {#if control}
      {@render control({
        describedBy,
        descriptionId: null,
        errorId,
        messageId,
        validationState,
      })}
    {:else}
      {@render children?.()}
    {/if}
  </div>

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

<style>
  .poodle-field {
    display: grid;
    gap: var(--poodle-space-stack-sm);
  }

  .poodle-field__header {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    gap: var(--poodle-space-inline-md);
  }

  .poodle-field__label-row {
    display: inline-flex;
    align-items: center;
    gap: 0.375rem;
    font-size: var(--poodle-typography-label-size);
  }

  .poodle-field__label,
  .poodle-field__optional,
  .poodle-field__message {
    margin: 0;
  }

  .poodle-field__label {
    color: var(--poodle-color-text-primary);
    font-family: var(--poodle-typography-label-family);
    font-size: var(--poodle-typography-label-size);
    font-weight: var(--poodle-typography-label-weight);
    line-height: var(--poodle-typography-label-lineHeight);
  }

  .poodle-field__required {
    color: var(--poodle-color-status-danger);
  }

  .poodle-field__optional {
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-body-family);
    font-size: 0.75rem;
    line-height: var(--poodle-typography-body-lineHeight);
  }

  .poodle-field__message {
    font-family: var(--poodle-typography-body-family);
    font-size: 0.75rem;
    line-height: var(--poodle-typography-body-lineHeight);
  }

  .poodle-field__message--pending {
    color: var(--poodle-color-text-secondary);
  }

  .poodle-field__message--error {
    color: var(--poodle-color-status-danger);
  }

  .poodle-field[data-size="xs"] .poodle-field__label-row { font-size: 0.6875rem; }
  .poodle-field[data-size="xs"] .poodle-field__message,
  .poodle-field[data-size="xs"] .poodle-field__optional { font-size: 0.625rem; }

  .poodle-field[data-size="sm"] .poodle-field__label-row { font-size: 0.75rem; }
  .poodle-field[data-size="sm"] .poodle-field__message,
  .poodle-field[data-size="sm"] .poodle-field__optional { font-size: 0.6875rem; }

  .poodle-field[data-size="lg"] .poodle-field__label-row { font-size: 0.875rem; }
  .poodle-field[data-size="lg"] .poodle-field__message,
  .poodle-field[data-size="lg"] .poodle-field__optional { font-size: 0.8125rem; }

  .poodle-field[data-size="xl"] .poodle-field__label-row { font-size: 0.9375rem; }
  .poodle-field[data-size="xl"] .poodle-field__message,
  .poodle-field[data-size="xl"] .poodle-field__optional { font-size: 0.875rem; }

  .poodle-field__info-trigger-wrap {
    display: inline-flex;
    align-items: center;
  }

  .poodle-field__info-icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1.25em;
    height: 1.25em;
    cursor: pointer;
    flex-shrink: 0;
    border-radius: var(--poodle-radius-pill);
    background: color-mix(in srgb, var(--poodle-color-text-secondary) 14%, transparent);
    color: var(--poodle-color-text-secondary);
    transition: background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .poodle-field__info-icon :global(svg) {
    width: 0.75em !important;
    height: 0.75em !important;
  }

  .poodle-field__info-trigger-wrap:hover .poodle-field__info-icon {
    background: color-mix(in srgb, var(--poodle-color-text-secondary) 26%, transparent);
    color: var(--poodle-color-text-primary);
  }

  .poodle-field__info-content {
    margin: 0;
    color: var(--poodle-color-text-primary);
    font-family: var(--poodle-typography-body-family);
    font-size: 0.75rem;
    line-height: 1.5;
  }

  .poodle-field__label-row :global(.poodle-popover__surface) {
    min-width: 10rem;
    max-width: 22rem;
    padding: 0.5rem 0.625rem;
  }

  .poodle-field__label-row :global(.poodle-popover__trigger:focus-visible) {
    outline: none;
  }

  .poodle-field__info-trigger-wrap:focus-visible .poodle-field__info-icon {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.0625rem;
  }
</style>
