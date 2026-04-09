<script lang="ts">
  import { Box } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
</script>

<div class="specimen">
  <SpecimenGroup label="Default (no padding)">
    <div class="demo-outline">
      <Box>Content inside a Box with no padding.</Box>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="With padding">
    <div class="demo-outline">
      <Box padding="lg">Content inside a Box with large padding.</Box>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Fixed dimensions">
    <div class="demo-outline">
      <Box padding="md" width="12rem" height="6rem">Fixed 12×6rem box.</Box>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Overflow hidden">
    <div class="demo-outline">
      <Box padding="sm" width="10rem" height="3rem" overflow="hidden">
        This text is too long and will be clipped by the overflow hidden setting on the box container.
      </Box>
    </div>
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .demo-outline {
    border: 1px dashed var(--poodle-color-border-default);
    border-radius: 4px;
  }
</style>
