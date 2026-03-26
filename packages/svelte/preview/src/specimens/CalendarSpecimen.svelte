<script lang="ts">
  import { Calendar, Eyebrow } from "@poodle/svelte-primitives";

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let selected = "";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default</Eyebrow>
    <Calendar ariaLabel="Select a date" on:valueChange={(e) => (selected = e.detail.value)} />
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
    <Eyebrow>With pre-selected date</Eyebrow>
    <Calendar defaultValue="2026-03-14" ariaLabel="Calendar with default" />
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <Calendar defaultValue="2026-03-01" disabled ariaLabel="Disabled calendar" />
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
