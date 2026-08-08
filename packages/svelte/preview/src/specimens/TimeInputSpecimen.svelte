<script lang="ts">
  import { TimeInput, Eyebrow, Surface } from "@inflatable-cookie/poodle-svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let time = $state("");
</script>

<SpecimenLayout>
  <Surface tone="panel" border="subtle" padding="md">
    <div class="poodle-specimen">
      <div class="poodle-specimen__item">
        <Eyebrow>Default</Eyebrow>
        <TimeInput id="start-time" ariaLabel="Start time" onValueChange={(value) => { if (value) time = value; }} />
        {#if time}<span class="poodle-specimen__value">{time}</span>{/if}
      </div>

      <div class="poodle-specimen__item">
        <Eyebrow>With default value</Eyebrow>
        <TimeInput id="meeting-time" defaultValue="14:30" ariaLabel="Meeting time" />
      </div>

      <div class="poodle-specimen__item">
        <Eyebrow>With min/max</Eyebrow>
        <TimeInput id="office" defaultValue="09:00" min="08:00" max="18:00" ariaLabel="Office hours" />
      </div>

      <div class="poodle-specimen__item">
        <Eyebrow>Disabled</Eyebrow>
        <TimeInput id="disabled-time" defaultValue="12:00" disabled ariaLabel="Disabled" />
      </div>
    </div>
  </Surface>

  {#snippet sizes(size)}
    <TimeInput id={"size-" + size} {size} ariaLabel={size} />
  {/snippet}

  {#snippet densities(density)}
    <TimeInput id={"density-" + density} {density} />
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen { display: flex; flex-direction: column; gap: 0.75rem; }
  .poodle-specimen__item { display: flex; align-items: center; gap: 0.75rem; }
  .poodle-specimen__value { font-size: 0.75rem; color: var(--poodle-color-text-secondary); }
</style>
