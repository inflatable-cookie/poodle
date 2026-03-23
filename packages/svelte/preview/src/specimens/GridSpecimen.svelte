<script lang="ts">
  import { Grid, Surface, Eyebrow } from "@flint/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Three columns</Eyebrow>
    <Grid columns="1fr 1fr 1fr" gap="md">
      <Surface padding="md" border="subtle"><p>Column 1</p></Surface>
      <Surface padding="md" border="subtle"><p>Column 2</p></Surface>
      <Surface padding="md" border="subtle"><p>Column 3</p></Surface>
    </Grid>
  </div>

  <div class="specimen__group">
    <Eyebrow>Mixed column widths</Eyebrow>
    <Grid columns="1fr 2fr" gap="md">
      <Surface padding="md" border="subtle"><p>Sidebar (1fr)</p></Surface>
      <Surface padding="md" border="subtle"><p>Main content (2fr)</p></Surface>
    </Grid>
  </div>

  <div class="specimen__group">
    <Eyebrow>Auto-fit responsive</Eyebrow>
    <Grid columns="repeat(auto-fit, minmax(8rem, 1fr))" gap="sm">
      <Surface padding="sm" border="subtle"><p>A</p></Surface>
      <Surface padding="sm" border="subtle"><p>B</p></Surface>
      <Surface padding="sm" border="subtle"><p>C</p></Surface>
      <Surface padding="sm" border="subtle"><p>D</p></Surface>
      <Surface padding="sm" border="subtle"><p>E</p></Surface>
    </Grid>
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

  p {
    margin: 0;
    font-size: 0.875rem;
  }
</style>
