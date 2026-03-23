<script lang="ts">
  import { Rating, Eyebrow } from "@flint/svelte-primitives";

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
    <Eyebrow>10-star scale</Eyebrow>
    <Rating defaultValue={7} max={10} ariaLabel="Score out of 10" />
  </div>

  <div class="specimen__group">
    <Eyebrow>Clearable</Eyebrow>
    <Rating defaultValue={4} allowClear ariaLabel="Clearable rating" />
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <Rating defaultValue={2} isDisabled ariaLabel="Disabled rating" />
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
    color: var(--flint-color-text-secondary);
  }
</style>
