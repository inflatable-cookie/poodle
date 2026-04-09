<script lang="ts">
  import { DateRangePicker } from "@poodle/svelte-primitives";
  import type { ControlDensity } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const densities: ControlDensity[] = ["compact", "default", "comfortable"];

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let range = { start: "", end: "" };
</script>

<div class="specimen">
  <SpecimenGroup label="Default">
    <DateRangePicker
      ariaLabel="Select date range"
      on:valueChange={(e) => (range = e.detail.value)}
    />
    {#if range.start}
      <p>{range.start} → {range.end || "…"}</p>
    {/if}
  </SpecimenGroup>

  <SpecimenGroup label="Sizes">
    <div class="specimen__stack">
      {#each controlSizes as size}
        <DateRangePicker {size} ariaLabel={size} />
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Densities">
    <div class="specimen__stack">
      {#each densities as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <DateRangePicker {density} />
        </div>
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="With default range">
    <DateRangePicker
      defaultValue={{ start: "2026-03-01", end: "2026-03-14" }}
      ariaLabel="Pre-filled range"
    />
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <DateRangePicker disabled ariaLabel="Disabled range picker" />
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    max-width: 20rem;
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
