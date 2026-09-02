<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/editable-label.css";
  import { editLabelTransition, type EditLabelEvent } from "@inflatable-cookie/poodle-core";
  import { onDestroy, tick, untrack } from "svelte";

  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { ControlDensity, ControlSize, EditableLabelActivationMode, SemanticControlSizeRole } from "./types";

  interface Props {
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    value?: string;
    ariaLabel?: string | undefined;
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
    ariaLabel = undefined,
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
  let committedValue = value;
  let inputElement = $state<HTMLInputElement | null>(null);
  let displayElement = $state<HTMLButtonElement | null>(null);
  let tearingDown = false;

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const displayValue = $derived(value || emptyText || "");
  const isEmpty = $derived(!value && !!emptyText);
  const accessibleName = $derived(ariaLabel || value || emptyText || "Edit label");

  $effect(() => {
    const next = value;
    untrack(() => {
      void send({ type: "REPLACE_VALUE", value: next });
    });
  });

  $effect(() => {
    const next = disabled;
    untrack(() => {
      void send({ type: "SET_DISABLED", disabled: next });
    });
  });

  $effect(() => {
    const onWindowBlur = (): void => {
      queueMicrotask(() => {
        if (!tearingDown) void send({ type: "COMMIT_BLUR" });
      });
    };

    window.addEventListener("blur", onWindowBlur);
    return () => window.removeEventListener("blur", onWindowBlur);
  });

  onDestroy(() => {
    tearingDown = true;
    untrack(() => {
      void send({ type: "TEARDOWN" });
    });
  });

  async function send(event: EditLabelEvent): Promise<void> {
    const result = editLabelTransition(
      isEditing ? "editing" : "view",
      {
        value: committedValue,
        draft: draftValue,
        disabled,
        maxLength,
      },
      event,
    );

    isEditing = result.state === "editing";
    draftValue = result.context.draft;
    committedValue = result.context.value;

    if (event.type === "SET_DRAFT" && inputElement && inputElement.value !== draftValue) {
      inputElement.value = draftValue;
    }

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
            else {
              const end = inputElement.value.length;
              inputElement.setSelectionRange(end, end);
            }
          }
          break;
        }
        case "emitCommit":
          onCommit?.({ value: effect.value, previousValue: effect.previousValue });
          if (effect.restoreFocus) {
            await tick();
            displayElement?.focus();
          }
          break;
        case "emitCancel":
          onCancel?.();
          if (effect.restoreFocus) {
            await tick();
            displayElement?.focus();
          }
          break;
      }
    }
  }

  export function focus(): void {
    if (isEditing) inputElement?.focus();
    else displayElement?.focus();
  }

  export function startEditing(): void {
    void send({ type: "START_EDIT" });
  }

  export function cancelEditing(): void {
    void send({ type: "CANCEL" });
  }

  function activateFromKey(event: KeyboardEvent): void {
    if (activationMode === "programmatic") return;
    if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      void send({ type: "START_EDIT" });
    }
  }

  function handleBlur(): void {
    queueMicrotask(() => {
      if (!tearingDown) void send({ type: "COMMIT_BLUR" });
    });
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
      aria-label={accessibleName}
      value={draftValue}
      {placeholder}
      oninput={(event) => void send({ type: "SET_DRAFT", draft: event.currentTarget.value })}
      onblur={handleBlur}
      onkeydown={(event) => {
        if (event.key === "Enter") {
          event.preventDefault();
          void send({ type: "COMMIT" });
        }
        if (event.key === "Escape") {
          event.preventDefault();
          void send({ type: "CANCEL" });
        }
      }}
    />
  {:else}
    <button
      bind:this={displayElement}
      type="button"
      class="poodle-editable-label__display"
      class:poodle-editable-label__display--empty={isEmpty}
      disabled={disabled}
      aria-label={accessibleName}
      ondblclick={() => {
        if (activationMode === "doubleClick") void send({ type: "START_EDIT" });
      }}
      onclick={() => {
        if (activationMode === "enterOrSpace") void send({ type: "START_EDIT" });
      }}
      onkeydown={activateFromKey}
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
