<script lang="ts">
  import { ConfirmAction } from "@poodle/svelte";
  import { Button, IconButton } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  let lastAction = "";
</script>

<div class="specimen">
  <SpecimenGroup label="Default trigger (danger)">
    <ConfirmAction
      title="Delete this record?"
      description="This record will be permanently removed."
      triggerLabel="Delete record"
      confirmLabel="Delete"
      on:confirm={() => (lastAction = "Record deleted")}
    />
  </SpecimenGroup>

  <SpecimenGroup label="Warning tone">
    <ConfirmAction
      title="Archive this project?"
      description="The project will be moved to the archive and can be restored later."
      tone="warning"
      triggerLabel="Archive project"
      confirmLabel="Archive"
      on:confirm={() => (lastAction = "Project archived")}
    />
  </SpecimenGroup>

  <SpecimenGroup label="Custom trigger slot">
    <ConfirmAction
      title="Remove all filters?"
      description="This will clear all active filters and show all items."
      tone="warning"
      confirmLabel="Clear all"
      on:confirm={() => (lastAction = "Filters cleared")}
    >
      <svelte:fragment slot="trigger">
        <Button variant="ghost">Clear filters</Button>
      </svelte:fragment>
    </ConfirmAction>
  </SpecimenGroup>

  <SpecimenGroup label="With body content">
    <ConfirmAction
      title="Revoke API key?"
      description="This key will immediately stop working."
      confirmLabel="Revoke"
      on:confirm={() => (lastAction = "Key revoked")}
    >
      <div class="key-display">
        <code>pk_live_abc123...xyz789</code>
      </div>
    </ConfirmAction>
  </SpecimenGroup>

  {#if lastAction}
    <SpecimenGroup label="Last action">
      <p>{lastAction}</p>
    </SpecimenGroup>
  {/if}
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .key-display {
    padding: 0.5rem 0.75rem;
    border-radius: 0.375rem;
    background: var(--poodle-color-background-panel, #1a1a1a);
  }

  .key-display code {
    font-family: var(--poodle-typography-mono-family, monospace);
    font-size: 0.8125rem;
  }

  p {
    margin: 0;
  }
</style>
