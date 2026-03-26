<script lang="ts">
  import { BulkActionBar, Eyebrow, type BulkAction } from "@poodle/svelte-primitives";

  const actions: BulkAction[] = [
    { id: "export", label: "Export", icon: "download" },
    { id: "archive", label: "Archive", icon: "inbox" },
    { id: "delete", label: "Delete", icon: "trash-2", tone: "danger" },
  ];

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let lastAction = "";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>With selection count</Eyebrow>
    <BulkActionBar
      selectionCount={5}
      totalCount={42}
      {actions}
      on:action={(e) => (lastAction = e.detail.id)}
    />
    {#if lastAction}
      <p>Last action: <strong>{lastAction}</strong></p>
    {/if}
  </div>

  <div class="specimen__group">
    <Eyebrow>Sizes</Eyebrow>
    <div class="specimen__stack">
      {#each controlSizes as size}
        <BulkActionBar selectionCount={5} {actions} {size} />
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Single item selected</Eyebrow>
    <BulkActionBar selectionCount={1} actions={actions.slice(0, 2)} />
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
  }

  .specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .specimen__group p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--poodle-color-text-secondary);
  }
</style>
