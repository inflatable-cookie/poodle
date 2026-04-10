<script lang="ts">
  import { Rating } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let rating = 3;
</script>

<SpecimenLayout>
  <SpecimenGroup label="Default (5 stars)">
    <Rating
      value={rating}
      ariaLabel="Rating"
      on:valueChange={(e) => { if (e.detail.value != null) rating = e.detail.value; }}
    />
    <p>Rating: <strong>{rating} / 5</strong></p>
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

  <svelte:fragment slot="sizes" let:size>
    <Rating value={3} ariaLabel={"Rating at " + size} />
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <Rating value={3} ariaLabel={"Rating at " + density + " density"} />
  </svelte:fragment>
</SpecimenLayout>
