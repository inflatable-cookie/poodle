<script lang="ts">
  import { Spacer, Stack, Surface, Eyebrow } from "@flint/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Push items apart</Eyebrow>
    <Stack direction="row" gap="md">
      <Surface padding="sm" border="subtle"><span>Logo</span></Surface>
      <Spacer />
      <Surface padding="sm" border="subtle"><span>Sign in</span></Surface>
    </Stack>
  </div>

  <div class="specimen__group">
    <Eyebrow>Between three items</Eyebrow>
    <Stack direction="row" gap="md">
      <Surface padding="sm" border="subtle"><span>Left</span></Surface>
      <Spacer />
      <Surface padding="sm" border="subtle"><span>Center</span></Surface>
      <Spacer />
      <Surface padding="sm" border="subtle"><span>Right</span></Surface>
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

  span {
    font-size: 0.875rem;
    white-space: nowrap;
  }
</style>
