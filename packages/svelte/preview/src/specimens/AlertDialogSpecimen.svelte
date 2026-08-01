<script lang="ts">
  import { AlertDialog, Button, Eyebrow, Surface } from "@poodle/svelte";

  let dangerOpen = $state(false);
  let warningOpen = $state(false);
  let asyncOpen = $state(false);
  let lastAction = $state("");

  async function simulateAsync(): Promise<void> {
    await new Promise((resolve) => setTimeout(resolve, 1500));
    lastAction = "Async confirm completed";
  }
</script>

<Surface tone="panel" border="subtle" padding="md">
  <div class="poodle-specimen">
    <div class="poodle-specimen__row">
      <Eyebrow>Danger tone</Eyebrow>
      <Button tone="danger" onClick={() => (dangerOpen = true)}>Delete item</Button>
      <AlertDialog
        open={dangerOpen}
        title="Delete this item?"
        description="This action cannot be undone. The item and all associated data will be permanently removed."
        confirmLabel="Delete"
        cancelLabel="Keep it"
        onConfirm={() => { lastAction = "Item deleted"; dangerOpen = false; }}
        onCancel={() => (dangerOpen = false)}
        onOpenChange={(open) => (dangerOpen = open)}
      />
    </div>

    <div class="poodle-specimen__row">
      <Eyebrow>Warning tone</Eyebrow>
      <Button variant="secondary" onClick={() => (warningOpen = true)}>Reset settings</Button>
      <AlertDialog
        open={warningOpen}
        title="Reset all settings?"
        description="Your customized settings will be restored to their default values."
        tone="warning"
        confirmLabel="Reset"
        cancelLabel="Cancel"
        onConfirm={() => { lastAction = "Settings reset"; warningOpen = false; }}
        onCancel={() => (warningOpen = false)}
        onOpenChange={(open) => (warningOpen = open)}
      />
    </div>

    <div class="poodle-specimen__row">
      <Eyebrow>Async confirm</Eyebrow>
      <Button tone="danger" onClick={() => (asyncOpen = true)}>Archive project</Button>
      <AlertDialog
        open={asyncOpen}
        title="Archive this project?"
        description="The project will be hidden from active lists but can still be restored later."
        confirmLabel="Archive"
        workingLabel="Archiving…"
        onConfirm={async () => { await simulateAsync(); asyncOpen = false; }}
        onCancel={() => (asyncOpen = false)}
        onOpenChange={(open) => (asyncOpen = open)}
      >
        <div class="poodle-user-card">
          <strong>Roadmap Cleanup</strong>
          <span>14 linked tasks will move to the archived view.</span>
        </div>
      </AlertDialog>
    </div>

    {#if lastAction}
      <p class="poodle-specimen__hint">Last action: <strong>{lastAction}</strong></p>
    {/if}
  </div>
</Surface>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .poodle-specimen__row {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .poodle-specimen__hint {
    margin: 0;
    font-size: 0.75rem;
    color: var(--poodle-color-text-secondary);
  }

  .poodle-user-card {
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
    padding: 0.5rem 0.75rem;
    border-radius: 0.375rem;
    background: var(--poodle-color-background-panel);
  }

  .poodle-user-card span {
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
  }
</style>
