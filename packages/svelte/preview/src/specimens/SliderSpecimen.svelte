<script lang="ts">
  import { Slider, Eyebrow } from "@poodle/svelte-primitives";
  import type { ControlDensity } from "@poodle/svelte-primitives";

  const densities: ControlDensity[] = ["compact", "default", "comfortable"];

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let volume = 65;
  let opacity = 100;
  let sizeValues = Object.fromEntries(controlSizes.map((s) => [s, 50]));
  let densityValues: Record<string, number> = { compact: 50, default: 50, comfortable: 50 };
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default</Eyebrow>
    <Slider
      value={volume}
      min={0}
      max={100}
      ariaLabel="Volume"
      on:valueChange={(e) => (volume = e.detail.value)}
    />
    <p>Volume: <strong>{volume}%</strong></p>
  </div>

  <div class="specimen__group">
    <Eyebrow>With step</Eyebrow>
    <Slider
      value={opacity}
      min={0}
      max={100}
      step={10}
      ariaLabel="Opacity"
      on:valueChange={(e) => (opacity = e.detail.value)}
    />
    <p>Opacity: <strong>{opacity}%</strong></p>
  </div>

  <div class="specimen__group">
    <Eyebrow>Sizes</Eyebrow>
    <div class="specimen__stack">
      {#each controlSizes as size}
        <Slider bind:value={sizeValues[size]} min={0} max={100} ariaLabel={"Slider at " + size} {size} />
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Densities</Eyebrow>
    <div class="specimen__stack">
      {#each densities as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <Slider bind:value={densityValues[density]} min={0} max={100} ariaLabel={"Slider at " + density + " density"} {density} />
        </div>
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <Slider value={40} min={0} max={100} ariaLabel="Disabled slider" disabled />
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
    color: var(--poodle-color-text-secondary);
  }

  .specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .specimen__row {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.5rem;
  }

  .specimen__label {
    font-size: 0.75rem;
    font-family: var(--poodle-typography-code-family);
    color: var(--poodle-color-text-muted);
    min-width: 6rem;
  }
</style>
