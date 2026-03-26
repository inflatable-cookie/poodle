<script lang="ts">
  import { createEventDispatcher, tick } from "svelte";

  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { ControlSize, EditableLabelActivationMode, SemanticControlSizeRole } from "./types";

  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";

  export let value: string;
  export let ariaLabel = "Edit label";
  export let disabled = false;
  export let activationMode: EditableLabelActivationMode = "doubleClick";
  export let selectOnFocus = true;
  export let variant: "default" | "flush" = "default";
  export let emptyText: string | null = null;
  export let placeholder: string | null = null;
  export let maxLength: number | null = null;
  export let showEditIcon = false;

  const uiPresentation = getUiPresentation();

  const dispatch = createEventDispatcher<{
    editStart: void;
    commit: { value: string; previousValue: string };
    cancel: void;
  }>();

  let isEditing = false;
  let draftValue = value;
  let inputElement: HTMLInputElement | null = null;

  $: resolvedSize = size ?? resolveSemanticControlSize(uiPresentation?.sizeScale ?? "md", sizeRole);
  $: if (!isEditing) {
    draftValue = value;
  }

  $: displayValue = value || emptyText || "";
  $: isEmpty = !value && !!emptyText;

  async function startEditing(): Promise<void> {
    if (disabled || activationMode === "programmatic") return;

    draftValue = value;
    isEditing = true;
    dispatch("editStart");
    await tick();

    if (inputElement) {
      inputElement.focus();
      if (selectOnFocus) inputElement.select();
    }
  }

  function commit(): void {
    const trimmed = draftValue.trim();
    const previousValue = value;
    isEditing = false;
    dispatch("commit", { value: trimmed, previousValue });
  }

  function cancel(): void {
    isEditing = false;
    draftValue = value;
    dispatch("cancel");
  }
</script>

<div
  class="editable-label"
  data-editing={isEditing}
  data-disabled={disabled}
  data-variant={variant}
  data-size={resolvedSize}
>
  {#if isEditing}
    <input
      bind:this={inputElement}
      class="editable-label__input"
      type="text"
      value={draftValue}
      {placeholder}
      maxlength={maxLength}
      on:input={(event) => (draftValue = (event.currentTarget).value)}
      on:blur={() => commit()}
      on:keydown={(event) => {
        if (event.key === "Enter") commit();
        if (event.key === "Escape") { event.preventDefault(); cancel(); }
      }}
    />
  {:else}
    <button
      type="button"
      class="editable-label__display"
      class:editable-label__display--empty={isEmpty}
      disabled={disabled}
      aria-label={ariaLabel}
      on:dblclick={() => { if (activationMode === "doubleClick") startEditing(); }}
      on:click={() => { if (activationMode === "enterOrSpace") startEditing(); }}
      on:keydown={(event) => {
        if (activationMode === "enterOrSpace" && (event.key === "Enter" || event.key === " ")) {
          event.preventDefault();
          startEditing();
        }
      }}
    >
      <span class="editable-label__text">{displayValue}</span>
      {#if showEditIcon}
        <svg class="editable-label__icon" viewBox="0 0 16 16" fill="none" aria-hidden="true">
          <path d="M11.5 2.5l2 2-8 8H3.5v-2l8-8z" stroke="currentColor" stroke-width="1.25" stroke-linecap="round" stroke-linejoin="round" />
        </svg>
      {/if}
    </button>
  {/if}
</div>

<style>
  .editable-label {
    display: inline-flex;
    min-width: 0;
  }

  .editable-label__display,
  .editable-label__input {
    width: 100%;
    min-width: 0;
    padding: var(--poodle-space-control-y) var(--poodle-space-control-x);
    border: 0.0625rem solid transparent;
    border-radius: var(--poodle-radius-control);
    background: transparent;
    color: var(--poodle-color-text-primary);
    font-family: var(--poodle-typography-label-family);
    font-size: var(--poodle-typography-label-size);
    font-weight: var(--poodle-typography-label-weight);
    line-height: var(--poodle-typography-label-lineHeight);
    text-align: left;
  }

  .editable-label__display {
    display: inline-flex;
    align-items: center;
    gap: var(--poodle-space-inline-sm);
    cursor: text;
  }

  .editable-label__display:hover:not(:disabled),
  .editable-label__display:focus-visible {
    border-color: color-mix(in srgb, var(--poodle-color-border-default) 72%, transparent);
    background: color-mix(in srgb, var(--poodle-color-background-surface) 52%, transparent);
    outline: none;
  }

  .editable-label__display:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.0625rem;
  }

  .editable-label__input {
    border-color: var(--poodle-color-accent-focusRing);
    background: var(--poodle-color-background-surface);
    outline: none;
    box-shadow:
      0 0 0 var(--poodle-border-width-focus)
      color-mix(in srgb, var(--poodle-color-accent-focusRing) 28%, transparent);
  }

  .editable-label[data-disabled="true"] .editable-label__display {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  .editable-label__display--empty .editable-label__text {
    color: var(--poodle-color-text-secondary);
    font-style: italic;
  }

  .editable-label__icon {
    width: 0.75rem;
    height: 0.75rem;
    color: var(--poodle-color-text-secondary);
    opacity: 0;
    transition: opacity var(--poodle-motion-duration-interaction, 0.15s) var(--poodle-motion-easing-standard, ease);
  }

  .editable-label__display:hover:not(:disabled) .editable-label__icon,
  .editable-label__display:focus-visible .editable-label__icon {
    opacity: 1;
  }

  /* Flush variant */
  .editable-label[data-variant="flush"] .editable-label__display,
  .editable-label[data-variant="flush"] .editable-label__input {
    padding: 0;
    border: none;
    border-radius: 0;
  }

  .editable-label[data-variant="flush"] .editable-label__display:hover:not(:disabled),
  .editable-label[data-variant="flush"] .editable-label__display:focus-visible {
    border: none;
    background: transparent;
  }

  .editable-label[data-variant="flush"] .editable-label__input {
    border-bottom: 0.0625rem solid var(--poodle-color-accent-focusRing);
    box-shadow: none;
    background: transparent;
  }
</style>
