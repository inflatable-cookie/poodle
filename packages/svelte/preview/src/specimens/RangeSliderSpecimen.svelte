<script lang="ts">
  import { RangeSlider } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let priceRange: [number, number] = [20, 80];
  let ageRange: [number, number] = [23, 43];
</script>

<SpecimenLayout>
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

  <SpecimenGroup label="Disabled">
    <RangeSlider value={[30, 70]} min={0} max={100} disabled ariaLabel="Disabled range" />
  </SpecimenGroup>

  <svelte:fragment slot="sizes" let:size>
    <RangeSlider value={[25, 75]} min={0} max={100} ariaLabel={"Range at " + size} {size} />
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <RangeSlider value={[25, 75]} min={0} max={100} ariaLabel={"Range at " + density + " density"} {density} />
  </svelte:fragment>
</SpecimenLayout>

<style>
  :global(.specimen-layout) {
    max-width: 20rem;
  }
</style>
