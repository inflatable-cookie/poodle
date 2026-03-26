<script lang="ts">
  import { TimeField, Eyebrow } from "@poodle/svelte-primitives";

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

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
    <Eyebrow>Sizes</Eyebrow>
    <div class="specimen__row">
      {#each controlSizes as size}
        <TimeField id={"size-" + size} ariaLabel={size} {size} />
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Densities</Eyebrow>
    <div class="specimen__stack">
      {#each ["compact", "default", "comfortable"] as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <TimeField id={"density-" + density} {density} />
        </div>
      {/each}
    </div>
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
    <TimeField id="disabled-time" defaultValue="12:00" disabled ariaLabel="Disabled time" />
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

  .specimen__row {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.5rem;
  }

  .specimen__stack {
    display: flex;
    flex-direction: column;
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
