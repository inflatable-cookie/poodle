<script lang="ts">
  import { ZonedDateTimePicker, Eyebrow } from "@poodle/svelte-primitives";

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default</Eyebrow>
    <ZonedDateTimePicker ariaLabel="Select date, time, and zone" />
  </div>

  <div class="specimen__group">
    <Eyebrow>Sizes</Eyebrow>
    <div class="specimen__stack">
      {#each controlSizes as size}
        <ZonedDateTimePicker {size} ariaLabel={size} />
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>With default value</Eyebrow>
    <ZonedDateTimePicker
      defaultValue={{ date: "2026-03-14", time: "10:00", timeZone: "America/Los_Angeles" }}
      ariaLabel="Pre-filled zoned date time"
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <ZonedDateTimePicker disabled ariaLabel="Disabled picker" />
  </div>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    max-width: 24rem;
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
</style>
