<script lang="ts">
  import { RangeSlider } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let priceRange: [number, number] = $state([20, 80]);
  let ageRange: [number, number] = $state([23, 43]);
  let embeddedUnipolarRange: [number, number] = $state([0.2, 0.75]);
  let embeddedBipolarRange: [number, number] = $state([-0.6, 0.35]);
</script>

<div class="poodle-range-slider-specimen">
<SpecimenLayout>
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

  <SpecimenGroup label="Embedded unipolar control">
    <RangeSlider variant="embedded" polarity="unipolar" value={embeddedUnipolarRange} min={0} max={1} step={0.01} ariaLabel="Unipolar modulation range" onValueChange={(value) => (embeddedUnipolarRange = value)} />
  </SpecimenGroup>

  <SpecimenGroup label="Embedded bipolar control">
    <RangeSlider variant="embedded" polarity="bipolar" value={embeddedBipolarRange} min={-1} max={1} step={0.01} ariaLabel="Bipolar modulation range" onValueChange={(value) => (embeddedBipolarRange = value)} />
  </SpecimenGroup>

  {#snippet sizes(size)}
    <RangeSlider value={[25, 75]} min={0} max={100} {size} ariaLabel={"Range at " + size} />
  {/snippet}

  {#snippet densities(density)}
    <RangeSlider variant="embedded" polarity="bipolar" value={[-0.5, 0.5]} min={-1} max={1} {density} ariaLabel={"Embedded range at " + density + " density"} />
  {/snippet}

</SpecimenLayout>
</div>

<style>
  .poodle-range-slider-specimen {
    max-width: 20rem;
  }
</style>
