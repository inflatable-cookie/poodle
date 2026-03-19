<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import Button from "./Button.svelte";
  import Dialog from "./Dialog.svelte";

  import type { AlertDialogTone } from "./types";

  export let open: boolean | null = null;
  export let title: string;
  export let description: string | null = null;
  export let tone: AlertDialogTone = "danger";
  export let confirmLabel = "Confirm";
  export let cancelLabel = "Cancel";
  export let ariaLabel: string | null = null;

  const dispatch = createEventDispatcher<{
    confirm: void;
    cancel: void;
    openChange: { open: boolean };
  }>();

  let working = false;

  async function handleConfirm(): Promise<void> {
    working = true;

    try {
      dispatch("confirm");
    } finally {
      working = false;
      setOpen(false);
    }
  }

  function handleCancel(): void {
    dispatch("cancel");
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

  $: confirmTone = tone === "danger" ? "danger" as const : "default" as const;
</script>

<Dialog
  {open}
  {title}
  {description}
  kind="alertdialog"
  {ariaLabel}
  dismissOnEscape={!working}
  dismissOnBackdrop={!working}
  on:openChange={handleOpenChange}
>
  <slot />

  <svelte:fragment slot="actions">
    <Button
      variant="ghost"
      on:click={handleCancel}
      isDisabled={working}
    >
      {cancelLabel}
    </Button>
    <Button
      variant="primary"
      tone={confirmTone}
      on:click={handleConfirm}
      isDisabled={working}
    >
      {working ? "Working\u2026" : confirmLabel}
    </Button>
  </svelte:fragment>
</Dialog>
