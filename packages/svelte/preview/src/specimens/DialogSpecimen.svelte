<script lang="ts">
  import { Dialog, Button, Eyebrow } from "@poodle/svelte-primitives";

  let basicOpen = false;
  let alertOpen = false;
  let noBackdropOpen = false;
  let closeButtonOpen = false;
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Basic dialog</Eyebrow>
    <Button variant="secondary" on:click={() => (basicOpen = true)}>Open dialog</Button>
    <Dialog
      open={basicOpen}
      title="Confirm action"
      description="Are you sure you want to proceed? This action cannot be undone."
      on:openChange={(e) => (basicOpen = e.detail.open)}
    >
      <p>This is the dialog body content. You can place any content here.</p>
      <svelte:fragment slot="actions">
        <Button variant="ghost" on:click={() => (basicOpen = false)}>Cancel</Button>
        <Button on:click={() => (basicOpen = false)}>Confirm</Button>
      </svelte:fragment>
    </Dialog>
  </div>

  <div class="specimen__group">
    <Eyebrow>Alert dialog</Eyebrow>
    <Button tone="danger" on:click={() => (alertOpen = true)}>Delete item</Button>
    <Dialog
      open={alertOpen}
      title="Delete item?"
      description="This will permanently remove the item and all associated data."
      kind="alertdialog"
      on:openChange={(e) => (alertOpen = e.detail.open)}
    >
      <svelte:fragment slot="actions">
        <Button variant="ghost" on:click={() => (alertOpen = false)}>Cancel</Button>
        <Button variant="primary" tone="danger" on:click={() => (alertOpen = false)}>Delete</Button>
      </svelte:fragment>
    </Dialog>
  </div>

  <div class="specimen__group">
    <Eyebrow>No backdrop dismiss</Eyebrow>
    <Button variant="secondary" on:click={() => (noBackdropOpen = true)}>Open persistent dialog</Button>
    <Dialog
      open={noBackdropOpen}
      title="Persistent dialog"
      description="This dialog can only be closed via the buttons or Escape key."
      dismissOnBackdrop={false}
      on:openChange={(e) => (noBackdropOpen = e.detail.open)}
    >
      <p>Click the backdrop — nothing happens. Use the button below to close.</p>
      <svelte:fragment slot="actions">
        <Button on:click={() => (noBackdropOpen = false)}>Got it</Button>
      </svelte:fragment>
    </Dialog>
  </div>

  <div class="specimen__group">
    <Eyebrow>Custom chrome with close button</Eyebrow>
    <Button variant="secondary" on:click={() => (closeButtonOpen = true)}>Open styled dialog</Button>
    <Dialog
      open={closeButtonOpen}
      title="Review upload notes"
      description="This dialog shows the built-in close button and custom surface classes."
      showCloseButton
      closeLabel="Close upload notes dialog"
      contentClassName="dialog--specimen"
      overlayClassName="dialog__backdrop--specimen"
      on:openChange={(e) => (closeButtonOpen = e.detail.open)}
    >
      <p>The dialog surface and overlay can be targeted with custom class names.</p>
      <svelte:fragment slot="actions">
        <Button on:click={() => (closeButtonOpen = false)}>Done</Button>
      </svelte:fragment>
    </Dialog>
  </div>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .specimen__group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    align-items: flex-start;
  }

  .specimen__group p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--poodle-color-text-secondary);
  }

  :global(.dialog--specimen) {
    width: min(40rem, 100%);
  }

  :global(.dialog__backdrop--specimen) {
    background: color-mix(in srgb, var(--poodle-color-background-overlay) 82%, #0f172a);
  }
</style>
