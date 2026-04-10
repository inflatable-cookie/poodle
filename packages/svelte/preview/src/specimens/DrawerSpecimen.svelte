<script lang="ts">
  import { Drawer, Button } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let rightOpen = false;
  let leftOpen = false;
  let sizeOpenMap: Record<string, boolean> = {};
  let densityOpenMap: Record<string, boolean> = {};
</script>

<SpecimenLayout>
  <SpecimenGroup label="Right edge (default)">
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
  </SpecimenGroup>

  <SpecimenGroup label="Left edge">
    <Button variant="secondary" on:click={() => (leftOpen = true)}>Open left drawer</Button>
    <Drawer
      open={leftOpen}
      edge="left"
      title="Navigation"
      on:openChange={(e) => (leftOpen = e.detail.open)}
    >
      <p>Side navigation or filters can live in a left-edge drawer.</p>
    </Drawer>
  </SpecimenGroup>

  <svelte:fragment slot="sizes" let:size>
    <Button variant="secondary" {size} on:click={() => (sizeOpenMap[size] = true)}>{size}</Button>
    <Drawer
      {size}
      open={sizeOpenMap[size] ?? false}
      title="Drawer at {size}"
      description="Header and action chrome scale with the size prop."
      on:openChange={(e) => (sizeOpenMap[size] = e.detail.open)}
    >
      <p>Content at <strong>{size}</strong> size.</p>
      <svelte:fragment slot="actions">
        <Button variant="secondary" {size} on:click={() => (sizeOpenMap[size] = false)}>Cancel</Button>
        <Button {size} on:click={() => (sizeOpenMap[size] = false)}>Save</Button>
      </svelte:fragment>
    </Drawer>
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <Button variant="secondary" {density} on:click={() => (densityOpenMap[density] = true)}>{density}</Button>
    <Drawer
      {density}
      open={densityOpenMap[density] ?? false}
      title="Drawer at {density} density"
      description="Internal spacing adjusts with the density prop."
      on:openChange={(e) => (densityOpenMap[density] = e.detail.open)}
    >
      <p>Content at <strong>{density}</strong> density.</p>
      <svelte:fragment slot="actions">
        <Button variant="secondary" {density} on:click={() => (densityOpenMap[density] = false)}>Cancel</Button>
        <Button {density} on:click={() => (densityOpenMap[density] = false)}>Save</Button>
      </svelte:fragment>
    </Drawer>
  </svelte:fragment>
</SpecimenLayout>

<style>
  p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--poodle-color-text-secondary);
  }
</style>
