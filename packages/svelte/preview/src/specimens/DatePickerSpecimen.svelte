<script lang="ts">
  import { DatePicker, Eyebrow } from "@flint/svelte-primitives";

  let selected = "";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default</Eyebrow>
    <DatePicker
      ariaLabel="Select date"
      on:valueChange={(e) => (selected = e.detail.value)}
    />
    {#if selected}
      <p>Selected: <strong>{selected}</strong></p>
    {/if}
  </div>

  <div class="specimen__group">
    <Eyebrow>With default value</Eyebrow>
    <DatePicker defaultValue="2026-03-14" ariaLabel="Pre-filled date" />
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <DatePicker placeholder="Disabled" isDisabled ariaLabel="Disabled date picker" />
  </div>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    max-width: 16rem;
  }

  .specimen__group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .specimen__group p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--flint-color-text-secondary);
  }
</style>
