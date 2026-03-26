<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import Button from "./Button.svelte";
  import Dialog from "./Dialog.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { AlertDialogTone, ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  export let open: boolean | null = null;
  export let title: string;
  export let description: string | null = null;
  export let tone: AlertDialogTone = "danger";
  export let confirmLabel = "Confirm";
  export let cancelLabel = "Cancel";
  export let ariaLabel: string | null = null;
  export let workingLabel = "Working…";
  export let onConfirm: (() => void | Promise<void>) | null = null;
  export let onCancel: (() => void) | null = null;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;

  const dispatch = createEventDispatcher<{
    confirm: void;
    cancel: void;
    openChange: { open: boolean };
  }>();

  const uiPresentation = getUiPresentation();

  let working = false;

  async function handleConfirm(): Promise<void> {
    if (working) {
      return;
    }

    working = true;

    try {
      if (onConfirm) {
        await onConfirm();
      } else {
        dispatch("confirm");
      }

      setOpen(false);
    } catch {
      // Keep the dialog open so the caller can recover or retry.
    } finally {
      working = false;
    }
  }

  function handleCancel(): void {
    if (onCancel) {
      onCancel();
    } else {
      dispatch("cancel");
    }

    setOpen(false);
  }

  function setOpen(nextOpen: boolean): void {
    if (open !== null) {
      dispatch("openChange", { open: nextOpen });
    }
  }

  function handleOpenChange(event: CustomEvent<{ open: boolean }>): void {
    if (!event.detail.open && !working) {
      handleCancel();
    } else {
      dispatch("openChange", event.detail);
    }
  }

  $: resolvedSize = size ?? resolveSemanticControlSize(uiPresentation?.sizeScale ?? "md", sizeRole);
  $: resolvedDensity = density ?? uiPresentation?.density ?? "default";
  $: confirmTone = tone === "danger" ? "danger" as const : "default" as const;
</script>

<div data-size={resolvedSize} data-density={resolvedDensity}>
<Dialog
  {open}
  {title}
  {description}
  kind="alertdialog"
  {ariaLabel}
  dismissOnEscape={!working}
  dismissOnBackdrop={!working}
  showCloseButton={!working}
  on:openChange={handleOpenChange}
>
  <slot />

  <svelte:fragment slot="actions">
    <Button
      variant="ghost"
      on:click={handleCancel}
      disabled={working}
    >
      {cancelLabel}
    </Button>
    <Button
      variant="primary"
      tone={confirmTone}
      on:click={handleConfirm}
      disabled={working}
    >
      {working ? workingLabel : confirmLabel}
    </Button>
  </svelte:fragment>
</Dialog>
</div>
