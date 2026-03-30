<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { AlertDialog, Button, getUiPresentation, resolveSemanticControlSize } from "@poodle/svelte-primitives";

  import type { AlertDialogTone, ControlDensity, ControlSize, SemanticControlSizeRole } from "@poodle/svelte-primitives";

  export let title: string;
  export let description: string | null = null;
  export let tone: AlertDialogTone = "danger";
  export let triggerLabel = "Delete";
  export let confirmLabel = "Confirm";
  export let cancelLabel = "Cancel";
  export let onConfirm: (() => void | Promise<void>) | null = null;
  export let onCancel: (() => void) | null = null;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;

  const dispatch = createEventDispatcher<{
    confirm: void;
    cancel: void;
  }>();

  const uiPresentation = getUiPresentation();

  let open: boolean = false;

  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;
  $: triggerTone = tone === "danger" ? "danger" as const : "default" as const;

  function handleTrigger(): void {
    open = true;
  }

  async function handleConfirm(): Promise<void> {
    if (onConfirm) {
      await onConfirm();
    } else {
      dispatch("confirm");
    }
    open = false;
  }

  function handleCancel(): void {
    if (onCancel) {
      onCancel();
    } else {
      dispatch("cancel");
    }
    open = false;
  }

  function handleOpenChange(event: CustomEvent<{ open: boolean }>): void {
    open = event.detail.open;
  }
</script>

{#if $$slots.trigger}
  <span
    class="confirm-action__trigger"
    data-size={resolvedSize}
    data-density={resolvedDensity}
    role="presentation"
    on:click={handleTrigger}
    on:keydown={(e) => {
      if (e.key === "Enter" || e.key === " ") {
        e.preventDefault();
        handleTrigger();
      }
    }}
  >
    <slot name="trigger" />
  </span>
{:else}
  <Button variant="secondary" tone={triggerTone} on:click={handleTrigger}>
    {triggerLabel}
  </Button>
{/if}

<AlertDialog
  open={open || null}
  {title}
  {description}
  {tone}
  {confirmLabel}
  {cancelLabel}
  on:confirm={handleConfirm}
  on:cancel={handleCancel}
  on:openChange={handleOpenChange}
>
  <slot />
</AlertDialog>
