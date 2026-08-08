<script lang="ts">
  import { Calendar } from "@inflatable-cookie/poodle-svelte";
  import type { DateRangeValue } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let selected = $state("");
  let range: DateRangeValue = $state({ start: "", end: "" });
</script>

<SpecimenLayout>
  <SpecimenGroup label="Default">
    <Calendar ariaLabel="Select a date" onValueChange={(value) => (selected = value as string)} />
    <p class="poodle-specimen__hint">Double-click the month to choose a month, or the year to edit the year directly.</p>
    {#if selected}
      <p>Selected: <strong>{selected}</strong></p>
    {/if}
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
      onValueChange={(value) => (range = value as DateRangeValue)}
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

  {#snippet sizes(size)}
    <Calendar {size} ariaLabel={size + " calendar"} />
  {/snippet}

  {#snippet densities(density)}
    <Calendar {density} />
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen__hint {
    margin: 0.5rem 0 0;
    font-size: 0.75rem;
    color: var(--poodle-color-text-secondary);
  }
</style>
