<script lang="ts">
  import { TimeField } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let time = "";
</script>

<SpecimenLayout>
  <SpecimenGroup label="Default">
    <TimeField
      id="start-time"
      ariaLabel="Start time"
      on:valueChange={(e) => { if (e.detail.value) time = e.detail.value; }}
    />
    {#if time}
      <p>Selected: <strong>{time}</strong></p>
    {/if}
  </SpecimenGroup>

  <SpecimenGroup label="With default value">
    <TimeField id="meeting-time" defaultValue="14:30" ariaLabel="Meeting time" />
  </SpecimenGroup>

  <SpecimenGroup label="With min/max constraints">
    <TimeField id="office" defaultValue="09:00" min="08:00" max="18:00" ariaLabel="Office hours" />
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <TimeField id="disabled-time" defaultValue="12:00" disabled ariaLabel="Disabled time" />
  </SpecimenGroup>

  <svelte:fragment slot="sizes" let:size>
    <TimeField id={"size-" + size} ariaLabel={size} {size} />
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <TimeField id={"density-" + density} {density} />
  </svelte:fragment>
</SpecimenLayout>
