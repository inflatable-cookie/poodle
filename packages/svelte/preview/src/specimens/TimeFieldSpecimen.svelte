<script lang="ts">
  import { TimeField } from "@poodle/svelte-primitives";
  import type { ControlDensity } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const densities: ControlDensity[] = ["compact", "default", "comfortable"];

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let time = "";
</script>

<div class="specimen">
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

  <SpecimenGroup label="Sizes">
    <div class="specimen__row">
      {#each controlSizes as size}
        <TimeField id={"size-" + size} ariaLabel={size} {size} />
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Densities">
    <div class="specimen__stack">
      {#each densities as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <TimeField id={"density-" + density} {density} />
        </div>
      {/each}
    </div>
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
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    max-width: 12rem;
  }

  .specimen__row {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.5rem;
  }

  .specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .specimen__label {
    font-size: 0.75rem;
    font-family: var(--poodle-typography-code-family);
    color: var(--poodle-color-text-muted);
    min-width: 6rem;
  }

</style>
