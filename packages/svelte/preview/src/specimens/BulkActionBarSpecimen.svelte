<script lang="ts">
  import { BulkActionBar, type BulkAction } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  const actions: BulkAction[] = [
    { id: "export", label: "Export", icon: "download" },
    { id: "archive", label: "Archive", icon: "inbox" },
    { id: "delete", label: "Delete", icon: "trash-2", tone: "danger" },
    { id: "review", label: "Review", icon: "triangle-alert", tone: "warning" },
  ];

  let lastAction = $state("");
  let allSelected = $state(false);
</script>

<SpecimenLayout bareVariants>
  {#snippet children()}
    <div class="poodle-specimen">
      <SpecimenGroup label="With selection count and select all" bare>
        <div class="poodle-bulk-action-bar-specimen__inline">
          <BulkActionBar
            selectionCount={5}
            totalCount={42}
            {actions}
            showSelectAll
            {allSelected}
            onAction={(id) => (lastAction = id)}
            onSelectAll={() => (allSelected = true)}
            onClear={() => (allSelected = false)}
          />
        </div>
        {#if lastAction}
          <p class="poodle-specimen__hint">Last action: <strong>{lastAction}</strong></p>
        {/if}
      </SpecimenGroup>

      <SpecimenGroup label="Single item selected" bare>
        <div class="poodle-bulk-action-bar-specimen__inline">
          <BulkActionBar selectionCount={1} actions={actions.slice(0, 2)} />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Loading and disabled actions" bare>
        <div class="poodle-bulk-action-bar-specimen__inline">
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
      </SpecimenGroup>
    </div>
  {/snippet}

  {#snippet sizes(size)}
    <div class="poodle-bulk-action-bar-specimen__inline">
      <BulkActionBar selectionCount={5} {actions} {size} />
    </div>
  {/snippet}

  {#snippet densities(density)}
    <div class="poodle-bulk-action-bar-specimen__inline">
      <BulkActionBar selectionCount={5} {actions} {density} />
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-bulk-action-bar-specimen__inline {
    display: flex;
  }

  .poodle-bulk-action-bar-specimen__inline :global(.poodle-bulk-action-bar) {
    position: static;
    right: auto;
    bottom: auto;
    left: auto;
    flex: 1 1 auto;
    width: 100%;
    max-width: none;
  }

  .poodle-specimen__hint {
    margin: 0;
    font-size: 0.75rem;
    color: var(--poodle-color-text-secondary);
  }
</style>
