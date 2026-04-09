<script lang="ts">
  import { RangeSlider } from "@poodle/svelte-primitives";
  import type { ControlDensity } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const densities: ControlDensity[] = ["compact", "default", "comfortable"];

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let priceRange: [number, number] = [20, 80];
  let ageRange: [number, number] = [23, 43];
</script>

<div class="specimen">
  <SpecimenGroup label="Default">
    <RangeSlider
      value={priceRange}
      min={0}
      max={100}
      ariaLabel="Price range"
      on:valueChange={(e) => (priceRange = e.detail.value)}
    />
    <p>${priceRange[0]} – ${priceRange[1]}</p>
  </SpecimenGroup>

  <SpecimenGroup label="With step">
    <RangeSlider
      value={ageRange}
      min={18}
      max={65}
      step={5}
      ariaLabel="Age range"
      on:valueChange={(e) => (ageRange = e.detail.value)}
    />
    <p>Ages {ageRange[0]} – {ageRange[1]}</p>
  </SpecimenGroup>

  <SpecimenGroup label="Sizes">
    <div class="specimen__stack">
      {#each controlSizes as size}
        <RangeSlider value={[25, 75]} min={0} max={100} ariaLabel={"Range at " + size} {size} />
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Densities">
    <div class="specimen__stack">
      {#each densities as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <RangeSlider value={[25, 75]} min={0} max={100} ariaLabel={"Range at " + density + " density"} {density} />
        </div>
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <RangeSlider value={[30, 70]} min={0} max={100} disabled ariaLabel="Disabled range" />
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    max-width: 20rem;
  }

  .specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .specimen__row {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.5rem;
  }

  .specimen__label {
    font-size: 0.75rem;
    font-family: var(--poodle-typography-code-family);
    color: var(--poodle-color-text-muted);
    min-width: 6rem;
  }
</style>
