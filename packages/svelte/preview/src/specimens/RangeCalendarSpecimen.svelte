<script lang="ts">
  import { RangeCalendar, Eyebrow } from "@poodle/svelte-primitives";

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let range = { start: "", end: "" };
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default</Eyebrow>
    <RangeCalendar
      ariaLabel="Select a date range"
      on:valueChange={(e) => (range = e.detail.value)}
    />
    {#if range.start}
      <p>{range.start} → {range.end || "…"}</p>
    {/if}
  </div>

  <div class="specimen__group">
    <Eyebrow>Sizes</Eyebrow>
    <div class="specimen__stack">
      {#each controlSizes as size}
        <RangeCalendar {size} ariaLabel={size + " range calendar"} />
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>With pre-selected range</Eyebrow>
    <RangeCalendar
      defaultValue={{ start: "2026-03-05", end: "2026-03-12" }}
      ariaLabel="Pre-selected range"
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <RangeCalendar disabled ariaLabel="Disabled range calendar" />
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

  .specimen__group p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--poodle-color-text-secondary);
  }
</style>
