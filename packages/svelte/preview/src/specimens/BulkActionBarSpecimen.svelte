<script lang="ts">
  import { BulkActionBar, Eyebrow, type BulkAction, type ControlDensity } from "@poodle/svelte-primitives";

  const densities: ControlDensity[] = ["compact", "default", "comfortable"];

  const actions: BulkAction[] = [
    { id: "export", label: "Export", icon: "download" },
    { id: "archive", label: "Archive", icon: "inbox" },
    { id: "delete", label: "Delete", icon: "trash-2", tone: "danger" },
    { id: "review", label: "Review", icon: "triangle-alert", tone: "warning" },
  ];

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let lastAction = "";
  let allSelected = false;
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>With selection count</Eyebrow>
    <BulkActionBar
      selectionCount={5}
      totalCount={42}
      {actions}
      showSelectAll
      {allSelected}
      on:action={(e) => (lastAction = e.detail.id)}
      on:selectAll={() => (allSelected = !allSelected)}
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
    <Eyebrow>Densities</Eyebrow>
    <div class="specimen__stack">
      {#each densities as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <BulkActionBar selectionCount={5} {actions} {density} />
        </div>
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Single item selected</Eyebrow>
    <BulkActionBar selectionCount={1} actions={actions.slice(0, 2)} />
  </div>

  <div class="specimen__group">
    <Eyebrow>Loading and disabled actions</Eyebrow>
    <BulkActionBar
      selectionCount={12}
      totalCount={12}
      actions={[
        { id: "publish", label: "Publish", icon: "rocket" },
        { id: "delete", label: "Delete", icon: "trash-2", tone: "danger", disabled: true },
      ]}
      showSelectAll
      allSelected
      loading
    />
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

  .specimen__row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .specimen__label {
    font-size: 0.75rem;
    font-family: var(--poodle-typography-code-family);
    color: var(--poodle-color-text-muted);
    min-width: 6rem;
  }

  .specimen__group p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--poodle-color-text-secondary);
  }
</style>
