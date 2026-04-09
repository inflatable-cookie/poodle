<script lang="ts">
  import { DateTimeRangePicker } from "@poodle/svelte-primitives";
  import type { ControlDensity } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const densities: ControlDensity[] = ["compact", "default", "comfortable"];

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;
</script>

<div class="specimen">
  <SpecimenGroup label="Default">
    <DateTimeRangePicker ariaLabel="Select date and time range" />
  </SpecimenGroup>

  <SpecimenGroup label="Sizes">
    <div class="specimen__stack">
      {#each controlSizes as size}
        <DateTimeRangePicker {size} ariaLabel={size} />
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Densities">
    <div class="specimen__stack">
      {#each densities as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <DateTimeRangePicker {density} />
        </div>
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="With default range">
    <DateTimeRangePicker
      defaultValue={{
        start: { date: "2026-03-10", time: "09:00" },
        end: { date: "2026-03-14", time: "17:00" },
      }}
      ariaLabel="Pre-filled range"
    />
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <DateTimeRangePicker disabled ariaLabel="Disabled range picker" />
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    max-width: 24rem;
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
