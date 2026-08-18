<script lang="ts">
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
  import { Drawer, Button } from "@inflatable-cookie/poodle-svelte";

  let rightOpen = $state(false);
  let leftOpen = $state(false);
  let axisOpen: Record<string, boolean> = $state({});
</script>

<SpecimenLayout>
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

{#snippet sizes(size)}
  <Button variant="secondary" onClick={() => (axisOpen[`size-${size}`] = true)}>Open {size} drawer</Button>
  <Drawer
    open={axisOpen[`size-${size}`] ?? false}
    onOpenChange={(v) => (axisOpen[`size-${size}`] = v)}
    title="Settings"
    description="Configure your preferences."
    {size}
  >
    <p>Drawer content goes here. You can put forms, navigation, or any other content.</p>
  </Drawer>
{/snippet}

{#snippet densities(density)}
  <Button variant="secondary" onClick={() => (axisOpen[`density-${density}`] = true)}>Open {density} drawer</Button>
  <Drawer
    open={axisOpen[`density-${density}`] ?? false}
    onOpenChange={(v) => (axisOpen[`density-${density}`] = v)}
    title="Settings"
    description="Configure your preferences."
    {density}
  >
    <p>Drawer content goes here. You can put forms, navigation, or any other content.</p>
  </Drawer>
{/snippet}
</SpecimenLayout>

<style>
  p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--poodle-color-text-secondary);
  }
</style>
