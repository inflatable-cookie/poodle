<script lang="ts">
  import { RangeSlider } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let priceRange: [number, number] = $state([20, 80]);
  let ageRange: [number, number] = $state([23, 43]);
  let embeddedUnipolarRange: [number, number] = $state([0.2, 0.75]);
  let embeddedBipolarRange: [number, number] = $state([-0.6, 0.35]);
  let sizeUnipolarRanges = $state<Record<string, [number, number]>>({ xs: [0.2, 0.75], sm: [0.2, 0.75], md: [0.2, 0.75], lg: [0.2, 0.75], xl: [0.2, 0.75] });
  let sizeBipolarRanges = $state<Record<string, [number, number]>>({ xs: [-0.5, 0.5], sm: [-0.5, 0.5], md: [-0.5, 0.5], lg: [-0.5, 0.5], xl: [-0.5, 0.5] });
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
    <div class="poodle-range-slider-specimen__size-variants">
      <span>{size.toUpperCase()} · standard</span>
      <RangeSlider value={sizeUnipolarRanges[size]} min={0} max={1} step={0.01} {size} ariaLabel={"Standard range at " + size} onValueChange={(value) => (sizeUnipolarRanges[size] = value)} />
      <span>{size.toUpperCase()} · embedded unipolar</span>
      <RangeSlider variant="embedded" polarity="unipolar" value={sizeUnipolarRanges[size]} min={0} max={1} step={0.01} {size} ariaLabel={"Embedded unipolar range at " + size} onValueChange={(value) => (sizeUnipolarRanges[size] = value)} />
      <span>{size.toUpperCase()} · embedded bipolar</span>
      <RangeSlider variant="embedded" polarity="bipolar" value={sizeBipolarRanges[size]} min={-1} max={1} step={0.01} {size} ariaLabel={"Embedded bipolar range at " + size} onValueChange={(value) => (sizeBipolarRanges[size] = value)} />
    </div>
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

  .poodle-range-slider-specimen__size-variants {
    display: flex;
    width: 100%;
    flex-direction: column;
    gap: 0.375rem;
  }

  .poodle-range-slider-specimen__size-variants > span {
    color: var(--poodle-color-text-secondary);
    font-size: var(--poodle-typography-label-size);
  }
</style>
