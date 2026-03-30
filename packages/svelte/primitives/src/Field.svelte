<script lang="ts">
  import Icon from "./Icon.svelte";
  import Popover from "./Popover.svelte";
  import type { ValidationState } from "./types";

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
  class="field"
  data-validation-state={validationState}
  style={[
    span ? (span === "full" ? "grid-column: 1 / -1" : `grid-column: span ${span}`) : "",
    gridArea ? `grid-area: ${gridArea}` : "",
  ].filter(Boolean).join("; ") || undefined}
>
  <div class="field__header">
    <div class="field__label-row">
      <label class="field__label" for={id}>
        {label}
        {#if required}
          <span class="field__required" aria-hidden="true">*</span>
        {/if}
      </label>
      {#if infoText}
        <Popover placement="top" offset={6} ariaLabel="Field description">
          <span slot="trigger" class="field__info-trigger-wrap">
            <span class="field__info-icon" aria-label="More information">
              <Icon name="info" />
            </span>
          </span>
          <p class="field__info-content">{infoText}</p>
        </Popover>
      {/if}
    </div>
    {#if !required && optionalLabel}
      <span class="field__optional">{optionalLabel}</span>
    {/if}
  </div>

  <div class="field__control">
    <slot
      {describedBy}
      descriptionId={null}
      {errorId}
      {messageId}
      {validationState}
    />
  </div>

  {#if validationState === "invalid" && error}
    <p class="field__message field__message--error" id={errorId} aria-live="polite">
      {error}
    </p>
  {:else if validationState === "pending" && pendingMessage}
    <p class="field__message field__message--pending" id={pendingId} aria-live="polite">
      {pendingMessage}
    </p>
  {/if}
</div>

<style>
  .field {
    display: grid;
    gap: var(--poodle-space-stack-sm);
  }

  .field__header {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    gap: var(--poodle-space-inline-md);
  }

  .field__label-row {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
  }

  .field__label,
  .field__optional,
  .field__message {
    margin: 0;
  }

  .field__label {
    color: var(--poodle-color-text-primary);
    font-family: var(--poodle-typography-label-family);
    font-size: var(--poodle-typography-label-size);
    font-weight: var(--poodle-typography-label-weight);
    line-height: var(--poodle-typography-label-lineHeight);
  }

  .field__required {
    color: var(--poodle-color-status-danger);
  }

  .field__optional {
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-body-family);
    font-size: 0.75rem;
    line-height: var(--poodle-typography-body-lineHeight);
  }

  .field__message {
    font-family: var(--poodle-typography-body-family);
    font-size: 0.75rem;
    line-height: var(--poodle-typography-body-lineHeight);
  }

  .field__message--pending {
    color: var(--poodle-color-text-secondary);
  }

  .field__message--error {
    color: var(--poodle-color-status-danger);
  }

  /* ── Info icon ── */

  .field__info-trigger-wrap {
    display: inline-flex;
    align-items: center;
  }

  .field__info-icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 0.8125rem;
    height: 0.8125rem;
    border-radius: var(--poodle-radius-pill);
    background: color-mix(in srgb, var(--poodle-color-text-secondary) 14%, transparent);
    color: var(--poodle-color-text-secondary);
    transition: background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .field__info-icon :global(svg) {
    width: 0.5625rem;
    height: 0.5625rem;
  }

  .field__info-trigger-wrap:hover .field__info-icon {
    background: color-mix(in srgb, var(--poodle-color-text-secondary) 26%, transparent);
    color: var(--poodle-color-text-primary);
  }

  /* ── Info popover content ── */

  .field__info-content {
    margin: 0;
    color: var(--poodle-color-text-primary);
    font-family: var(--poodle-typography-body-family);
    font-size: 0.75rem;
    line-height: 1.5;
  }

  /* Override Popover's min-width for this compact use case */
  .field__label-row :global(.popover__surface) {
    min-width: 10rem;
    max-width: 22rem;
    padding: 0.5rem 0.625rem;
  }

  /* Remove Popover trigger's default focus ring — the icon handles it */
  .field__label-row :global(.popover__trigger:focus-visible) {
    outline: none;
  }

  .field__info-trigger-wrap:focus-visible .field__info-icon {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.0625rem;
  }
</style>
