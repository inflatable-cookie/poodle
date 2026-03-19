<script lang="ts">
  import { Stack, Surface, Eyebrow } from "@pug/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Column (default)</Eyebrow>
    <Stack gap="md">
      <Surface padding="sm" border="subtle"><p>First item</p></Surface>
      <Surface padding="sm" border="subtle"><p>Second item</p></Surface>
      <Surface padding="sm" border="subtle"><p>Third item</p></Surface>
    </Stack>
  </div>

  <div class="specimen__group">
    <Eyebrow>Column — large gap, center aligned</Eyebrow>
    <Stack gap="lg" align="center">
      <Surface padding="sm" border="subtle"><p>Centered A</p></Surface>
      <Surface padding="sm" border="subtle"><p>Centered B</p></Surface>
    </Stack>
  </div>

  <div class="specimen__group">
    <Eyebrow>Row</Eyebrow>
    <Stack direction="row" gap="md">
      <Surface padding="sm" border="subtle"><span>Item A</span></Surface>
      <Surface padding="sm" border="subtle"><span>Taller item B with more text</span></Surface>
      <Surface padding="sm" border="subtle"><span>Item C</span></Surface>
    </Stack>
  </div>

  <div class="specimen__group">
    <Eyebrow>Row — justify: between</Eyebrow>
    <Stack direction="row" gap="md" justify="between">
      <Surface padding="sm" border="subtle"><span>Left</span></Surface>
      <Surface padding="sm" border="subtle"><span>Center</span></Surface>
      <Surface padding="sm" border="subtle"><span>Right</span></Surface>
    </Stack>
  </div>

  <div class="specimen__group">
    <Eyebrow>Row — wrapping</Eyebrow>
    <Stack direction="row" gap="sm" wrap>
      {#each Array(8) as _, i}
        <Surface padding="sm" border="subtle"><span>Tag {i + 1}</span></Surface>
      {/each}
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

  p, span {
    margin: 0;
    font-size: 0.875rem;
    white-space: nowrap;
  }
</style>
