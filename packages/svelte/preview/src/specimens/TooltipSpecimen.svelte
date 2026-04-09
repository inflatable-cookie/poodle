<script lang="ts">
  import { Tooltip, Button } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
</script>

<div class="specimen">
  <SpecimenGroup label="Default">
    <div class="specimen__row">
      <Tooltip content="Save your changes">
        <Button variant="secondary">Hover me</Button>
      </Tooltip>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Placements">
    <div class="specimen__row">
      <Tooltip content="Top tooltip" placement="top">
        <Button variant="ghost">Top</Button>
      </Tooltip>
      <Tooltip content="Bottom tooltip" placement="bottom">
        <Button variant="ghost">Bottom</Button>
      </Tooltip>
      <Tooltip content="Left tooltip" placement="left">
        <Button variant="ghost">Left</Button>
      </Tooltip>
      <Tooltip content="Right tooltip" placement="right">
        <Button variant="ghost">Right</Button>
      </Tooltip>
    </div>
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
    gap: 0.75rem;
    align-items: center;
  }
</style>
