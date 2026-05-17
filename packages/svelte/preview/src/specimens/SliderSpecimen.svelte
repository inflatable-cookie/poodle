<script lang="ts">
  import { Slider } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let volume = 65;
  let opacity = 100;
</script>

<div class="poodle-slider-specimen">
<SpecimenLayout showDensities={false}>
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

  <svelte:fragment slot="sizes" let:size>
    <Slider value={50} min={0} max={100} {size} ariaLabel={"Slider at " + size} />
  </svelte:fragment>

</SpecimenLayout>
</div>

<style>
  .poodle-slider-specimen {
    max-width: 20rem;
  }
</style>
