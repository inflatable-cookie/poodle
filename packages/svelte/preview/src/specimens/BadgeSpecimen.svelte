<script lang="ts">
  import { Badge, Eyebrow } from "@pug/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Accent (default)</Eyebrow>
    <div class="specimen__row">
      <Badge>3</Badge>
      <Badge>12</Badge>
      <Badge>99+</Badge>
      <Badge>New</Badge>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Muted</Eyebrow>
    <div class="specimen__row">
      <Badge variant="muted">3</Badge>
      <Badge variant="muted">12</Badge>
      <Badge variant="muted">99+</Badge>
      <Badge variant="muted">Draft</Badge>
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

  .specimen__row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    align-items: center;
  }
</style>
