<script lang="ts">
  import { DateRangePicker } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let range = { start: "", end: "" };
</script>

<SpecimenLayout>
  <SpecimenGroup label="Default">
    <DateRangePicker
      ariaLabel="Select date range"
      on:valueChange={(e) => (range = e.detail.value)}
    />
    {#if range.start}
      <p>{range.start} → {range.end || "…"}</p>
    {/if}
  </SpecimenGroup>

  <SpecimenGroup label="With default range">
    <DateRangePicker
      defaultValue={{ start: "2026-03-01", end: "2026-03-14" }}
      ariaLabel="Pre-filled range"
    />
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <DateRangePicker disabled ariaLabel="Disabled range picker" />
  </SpecimenGroup>

  <svelte:fragment slot="sizes" let:size>
    <DateRangePicker {size} ariaLabel={size} />
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <DateRangePicker {density} />
  </svelte:fragment>
</SpecimenLayout>
