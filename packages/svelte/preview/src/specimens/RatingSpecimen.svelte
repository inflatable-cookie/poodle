<script lang="ts">
  import { Rating } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let rating = 3;
  let fractionalRating = 3.5;
</script>

<SpecimenLayout>
  <SpecimenGroup label="Default (5 stars)">
    <Rating
      value={rating}
      ariaLabel="Rating"
      onValueChange={(value) => { if (value != null) rating = value; }}
    />
    <p>Rating: <strong>{rating} / 5</strong></p>
  </SpecimenGroup>

  <SpecimenGroup label="10-star scale">
    <Rating defaultValue={7} max={10} ariaLabel="Score out of 10" />
  </SpecimenGroup>

  <SpecimenGroup label="Half-star steps">
    <Rating
      value={fractionalRating}
      step={0.5}
      allowClear
      ariaLabel="Half-star rating"
      onValueChange={(value) => { fractionalRating = value ?? 0; }}
    />
    <p>Rating: <strong>{fractionalRating} / 5</strong></p>
  </SpecimenGroup>

  <SpecimenGroup label="Clearable">
    <Rating defaultValue={4} allowClear ariaLabel="Clearable rating" />
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <Rating defaultValue={2} disabled ariaLabel="Disabled rating" />
  </SpecimenGroup>

  <svelte:fragment slot="sizes" let:size>
    <Rating value={3} {size} ariaLabel={"Rating at " + size} />
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <Rating value={3} {density} ariaLabel={"Rating at " + density + " density"} />
  </svelte:fragment>
</SpecimenLayout>
