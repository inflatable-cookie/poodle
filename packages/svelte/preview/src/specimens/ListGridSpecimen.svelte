<script lang="ts">
  import { Button, IconButton, ListGrid, Surface } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
</script>

<div class="poodle-specimen">
  <SpecimenGroup label="Default — auto-fill by min item width (narrow tiles in preview)">
    <ListGrid minItemWidth={14}>
      {#snippet children()}
        {#each ["Alpha", "Bravo", "Charlie", "Delta", "Echo", "Foxtrot"] as title}
          <Surface padding="md" border="subtle">
            <p class="poodle-tile-title">{title}</p>
            <p class="poodle-tile-body">Responsive columns from <code>minItemWidth</code>.</p>
          </Surface>
        {/each}
      {/snippet}
    </ListGrid>
  </SpecimenGroup>

  <SpecimenGroup label="With header actions">
    <ListGrid minItemWidth={16}>
      {#snippet actions()}
        <Button variant="secondary">Export</Button>
        <IconButton icon="refresh-cw" ariaLabel="Refresh list" variant="secondary" />
      {/snippet}
      {#snippet children()}
        {#each ["Project A", "Project B", "Project C"] as title}
          <Surface padding="md" border="subtle">
            <p class="poodle-tile-title">{title}</p>
            <p class="poodle-tile-body">Actions row uses contract header anatomy.</p>
          </Surface>
        {/each}
      {/snippet}
    </ListGrid>
  </SpecimenGroup>

  <SpecimenGroup label="Compact — single column stack">
    <ListGrid variant="compact">
      {#snippet children()}
        {#each ["One", "Two", "Three"] as title}
          <Surface padding="sm" border="subtle">
            <p class="poodle-tile-title">{title}</p>
            <p class="poodle-tile-body">Compact uses one column and tighter default gap.</p>
          </Surface>
        {/each}
      {/snippet}
    </ListGrid>
  </SpecimenGroup>
</div>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-tile-title {
    margin: 0 0 0.25rem;
    font-size: 0.875rem;
    font-weight: 600;
  }

  .poodle-tile-body {
    margin: 0;
    font-size: 0.8125rem;
    color: var(--poodle-color-text-secondary, #888);
  }

  code {
    font-size: 0.75em;
  }
</style>
