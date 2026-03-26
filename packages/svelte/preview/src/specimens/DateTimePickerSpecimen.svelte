<script lang="ts">
  import { DateTimePicker, Eyebrow } from "@poodle/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default</Eyebrow>
    <DateTimePicker ariaLabel="Select date and time" />
  </div>

  <div class="specimen__group">
    <Eyebrow>With default value</Eyebrow>
    <DateTimePicker
      defaultValue={{ date: "2026-03-14", time: "14:30" }}
      ariaLabel="Pre-filled date time"
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <DateTimePicker disabled ariaLabel="Disabled date time picker" />
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
</style>
