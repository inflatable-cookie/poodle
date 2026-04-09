<script lang="ts">
  import { Rating } from "@poodle/svelte-primitives";
  import type { ControlDensity } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const densities: ControlDensity[] = ["compact", "default", "comfortable"];

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let rating = 3;
</script>

<div class="specimen">
  <SpecimenGroup label="Default (5 stars)">
    <Rating
      value={rating}
      ariaLabel="Rating"
      on:valueChange={(e) => { if (e.detail.value != null) rating = e.detail.value; }}
    />
    <p>Rating: <strong>{rating} / 5</strong></p>
  </SpecimenGroup>

  <SpecimenGroup label="Sizes">
    <div class="specimen__row">
      {#each controlSizes as size}
        <Rating value={3} ariaLabel={"Rating at " + size} {size} />
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Densities">
    <div class="specimen__stack">
      {#each densities as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <Rating value={3} ariaLabel={"Rating at " + density + " density"} {density} />
        </div>
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="10-star scale">
    <Rating defaultValue={7} max={10} ariaLabel="Score out of 10" />
  </SpecimenGroup>

  <SpecimenGroup label="Clearable">
    <Rating defaultValue={4} allowClear ariaLabel="Clearable rating" />
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <Rating defaultValue={2} disabled ariaLabel="Disabled rating" />
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .specimen__row {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.75rem;
  }

  .specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .specimen__label {
    font-size: 0.75rem;
    font-family: var(--poodle-typography-code-family);
    color: var(--poodle-color-text-muted);
    min-width: 6rem;
  }
</style>
