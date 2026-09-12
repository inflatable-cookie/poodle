<script lang="ts">
  import { Slider } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let volume = $state(65);
  let opacity = $state(100);
  let unipolar = $state(0.35);
  let bipolar = $state(-0.45);
  let sizeValues = $state<Record<string, number>>({ xs: 0.4, sm: 0.4, md: 0.4, lg: 0.4, xl: 0.4 });
  let densityValues = $state<Record<string, number>>({ compact: -0.4, default: -0.4, comfortable: -0.4 });
</script>

<div class="poodle-slider-specimen">
<SpecimenLayout>
  <SpecimenGroup label="Default">
    <Slider
      value={volume}
      min={0}
      max={100}
      ariaLabel="Volume"
      onValueChange={(value) => (volume = value)}
    />
    <p>Volume: <strong>{volume}%</strong></p>
  </SpecimenGroup>

  <SpecimenGroup label="With step">
    <Slider
      value={opacity}
      min={0}
      max={100}
      step={10}
      ariaLabel="Opacity"
      onValueChange={(value) => (opacity = value)}
    />
    <p>Opacity: <strong>{opacity}%</strong></p>
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <Slider value={40} min={0} max={100} ariaLabel="Disabled slider" disabled />
  </SpecimenGroup>

  <SpecimenGroup label="Embedded controls">
    <Slider variant="embedded" polarity="unipolar" value={unipolar} min={0} max={1} step={0.01} ariaLabel="Unipolar modulation" onValueChange={(value) => (unipolar = value)} />
    <Slider variant="embedded" polarity="bipolar" value={bipolar} min={-1} max={1} step={0.01} ariaLabel="Bipolar modulation" onValueChange={(value) => (bipolar = value)} />
  </SpecimenGroup>

  <!-- g18.022: block is the default variant; vertical and bipolar block are
       native, and the value never leaves the capsule at any value. -->
  <SpecimenGroup label="Block — the default capsule">
    <Slider value={volume} min={0} max={100} visibleLabel="Volume" ariaLabel="Volume" onValueChange={(value) => (volume = value)} />
    <Slider direction="rtl" value={opacity} min={0} max={100} visibleLabel="Opacity" ariaLabel="Opacity" onValueChange={(value) => (opacity = value)} />
  </SpecimenGroup>

  <SpecimenGroup label="Vertical block — value top, label centered, upright text">
    <div class="poodle-slider-specimen__vertical">
      <Slider orientation="vertical" value={volume} min={0} max={100} visibleLabel="Volume" ariaLabel="Vertical volume" onValueChange={(value) => (volume = value)} />
      <Slider orientation="vertical" polarity="bipolar" value={bipolar} min={-1} max={1} step={0.01} ariaLabel="Vertical bipolar block" onValueChange={(value) => (bipolar = value)} />
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Bipolar block — fill grows from the center">
    <Slider polarity="bipolar" value={bipolar} min={-1} max={1} step={0.01} visibleLabel="Drive" ariaLabel="Bipolar block" onValueChange={(value) => (bipolar = value)} />
  </SpecimenGroup>

  {#snippet sizes(size)}
    <div class="poodle-slider-specimen__variant-pair">
      <span>{size.toUpperCase()} · block</span>
      <Slider value={sizeValues[size]} min={0} max={1} step={0.01} {size} ariaLabel={"Standard slider at " + size} onValueChange={(value) => (sizeValues[size] = value)} />
      <span>{size.toUpperCase()} · embedded</span>
      <Slider variant="embedded" polarity="unipolar" value={sizeValues[size]} min={0} max={1} step={0.01} {size} ariaLabel={"Embedded slider at " + size} onValueChange={(value) => (sizeValues[size] = value)} />
    </div>
  {/snippet}

  {#snippet densities(density)}
    <div class="poodle-slider-specimen__density">
      <span>{density}</span>
      <Slider variant="embedded" polarity="bipolar" value={densityValues[density]} min={-1} max={1} step={0.01} {density} ariaLabel={"Embedded slider at " + density + " density"} onValueChange={(value) => (densityValues[density] = value)} />
    </div>
  {/snippet}

</SpecimenLayout>
</div>

<style>
  .poodle-slider-specimen {
    max-width: 20rem;
  }

  .poodle-slider-specimen__vertical {
    display: flex;
    align-items: flex-start;
    gap: 2rem;
    height: 12rem;
  }

  .poodle-slider-specimen__variant-pair,
  .poodle-slider-specimen__density {
    display: flex;
    width: 100%;
    flex-direction: column;
    gap: 0.375rem;
  }

  .poodle-slider-specimen__variant-pair > span,
  .poodle-slider-specimen__density > span {
    color: var(--poodle-color-text-secondary);
    font-size: var(--poodle-typography-label-size);
  }
</style>
