<script lang="ts">
  import { createEventDispatcher, tick } from "svelte";

  import { TextInput } from "@pug/svelte-primitives";

  export let value = "";
  export let placeholder: string | null = "Click to edit";
  export let isEditing = false;
  export let isDisabled = false;
  export let ariaLabel: string | null = null;
  export let emptyText = "Empty";
  export let maxLength: number | null = null;

  const dispatch = createEventDispatcher<{
    save: { value: string; previousValue: string };
    cancel: { value: string };
    editStart: void;
  }>();

  let editValue = value;
  let inputElement: HTMLDivElement | null = null;

  $: displayValue = value || emptyText;
  $: isEmpty = !value;

  function startEdit(): void {
    if (isDisabled) return;
    editValue = value;
    isEditing = true;
    dispatch("editStart");
    tick().then(() => {
      const input = inputElement?.querySelector("input");
      input?.focus();
      input?.select();
    });
  }

  function save(): void {
    const trimmed = editValue.trim();
    const previousValue = value;
    isEditing = false;
    if (trimmed !== value) {
      value = trimmed;
      dispatch("save", { value: trimmed, previousValue });
    }
  }

  function cancel(): void {
    isEditing = false;
    editValue = value;
    dispatch("cancel", { value });
  }

  function handleKeydown(event: CustomEvent<{ value: string }>): void {
    // submit event from TextInput fires on Enter
    save();
  }

  function handleCancel(): void {
    cancel();
  }

  function handleBlur(): void {
    if (isEditing) {
      save();
    }
  }
</script>

<div class="inline-editable" data-editing={isEditing} data-disabled={isDisabled}>
  {#if isEditing}
    <div class="inline-editable__input" bind:this={inputElement}>
      <TextInput
        id="inline-edit-field"
        bind:value={editValue}
        {placeholder}
        {maxLength}
        ariaLabel={ariaLabel ?? "Edit value"}
        on:submit={handleKeydown}
        on:cancel={handleCancel}
        on:blur={handleBlur}
      />
    </div>
  {:else}
    <button
      type="button"
      class="inline-editable__display"
      class:inline-editable__display--empty={isEmpty}
      disabled={isDisabled}
      aria-label={ariaLabel ? `${ariaLabel}: ${displayValue}. Click to edit.` : `${displayValue}. Click to edit.`}
      on:click={startEdit}
      on:keydown={(e) => {
        if (e.key === "Enter" || e.key === " ") {
          e.preventDefault();
          startEdit();
        }
      }}
    >
      <span class="inline-editable__text">{displayValue}</span>
      <svg class="inline-editable__icon" viewBox="0 0 16 16" fill="none" aria-hidden="true">
        <path d="M11.5 2.5l2 2-8 8H3.5v-2l8-8z" stroke="currentColor" stroke-width="1.25" stroke-linecap="round" stroke-linejoin="round" />
      </svg>
    </button>
  {/if}
</div>

<style>
  .inline-editable {
    display: inline-flex;
    min-width: 0;
  }

  .inline-editable__input {
    flex: 1;
    min-width: 8rem;
  }

  .inline-editable__display {
    display: inline-flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.25rem 0.375rem;
    border: 0.0625rem solid transparent;
    border-radius: var(--pug-radius-control);
    background: transparent;
    color: var(--pug-color-text-primary);
    cursor: pointer;
    font: inherit;
    font-size: var(--pug-typography-body-size);
    line-height: var(--pug-typography-body-lineHeight);
    text-align: left;
    transition:
      border-color var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard),
      background var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard);
  }

  .inline-editable__display:hover:not(:disabled) {
    border-color: var(--pug-color-border-subtle);
    background: color-mix(in srgb, var(--pug-color-background-surface) 52%, transparent);
  }

  .inline-editable__display:focus-visible {
    outline: var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing);
    outline-offset: 0.0625rem;
  }

  .inline-editable__display:disabled {
    cursor: not-allowed;
    opacity: var(--pug-state-opacity-disabled);
  }

  .inline-editable__display--empty .inline-editable__text {
    color: var(--pug-color-text-secondary);
    font-style: italic;
  }

  .inline-editable__icon {
    width: 0.75rem;
    height: 0.75rem;
    color: var(--pug-color-text-secondary);
    opacity: 0;
    transition: opacity var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard);
  }

  .inline-editable__display:hover:not(:disabled) .inline-editable__icon,
  .inline-editable__display:focus-visible .inline-editable__icon {
    opacity: 1;
  }
</style>
