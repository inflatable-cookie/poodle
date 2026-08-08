<script lang="ts">
  import { Grid, Icon, NavCard } from "@inflatable-cookie/poodle-svelte";
  import { house, layers, slidersHorizontal, fileText } from "@inflatable-cookie/poodle-core/icons";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let lastClick = $state("");
</script>

<SpecimenLayout showSizes={false}>
  <div class="poodle-specimen">
    <SpecimenGroup label="Navigation cards in a grid (2 columns)">
      <Grid columns="1fr 1fr" gap="md" asRole="navigation" ariaLabel="Navigation cards">
        <NavCard
          title="Getting Started"
          description="Learn the basics of the component library."
          onClick={() => (lastClick = "Getting Started")}
        >
          {#snippet icon()}<Icon icon={house} />{/snippet}
        </NavCard>
        <NavCard
          title="Components"
          description="Browse all available components."
          badge="New"
          onClick={() => (lastClick = "Components")}
        >
          {#snippet icon()}<Icon icon={layers} />{/snippet}
        </NavCard>
        <NavCard
          title="Tokens"
          description="Design tokens and theming system."
          onClick={() => (lastClick = "Tokens")}
        >
          {#snippet icon()}<Icon icon={slidersHorizontal} />{/snippet}
        </NavCard>
        <NavCard
          title="API Reference"
          description="Complete component API documentation."
          disabled
        >
          {#snippet icon()}<Icon icon={fileText} />{/snippet}
        </NavCard>
      </Grid>
    </SpecimenGroup>

    <SpecimenGroup label="Single card (as link)">
      <NavCard
        title="View Documentation"
        description="Open the full documentation site."
        href="#"
      />
    </SpecimenGroup>

    {#if lastClick}
      <SpecimenGroup label="Last click">
        <p>{lastClick}</p>
      </SpecimenGroup>
    {/if}
  </div>

  {#snippet densities(density)}
    <div class="poodle-density-demo">
      <NavCard
        title="Components"
        description="Browse all available components."
        badge="New"
        {density}
      >
        {#snippet icon()}<Icon icon={layers} />{/snippet}
      </NavCard>
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen { display: flex; flex-direction: column; gap: 1rem; }
  .poodle-density-demo { width: min(100%, 18rem); }
  p { margin: 0; }
</style>
