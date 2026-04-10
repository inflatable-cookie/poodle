<script lang="ts">
  import { AlertDialog, Button, Eyebrow, Surface } from "@poodle/svelte-primitives";

  let dangerOpen: boolean | null = null;
  let warningOpen: boolean | null = null;
  let asyncOpen: boolean | null = null;
  let lastAction = "";

  async function simulateAsync(): Promise<void> {
    await new Promise((resolve) => setTimeout(resolve, 1500));
    lastAction = "Async confirm completed";
  }
</script>

<Surface tone="panel" border="subtle" padding="md">
  <div class="specimen">
    <div class="specimen__row">
      <Eyebrow>Danger tone</Eyebrow>
      <Button tone="danger" on:click={() => (dangerOpen = true)}>Delete item</Button>
      <AlertDialog
        open={dangerOpen}
        title="Delete this item?"
        description="This action cannot be undone. The item and all associated data will be permanently removed."
        confirmLabel="Delete"
        cancelLabel="Keep it"
        on:confirm={() => { lastAction = "Item deleted"; dangerOpen = false; }}
        on:cancel={() => (dangerOpen = false)}
        on:openChange={(e) => (dangerOpen = e.detail.open ? true : null)}
      />
    </div>

    <div class="specimen__row">
      <Eyebrow>Warning tone</Eyebrow>
      <Button variant="secondary" on:click={() => (warningOpen = true)}>Reset settings</Button>
      <AlertDialog
        open={warningOpen}
        title="Reset all settings?"
        description="Your customized settings will be restored to their default values."
        tone="warning"
        confirmLabel="Reset"
        cancelLabel="Cancel"
        on:confirm={() => { lastAction = "Settings reset"; warningOpen = false; }}
        on:cancel={() => (warningOpen = false)}
        on:openChange={(e) => (warningOpen = e.detail.open ? true : null)}
      />
    </div>

    <div class="specimen__row">
      <Eyebrow>Async confirm</Eyebrow>
      <Button tone="danger" on:click={() => (asyncOpen = true)}>Archive project</Button>
      <AlertDialog
        open={asyncOpen}
        title="Archive this project?"
        description="The project will be hidden from active lists but can still be restored later."
        confirmLabel="Archive"
        workingLabel="Archiving…"
        onConfirm={async () => { await simulateAsync(); asyncOpen = false; }}
        on:cancel={() => (asyncOpen = false)}
        on:openChange={(e) => (asyncOpen = e.detail.open ? true : null)}
      >
        <div class="user-card">
          <strong>Roadmap Cleanup</strong>
          <span>14 linked tasks will move to the archived view.</span>
        </div>
      </AlertDialog>
    </div>

    {#if lastAction}
      <p class="specimen__hint">Last action: <strong>{lastAction}</strong></p>
    {/if}
  </div>
</Surface>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .specimen__row {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .specimen__hint {
    margin: 0;
    font-size: 0.75rem;
    color: var(--poodle-color-text-secondary);
  }

  .user-card {
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
    padding: 0.5rem 0.75rem;
    border-radius: 0.375rem;
    background: var(--poodle-color-background-panel);
  }

  .user-card span {
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
  }
</style>
