<script lang="ts">
  import { DateRangePicker, Eyebrow } from "@pug/svelte-primitives";

  let range = { start: "", end: "" };
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default</Eyebrow>
    <DateRangePicker
      ariaLabel="Select date range"
      on:valueChange={(e) => (range = e.detail.value)}
    />
    {#if range.start}
      <p>{range.start} → {range.end || "…"}</p>
    {/if}
  </div>

  <div class="specimen__group">
    <Eyebrow>With default range</Eyebrow>
    <DateRangePicker
      defaultValue={{ start: "2026-03-01", end: "2026-03-14" }}
      ariaLabel="Pre-filled range"
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <DateRangePicker isDisabled ariaLabel="Disabled range picker" />
  </div>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    max-width: 20rem;
  }

  .specimen__group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .specimen__group p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--pug-color-text-secondary);
  }
</style>
