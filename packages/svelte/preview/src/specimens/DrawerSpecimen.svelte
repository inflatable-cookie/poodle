<script lang="ts">
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import { Drawer, Button } from "@inflatable-cookie/poodle-svelte";

  let rightOpen = $state(false);
  let leftOpen = $state(false);
</script>

  <SpecimenGroup label="Right edge (default)">
    <Button variant="secondary" onClick={() => (rightOpen = true)}>Open right drawer</Button>
  </SpecimenGroup>

      <SpecimenGroup label="Left edge">
    <Button variant="secondary" onClick={() => (leftOpen = true)}>Open left drawer</Button>
  </SpecimenGroup>

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
