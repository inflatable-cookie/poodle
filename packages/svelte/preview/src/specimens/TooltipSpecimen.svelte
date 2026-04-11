<script lang="ts">
  import { Tooltip, Button } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
</script>

<SpecimenLayout>
  <SpecimenGroup label="Default">
    <Tooltip content="Save your changes">
      <Button variant="secondary">Hover me</Button>
    </Tooltip>
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

  <svelte:fragment slot="sizes" let:size>
    <Tooltip content="Tooltip at {size}">
      <Button variant="secondary" {size}>Hover ({size})</Button>
    </Tooltip>
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <Tooltip content="Tooltip at {density}">
      <Button variant="secondary" {density}>Hover ({density})</Button>
    </Tooltip>
  </svelte:fragment>
</SpecimenLayout>

<style>
  .specimen__row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    align-items: center;
  }
</style>
