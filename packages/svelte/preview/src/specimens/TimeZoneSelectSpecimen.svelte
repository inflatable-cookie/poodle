<script lang="ts">
  import { TimeZoneSelect } from "@poodle/svelte-primitives";
  import type { ControlDensity } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const densities: ControlDensity[] = ["compact", "default", "comfortable"];

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let zone = "";
</script>

<div class="specimen">
  <SpecimenGroup label="Default">
    <TimeZoneSelect
      ariaLabel="Time zone"
      on:valueChange={(e) => (zone = e.detail.value)}
    />
    {#if zone}
      <p>Selected: <strong>{zone}</strong></p>
    {/if}
  </SpecimenGroup>

  <SpecimenGroup label="Sizes">
    <div class="specimen__stack">
      {#each controlSizes as size}
        <TimeZoneSelect {size} ariaLabel={size} />
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Densities">
    <div class="specimen__stack">
      {#each densities as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <TimeZoneSelect id={"density-" + density} {density} />
        </div>
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="With pre-selected zone">
    <TimeZoneSelect defaultValue="America/New_York" ariaLabel="Pre-filled time zone" />
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <TimeZoneSelect disabled ariaLabel="Disabled time zone" />
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    max-width: 20rem;
  }

  .specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .specimen__row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .specimen__label {
    font-size: 0.75rem;
    font-family: var(--poodle-typography-code-family);
    color: var(--poodle-color-text-muted);
    min-width: 6rem;
  }

</style>
