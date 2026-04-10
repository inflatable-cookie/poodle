<script lang="ts">
  import { Calendar } from "@poodle/svelte-primitives";
  import type { DateRangeValue } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let selected = "";
  let range: DateRangeValue = { start: "", end: "" };
</script>

<SpecimenLayout>
  <SpecimenGroup label="Default">
    <Calendar ariaLabel="Select a date" on:valueChange={(e) => (selected = e.detail.value as string)} />
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

  <svelte:fragment slot="sizes" let:size>
    <Calendar ariaLabel={size + " calendar"} />
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <Calendar />
  </svelte:fragment>
</SpecimenLayout>
