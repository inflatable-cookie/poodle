<script lang="ts">
  import { Slider } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let volume = $state(65);
  let opacity = $state(100);
  let unipolar = $state(0.35);
  let bipolar = $state(-0.45);
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

  {#snippet sizes(size)}
    <Slider value={50} min={0} max={100} {size} ariaLabel={"Slider at " + size} />
  {/snippet}

  {#snippet densities(density)}
    <Slider variant="embedded" polarity="bipolar" value={-0.4} min={-1} max={1} {density} ariaLabel={"Embedded slider at " + density + " density"} />
  {/snippet}

</SpecimenLayout>
</div>

<style>
  .poodle-slider-specimen {
    max-width: 20rem;
  }
</style>
