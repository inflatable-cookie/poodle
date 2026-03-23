<script lang="ts">
  import { Eyebrow, Icon, NavCard, NavCardGrid } from "@poodle/svelte-primitives";
  import { house, layers, slidersHorizontal, fileText } from "@poodle/icons-lucide";

  let lastClick = "";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Navigation card grid (2 columns)</Eyebrow>
    <NavCardGrid columns={2}>
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
        isDisabled
      >
        <svelte:fragment slot="icon"><Icon icon={fileText} /></svelte:fragment>
      </NavCard>
    </NavCardGrid>
  </div>

  <div class="specimen__group">
    <Eyebrow>Single card (as link)</Eyebrow>
    <NavCard
      title="View Documentation"
      description="Open the full documentation site."
      href="#"
    />
  </div>

  {#if lastClick}
    <div class="specimen__group">
      <Eyebrow>Last click</Eyebrow>
      <p>{lastClick}</p>
    </div>
  {/if}
</div>

<style>
  .specimen { display: flex; flex-direction: column; gap: 1.5rem; }
  .specimen__group { display: flex; flex-direction: column; gap: 0.5rem; }
  p { margin: 0; }
</style>
