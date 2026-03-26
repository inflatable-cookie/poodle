<script lang="ts">
  import { Rating, Eyebrow } from "@poodle/svelte-primitives";

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let rating = 3;
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default (5 stars)</Eyebrow>
    <Rating
      value={rating}
      ariaLabel="Rating"
      on:valueChange={(e) => { if (e.detail.value != null) rating = e.detail.value; }}
    />
    <p>Rating: <strong>{rating} / 5</strong></p>
  </div>

  <div class="specimen__group">
    <Eyebrow>Sizes</Eyebrow>
    <div class="specimen__row">
      {#each controlSizes as size}
        <Rating value={3} ariaLabel={"Rating at " + size} {size} />
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Densities</Eyebrow>
    <div class="specimen__stack">
      {#each ["compact", "default", "comfortable"] as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <Rating id={"density-" + density} value={3} ariaLabel={"Rating at " + density + " density"} {density} />
        </div>
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>10-star scale</Eyebrow>
    <Rating defaultValue={7} max={10} ariaLabel="Score out of 10" />
  </div>

  <div class="specimen__group">
    <Eyebrow>Clearable</Eyebrow>
    <Rating defaultValue={4} allowClear ariaLabel="Clearable rating" />
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <Rating defaultValue={2} disabled ariaLabel="Disabled rating" />
  </div>
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

  .specimen__group p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--poodle-color-text-secondary);
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
