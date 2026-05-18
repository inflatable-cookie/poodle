<script lang="ts">
  import { DurationInput } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let hours = 1;
  let minutes = 30;
  let seconds = 0;
  let lastChange = "";
</script>

<SpecimenLayout>
  <SpecimenGroup label="Hours, minutes, seconds">
    <DurationInput
      bind:hours
      bind:minutes
      bind:seconds
      onChange={(detail) => (lastChange = `${detail.totalSeconds}s total`)}
    />
    <p>Total: {hours}h {minutes}m {seconds}s</p>
  </SpecimenGroup>

  <SpecimenGroup label="Hours and minutes only">
    <DurationInput hours={0} minutes={45} showSeconds={false} />
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <DurationInput hours={2} minutes={15} seconds={30} disabled />
  </SpecimenGroup>

  {#if lastChange}
    <SpecimenGroup label="Last change">
      <p>{lastChange}</p>
    </SpecimenGroup>
  {/if}

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
