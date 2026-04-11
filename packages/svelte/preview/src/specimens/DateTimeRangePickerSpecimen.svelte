<script lang="ts">
  import { DateTimeRangePicker } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
</script>

<SpecimenLayout>
  <SpecimenGroup label="Default">
    <DateTimeRangePicker ariaLabel="Select date and time range" />
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

  <svelte:fragment slot="sizes" let:size>
    <DateTimeRangePicker {size} ariaLabel={size} />
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <DateTimeRangePicker {density} />
  </svelte:fragment>
</SpecimenLayout>
