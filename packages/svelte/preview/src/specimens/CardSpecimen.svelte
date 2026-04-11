<script lang="ts">
  import { Card } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
</script>

<div class="specimen">
  <SpecimenGroup label="Default variant">
    <div class="specimen__cards">
      <Card ariaLabel="Project card">
        <svelte:fragment slot="header">
          <h3 class="card-title">Project Alpha</h3>
        </svelte:fragment>
        <p class="card-body">A design system component library for building consistent interfaces.</p>
        <svelte:fragment slot="footer">
          <span class="card-meta">Updated 2 days ago</span>
        </svelte:fragment>
      </Card>

      <Card ariaLabel="Stats card">
        <svelte:fragment slot="header">
          <h3 class="card-title">Monthly report</h3>
        </svelte:fragment>
        <p class="card-body">48 components shipped across 3 packages this month.</p>
      </Card>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Outlined variant">
    <Card variant="outlined" ariaLabel="Outlined card">
      <svelte:fragment slot="header">
        <h3 class="card-title">Outlined card</h3>
      </svelte:fragment>
      <p class="card-body">This card uses a subtle border instead of elevation.</p>
    </Card>
  </SpecimenGroup>

  <SpecimenGroup label="Elevated variant">
    <Card variant="elevated" ariaLabel="Elevated card">
      <svelte:fragment slot="header">
        <h3 class="card-title">Elevated card</h3>
      </svelte:fragment>
      <p class="card-body">This card uses a drop shadow for visual prominence.</p>
    </Card>
  </SpecimenGroup>

  <SpecimenGroup label="Interactive">
    <Card interactive ariaLabel="Clickable card">
      <svelte:fragment slot="header">
        <h3 class="card-title">Interactive card</h3>
      </svelte:fragment>
      <p class="card-body">Hover to see the interactive state. Cursor changes to pointer.</p>
    </Card>
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .specimen__cards {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(14rem, 1fr));
    gap: 1rem;
  }

  .card-title {
    font-size: 1rem;
    font-weight: 600;
    margin: 0;
    color: var(--poodle-color-text-primary);
  }

  .card-body {
    font-size: 0.875rem;
    margin: 0;
    color: var(--poodle-color-text-secondary);
    line-height: 1.5;
  }

  .card-meta {
    font-size: 0.75rem;
    color: var(--poodle-color-text-secondary);
  }
</style>
