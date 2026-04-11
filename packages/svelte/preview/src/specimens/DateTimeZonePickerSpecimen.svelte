<script lang="ts">
  import { DateTimeZonePicker } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
</script>

<SpecimenLayout>
  <SpecimenGroup label="Default">
    <DateTimeZonePicker ariaLabel="Select date, time, and zone" />
  </SpecimenGroup>

  <SpecimenGroup label="With default value">
    <DateTimeZonePicker
      defaultValue={{ date: "2026-03-14", time: "10:00", timeZone: "America/Los_Angeles" }}
      ariaLabel="Pre-filled zoned date time"
    />
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <DateTimeZonePicker disabled ariaLabel="Disabled picker" />
  </SpecimenGroup>

  <svelte:fragment slot="sizes" let:size>
    <DateTimeZonePicker {size} ariaLabel={size} />
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <DateTimeZonePicker {density} />
  </svelte:fragment>
</SpecimenLayout>
