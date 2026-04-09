<script lang="ts">
  import { Calendar } from "@poodle/svelte-primitives";
  import type { ControlDensity, DateRangeValue } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const densities: ControlDensity[] = ["compact", "default", "comfortable"];

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let selected = "";
  let range: DateRangeValue = { start: "", end: "" };
</script>

<div class="specimen">
  <SpecimenGroup label="Default">
    <Calendar ariaLabel="Select a date" on:valueChange={(e) => (selected = e.detail.value as string)} />
    {#if selected}
      <p>Selected: <strong>{selected}</strong></p>
    {/if}
  </SpecimenGroup>

  <SpecimenGroup label="Sizes">
    <div class="specimen__stack">
      {#each controlSizes as size}
        <Calendar {size} ariaLabel={size + " calendar"} />
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Densities">
    <div class="specimen__stack">
      {#each densities as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <Calendar {density} />
        </div>
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="With pre-selected date">
    <Calendar defaultValue="2026-03-14" ariaLabel="Calendar with default" />
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <Calendar defaultValue="2026-03-01" disabled ariaLabel="Disabled calendar" />
  </SpecimenGroup>

  <SpecimenGroup label="Range selection">
    <Calendar
      mode="range"
      ariaLabel="Select a date range"
      on:valueChange={(e) => (range = e.detail.value as DateRangeValue)}
    />
    {#if range.start}
      <p>{range.start} &rarr; {range.end || "..."}</p>
    {/if}
  </SpecimenGroup>

  <SpecimenGroup label="Range with pre-selected range">
    <Calendar
      mode="range"
      defaultValue={{ start: "2026-03-05", end: "2026-03-12" }}
      ariaLabel="Pre-selected range"
    />
  </SpecimenGroup>

  <SpecimenGroup label="Range disabled">
    <Calendar mode="range" disabled ariaLabel="Disabled range calendar" />
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
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
