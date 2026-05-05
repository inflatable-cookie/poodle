<script lang="ts">
  import { Meter } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
</script>

<div class="poodle-specimen">
  <SpecimenGroup label="Default (50%)">
    <Meter value={50} ariaLabel="Storage usage" />
  </SpecimenGroup>

  <SpecimenGroup label="With thresholds">
    <Meter value={82} low={25} high={75} optimum={50} ariaLabel="CPU usage" />
    <p>82% — above high threshold</p>
  </SpecimenGroup>

  <SpecimenGroup label="Low value (optimal range)">
    <Meter value={30} low={25} high={75} optimum={50} ariaLabel="Memory usage" />
    <p>30% — within normal range</p>
  </SpecimenGroup>

  <SpecimenGroup label="Custom range (0–500)">
    <Meter value={350} min={0} max={500} ariaLabel="API calls" />
    <p>350 / 500 API calls used</p>
  </SpecimenGroup>
</div>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    max-width: 20rem;
  }

</style>
