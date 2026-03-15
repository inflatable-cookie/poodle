<script lang="ts">
  import { Drawer, Button, Eyebrow } from "@pug/svelte-primitives";

  let rightOpen = false;
  let leftOpen = false;
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Right edge (default)</Eyebrow>
    <Button on:click={() => (rightOpen = true)}>Open right drawer</Button>
    <Drawer
      open={rightOpen}
      title="Settings"
      description="Configure your preferences."
      on:openChange={(e) => (rightOpen = e.detail.open)}
    >
      <p>Drawer content goes here. You can put forms, navigation, or any other content.</p>
      <svelte:fragment slot="actions">
        <Button variant="secondary" on:click={() => (rightOpen = false)}>Cancel</Button>
        <Button on:click={() => (rightOpen = false)}>Save</Button>
      </svelte:fragment>
    </Drawer>
  </div>

  <div class="specimen__group">
    <Eyebrow>Left edge</Eyebrow>
    <Button variant="secondary" on:click={() => (leftOpen = true)}>Open left drawer</Button>
    <Drawer
      open={leftOpen}
      edge="left"
      title="Navigation"
      on:openChange={(e) => (leftOpen = e.detail.open)}
    >
      <p>Side navigation or filters can live in a left-edge drawer.</p>
    </Drawer>
  </div>
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

  p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--pug-color-text-secondary);
  }
</style>
