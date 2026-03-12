<script lang="ts">
  import { createEventDispatcher, tick } from "svelte";

  import type { EditableLabelActivationMode } from "./types";

  export let value: string;
  export let ariaLabel = "Edit label";
  export let isDisabled = false;
  export let activationMode: EditableLabelActivationMode = "doubleClick";
  export let selectOnFocus = true;

  const dispatch = createEventDispatcher<{
    editStart: void;
    commit: { value: string };
    cancel: void;
  }>();

  let isEditing = false;
  let draftValue = value;
  let inputElement: HTMLInputElement | null = null;

  $: if (!isEditing) {
    draftValue = value;
  }

  async function startEditing(): Promise<void> {
    if (isDisabled || activationMode === "programmatic") {
      return;
    }

    draftValue = value;
    isEditing = true;
    dispatch("editStart");
    await tick();

    if (inputElement) {
      inputElement.focus();

      if (selectOnFocus) {
        inputElement.select();
      }
    }
  }

  function commit(): void {
    isEditing = false;
    dispatch("commit", { value: draftValue.trim() });
  }

  function cancel(): void {
    isEditing = false;
    draftValue = value;
    dispatch("cancel");
  }
</script>

<div class="editable-label" data-editing={isEditing} data-disabled={isDisabled}>
  {#if isEditing}
    <input
      bind:this={inputElement}
      class="editable-label__input"
      type="text"
      value={draftValue}
      on:input={(event) => (draftValue = (event.currentTarget as HTMLInputElement).value)}
      on:blur={() => commit()}
      on:keydown={(event) => {
        if (event.key === "Enter") {
          commit();
        }

        if (event.key === "Escape") {
          event.preventDefault();
          cancel();
        }
      }}
    />
  {:else}
    <button
      type="button"
      class="editable-label__display"
      disabled={isDisabled}
      aria-label={ariaLabel}
      on:dblclick={() => {
        if (activationMode === "doubleClick") {
          startEditing();
        }
      }}
      on:keydown={(event) => {
        if (
          activationMode === "enterOrSpace" &&
          (event.key === "Enter" || event.key === " ")
        ) {
          event.preventDefault();
          startEditing();
        }
      }}
      on:click={() => {
        if (activationMode === "enterOrSpace") {
          startEditing();
        }
      }}
    >
      <span class="editable-label__text">{value}</span>
    </button>
  {/if}
</div>

<style>
  .editable-label {
    min-width: 0;
  }

  .editable-label__display,
  .editable-label__input {
    width: 100%;
    min-width: 0;
    padding: 0.375rem 0.5rem;
    border: 0.0625rem solid transparent;
    border-radius: var(--pug-radius-control);
    background: transparent;
    color: var(--pug-color-text-primary);
    font-family: var(--pug-typography-label-family);
    font-size: var(--pug-typography-label-size);
    font-weight: var(--pug-typography-label-weight);
    line-height: var(--pug-typography-label-lineHeight);
    text-align: left;
  }

  .editable-label__display {
    cursor: text;
  }

  .editable-label__display:hover:not(:disabled),
  .editable-label__display:focus-visible {
    border-color: color-mix(in srgb, var(--pug-color-border-default) 72%, transparent);
    background: color-mix(in srgb, var(--pug-color-background-surface) 84%, transparent);
    outline: none;
  }

  .editable-label__input {
    border-color: var(--pug-color-accent-focusRing);
    background: var(--pug-color-background-surface);
    outline: none;
    box-shadow:
      0 0 0 var(--pug-border-width-focus)
      color-mix(in srgb, var(--pug-color-accent-focusRing) 28%, transparent);
  }

  .editable-label[data-disabled="true"] .editable-label__display {
    cursor: not-allowed;
    opacity: var(--pug-state-opacity-disabled);
  }
</style>
