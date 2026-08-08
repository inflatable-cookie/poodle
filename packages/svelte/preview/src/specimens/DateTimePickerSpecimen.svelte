<script lang="ts">
  import { DateTimePicker } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
</script>

<SpecimenLayout>
  <SpecimenGroup label="Default">
    <div class="poodle-specimen__control">
      <DateTimePicker ariaLabel="Select date and time" />
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="With default value">
    <div class="poodle-specimen__control">
      <DateTimePicker
        defaultValue={{ date: "2026-03-14", time: "14:30" }}
        ariaLabel="Pre-filled date time"
      />
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <div class="poodle-specimen__control">
      <DateTimePicker disabled ariaLabel="Disabled date time picker" />
    </div>
  </SpecimenGroup>

  {#snippet sizes(size)}
    <div class="poodle-specimen__control">
      <DateTimePicker {size} ariaLabel={size} />
    </div>
  {/snippet}

  {#snippet densities(density)}
    <div class="poodle-specimen__control">
      <DateTimePicker {density} />
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen__control {
    max-width: 20rem;
  }
</style>
