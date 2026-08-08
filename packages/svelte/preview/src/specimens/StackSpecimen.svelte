<script lang="ts">
  import { Stack, Surface } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
</script>

<div class="poodle-specimen">
  <SpecimenGroup label="Column (default)">
    <Stack gap="md">
      <Surface padding="sm" border="subtle"><p>First item</p></Surface>
      <Surface padding="sm" border="subtle"><p>Second item</p></Surface>
      <Surface padding="sm" border="subtle"><p>Third item</p></Surface>
    </Stack>
  </SpecimenGroup>

  <SpecimenGroup label="Column — large gap, center aligned">
    <Stack gap="lg" align="center">
      <Surface padding="sm" border="subtle"><p>Centered A</p></Surface>
      <Surface padding="sm" border="subtle"><p>Centered B</p></Surface>
    </Stack>
  </SpecimenGroup>

  <SpecimenGroup label="Row">
    <Stack direction="row" gap="md">
      <Surface padding="sm" border="subtle"><span>Item A</span></Surface>
      <Surface padding="sm" border="subtle"><span>Taller item B with more text</span></Surface>
      <Surface padding="sm" border="subtle"><span>Item C</span></Surface>
    </Stack>
  </SpecimenGroup>

  <SpecimenGroup label="Row — justify: between">
    <Stack direction="row" gap="md" justify="between">
      <Surface padding="sm" border="subtle"><span>Left</span></Surface>
      <Surface padding="sm" border="subtle"><span>Center</span></Surface>
      <Surface padding="sm" border="subtle"><span>Right</span></Surface>
    </Stack>
  </SpecimenGroup>

  <SpecimenGroup label="Row — wrapping">
    <Stack direction="row" gap="sm" wrap>
      {#each Array(8) as _, i}
        <Surface padding="sm" border="subtle"><span>Tag {i + 1}</span></Surface>
      {/each}
    </Stack>
  </SpecimenGroup>
</div>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  p, span {
    margin: 0;
    font-size: 0.875rem;
    white-space: nowrap;
  }
</style>
