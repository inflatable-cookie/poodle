<script lang="ts">
  import { Slider } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let volume = 65;
  let opacity = 100;
</script>

<div class="slider-specimen">
<SpecimenLayout>
  <SpecimenGroup label="Default">
    <Slider
      value={volume}
      min={0}
      max={100}
      ariaLabel="Volume"
      on:valueChange={(e) => (volume = e.detail.value)}
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
      on:valueChange={(e) => (opacity = e.detail.value)}
    />
    <p>Opacity: <strong>{opacity}%</strong></p>
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <Slider value={40} min={0} max={100} ariaLabel="Disabled slider" disabled />
  </SpecimenGroup>

  <svelte:fragment slot="sizes" let:size>
    <Slider value={50} min={0} max={100} ariaLabel={"Slider at " + size} />
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <Slider value={50} min={0} max={100} ariaLabel={"Slider at " + density + " density"} />
  </svelte:fragment>
</SpecimenLayout>
</div>

<style>
  .slider-specimen {
    max-width: 20rem;
  }
</style>
