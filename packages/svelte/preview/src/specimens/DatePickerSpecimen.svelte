<script lang="ts">
  import { DatePicker } from "@poodle/svelte-primitives";
  import type { ControlDensity } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const densities: ControlDensity[] = ["compact", "default", "comfortable"];

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let selected = "";
</script>

<div class="specimen">
  <SpecimenGroup label="Default">
    <DatePicker
      ariaLabel="Select date"
      on:valueChange={(e) => (selected = e.detail.value)}
    />
    {#if selected}
      <p>Selected: <strong>{selected}</strong></p>
    {/if}
  </SpecimenGroup>

  <SpecimenGroup label="Sizes">
    <div class="specimen__stack">
      {#each controlSizes as size}
        <DatePicker {size} ariaLabel={size} />
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Densities">
    <div class="specimen__stack">
      {#each densities as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <DatePicker {density} />
        </div>
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="With default value">
    <DatePicker defaultValue="2026-03-14" ariaLabel="Pre-filled date" />
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <DatePicker placeholder="Disabled" disabled ariaLabel="Disabled date picker" />
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    max-width: 16rem;
  }

  .specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .specimen__row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .specimen__label {
    font-size: 0.75rem;
    font-family: var(--poodle-typography-code-family);
    color: var(--poodle-color-text-muted);
    min-width: 6rem;
  }

</style>
