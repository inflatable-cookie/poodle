<script lang="ts">
  import { DateTimePicker } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
</script>

<SpecimenLayout>
  <SpecimenGroup label="Default">
    <DateTimePicker ariaLabel="Select date and time" />
  </SpecimenGroup>

  <SpecimenGroup label="With default value">
    <DateTimePicker
      defaultValue={{ date: "2026-03-14", time: "14:30" }}
      ariaLabel="Pre-filled date time"
    />
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <DateTimePicker disabled ariaLabel="Disabled date time picker" />
  </SpecimenGroup>

  <svelte:fragment slot="sizes" let:size>
    <DateTimePicker {size} ariaLabel={size} />
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <DateTimePicker {density} />
  </svelte:fragment>
</SpecimenLayout>
