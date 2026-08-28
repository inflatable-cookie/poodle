<script lang="ts">
  import { TimeInput } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let time = $state("");
</script>

<SpecimenLayout>
  <SpecimenGroup label="Default">
    <div class="poodle-specimen__item">
      <TimeInput id="start-time" ariaLabel="Start time" onValueChange={(value) => { if (value) time = value; }} />
      {#if time}<span class="poodle-specimen__value">{time}</span>{/if}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="With default value">
    <TimeInput id="meeting-time" defaultValue="14:30" ariaLabel="Meeting time" />
  </SpecimenGroup>

  <SpecimenGroup label="With min/max">
    <TimeInput id="office" defaultValue="09:00" min="08:00" max="18:00" ariaLabel="Office hours" />
  </SpecimenGroup>

  <SpecimenGroup label="Seconds step">
    <TimeInput id="cue-time" defaultValue="09:30:15" step={15} ariaLabel="Cue time" />
  </SpecimenGroup>

  <SpecimenGroup label="Overnight">
    <TimeInput id="quiet-hours" defaultValue="23:30" min="22:00" max="06:00" step={1800} ariaLabel="Quiet hours" />
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <TimeInput id="disabled-time" defaultValue="12:00" disabled ariaLabel="Disabled" />
  </SpecimenGroup>

  {#snippet sizes(size)}
    <TimeInput id={"size-" + size} {size} ariaLabel={size} />
  {/snippet}

  {#snippet densities(density)}
    <TimeInput id={"density-" + density} {density} />
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen__item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .poodle-specimen__value {
    font-size: 0.75rem;
    color: var(--poodle-color-text-secondary);
  }
</style>
