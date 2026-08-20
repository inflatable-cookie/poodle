<script lang="ts">
  import { DurationInput } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let hours = $state(1);
  let minutes = $state(30);
  let seconds = $state(0);
</script>

<SpecimenLayout>
  <SpecimenGroup label="Hours, minutes, seconds">
    <DurationInput bind:hours bind:minutes bind:seconds />
    <p>Total: {hours}h {minutes}m {seconds}s</p>
  </SpecimenGroup>

  <SpecimenGroup label="Hours and minutes only">
    <DurationInput hours={0} minutes={45} showSeconds={false} />
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <DurationInput hours={2} minutes={15} seconds={30} disabled />
  </SpecimenGroup>

  {#snippet sizes(size)}
    <DurationInput hours={1} minutes={30} seconds={0} {size} />
  {/snippet}

  {#snippet densities(density)}
    <DurationInput {density} />
  {/snippet}
</SpecimenLayout>

<style>
  p { margin: 0; }
</style>
