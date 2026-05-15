<script lang="ts">
  import { Drawer, Button, Eyebrow, Surface } from "@poodle/svelte";

  let rightOpen = false;
  let leftOpen = false;
</script>

<Surface tone="panel" border="subtle" padding="md">
  <div class="poodle-specimen">
    <div class="poodle-specimen__row">
      <Eyebrow>Right edge (default)</Eyebrow>
      <Button variant="secondary" onClick={() => (rightOpen = true)}>Open right drawer</Button>
    </div>

    <div class="poodle-specimen__row">
      <Eyebrow>Left edge</Eyebrow>
      <Button variant="secondary" onClick={() => (leftOpen = true)}>Open left drawer</Button>
    </div>
  </div>
</Surface>

<Drawer
  open={rightOpen}
  title="Settings"
  description="Configure your preferences."
  onOpenChange={(open) => (rightOpen = open)}
>
  <p>Drawer content goes here. You can put forms, navigation, or any other content.</p>
  {#snippet actions()}
    <Button variant="secondary" onClick={() => (rightOpen = false)}>Cancel</Button>
    <Button onClick={() => (rightOpen = false)}>Save</Button>
  {/snippet}
</Drawer>

<Drawer
  open={leftOpen}
  edge="left"
  title="Navigation"
  onOpenChange={(open) => (leftOpen = open)}
>
  <p>Side navigation or filters can live in a left-edge drawer.</p>
</Drawer>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .poodle-specimen__row {
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
