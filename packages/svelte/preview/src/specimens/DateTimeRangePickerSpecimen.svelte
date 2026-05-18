<script lang="ts">
  import { DateTimeRangePicker } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
</script>

<SpecimenLayout>
  <SpecimenGroup label="Default">
    <div class="poodle-specimen__control">
      <DateTimeRangePicker ariaLabel="Select date and time range" />
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="With default range">
    <div class="poodle-specimen__control">
      <DateTimeRangePicker
        defaultValue={{
          start: { date: "2026-03-10", time: "09:00" },
          end: { date: "2026-03-14", time: "17:00" },
        }}
        ariaLabel="Pre-filled range"
      />
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <div class="poodle-specimen__control">
      <DateTimeRangePicker disabled ariaLabel="Disabled range picker" />
    </div>
  </SpecimenGroup>

  {#snippet sizes(size)}
    <div class="poodle-specimen__control">
      <DateTimeRangePicker {size} ariaLabel={size} />
    </div>
  {/snippet}

  {#snippet densities(density)}
    <div class="poodle-specimen__control">
      <DateTimeRangePicker {density} />
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen__control {
    max-width: 20rem;
  }
</style>
