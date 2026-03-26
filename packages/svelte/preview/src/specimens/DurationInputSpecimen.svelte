<script lang="ts">
  import { DurationInput, Eyebrow } from "@poodle/svelte-primitives";

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let hours = 1;
  let minutes = 30;
  let seconds = 0;
  let lastChange = "";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Hours, minutes, seconds</Eyebrow>
    <DurationInput
      bind:hours
      bind:minutes
      bind:seconds
      on:change={(e) => (lastChange = `${e.detail.totalSeconds}s total`)}
    />
    <p>Total: {hours}h {minutes}m {seconds}s</p>
  </div>

  <div class="specimen__group">
    <Eyebrow>Sizes</Eyebrow>
    <div class="specimen__stack">
      {#each controlSizes as size}
        <DurationInput hours={1} minutes={30} seconds={0} {size} />
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Hours and minutes only</Eyebrow>
    <DurationInput hours={0} minutes={45} showSeconds={false} />
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <DurationInput hours={2} minutes={15} seconds={30} disabled />
  </div>

  {#if lastChange}
    <div class="specimen__group">
      <Eyebrow>Last change</Eyebrow>
      <p>{lastChange}</p>
    </div>
  {/if}
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
    gap: 0.75rem;
  }

  p { margin: 0; }
</style>
