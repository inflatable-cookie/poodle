<script lang="ts">
  import { editLabelTransition, type EditLabelEvent } from "@poodle/headless";
  import { tick } from "svelte";

  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { ControlDensity, ControlSize, EditableLabelActivationMode, SemanticControlSizeRole } from "./types";

  interface Props {
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    value?: string;
    ariaLabel?: string;
    disabled?: boolean;
    activationMode?: EditableLabelActivationMode;
    selectOnFocus?: boolean;
    variant?: "default" | "flush";
    emptyText?: string | null;
    placeholder?: string | null;
    maxLength?: number | null;
    showEditIcon?: boolean;
    onEditStart?: (() => void) | undefined;
    onCommit?: ((detail: { value: string; previousValue: string }) => void) | undefined;
    onCancel?: (() => void) | undefined;
  }

  let {
    size = null,
    sizeRole = "control",
    density = null,
    value = $bindable(""),
    ariaLabel = "Edit label",
    disabled = false,
    activationMode = "doubleClick",
    selectOnFocus = true,
    variant = "default",
    emptyText = null,
    placeholder = null,
    maxLength = null,
    showEditIcon = false,
    onEditStart = undefined,
    onCommit = undefined,
    onCancel = undefined,
  }: Props = $props();

  const uiPresentation = getUiPresentation();

  let isEditing = $state(false);
  let draftValue = $state(value);
  let inputElement = $state<HTMLInputElement | null>(null);

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const displayValue = $derived(value || emptyText || "");
  const isEmpty = $derived(!value && !!emptyText);

  $effect(() => {
    if (!isEditing) {
      draftValue = value;
    }
  });

  async function send(event: EditLabelEvent): Promise<void> {
    const result = editLabelTransition(
      isEditing ? "editing" : "view",
      {
        value,
        draft: draftValue,
        disabled,
        canStartEdit: activationMode !== "programmatic",
      },
      event,
    );

    isEditing = result.state === "editing";
    draftValue = result.context.draft;

    for (const effect of result.effects) {
      switch (effect.type) {
        case "emitEditStart":
          onEditStart?.();
          break;
        case "focusInput": {
          await tick();

          if (inputElement) {
            inputElement.focus();
            if (selectOnFocus) inputElement.select();
          }
          break;
        }
        case "emitCommit":
          onCommit?.({ value: effect.value, previousValue: effect.previousValue });
          break;
        case "emitCancel":
          onCancel?.();
          break;
      }
    }
  }

  function startEditing(): void {
    void send({ type: "START_EDIT" });
  }

  function commit(): void {
    void send({ type: "COMMIT" });
  }

  function cancel(): void {
    void send({ type: "CANCEL" });
  }
</script>

<div
  class="poodle-editable-label"
  data-editing={isEditing}
  data-disabled={disabled}
  data-variant={variant}
  data-size={resolvedSize}
  data-density={resolvedDensity}
