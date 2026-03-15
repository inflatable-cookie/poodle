<script lang="ts">
  import { Spacer, Inline, Surface, Eyebrow } from "@pug/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Push items apart</Eyebrow>
    <Inline gap="md">
      <Surface padding="sm" border="subtle"><span>Logo</span></Surface>
      <Spacer />
      <Surface padding="sm" border="subtle"><span>Sign in</span></Surface>
    </Inline>
  </div>

  <div class="specimen__group">
    <Eyebrow>Between three items</Eyebrow>
    <Inline gap="md">
      <Surface padding="sm" border="subtle"><span>Left</span></Surface>
      <Spacer />
      <Surface padding="sm" border="subtle"><span>Center</span></Surface>
      <Spacer />
      <Surface padding="sm" border="subtle"><span>Right</span></Surface>
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
