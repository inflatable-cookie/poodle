<script lang="ts">
  import { Grid, Icon, NavCard } from "@poodle/svelte";
  import { house, layers, slidersHorizontal, fileText } from "@poodle/icons-lucide";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  let lastClick = "";
</script>

<div class="poodle-specimen">
  <SpecimenGroup label="Navigation cards in a grid (2 columns)">
    <Grid columns="1fr 1fr" gap="md" asRole="navigation" ariaLabel="Navigation cards">
      <NavCard
        title="Getting Started"
        description="Learn the basics of the component library."
        on:click={() => (lastClick = "Getting Started")}
      >
        <svelte:fragment slot="icon"><Icon icon={house} /></svelte:fragment>
      </NavCard>
      <NavCard
        title="Components"
        description="Browse all available components."
        badge="New"
        on:click={() => (lastClick = "Components")}
      >
        <svelte:fragment slot="icon"><Icon icon={layers} /></svelte:fragment>
      </NavCard>
      <NavCard
        title="Tokens"
        description="Design tokens and theming system."
        on:click={() => (lastClick = "Tokens")}
      >
        <svelte:fragment slot="icon"><Icon icon={slidersHorizontal} /></svelte:fragment>
      </NavCard>
      <NavCard
        title="API Reference"
        description="Complete component API documentation."
        disabled
      >
        <svelte:fragment slot="icon"><Icon icon={fileText} /></svelte:fragment>
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

<style>
  .poodle-specimen { display: flex; flex-direction: column; gap: 1rem; }
  p { margin: 0; }
</style>
