<script lang="ts">
  import { RangeSlider, Eyebrow } from "@poodle/svelte-primitives";

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let priceRange: [number, number] = [20, 80];
  let ageRange: [number, number] = [25, 45];
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default</Eyebrow>
    <RangeSlider
      value={priceRange}
      min={0}
      max={100}
      ariaLabel="Price range"
      on:valueChange={(e) => (priceRange = e.detail.value)}
    />
    <p>${priceRange[0]} – ${priceRange[1]}</p>
  </div>

  <div class="specimen__group">
    <Eyebrow>With step</Eyebrow>
    <RangeSlider
      value={ageRange}
      min={18}
      max={65}
      step={5}
      ariaLabel="Age range"
      on:valueChange={(e) => (ageRange = e.detail.value)}
    />
    <p>Ages {ageRange[0]} – {ageRange[1]}</p>
  </div>

  <div class="specimen__group">
    <Eyebrow>Sizes</Eyebrow>
    <div class="specimen__stack">
      {#each controlSizes as size}
        <RangeSlider value={[25, 75]} min={0} max={100} ariaLabel={"Range at " + size} {size} />
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Densities</Eyebrow>
    <div class="specimen__stack">
      {#each ["compact", "default", "comfortable"] as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <RangeSlider id={"density-" + density} value={[25, 75]} min={0} max={100} ariaLabel={"Range at " + density + " density"} {density} />
        </div>
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <RangeSlider value={[30, 70]} min={0} max={100} disabled ariaLabel="Disabled range" />
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
