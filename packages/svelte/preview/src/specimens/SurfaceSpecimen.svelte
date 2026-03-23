<script lang="ts">
  import { Surface, Eyebrow } from "@poodle/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Panel tone (default)</Eyebrow>
    <Surface padding="md" border="subtle">
      <p>Panel surface with subtle border — the standard container.</p>
    </Surface>
  </div>

  <div class="specimen__group">
    <Eyebrow>Canvas tone</Eyebrow>
    <Surface tone="canvas" padding="md" border="subtle">
      <p>Canvas surface sits behind panels as a background layer.</p>
    </Surface>
  </div>

  <div class="specimen__group">
    <Eyebrow>Elevated tone</Eyebrow>
    <Surface tone="elevated" padding="md" border="subtle" isElevated>
      <p>Elevated surface with shadow for overlays and cards.</p>
    </Surface>
  </div>

  <div class="specimen__group">
    <Eyebrow>No border</Eyebrow>
    <Surface padding="md" border="none">
      <p>Surface with no border — just padding and background.</p>
    </Surface>
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
