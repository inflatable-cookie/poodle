<script lang="ts">
  import { DurationInput } from "@poodle/svelte-primitives";
  import type { ControlDensity } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const densities: ControlDensity[] = ["compact", "default", "comfortable"];

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let hours = 1;
  let minutes = 30;
  let seconds = 0;
  let lastChange = "";
</script>

<div class="specimen">
  <SpecimenGroup label="Hours, minutes, seconds">
    <DurationInput
      bind:hours
      bind:minutes
      bind:seconds
      on:change={(e) => (lastChange = `${e.detail.totalSeconds}s total`)}
    />
    <p>Total: {hours}h {minutes}m {seconds}s</p>
  </SpecimenGroup>

  <SpecimenGroup label="Sizes">
    <div class="specimen__stack">
      {#each controlSizes as size}
        <DurationInput hours={1} minutes={30} seconds={0} {size} />
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Densities">
    <div class="specimen__stack">
      {#each densities as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <DurationInput {density} />
        </div>
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Hours and minutes only">
    <DurationInput hours={0} minutes={45} showSeconds={false} />
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <DurationInput hours={2} minutes={15} seconds={30} disabled />
  </SpecimenGroup>

  {#if lastChange}
    <SpecimenGroup label="Last change">
      <p>{lastChange}</p>
    </SpecimenGroup>
  {/if}
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .specimen__row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .specimen__label {
    font-size: 0.75rem;
    font-family: var(--poodle-typography-code-family);
    color: var(--poodle-color-text-muted);
    min-width: 6rem;
  }

  p { margin: 0; }
</style>
