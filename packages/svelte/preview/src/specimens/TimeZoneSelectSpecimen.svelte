<script lang="ts">
  import { TimeZoneSelect, Eyebrow } from "@poodle/svelte-primitives";

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let zone = "";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default</Eyebrow>
    <TimeZoneSelect
      ariaLabel="Time zone"
      on:valueChange={(e) => (zone = e.detail.value)}
    />
    {#if zone}
      <p>Selected: <strong>{zone}</strong></p>
    {/if}
  </div>

  <div class="specimen__group">
    <Eyebrow>Sizes</Eyebrow>
    <div class="specimen__stack">
      {#each controlSizes as size}
        <TimeZoneSelect {size} ariaLabel={size} />
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>With pre-selected zone</Eyebrow>
    <TimeZoneSelect defaultValue="America/New_York" ariaLabel="Pre-filled time zone" />
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <TimeZoneSelect disabled ariaLabel="Disabled time zone" />
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
