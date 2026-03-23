<script lang="ts">
  import { Box, Eyebrow } from "@flint/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default (no padding)</Eyebrow>
    <div class="demo-outline">
      <Box>Content inside a Box with no padding.</Box>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>With padding</Eyebrow>
    <div class="demo-outline">
      <Box padding="lg">Content inside a Box with large padding.</Box>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Fixed dimensions</Eyebrow>
    <div class="demo-outline">
      <Box padding="md" width="12rem" height="6rem">Fixed 12×6rem box.</Box>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Overflow hidden</Eyebrow>
    <div class="demo-outline">
      <Box padding="sm" width="10rem" height="3rem" overflow="hidden">
        This text is too long and will be clipped by the overflow hidden setting on the box container.
      </Box>
    </div>
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

  .demo-outline {
    border: 1px dashed var(--flint-color-border-default);
    border-radius: 4px;
  }
</style>
