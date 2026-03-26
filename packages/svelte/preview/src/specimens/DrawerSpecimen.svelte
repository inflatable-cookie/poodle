<script lang="ts">
  import { Drawer, Button, Eyebrow } from "@poodle/svelte-primitives";

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let rightOpen = false;
  let leftOpen = false;
  let sizeOpenMap: Record<string, boolean> = {};
  let densityOpenMap: Record<string, boolean> = {};
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
    <Eyebrow>Sizes</Eyebrow>
    <div class="specimen__row">
      {#each controlSizes as size}
        <Button variant="secondary" {size} on:click={() => (sizeOpenMap[size] = true)}>{size}</Button>
        <Drawer
          open={sizeOpenMap[size] ?? false}
          {size}
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
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Densities</Eyebrow>
    <div class="specimen__row">
      {#each ["compact", "default", "comfortable"] as density}
        <Button variant="secondary" on:click={() => (densityOpenMap[density] = true)}>{density}</Button>
        <Drawer
          open={densityOpenMap[density] ?? false}
          {density}
          title="Drawer at {density} density"
          description="Internal spacing adjusts with the density prop."
          on:openChange={(e) => (densityOpenMap[density] = e.detail.open)}
        >
          <p>Content at <strong>{density}</strong> density.</p>
          <svelte:fragment slot="actions">
            <Button variant="secondary" on:click={() => (densityOpenMap[density] = false)}>Cancel</Button>
            <Button on:click={() => (densityOpenMap[density] = false)}>Save</Button>
          </svelte:fragment>
        </Drawer>
      {/each}
    </div>
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
