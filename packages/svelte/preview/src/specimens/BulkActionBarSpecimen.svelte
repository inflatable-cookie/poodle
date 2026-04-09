<script lang="ts">
  import { BulkActionBar, type BulkAction, type ControlDensity } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

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
  <SpecimenGroup label="With selection count">
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
  </SpecimenGroup>

  <SpecimenGroup label="Sizes">
    <div class="specimen__stack">
      {#each controlSizes as size}
        <BulkActionBar selectionCount={5} {actions} {size} />
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Densities">
    <div class="specimen__stack">
      {#each densities as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <BulkActionBar selectionCount={5} {actions} {density} />
        </div>
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Single item selected">
    <BulkActionBar selectionCount={1} actions={actions.slice(0, 2)} />
  </SpecimenGroup>

  <SpecimenGroup label="Loading and disabled actions">
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
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
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

</style>
