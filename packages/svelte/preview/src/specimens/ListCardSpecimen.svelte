<script lang="ts">
  import { ListCard } from "@pug/svelte-composites";
  import { Eyebrow, Icon } from "@pug/svelte-primitives";

  let lastClick = "";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Interactive list cards</Eyebrow>
    <div class="specimen__stack">
      <ListCard
        title="design-system-v2.figma"
        subtitle="Updated by Clay · 2h ago"
        meta="14.2 MB"
        isInteractive
        on:click={() => (lastClick = "design-system-v2.figma")}
      >
        <svelte:fragment slot="leading"><Icon name="folder" /></svelte:fragment>
      </ListCard>
      <ListCard
        title="component-specs.pdf"
        subtitle="Shared with team · Yesterday"
        meta="2.8 MB"
        isInteractive
        on:click={() => (lastClick = "component-specs.pdf")}
      >
        <svelte:fragment slot="leading"><Icon name="file-text" /></svelte:fragment>
      </ListCard>
      <ListCard
        title="brand-assets.zip"
        subtitle="Archived"
        meta="48 MB"
        isDisabled
      >
        <svelte:fragment slot="leading"><Icon name="layers" /></svelte:fragment>
      </ListCard>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Static list card</Eyebrow>
    <ListCard title="Read-only item" subtitle="No click handler" />
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
  .specimen__stack { display: flex; flex-direction: column; gap: 0.25rem; }
  p { margin: 0; }
</style>
