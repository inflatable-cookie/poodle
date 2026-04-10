<script lang="ts">
  import { TimeZoneSelect } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let zone = "";
</script>

<SpecimenLayout>
  <SpecimenGroup label="Default">
    <TimeZoneSelect
      ariaLabel="Time zone"
      on:valueChange={(e) => (zone = e.detail.value)}
    />
    {#if zone}
      <p>Selected: <strong>{zone}</strong></p>
    {/if}
  </SpecimenGroup>

  <SpecimenGroup label="With pre-selected zone">
    <TimeZoneSelect defaultValue="America/New_York" ariaLabel="Pre-filled time zone" />
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <TimeZoneSelect disabled ariaLabel="Disabled time zone" />
  </SpecimenGroup>

  <svelte:fragment slot="sizes" let:size>
    <TimeZoneSelect ariaLabel={size} />
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <TimeZoneSelect id={"density-" + density} />
  </svelte:fragment>
</SpecimenLayout>
