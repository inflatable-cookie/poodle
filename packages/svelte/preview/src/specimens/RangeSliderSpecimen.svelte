<script lang="ts">
  import { RangeSlider } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let priceRange: [number, number] = $state([20, 80]);
  let ageRange: [number, number] = $state([23, 43]);
  let embeddedUnipolarRange: [number, number] = $state([0.2, 0.75]);
  let embeddedBipolarRange: [number, number] = $state([-0.6, 0.35]);
  let verticalRange: [number, number] = $state([30, 70]);
  let sizeRanges = $state<Record<string, [number, number]>>({ xs: [0.2, 0.75], sm: [0.2, 0.75], md: [0.2, 0.75], lg: [0.2, 0.75], xl: [0.2, 0.75] });
</script>

<SpecimenLayout>
  <div class="poodle-range-slider-specimen">
    <SpecimenGroup label="A lower and upper bound the reader drags">
      <RangeSlider
        value={priceRange}
        min={0}
        max={100}
        ariaLabel="Price range"
        onValueChange={(value) => (priceRange = value)}
      />
      <p>${priceRange[0]} – ${priceRange[1]}</p>
    </SpecimenGroup>

    <SpecimenGroup label="Stepped — the thumbs land on whole increments">
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

    <!-- The embedded variant is the dense control used inside audio and
         modulation panels; unipolar fills from the floor, bipolar from centre. -->
    <SpecimenGroup label="Embedded variant — unipolar fills from the floor, bipolar from centre">
      <RangeSlider variant="embedded" polarity="unipolar" value={embeddedUnipolarRange} min={0} max={1} step={0.01} ariaLabel="Unipolar modulation range" onValueChange={(value) => (embeddedUnipolarRange = value)} />
      <RangeSlider variant="embedded" polarity="bipolar" value={embeddedBipolarRange} min={-1} max={1} step={0.01} ariaLabel="Bipolar modulation range" onValueChange={(value) => (embeddedBipolarRange = value)} />
    </SpecimenGroup>

    <SpecimenGroup label="Disabled">
      <RangeSlider value={[30, 70]} min={0} max={100} disabled ariaLabel="Disabled range" />
    </SpecimenGroup>
  </div>

  <SpecimenGroup label="Vertical — the same control on the other axis">
    <div class="poodle-range-slider-specimen__vertical">
      <RangeSlider
        orientation="vertical"
        value={verticalRange}
        min={0}
        max={100}
        ariaLabel="Vertical range"
        onValueChange={(value) => (verticalRange = value)}
      />
      <RangeSlider
        orientation="vertical"
        variant="embedded"
        polarity="bipolar"
        value={[-0.4, 0.6]}
        min={-1}
        max={1}
        step={0.01}
        ariaLabel="Vertical embedded range"
      />
    </div>
  </SpecimenGroup>

  <!-- One control per step. The axis tabs exist so Examples does not have to
       carry a matrix; filling them with a matrix defeats the point. -->
  {#snippet sizes(size)}
    <div class="poodle-range-slider-specimen__axis">
      <RangeSlider value={sizeRanges[size]} min={0} max={1} step={0.01} {size} ariaLabel={"Range at " + size} onValueChange={(value) => (sizeRanges[size] = value)} />
    </div>
  {/snippet}

  {#snippet densities(density)}
    <div class="poodle-range-slider-specimen__axis">
      <RangeSlider value={[20, 80]} min={0} max={100} {density} ariaLabel={"Range at " + density + " density"} />
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-range-slider-specimen {
    max-width: 20rem;
  }

  .poodle-range-slider-specimen__axis {
    width: min(100%, 20rem);
  }

  .poodle-range-slider-specimen__vertical {
    display: flex;
    align-items: flex-start;
    gap: 2rem;
    height: 12rem;
  }
</style>
