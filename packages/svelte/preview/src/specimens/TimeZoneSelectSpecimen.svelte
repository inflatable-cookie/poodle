<script lang="ts">
  import { TimeZoneSelect, Eyebrow, Surface } from "@poodle/svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let zone = "";
</script>

<SpecimenLayout>
  <Surface tone="panel" border="subtle" padding="md">
    <div class="poodle-specimen">
      <div class="poodle-specimen__item">
        <Eyebrow>Default</Eyebrow>
        <TimeZoneSelect ariaLabel="Time zone" on:valueChange={(e) => (zone = e.detail.value)} />
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

  <svelte:fragment slot="sizes" let:size>
    <TimeZoneSelect {size} ariaLabel={size} />
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <TimeZoneSelect {density} ariaLabel={density} />
  </svelte:fragment>
</SpecimenLayout>

<style>
  .poodle-specimen { display: flex; flex-direction: column; gap: 0.75rem; }
  .poodle-specimen__item { display: flex; align-items: center; gap: 0.75rem; }
  .poodle-specimen__value { font-size: 0.75rem; color: var(--poodle-color-text-secondary); }
</style>
