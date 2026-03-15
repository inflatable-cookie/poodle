<script lang="ts">
  import { Stack, Surface, Eyebrow } from "@pug/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default (md gap)</Eyebrow>
    <Stack gap="md">
      <Surface padding="sm" border="subtle"><p>First item</p></Surface>
      <Surface padding="sm" border="subtle"><p>Second item</p></Surface>
      <Surface padding="sm" border="subtle"><p>Third item</p></Surface>
    </Stack>
  </div>

  <div class="specimen__group">
    <Eyebrow>Large gap with center alignment</Eyebrow>
    <Stack gap="lg" align="center">
      <Surface padding="sm" border="subtle"><p>Centered A</p></Surface>
      <Surface padding="sm" border="subtle"><p>Centered B</p></Surface>
    </Stack>
  </div>

  <div class="specimen__group">
    <Eyebrow>Small gap, compact</Eyebrow>
    <Stack gap="sm">
      <Surface padding="sm" border="subtle"><p>Line 1</p></Surface>
      <Surface padding="sm" border="subtle"><p>Line 2</p></Surface>
      <Surface padding="sm" border="subtle"><p>Line 3</p></Surface>
      <Surface padding="sm" border="subtle"><p>Line 4</p></Surface>
    </Stack>
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
