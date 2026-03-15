<script lang="ts">
  import { Inline, Surface, Eyebrow } from "@pug/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default (center-aligned)</Eyebrow>
    <Inline gap="md">
      <Surface padding="sm" border="subtle"><span>Item A</span></Surface>
      <Surface padding="sm" border="subtle"><span>Taller item B with more text</span></Surface>
      <Surface padding="sm" border="subtle"><span>Item C</span></Surface>
    </Inline>
  </div>

  <div class="specimen__group">
    <Eyebrow>Justify: between</Eyebrow>
    <Inline gap="md" justify="between">
      <Surface padding="sm" border="subtle"><span>Left</span></Surface>
      <Surface padding="sm" border="subtle"><span>Center</span></Surface>
      <Surface padding="sm" border="subtle"><span>Right</span></Surface>
    </Inline>
  </div>

  <div class="specimen__group">
    <Eyebrow>Wrapping with small gap</Eyebrow>
    <Inline gap="sm" wrap>
      {#each Array(8) as _, i}
        <Surface padding="sm" border="subtle"><span>Tag {i + 1}</span></Surface>
      {/each}
    </Inline>
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

  span {
    font-size: 0.875rem;
    white-space: nowrap;
  }
</style>
