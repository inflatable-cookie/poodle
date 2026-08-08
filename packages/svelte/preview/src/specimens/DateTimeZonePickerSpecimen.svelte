<script lang="ts">
  import { DateTimeZonePicker } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
</script>

<SpecimenLayout>
  <SpecimenGroup label="Default">
    <div class="poodle-specimen__control">
      <DateTimeZonePicker ariaLabel="Select date, time, and zone" />
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="With default value">
    <div class="poodle-specimen__control">
      <DateTimeZonePicker
        defaultValue={{ date: "2026-03-14", time: "10:00", timeZone: "America/Los_Angeles" }}
        ariaLabel="Pre-filled zoned date time"
      />
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <div class="poodle-specimen__control">
      <DateTimeZonePicker disabled ariaLabel="Disabled picker" />
    </div>
  </SpecimenGroup>

  {#snippet sizes(size)}
    <div class="poodle-specimen__control">
      <DateTimeZonePicker {size} ariaLabel={size} />
    </div>
  {/snippet}

  {#snippet densities(density)}
    <div class="poodle-specimen__control">
      <DateTimeZonePicker {density} />
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen__control {
    max-width: 20rem;
  }
</style>
