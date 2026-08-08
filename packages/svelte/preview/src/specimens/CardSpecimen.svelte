<script lang="ts">
  import { Card } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
</script>

<SpecimenLayout showSizes={false}>
  <div class="poodle-specimen">
    <SpecimenGroup label="Default variant">
      <div class="poodle-specimen__cards">
        <Card ariaLabel="Project card">
          {#snippet header()}
            <h3 class="poodle-card-title">Project Alpha</h3>
          {/snippet}
          <p class="poodle-card-body">A design system component library for building consistent interfaces.</p>
          {#snippet footer()}
            <span class="poodle-card-meta">Updated 2 days ago</span>
          {/snippet}
        </Card>

        <Card ariaLabel="Stats card">
          {#snippet header()}
            <h3 class="poodle-card-title">Monthly report</h3>
          {/snippet}
          <p class="poodle-card-body">48 components shipped across 3 packages this month.</p>
        </Card>
      </div>
    </SpecimenGroup>

    <SpecimenGroup label="Outlined variant">
      <Card variant="outlined" ariaLabel="Outlined card">
        {#snippet header()}
          <h3 class="poodle-card-title">Outlined card</h3>
        {/snippet}
        <p class="poodle-card-body">This card uses a subtle border instead of elevation.</p>
      </Card>
    </SpecimenGroup>

    <SpecimenGroup label="Elevated variant">
      <Card variant="elevated" ariaLabel="Elevated card">
        {#snippet header()}
          <h3 class="poodle-card-title">Elevated card</h3>
        {/snippet}
        <p class="poodle-card-body">This card uses a drop shadow for visual prominence.</p>
      </Card>
    </SpecimenGroup>

    <SpecimenGroup label="Interactive">
      <Card interactive ariaLabel="Clickable card">
        {#snippet header()}
          <h3 class="poodle-card-title">Interactive card</h3>
        {/snippet}
        <p class="poodle-card-body">Hover to see the interactive state. Cursor changes to pointer.</p>
      </Card>
    </SpecimenGroup>
  </div>

  {#snippet densities(density)}
    <div class="poodle-card-specimen__variant">
      <Card ariaLabel={`${density} density card`} {density}>
        {#snippet header()}
          <h3 class="poodle-card-title">Project Alpha</h3>
        {/snippet}
        <p class="poodle-card-body">A design system component library for building consistent interfaces.</p>
        {#snippet footer()}
          <span class="poodle-card-meta">Updated 2 days ago</span>
        {/snippet}
      </Card>
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-specimen__cards {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(14rem, 1fr));
    gap: 1rem;
  }

  .poodle-card-title {
    font-size: 1rem;
    font-weight: 600;
    margin: 0;
    color: var(--poodle-color-text-primary);
  }

  .poodle-card-body {
    font-size: 0.875rem;
    margin: 0;
    color: var(--poodle-color-text-secondary);
    line-height: 1.5;
  }

  .poodle-card-meta {
    font-size: 0.75rem;
    color: var(--poodle-color-text-secondary);
  }

  .poodle-card-specimen__variant {
    width: min(100%, 22rem);
  }
</style>
