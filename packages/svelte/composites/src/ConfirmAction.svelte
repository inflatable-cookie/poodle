<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { AlertDialog, Button } from "@poodle/svelte-primitives";

  import type { AlertDialogTone } from "@poodle/svelte-primitives";

  export let title: string;
  export let description: string | null = null;
  export let tone: AlertDialogTone = "danger";
  export let triggerLabel = "Delete";
  export let confirmLabel = "Confirm";
  export let cancelLabel = "Cancel";

  const dispatch = createEventDispatcher<{
    confirm: void;
    cancel: void;
  }>();

  let open: boolean = false;

  $: triggerTone = tone === "danger" ? "danger" as const : "default" as const;

  function handleTrigger(): void {
    open = true;
  }

  function handleConfirm(): void {
    dispatch("confirm");
    open = false;
  }

  function handleCancel(): void {
    dispatch("cancel");
    open = false;
  }

  function handleOpenChange(event: CustomEvent<{ open: boolean }>): void {
    open = event.detail.open;
  }
</script>

{#if $$slots.trigger}
  <span
    class="confirm-action__trigger"
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
