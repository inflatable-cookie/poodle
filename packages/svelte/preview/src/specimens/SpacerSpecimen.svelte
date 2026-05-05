<script lang="ts">
  import { Spacer, Stack, Surface } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
</script>

<div class="poodle-specimen">
  <SpecimenGroup label="Push items apart">
    <Stack direction="row" gap="md">
      <Surface padding="sm" border="subtle"><span>Logo</span></Surface>
      <Spacer />
      <Surface padding="sm" border="subtle"><span>Sign in</span></Surface>
    </Stack>
  </SpecimenGroup>

  <SpecimenGroup label="Between three items">
    <Stack direction="row" gap="md">
      <Surface padding="sm" border="subtle"><span>Left</span></Surface>
      <Spacer />
      <Surface padding="sm" border="subtle"><span>Center</span></Surface>
      <Spacer />
      <Surface padding="sm" border="subtle"><span>Right</span></Surface>
    </Stack>
  </SpecimenGroup>
</div>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  span {
    font-size: 0.875rem;
    white-space: nowrap;
  }
</style>
