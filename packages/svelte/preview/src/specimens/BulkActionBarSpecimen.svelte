<script lang="ts">
  import { BulkActionBar, type BulkAction } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  const actions: BulkAction[] = [
    { id: "export", label: "Export", icon: "download" },
    { id: "archive", label: "Archive", icon: "inbox" },
    { id: "delete", label: "Delete", icon: "trash-2", tone: "danger" },
    { id: "review", label: "Review", icon: "triangle-alert", tone: "warning" },
  ];

  let lastAction = "";
  let allSelected = false;
</script>

<SpecimenLayout>
  <SpecimenGroup label="With selection count and select all" bare>
    <BulkActionBar
      selectionCount={5}
      totalCount={42}
      {actions}
      showSelectAll
      {allSelected}
      on:action={(e) => (lastAction = e.detail.id)}
      on:selectAll={() => (allSelected = true)}
      on:clear={() => (allSelected = false)}
    />
    {#if lastAction}
      <p class="specimen__hint">Last action: <strong>{lastAction}</strong></p>
    {/if}
  </SpecimenGroup>

  <SpecimenGroup label="Single item selected" bare>
    <BulkActionBar selectionCount={1} actions={actions.slice(0, 2)} />
  </SpecimenGroup>

  <SpecimenGroup label="Loading and disabled actions" bare>
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

  <svelte:fragment slot="sizes" let:size>
    <BulkActionBar selectionCount={5} {actions} {size} />
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <BulkActionBar selectionCount={5} {actions} {density} />
  </svelte:fragment>
</SpecimenLayout>

<style>
  .specimen__hint {
    margin: 0;
    font-size: 0.75rem;
    color: var(--poodle-color-text-secondary);
  }
</style>
