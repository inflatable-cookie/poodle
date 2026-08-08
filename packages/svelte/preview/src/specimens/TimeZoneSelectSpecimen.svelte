<script lang="ts">
  import { TimeZoneSelect, Eyebrow, Surface } from "@inflatable-cookie/poodle-svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let zone = $state("");
</script>

<SpecimenLayout>
  <Surface tone="panel" border="subtle" padding="md">
    <div class="poodle-specimen">
      <div class="poodle-specimen__item">
        <Eyebrow>Default</Eyebrow>
        <TimeZoneSelect ariaLabel="Time zone" onValueChange={(value) => (zone = value)} />
        {#if zone}<span class="poodle-specimen__value">{zone}</span>{/if}
      </div>

      <div class="poodle-specimen__item">
        <Eyebrow>Pre-selected</Eyebrow>
        <TimeZoneSelect defaultValue="America/New_York" ariaLabel="Pre-filled" />
      </div>

      <div class="poodle-specimen__item">
        <Eyebrow>Disabled</Eyebrow>
        <TimeZoneSelect disabled ariaLabel="Disabled" />
      </div>
    </div>
  </Surface>

  {#snippet sizes(size)}
    <TimeZoneSelect {size} ariaLabel={size} />
  {/snippet}

  {#snippet densities(density)}
    <TimeZoneSelect {density} ariaLabel={density} />
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen { display: flex; flex-direction: column; gap: 0.75rem; }
  .poodle-specimen__item { display: flex; align-items: center; gap: 0.75rem; }
  .poodle-specimen__value { font-size: 0.75rem; color: var(--poodle-color-text-secondary); }
</style>
