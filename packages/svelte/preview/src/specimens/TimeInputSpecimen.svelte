<script lang="ts">
  import { TimeInput } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let time = "";
</script>

<SpecimenLayout>
  <SpecimenGroup label="Default">
    <TimeInput
      id="start-time"
      ariaLabel="Start time"
      on:valueChange={(e) => { if (e.detail.value) time = e.detail.value; }}
    />
    {#if time}
      <p>Selected: <strong>{time}</strong></p>
    {/if}
  </SpecimenGroup>

  <SpecimenGroup label="With default value">
    <TimeInput id="meeting-time" defaultValue="14:30" ariaLabel="Meeting time" />
  </SpecimenGroup>

  <SpecimenGroup label="With min/max constraints">
    <TimeInput id="office" defaultValue="09:00" min="08:00" max="18:00" ariaLabel="Office hours" />
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <TimeInput id="disabled-time" defaultValue="12:00" disabled ariaLabel="Disabled time" />
  </SpecimenGroup>

  <svelte:fragment slot="sizes" let:size>
    <TimeInput id={"size-" + size} {size} ariaLabel={size} />
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <TimeInput id={"density-" + density} {density} />
  </svelte:fragment>
</SpecimenLayout>
