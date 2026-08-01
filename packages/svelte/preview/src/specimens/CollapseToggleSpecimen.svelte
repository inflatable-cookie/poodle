<script lang="ts">
  import { CollapseToggle } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let collapsedLeft = $state(false);
  let collapsedRight = $state(false);
  let collapsedUp = $state(false);
  let collapsedDown = $state(false);
</script>

<SpecimenLayout>
  <SpecimenGroup label="Directions">
    <div class="poodle-specimen__row">
      <div class="poodle-specimen__labeled">
        <CollapseToggle
          direction="left"
          collapsed={collapsedLeft}
          onToggle={(isCollapsed) => (collapsedLeft = isCollapsed)}
        />
        <span>Left {collapsedLeft ? "(collapsed)" : "(expanded)"}</span>
      </div>
      <div class="poodle-specimen__labeled">
        <CollapseToggle
          direction="right"
          collapsed={collapsedRight}
          onToggle={(isCollapsed) => (collapsedRight = isCollapsed)}
        />
        <span>Right {collapsedRight ? "(collapsed)" : "(expanded)"}</span>
      </div>
      <div class="poodle-specimen__labeled">
        <CollapseToggle
          direction="up"
          collapsed={collapsedUp}
          onToggle={(isCollapsed) => (collapsedUp = isCollapsed)}
        />
        <span>Up {collapsedUp ? "(collapsed)" : "(expanded)"}</span>
      </div>
      <div class="poodle-specimen__labeled">
        <CollapseToggle
          direction="down"
          collapsed={collapsedDown}
          onToggle={(isCollapsed) => (collapsedDown = isCollapsed)}
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
