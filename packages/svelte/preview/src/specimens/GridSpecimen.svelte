<script lang="ts">
  import { Grid, Surface } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
</script>

<div class="specimen">
  <SpecimenGroup label="Three columns">
    <Grid columns="1fr 1fr 1fr" gap="md">
      <Surface padding="md" border="subtle"><p>Column 1</p></Surface>
      <Surface padding="md" border="subtle"><p>Column 2</p></Surface>
      <Surface padding="md" border="subtle"><p>Column 3</p></Surface>
    </Grid>
  </SpecimenGroup>

  <SpecimenGroup label="Mixed column widths">
    <Grid columns="1fr 2fr" gap="md">
      <Surface padding="md" border="subtle"><p>Sidebar (1fr)</p></Surface>
      <Surface padding="md" border="subtle"><p>Main content (2fr)</p></Surface>
    </Grid>
  </SpecimenGroup>

  <SpecimenGroup label="Auto-fit responsive">
    <Grid columns="repeat(auto-fit, minmax(8rem, 1fr))" gap="sm">
      <Surface padding="sm" border="subtle"><p>A</p></Surface>
      <Surface padding="sm" border="subtle"><p>B</p></Surface>
      <Surface padding="sm" border="subtle"><p>C</p></Surface>
      <Surface padding="sm" border="subtle"><p>D</p></Surface>
      <Surface padding="sm" border="subtle"><p>E</p></Surface>
    </Grid>
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  p {
    margin: 0;
    font-size: 0.875rem;
  }
</style>
