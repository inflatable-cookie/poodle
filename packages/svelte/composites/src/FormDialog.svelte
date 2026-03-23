<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { Dialog, Button } from "@poodle/svelte-primitives";

  import FormLayout from "./FormLayout.svelte";

  export let open: boolean | null = null;
  export let title: string;
  export let description: string | null = null;
  export let submitLabel = "Submit";
  export let cancelLabel = "Cancel";
  export let submitting = false;
  export let error: string | null = null;
  export let ariaLabel: string | null = null;

  const dispatch = createEventDispatcher<{
    submit: void;
    cancel: void;
    openChange: { open: boolean };
  }>();

  function handleSubmit(): void {
    dispatch("submit");
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
    if (!event.detail.open && !submitting) {
      handleCancel();
    } else {
      dispatch("openChange", event.detail);
    }
  }
</script>

<Dialog
  {open}
  {title}
  {description}
  kind="dialog"
  {ariaLabel}
  dismissOnEscape={!submitting}
  dismissOnBackdrop={!submitting}
  on:openChange={handleOpenChange}
>
  <FormLayout {error} columns={1}>
    <slot {submitting} />
  </FormLayout>

  <svelte:fragment slot="actions">
    <Button
      variant="ghost"
      on:click={handleCancel}
      isDisabled={submitting}
    >
      {cancelLabel}
    </Button>
    <Button
      variant="primary"
      on:click={handleSubmit}
      isDisabled={submitting}
    >
      {submitting ? "Submitting\u2026" : submitLabel}
    </Button>
  </svelte:fragment>
</Dialog>
