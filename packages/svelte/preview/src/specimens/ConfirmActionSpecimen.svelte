<script lang="ts">
  import { ConfirmAction } from "@flint/svelte-composites";
  import { Eyebrow, Button, IconButton } from "@flint/svelte-primitives";

  let lastAction = "";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default trigger (danger)</Eyebrow>
    <ConfirmAction
      title="Delete this record?"
      description="This record will be permanently removed."
      triggerLabel="Delete record"
      confirmLabel="Delete"
      on:confirm={() => (lastAction = "Record deleted")}
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>Warning tone</Eyebrow>
    <ConfirmAction
      title="Archive this project?"
      description="The project will be moved to the archive and can be restored later."
      tone="warning"
      triggerLabel="Archive project"
      confirmLabel="Archive"
      on:confirm={() => (lastAction = "Project archived")}
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>Custom trigger slot</Eyebrow>
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
  </div>

  <div class="specimen__group">
    <Eyebrow>With body content</Eyebrow>
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

  .key-display {
    padding: 0.5rem 0.75rem;
    border-radius: 0.375rem;
    background: var(--flint-color-background-panel, #1a1a1a);
  }

  .key-display code {
    font-family: var(--flint-typography-mono-family, monospace);
    font-size: 0.8125rem;
  }

  p {
    margin: 0;
  }
</style>
