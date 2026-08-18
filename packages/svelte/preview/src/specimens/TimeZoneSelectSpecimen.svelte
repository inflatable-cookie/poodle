<script lang="ts">
  import { TimeZoneSelect } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let zone = $state("");
</script>

<SpecimenLayout>
    <SpecimenGroup label="Default">
    <TimeZoneSelect ariaLabel="Time zone" onValueChange={(value) => (zone = value)} />
        {#if zone}<span class="poodle-specimen__value">{zone}</span>{/if}
  </SpecimenGroup>

        <SpecimenGroup label="Pre-selected">
    <TimeZoneSelect defaultValue="America/New_York" ariaLabel="Pre-filled" />
  </SpecimenGroup>

        <SpecimenGroup label="Disabled">
    <TimeZoneSelect disabled ariaLabel="Disabled" />
  </SpecimenGroup>

  {#snippet sizes(size)}
    <TimeZoneSelect {size} ariaLabel={size} />
  {/snippet}

  {#snippet densities(density)}
    <TimeZoneSelect {density} ariaLabel={density} />
  {/snippet}
</SpecimenLayout>

<style>
    .poodle-specimen__item { display: flex; align-items: center; gap: 0.75rem; }
  .poodle-specimen__value { font-size: 0.75rem; color: var(--poodle-color-text-secondary); }
</style>
