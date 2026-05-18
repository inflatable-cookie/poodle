<script lang="ts">
  import { CollapseToggle } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let collapsedLeft = false;
  let collapsedRight = false;
  let collapsedUp = false;
  let collapsedDown = false;
</script>

<SpecimenLayout>
  <SpecimenGroup label="Directions">
    <div class="poodle-specimen__row">
      <div class="poodle-specimen__labeled">
        <CollapseToggle
          direction="left"
          collapsed={collapsedLeft}
          on:toggle={(e) => (collapsedLeft = e.detail.isCollapsed)}
        />
        <span>Left {collapsedLeft ? "(collapsed)" : "(expanded)"}</span>
      </div>
      <div class="poodle-specimen__labeled">
        <CollapseToggle
          direction="right"
          collapsed={collapsedRight}
          on:toggle={(e) => (collapsedRight = e.detail.isCollapsed)}
        />
        <span>Right {collapsedRight ? "(collapsed)" : "(expanded)"}</span>
      </div>
      <div class="poodle-specimen__labeled">
        <CollapseToggle
          direction="up"
          collapsed={collapsedUp}
          on:toggle={(e) => (collapsedUp = e.detail.isCollapsed)}
        />
        <span>Up {collapsedUp ? "(collapsed)" : "(expanded)"}</span>
      </div>
      <div class="poodle-specimen__labeled">
        <CollapseToggle
          direction="down"
          collapsed={collapsedDown}
          on:toggle={(e) => (collapsedDown = e.detail.isCollapsed)}
        />
        <span>Down {collapsedDown ? "(collapsed)" : "(expanded)"}</span>
      </div>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <div class="poodle-specimen__row">
      <CollapseToggle direction="left" disabled />
      <CollapseToggle direction="right" disabled />
    </div>
  </SpecimenGroup>

  {#snippet sizes(size)}
    <CollapseToggle direction="left" {size} />
  {/snippet}

  {#snippet densities(density)}
    <CollapseToggle direction="left" {density} />
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen__row {
    display: flex;
    gap: 1.5rem;
    align-items: center;
    flex-wrap: wrap;
  }

  .poodle-specimen__labeled {
    display: flex;
    align-items: center;
    gap: 0.375rem;
  }

  .poodle-specimen__labeled span {
    font-size: 0.75rem;
    color: var(--poodle-color-text-secondary);
  }
</style>
