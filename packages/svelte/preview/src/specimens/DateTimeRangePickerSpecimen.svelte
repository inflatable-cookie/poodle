<script lang="ts">
  import { DateTimeRangePicker, Eyebrow } from "@poodle/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default</Eyebrow>
    <DateTimeRangePicker ariaLabel="Select date and time range" />
  </div>

  <div class="specimen__group">
    <Eyebrow>With default range</Eyebrow>
    <DateTimeRangePicker
      defaultValue={{
        start: { date: "2026-03-10", time: "09:00" },
        end: { date: "2026-03-14", time: "17:00" },
      }}
      ariaLabel="Pre-filled range"
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <DateTimeRangePicker isDisabled ariaLabel="Disabled range picker" />
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
</style>
