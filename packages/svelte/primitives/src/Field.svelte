<script lang="ts">
  import type { ValidationState } from "./types";

  export let id: string;
  export let label: string;
  export let description: string | null = null;
  export let error: string | null = null;
  export let pendingMessage: string | null = null;
  export let validationState: ValidationState = "none";
  export let isRequired = false;
  export let optionalLabel: string | null = "Optional";

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

<div class="field" data-validation-state={validationState}>
  <div class="field__header">
    <label class="field__label" for={id}>
      {label}
      {#if isRequired}
        <span class="field__required" aria-hidden="true">*</span>
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
    gap: var(--pug-space-stack-sm);
  }

  .field__header {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    gap: var(--pug-space-inline-md);
  }

  .field__label,
  .field__optional,
  .field__description,
  .field__message {
    margin: 0;
  }

  .field__label {
    color: var(--pug-color-text-primary);
    font-family: var(--pug-typography-label-family);
    font-size: var(--pug-typography-label-size);
    font-weight: var(--pug-typography-label-weight);
    line-height: var(--pug-typography-label-lineHeight);
  }

  .field__required {
    color: var(--pug-color-status-danger);
  }

  .field__optional,
  .field__description,
  .field__message--pending {
    color: var(--pug-color-text-secondary);
    font-family: var(--pug-typography-body-family);
    font-size: var(--pug-typography-body-size);
    line-height: var(--pug-typography-body-lineHeight);
  }

  .field__message--error {
    color: var(--pug-color-status-danger);
  }
</style>
