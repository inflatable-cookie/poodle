<script lang="ts">
  import { DatePicker } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let selected = "";
</script>

<SpecimenLayout>
  <SpecimenGroup label="Default">
    <DatePicker
      ariaLabel="Select date"
      on:valueChange={(e) => (selected = e.detail.value)}
    />
    {#if selected}
      <p>Selected: <strong>{selected}</strong></p>
    {/if}
  </SpecimenGroup>

  <SpecimenGroup label="With default value">
    <DatePicker defaultValue="2026-03-14" ariaLabel="Pre-filled date" />
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <DatePicker placeholder="Disabled" disabled ariaLabel="Disabled date picker" />
  </SpecimenGroup>

  <svelte:fragment slot="sizes" let:size>
    <DatePicker {size} ariaLabel={size} />
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <DatePicker {density} />
  </svelte:fragment>
</SpecimenLayout>
