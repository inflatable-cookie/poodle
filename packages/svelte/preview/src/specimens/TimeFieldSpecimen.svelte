<script lang="ts">
  import { TimeField, Eyebrow } from "@pug/svelte-primitives";

  let time = "";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default</Eyebrow>
    <TimeField
      id="start-time"
      ariaLabel="Start time"
      on:valueChange={(e) => { if (e.detail.value) time = e.detail.value; }}
    />
    {#if time}
      <p>Selected: <strong>{time}</strong></p>
    {/if}
  </div>

  <div class="specimen__group">
    <Eyebrow>With default value</Eyebrow>
    <TimeField id="meeting-time" defaultValue="14:30" ariaLabel="Meeting time" />
  </div>

  <div class="specimen__group">
    <Eyebrow>With min/max constraints</Eyebrow>
    <TimeField id="office" defaultValue="09:00" min="08:00" max="18:00" ariaLabel="Office hours" />
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <TimeField id="disabled-time" defaultValue="12:00" isDisabled ariaLabel="Disabled time" />
  </div>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    max-width: 12rem;
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
