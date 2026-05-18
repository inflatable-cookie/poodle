<script lang="ts">
  import { DateRangePicker } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let range = { start: "", end: "" };
</script>

<SpecimenLayout>
  <SpecimenGroup label="Default">
    <div class="poodle-specimen__control">
      <DateRangePicker
        ariaLabel="Select date range"
        onValueChange={(value) => (range = value)}
      />
    </div>
    {#if range.start}
      <p>{range.start} → {range.end || "…"}</p>
    {/if}
  </SpecimenGroup>

  <SpecimenGroup label="With default range">
    <div class="poodle-specimen__control">
      <DateRangePicker
        defaultValue={{ start: "2026-03-01", end: "2026-03-14" }}
        ariaLabel="Pre-filled range"
      />
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <div class="poodle-specimen__control">
      <DateRangePicker disabled ariaLabel="Disabled range picker" />
    </div>
  </SpecimenGroup>

  {#snippet sizes(size)}
    <div class="poodle-specimen__control">
      <DateRangePicker {size} ariaLabel={size} />
    </div>
  {/snippet}

  {#snippet densities(density)}
    <div class="poodle-specimen__control">
      <DateRangePicker {density} />
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen__control {
    max-width: 20rem;
  }
</style>
