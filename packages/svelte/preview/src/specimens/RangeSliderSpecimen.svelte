<script lang="ts">
  import { RangeSlider, Eyebrow } from "@poodle/svelte-primitives";

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
    <Eyebrow>Disabled</Eyebrow>
    <RangeSlider value={[30, 70]} min={0} max={100} isDisabled ariaLabel="Disabled range" />
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
</style>
