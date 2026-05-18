<script lang="ts">
  import { RangeSlider } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let priceRange: [number, number] = [20, 80];
  let ageRange: [number, number] = [23, 43];
</script>

<div class="poodle-range-slider-specimen">
<SpecimenLayout showDensities={false}>
  <SpecimenGroup label="Default">
    <RangeSlider
      value={priceRange}
      min={0}
      max={100}
      ariaLabel="Price range"
      onValueChange={(value) => (priceRange = value)}
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
      onValueChange={(value) => (ageRange = value)}
    />
    <p>Ages {ageRange[0]} – {ageRange[1]}</p>
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <RangeSlider value={[30, 70]} min={0} max={100} disabled ariaLabel="Disabled range" />
  </SpecimenGroup>

  {#snippet sizes(size)}
    <RangeSlider value={[25, 75]} min={0} max={100} {size} ariaLabel={"Range at " + size} />
  {/snippet}

</SpecimenLayout>
</div>

<style>
  .poodle-range-slider-specimen {
    max-width: 20rem;
  }
</style>
