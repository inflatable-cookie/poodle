<script lang="ts">
  import { Separator, Eyebrow } from "@flint/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Horizontal (default)</Eyebrow>
    <p>Content above</p>
    <Separator />
    <p>Content below</p>
  </div>

  <div class="specimen__group">
    <Eyebrow>Vertical</Eyebrow>
    <div class="specimen__row">
      <span>Left</span>
      <Separator orientation="vertical" />
      <span>Center</span>
      <Separator orientation="vertical" />
      <span>Right</span>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Decorative</Eyebrow>
    <Separator decorative />
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

  .specimen__group p,
  .specimen__group span {
    margin: 0;
    font-size: 0.875rem;
    color: var(--flint-color-text-secondary);
  }

  .specimen__row {
    display: flex;
    gap: 0.75rem;
    align-items: center;
    height: 2rem;
  }
</style>
