<script lang="ts">
  import Icon from "./Icon.svelte";
  import Popover from "./Popover.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole, ValidationState } from "./types";

  export let id: string;
  export let label: string;
  /** Description shown in an info popover next to the label. */
  export let description: string | null = null;
  /** @deprecated Use `description` instead. Alias kept for backward compatibility. */
  export let hint: string | null = null;
  export let error: string | null = null;
  export let pendingMessage: string | null = null;
  export let validationState: ValidationState = "none";
  export let required = false;
  export let optionalLabel: string | null = null;
  export let span: number | "full" | null = null;
  export let gridArea: string | null = null;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;

  const uiPresentation = getUiPresentation();

  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;

  // Merge hint and description — both render in the same popover
  $: infoText = description ?? hint;

  $: errorId = error ? `${id}-error` : null;
  $: pendingId = pendingMessage ? `${id}-pending` : null;
  $: messageId =
    validationState === "invalid" && errorId
      ? errorId
      : validationState === "pending" && pendingId
        ? pendingId
        : null;
  $: describedBy = messageId ?? null;
</script>

<div
  class="poodle-field"
  data-size={resolvedSize}
  data-density={resolvedDensity}
  data-validation-state={validationState}
  style={[
    span ? (span === "full" ? "grid-column: 1 / -1" : `grid-column: span ${span}`) : "",
    gridArea ? `grid-area: ${gridArea}` : "",
  ].filter(Boolean).join("; ") || undefined}
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
          <span slot="trigger" class="poodle-field__info-trigger-wrap">
            <span class="poodle-field__info-icon" aria-label="More information">
              <Icon name="info" />
            </span>
          </span>
          <p class="poodle-field__info-content">{infoText}</p>
        </Popover>
      {/if}
    </div>
    {#if !required && optionalLabel}
      <span class="poodle-field__optional">{optionalLabel}</span>
    {/if}
  </div>

  <div class="poodle-field__control">
    <slot
      {describedBy}
      descriptionId={null}
      {errorId}
      {messageId}
      {validationState}
    />
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

  /* Size variants */
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

  /* ── Info icon ── */

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

  /* ── Info popover content ── */

  .poodle-field__info-content {
    margin: 0;
    color: var(--poodle-color-text-primary);
    font-family: var(--poodle-typography-body-family);
    font-size: 0.75rem;
    line-height: 1.5;
  }

  /* Override Popover's min-width for this compact use case */
  .poodle-field__label-row :global(.poodle-popover__surface) {
    min-width: 10rem;
    max-width: 22rem;
    padding: 0.5rem 0.625rem;
  }

  /* Remove Popover trigger's default focus ring — the icon handles it */
  .poodle-field__label-row :global(.poodle-popover__trigger:focus-visible) {
    outline: none;
  }

  .poodle-field__info-trigger-wrap:focus-visible .poodle-field__info-icon {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.0625rem;
  }
</style>
