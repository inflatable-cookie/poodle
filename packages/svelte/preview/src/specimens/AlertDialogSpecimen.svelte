<script lang="ts">
  import { AlertDialog, Button, Eyebrow } from "@pug/svelte-primitives";

  let dangerOpen: boolean | null = null;
  let warningOpen: boolean | null = null;
  let asyncOpen: boolean | null = null;
  let lastAction = "";

  function simulateAsync(): void {
    setTimeout(() => {
      lastAction = "Async confirm completed";
    }, 1500);
  }
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Danger tone (default)</Eyebrow>
    <Button variant="danger" on:click={() => (dangerOpen = true)}>Delete item</Button>
    <AlertDialog
      open={dangerOpen}
      title="Delete this item?"
      description="This action cannot be undone. The item and all associated data will be permanently removed."
      confirmLabel="Delete"
      cancelLabel="Keep it"
      on:confirm={() => {
        lastAction = "Item deleted";
        dangerOpen = false;
      }}
      on:cancel={() => (dangerOpen = false)}
      on:openChange={(e) => (dangerOpen = e.detail.open ? true : null)}
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>Warning tone</Eyebrow>
    <Button variant="secondary" on:click={() => (warningOpen = true)}>Reset settings</Button>
    <AlertDialog
      open={warningOpen}
      title="Reset all settings?"
      description="Your customized settings will be restored to their default values."
      tone="warning"
      confirmLabel="Reset"
      cancelLabel="Cancel"
      on:confirm={() => {
        lastAction = "Settings reset";
        warningOpen = false;
      }}
      on:cancel={() => (warningOpen = false)}
      on:openChange={(e) => (warningOpen = e.detail.open ? true : null)}
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>With body content</Eyebrow>
    <Button variant="danger" on:click={() => (asyncOpen = true)}>Remove user</Button>
    <AlertDialog
      open={asyncOpen}
      title="Remove this user?"
      description="The following user will lose access to this workspace."
      confirmLabel="Remove"
      on:confirm={() => {
        simulateAsync();
        asyncOpen = false;
      }}
      on:cancel={() => (asyncOpen = false)}
      on:openChange={(e) => (asyncOpen = e.detail.open ? true : null)}
    >
      <div class="user-card">
        <strong>Clay Tercek</strong>
        <span>clay@example.com</span>
      </div>
    </AlertDialog>
  </div>

  {#if lastAction}
    <div class="specimen__group">
      <Eyebrow>Last action</Eyebrow>
      <p>{lastAction}</p>
    </div>
  {/if}
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
  }

  .user-card {
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
    padding: 0.5rem 0.75rem;
    border-radius: 0.375rem;
    background: var(--pug-color-background-panel, #1a1a1a);
  }

  .user-card span {
    color: var(--pug-color-text-secondary, #999);
    font-size: 0.8125rem;
  }

  p {
    margin: 0;
  }
</style>
