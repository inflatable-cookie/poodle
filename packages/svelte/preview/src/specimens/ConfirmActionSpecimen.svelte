<script lang="ts">
  import { ConfirmAction } from "@inflatable-cookie/poodle-svelte";
  import { Button, IconButton } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let lastAction = $state("");
</script>

<SpecimenLayout>
  <SpecimenGroup label="Default trigger (danger)">
    <ConfirmAction
      title="Delete this record?"
      description="This record will be permanently removed."
      triggerLabel="Delete record"
      confirmLabel="Delete"
      onConfirm={() => {
        lastAction = "Record deleted";
      }}
    />
  </SpecimenGroup>

  <SpecimenGroup label="Warning tone">
    <ConfirmAction
      title="Archive this project?"
      description="The project will be moved to the archive and can be restored later."
      tone="warning"
      triggerLabel="Archive project"
      confirmLabel="Archive"
      onConfirm={() => {
        lastAction = "Project archived";
      }}
    />
  </SpecimenGroup>

  <SpecimenGroup label="Custom trigger slot">
    <ConfirmAction
      title="Remove all filters?"
      description="This will clear all active filters and show all items."
      tone="warning"
      confirmLabel="Clear all"
      onConfirm={() => {
        lastAction = "Filters cleared";
      }}
    >
      {#snippet trigger()}
        <Button variant="ghost">Clear filters</Button>
      {/snippet}
    </ConfirmAction>
  </SpecimenGroup>

  <SpecimenGroup label="With body content">
    <ConfirmAction
      title="Revoke API key?"
      description="This key will immediately stop working."
      confirmLabel="Revoke"
      onConfirm={() => {
        lastAction = "Key revoked";
      }}
    >
      <div class="poodle-key-display">
        <code>pk_live_abc123...xyz789</code>
      </div>
    </ConfirmAction>
  </SpecimenGroup>

  {#if lastAction}
    <SpecimenGroup label="Last action">
      <p>{lastAction}</p>
    </SpecimenGroup>
  {/if}

  {#snippet sizes(size)}
    <ConfirmAction
      title="Delete this record?"
      description="This record will be permanently removed."
      triggerLabel="Delete record"
      confirmLabel="Delete"
      {size}
    />
  {/snippet}

  {#snippet densities(density)}
    <ConfirmAction
      title="Delete this record?"
      description="This record will be permanently removed."
      triggerLabel="Delete record"
      confirmLabel="Delete"
      {density}
    />
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-key-display {
    padding: 0.5rem 0.75rem;
    border-radius: 0.375rem;
    background: var(--poodle-color-background-panel, #1a1a1a);
  }

  .poodle-key-display code {
    font-family: var(--poodle-typography-mono-family, monospace);
    font-size: 0.8125rem;
  }

  p {
    margin: 0;
  }
</style>
