<script lang="ts">
  import { BulkActionBar, type BulkAction } from "@pug/svelte-composites";
  import { Eyebrow } from "@pug/svelte-primitives";

  const actions: BulkAction[] = [
    { id: "export", label: "Export" },
    { id: "archive", label: "Archive" },
    { id: "delete", label: "Delete", tone: "danger" },
  ];

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

  .specimen__group p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--pug-color-text-secondary);
  }
</style>
