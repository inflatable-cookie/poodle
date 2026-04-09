<script lang="ts">
  import { CollapseToggle } from "@poodle/svelte-primitives";
  import type { ControlDensity } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const densities: ControlDensity[] = ["compact", "default", "comfortable"];

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let collapsedLeft = false;
  let collapsedRight = false;
  let collapsedUp = false;
  let collapsedDown = false;
</script>

<div class="specimen">
  <SpecimenGroup label="Directions">
    <div class="specimen__row">
      <div class="specimen__labeled">
        <CollapseToggle
          direction="left"
          collapsed={collapsedLeft}
          on:toggle={(e) => (collapsedLeft = e.detail.isCollapsed)}
        />
        <span>Left {collapsedLeft ? "(collapsed)" : "(expanded)"}</span>
      </div>
      <div class="specimen__labeled">
        <CollapseToggle
          direction="right"
          collapsed={collapsedRight}
          on:toggle={(e) => (collapsedRight = e.detail.isCollapsed)}
        />
        <span>Right {collapsedRight ? "(collapsed)" : "(expanded)"}</span>
      </div>
      <div class="specimen__labeled">
        <CollapseToggle
          direction="up"
          collapsed={collapsedUp}
          on:toggle={(e) => (collapsedUp = e.detail.isCollapsed)}
        />
        <span>Up {collapsedUp ? "(collapsed)" : "(expanded)"}</span>
      </div>
      <div class="specimen__labeled">
        <CollapseToggle
          direction="down"
          collapsed={collapsedDown}
          on:toggle={(e) => (collapsedDown = e.detail.isCollapsed)}
        />
        <span>Down {collapsedDown ? "(collapsed)" : "(expanded)"}</span>
      </div>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Sizes">
    <div class="specimen__row">
      {#each controlSizes as size}
        <CollapseToggle direction="left" {size} />
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Densities">
    <div class="specimen__row">
      {#each densities as density}
        <div class="specimen__labeled">
          <CollapseToggle direction="left" {density} />
          <span>{density}</span>
        </div>
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <div class="specimen__row">
      <CollapseToggle direction="left" disabled />
      <CollapseToggle direction="right" disabled />
    </div>
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .specimen__row {
    display: flex;
    gap: 1.5rem;
    align-items: center;
    flex-wrap: wrap;
  }

  .specimen__labeled {
    display: flex;
    align-items: center;
    gap: 0.375rem;
  }

  .specimen__labeled span {
    font-size: 0.75rem;
    color: var(--poodle-color-text-secondary);
  }
</style>
