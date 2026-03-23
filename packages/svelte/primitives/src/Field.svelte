<script lang="ts">
  import Icon from "./Icon.svelte";
  import type { ValidationState } from "./types";

  export let id: string;
  export let label: string;
  export let description: string | null = null;
  export let hint: string | null = null;
  export let error: string | null = null;
  export let pendingMessage: string | null = null;
  export let validationState: ValidationState = "none";
  export let isRequired = false;
  export let optionalLabel: string | null = "Optional";
  export let span: number | "full" | null = null;
  export let gridArea: string | null = null;

  const hintId = `${id}-hint`;
  let hintOpen = false;
  let hintTimer: ReturnType<typeof setTimeout> | null = null;

  function showHint(): void {
    clearHintTimer();
    hintTimer = setTimeout(() => (hintOpen = true), 200);
  }

  function hideHint(): void {
    clearHintTimer();
    hintOpen = false;
  }

  function clearHintTimer(): void {
    if (hintTimer) {
      clearTimeout(hintTimer);
      hintTimer = null;
    }
  }

  $: descriptionId = description ? `${id}-description` : null;
  $: errorId = error ? `${id}-error` : null;
  $: pendingId = pendingMessage ? `${id}-pending` : null;
  $: messageId =
    validationState === "invalid" && errorId
      ? errorId
      : validationState === "pending" && pendingId
        ? pendingId
        : null;
  $: describedBy = [descriptionId, messageId].filter(Boolean).join(" ") || null;
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
    <label class="field__label" for={id}>
      {label}
      {#if isRequired}
        <span class="field__required" aria-hidden="true">*</span>
      {/if}
      {#if hint}
        <span class="field__hint-wrap" role="presentation">
          <button
            class="field__hint-trigger"
            type="button"
            aria-label="More information"
            aria-describedby={hintOpen ? hintId : undefined}
            on:mouseenter={showHint}
            on:mouseleave={hideHint}
            on:focus={showHint}
            on:blur={hideHint}
            on:keydown={(e) => { if (e.key === "Escape") hideHint(); }}
          >
            <Icon name="info" size="sm" />
          </button>
          {#if hintOpen}
            <span id={hintId} class="field__hint-tooltip" role="tooltip">
              {hint}
            </span>
          {/if}
        </span>
      {/if}
    </label>
    {#if !isRequired && optionalLabel}
      <span class="field__optional">{optionalLabel}</span>
    {/if}
  </div>

  {#if description}
    <p class="field__description" id={descriptionId}>{description}</p>
  {/if}

  <div class="field__control">
    <slot
      {describedBy}
      {descriptionId}
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
    gap: var(--flint-space-stack-sm);
  }

  .field__header {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    gap: var(--flint-space-inline-md);
  }

  .field__label,
  .field__optional,
  .field__description,
  .field__message {
    margin: 0;
  }

  .field__label {
    color: var(--flint-color-text-primary);
    font-family: var(--flint-typography-label-family);
    font-size: var(--flint-typography-label-size);
    font-weight: var(--flint-typography-label-weight);
    line-height: var(--flint-typography-label-lineHeight);
  }

  .field__required {
    color: var(--flint-color-status-danger);
  }

  .field__optional {
    color: var(--flint-color-text-secondary);
    font-family: var(--flint-typography-body-family);
    font-size: 0.75rem;
    line-height: var(--flint-typography-body-lineHeight);
  }

  .field__description,
  .field__message {
    font-family: var(--flint-typography-body-family);
    font-size: 0.75rem;
    line-height: var(--flint-typography-body-lineHeight);
  }

  .field__description,
  .field__message--pending {
    color: var(--flint-color-text-secondary);
  }

  .field__message--error {
    color: var(--flint-color-status-danger);
  }

  /* ── Hint ── */

  .field__hint-wrap {
    position: relative;
    display: inline-flex;
    align-items: center;
    margin-left: 0.25rem;
    vertical-align: middle;
  }

  .field__hint-trigger {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1rem;
    height: 1rem;
    padding: 0;
    border: none;
    border-radius: var(--flint-radius-pill);
    background: color-mix(in srgb, var(--flint-color-text-secondary) 16%, transparent);
    color: var(--flint-color-text-secondary);
    cursor: pointer;
    transition: background var(--flint-motion-duration-interaction) var(--flint-motion-easing-standard);
  }

  .field__hint-trigger:hover {
    background: color-mix(in srgb, var(--flint-color-text-secondary) 28%, transparent);
    color: var(--flint-color-text-primary);
  }

  .field__hint-trigger:focus-visible {
    outline: var(--flint-border-width-focus) solid var(--flint-color-accent-focusRing);
    outline-offset: 0.0625rem;
  }

  .field__hint-tooltip {
    position: absolute;
    z-index: var(--flint-overlay-z-menu);
    bottom: calc(100% + 0.375rem);
    left: 50%;
    transform: translateX(-50%);
    max-width: 18rem;
    padding: 0.375rem 0.5rem;
    border: 0.0625rem solid color-mix(in srgb, var(--flint-color-border-default) 72%, transparent);
    border-radius: calc(var(--flint-radius-control) - 0.125rem);
    background: color-mix(in srgb, var(--flint-color-background-elevated) 98%, var(--flint-color-background-panel));
    box-shadow: var(--flint-elevation-overlay);
    color: var(--flint-color-text-primary);
    font-family: var(--flint-typography-body-family);
    font-size: 0.6875rem;
    font-weight: var(--flint-typography-body-weight);
    line-height: 1.4;
    white-space: normal;
    pointer-events: none;
  }
</style>
