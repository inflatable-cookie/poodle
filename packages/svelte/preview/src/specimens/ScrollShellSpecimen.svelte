<script lang="ts">
  import { ScrollShell, Surface } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
</script>

<div class="poodle-specimen">
  <SpecimenGroup label="Vertical scroll">
    <div class="poodle-scroll-container">
      <ScrollShell direction="vertical" label="Scrollable content">
        {#each Array(12) as _, i}
          <Surface padding="sm" border="subtle">
            <p>Item {i + 1} — Scroll to see more content below</p>
          </Surface>
        {/each}
      </ScrollShell>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Horizontal scroll">
    <ScrollShell direction="horizontal" label="Horizontal items">
      <div class="poodle-h-row">
        {#each Array(10) as _, i}
          <Surface padding="sm" border="subtle">
            <p class="poodle-h-item">Column {i + 1}</p>
          </Surface>
        {/each}
      </div>
    </ScrollShell>
  </SpecimenGroup>
</div>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-scroll-container {
    height: 10rem;
    border: 1px solid var(--poodle-color-border-default);
    border-radius: 4px;
  }

  .poodle-h-row {
    display: flex;
    gap: 0.5rem;
  }

  .poodle-h-item {
    white-space: nowrap;
  }

  p {
    margin: 0;
    font-size: 0.875rem;
  }
</style>
