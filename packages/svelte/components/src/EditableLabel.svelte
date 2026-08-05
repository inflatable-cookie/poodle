<script lang="ts">
  import "@poodle/styles/editable-label.css";
  import { editLabelTransition, type EditLabelEvent } from "@poodle/headless";
  import { tick } from "svelte";

  import { getUiPresentation, resolveSemanticControlSize } from "./presentation.ts";

  import type { ControlDensity, ControlSize, EditableLabelActivationMode, SemanticControlSizeRole } from "./types.ts";

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