>
  {#if isEditing}
    <input
      bind:this={inputElement}
      class="poodle-editable-label__input"
      type="text"
      value={draftValue}
      {placeholder}
      maxlength={maxLength}
      oninput={(event) => (draftValue = (event.currentTarget).value)}
      onblur={() => commit()}
      onkeydown={(event) => {
        if (event.key === "Enter") commit();
        if (event.key === "Escape") { event.preventDefault(); cancel(); }
      }}
    />
  {:else}
    <button
      type="button"
      class="poodle-editable-label__display"
      class:poodle-editable-label__display--empty={isEmpty}
      disabled={disabled}
      aria-label={ariaLabel}
      ondblclick={() => { if (activationMode === "doubleClick") startEditing(); }}
      onclick={() => { if (activationMode === "enterOrSpace") startEditing(); }}
      onkeydown={(event) => {
        if (activationMode === "enterOrSpace" && (event.key === "Enter" || event.key === " ")) {
          event.preventDefault();
          startEditing();
        }
      }}
    >
      <span class="poodle-editable-label__text">{displayValue}</span>
      {#if showEditIcon}
        <svg class="poodle-editable-label__icon" viewBox="0 0 16 16" fill="none" aria-hidden="true">
          <path d="M11.5 2.5l2 2-8 8H3.5v-2l8-8z" stroke="currentColor" stroke-width="1.25" stroke-linecap="round" stroke-linejoin="round" />
        </svg>
      {/if}
    </button>
  {/if}
</div>

<style>
  .poodle-editable-label {
    display: inline-flex;
    min-width: 0;
  }

  .poodle-editable-label__display,
  .poodle-editable-label__input {
    box-sizing: border-box;
    width: 100%;
    min-width: 0;
    min-height: var(--poodle-size-control-height);
    margin: 0;
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

  .poodle-editable-label__display {
    display: inline-flex;
    align-items: center;
    gap: var(--poodle-space-inline-sm);
    cursor: text;
  }

  .poodle-editable-label__display:hover:not(:disabled),
  .poodle-editable-label__display:focus-visible {
    border-color: color-mix(in srgb, var(--poodle-color-border-default) 72%, transparent);
    background: color-mix(in srgb, var(--poodle-color-background-surface) 52%, transparent);
    outline: none;
  }

  .poodle-editable-label__display:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.0625rem;
  }

  .poodle-editable-label__input {
    appearance: none;
    border-color: var(--poodle-color-accent-focusRing);
    background: var(--poodle-color-background-surface);
    outline: none;
    box-shadow:
      0 0 0 var(--poodle-border-width-focus)
      color-mix(in srgb, var(--poodle-color-accent-focusRing) 28%, transparent);
  }

  .poodle-editable-label[data-disabled="true"] .poodle-editable-label__display {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  .poodle-editable-label__display--empty .poodle-editable-label__text {
    color: var(--poodle-color-text-secondary);
    font-style: italic;
  }

  .poodle-editable-label__icon {
    width: 0.75rem;
    height: 0.75rem;
    color: var(--poodle-color-text-secondary);
    opacity: 0;
    transition: opacity var(--poodle-motion-duration-interaction, 0.15s) var(--poodle-motion-easing-standard, ease);
  }

  .poodle-editable-label__display:hover:not(:disabled) .poodle-editable-label__icon,
  .poodle-editable-label__display:focus-visible .poodle-editable-label__icon {
    opacity: 1;
  }

  /* Flush variant */
  .poodle-editable-label[data-variant="flush"] .poodle-editable-label__display,
  .poodle-editable-label[data-variant="flush"] .poodle-editable-label__input {
    padding: 0;
    border: none;
    border-radius: 0;
  }

  .poodle-editable-label[data-variant="flush"] .poodle-editable-label__display:hover:not(:disabled),
  .poodle-editable-label[data-variant="flush"] .poodle-editable-label__display:focus-visible {
    border: none;
    background: transparent;
  }

  .poodle-editable-label[data-variant="flush"] .poodle-editable-label__input {
    border-bottom: 0.0625rem solid var(--poodle-color-accent-focusRing);
    box-shadow: none;
    background: transparent;
  }

  /* Size variants */
  .poodle-editable-label[data-size="xs"] .poodle-editable-label__display,
  .poodle-editable-label[data-size="xs"] .poodle-editable-label__input {
    padding: calc(var(--poodle-space-control-y) - 0.125rem) calc(var(--poodle-space-control-x) - 0.125rem);
    font-size: 0.75rem;
  }

  .poodle-editable-label[data-size="sm"] .poodle-editable-label__display,
  .poodle-editable-label[data-size="sm"] .poodle-editable-label__input {
    padding: calc(var(--poodle-space-control-y) - 0.0625rem) calc(var(--poodle-space-control-x) - 0.0625rem);
  }

  .poodle-editable-label[data-size="lg"] .poodle-editable-label__display,
  .poodle-editable-label[data-size="lg"] .poodle-editable-label__input {
    padding: calc(var(--poodle-space-control-y) + 0.0625rem) calc(var(--poodle-space-control-x) + 0.125rem);
    font-size: 0.9375rem;
  }

  .poodle-editable-label[data-size="xl"] .poodle-editable-label__display,
  .poodle-editable-label[data-size="xl"] .poodle-editable-label__input {
    padding: calc(var(--poodle-space-control-y) + 0.125rem) calc(var(--poodle-space-control-x) + 0.1875rem);
    font-size: 1rem;
  }

  /* Density variants */
  .poodle-editable-label[data-density="compact"] .poodle-editable-label__input { padding: 0 calc(var(--poodle-space-control-x) - 0.125rem); }
  .poodle-editable-label[data-density="comfortable"] .poodle-editable-label__input { padding: 0 calc(var(--poodle-space-control-x) + 0.125rem); }
</style>
