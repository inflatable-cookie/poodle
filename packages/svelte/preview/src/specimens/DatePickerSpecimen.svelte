<script lang="ts">
  import { DatePicker } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let selected = "";
</script>

<SpecimenLayout>
  <SpecimenGroup label="Default">
    <div class="poodle-specimen__control">
      <DatePicker
        ariaLabel="Select date"
        onValueChange={(value) => (selected = value)}
      />
    </div>
    {#if selected}
      <p>Selected: <strong>{selected}</strong></p>
    {/if}
  </SpecimenGroup>

  <SpecimenGroup label="With default value">
    <div class="poodle-specimen__control">
      <DatePicker defaultValue="2026-03-14" ariaLabel="Pre-filled date" />
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <div class="poodle-specimen__control">
      <DatePicker placeholder="Disabled" disabled ariaLabel="Disabled date picker" />
    </div>
  </SpecimenGroup>

  <svelte:fragment slot="sizes" let:size>
    <div class="poodle-specimen__control">
      <DatePicker {size} ariaLabel={size} />
    </div>
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <div class="poodle-specimen__control">
      <DatePicker {density} />
    </div>
  </svelte:fragment>
</SpecimenLayout>

<style>
  .poodle-specimen__control {
    max-width: 20rem;
  }
</style>
