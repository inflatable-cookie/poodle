<script lang="ts">
  import { AlertDialog, Button, Eyebrow } from "@poodle/svelte-primitives";

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;
  let sizeOpenMap: Record<string, boolean | null> = {};

  let dangerOpen: boolean | null = null;
  let warningOpen: boolean | null = null;
  let asyncOpen: boolean | null = null;
  let lastAction = "";

  async function simulateAsync(): Promise<void> {
    await new Promise((resolve) => setTimeout(resolve, 1500));
    lastAction = "Async confirm completed";
  }
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Danger tone (default)</Eyebrow>
    <Button tone="danger" on:click={() => (dangerOpen = true)}>Delete item</Button>
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
    <Eyebrow>Async confirm callback</Eyebrow>
    <Button tone="danger" on:click={() => (asyncOpen = true)}>Archive project</Button>
    <AlertDialog
      open={asyncOpen}
      title="Archive this project?"
      description="The project will be hidden from active lists but can still be restored later."
      confirmLabel="Archive"
      workingLabel="Archiving…"
      onConfirm={async () => {
        await simulateAsync();
        asyncOpen = false;
      }}
      on:cancel={() => (asyncOpen = false)}
      on:openChange={(e) => (asyncOpen = e.detail.open ? true : null)}
    >
      <div class="user-card">
        <strong>Roadmap Cleanup</strong>
        <span>14 linked tasks will move to the archived view.</span>
      </div>
    </AlertDialog>
  </div>

  <div class="specimen__group">
    <Eyebrow>Sizes</Eyebrow>
    <div class="specimen__row">
      {#each controlSizes as size}
        <Button variant="secondary" {size} on:click={() => (sizeOpenMap[size] = true)}>{size}</Button>
        <AlertDialog
          open={sizeOpenMap[size] ?? null}
          {size}
          title="Alert at {size}"
          description="Header and action chrome scale with the size prop."
          confirmLabel="Confirm"
          cancelLabel="Cancel"
          on:confirm={() => (sizeOpenMap[size] = false)}
          on:cancel={() => (sizeOpenMap[size] = false)}
          on:openChange={(e) => (sizeOpenMap[size] = e.detail.open ? true : null)}
        />
      {/each}
    </div>
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

  .specimen__row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    align-items: center;
  }

  .user-card {
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
    padding: 0.5rem 0.75rem;
    border-radius: 0.375rem;
    background: var(--poodle-color-background-panel, #1a1a1a);
  }

  .user-card span {
    color: var(--poodle-color-text-secondary, #999);
    font-size: 0.8125rem;
  }

  p {
    margin: 0;
  }
</style>
