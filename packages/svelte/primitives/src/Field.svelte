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
  export let required = false;
  export let optionalLabel: string | null = null;
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
      {#if required}
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
            <Icon name="info" />
          </button>
          {#if hintOpen}
            <span id={hintId} class="field__hint-tooltip" role="tooltip">
              {hint}
            </span>
          {/if}
        </span>
      {/if}
    </label>
    {#if !required && optionalLabel}
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
    gap: var(--poodle-space-stack-sm);
  }

  .field__header {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    gap: var(--poodle-space-inline-md);
  }

  .field__label,
  .field__optional,
  .field__description,
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

  .field__description,
  .field__message {
    font-family: var(--poodle-typography-body-family);
    font-size: 0.75rem;
    line-height: var(--poodle-typography-body-lineHeight);
  }

  .field__description,
  .field__message--pending {
    color: var(--poodle-color-text-secondary);
  }

  .field__message--error {
    color: var(--poodle-color-status-danger);
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
    border-radius: var(--poodle-radius-pill);
    background: color-mix(in srgb, var(--poodle-color-text-secondary) 16%, transparent);
    color: var(--poodle-color-text-secondary);
    cursor: pointer;
    transition: background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .field__hint-trigger:hover {
    background: color-mix(in srgb, var(--poodle-color-text-secondary) 28%, transparent);
    color: var(--poodle-color-text-primary);
  }

  .field__hint-trigger:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.0625rem;
  }

  .field__hint-tooltip {
    position: absolute;
    z-index: var(--poodle-overlay-z-menu);
    bottom: calc(100% + 0.375rem);
    left: 50%;
    transform: translateX(-50%);
    max-width: 18rem;
    padding: 0.375rem 0.5rem;
    border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-default) 72%, transparent);
    border-radius: calc(var(--poodle-radius-control) - 0.125rem);
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 98%, var(--poodle-color-background-panel));
    box-shadow: var(--poodle-elevation-overlay);
    color: var(--poodle-color-text-primary);
    font-family: var(--poodle-typography-body-family);
    font-size: 0.6875rem;
    font-weight: var(--poodle-typography-body-weight);
    line-height: 1.4;
    white-space: normal;
    pointer-events: none;
  }
</style>
