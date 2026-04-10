<script lang="ts">
  import { Drawer, Button, Eyebrow, Surface } from "@poodle/svelte-primitives";

  let rightOpen = false;
  let leftOpen = false;
</script>

<Surface tone="panel" border="subtle" padding="md">
  <div class="specimen">
    <div class="specimen__row">
      <Eyebrow>Right edge (default)</Eyebrow>
      <Button variant="secondary" on:click={() => (rightOpen = true)}>Open right drawer</Button>
    </div>

    <div class="specimen__row">
      <Eyebrow>Left edge</Eyebrow>
      <Button variant="secondary" on:click={() => (leftOpen = true)}>Open left drawer</Button>
    </div>
  </div>
</Surface>

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

<Drawer
  open={leftOpen}
  edge="left"
  title="Navigation"
  on:openChange={(e) => (leftOpen = e.detail.open)}
>
  <p>Side navigation or filters can live in a left-edge drawer.</p>
</Drawer>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .specimen__row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    align-items: center;
  }

  p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--poodle-color-text-secondary);
  }
</style>
