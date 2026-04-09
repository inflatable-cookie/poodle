<script lang="ts">
  import { Calendar, Eyebrow } from "@poodle/svelte-primitives";
  import type { ControlDensity, DateRangeValue } from "@poodle/svelte-primitives";

  const densities: ControlDensity[] = ["compact", "default", "comfortable"];

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let selected = "";
  let range: DateRangeValue = { start: "", end: "" };
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default</Eyebrow>
    <Calendar ariaLabel="Select a date" on:valueChange={(e) => (selected = e.detail.value as string)} />
    {#if selected}
      <p>Selected: <strong>{selected}</strong></p>
    {/if}
  </div>

  <div class="specimen__group">
    <Eyebrow>Sizes</Eyebrow>
    <div class="specimen__stack">
      {#each controlSizes as size}
        <Calendar {size} ariaLabel={size + " calendar"} />
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Densities</Eyebrow>
    <div class="specimen__stack">
      {#each densities as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <Calendar {density} />
        </div>
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>With pre-selected date</Eyebrow>
    <Calendar defaultValue="2026-03-14" ariaLabel="Calendar with default" />
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <Calendar defaultValue="2026-03-01" disabled ariaLabel="Disabled calendar" />
  </div>

  <div class="specimen__group">
    <Eyebrow>Range selection</Eyebrow>
    <Calendar
      mode="range"
      ariaLabel="Select a date range"
      on:valueChange={(e) => (range = e.detail.value as DateRangeValue)}
    />
    {#if range.start}
      <p>{range.start} &rarr; {range.end || "..."}</p>
    {/if}
  </div>

  <div class="specimen__group">
    <Eyebrow>Range with pre-selected range</Eyebrow>
    <Calendar
      mode="range"
      defaultValue={{ start: "2026-03-05", end: "2026-03-12" }}
      ariaLabel="Pre-selected range"
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>Range disabled</Eyebrow>
    <Calendar mode="range" disabled ariaLabel="Disabled range calendar" />
  </div>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .specimen__group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
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

  .specimen__group p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--poodle-color-text-secondary);
  }
</style>
