<script lang="ts">
  import { HoverCard, Eyebrow } from "@pug/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default (top placement)</Eyebrow>
    <HoverCard ariaLabel="User preview">
      <span slot="trigger" class="trigger-link">@clay</span>
      <div class="card-content">
        <strong>Clay</strong>
        <p>Design systems engineer working on Pug. Loves component architecture and accessibility.</p>
      </div>
    </HoverCard>
  </div>

  <div class="specimen__group">
    <Eyebrow>Bottom placement</Eyebrow>
    <HoverCard placement="bottom" ariaLabel="Repository info">
      <span slot="trigger" class="trigger-link">pug/svelte-primitives</span>
      <div class="card-content">
        <strong>svelte-primitives</strong>
        <p>Core primitive components for the Pug design system. 64 components, 94% coverage.</p>
      </div>
    </HoverCard>
  </div>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }

  .specimen__group {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 0.5rem;
  }

  .trigger-link {
    color: var(--pug-color-text-accent);
    text-decoration: underline;
    cursor: pointer;
    font-size: 0.875rem;
  }

  .card-content {
    padding: 0.5rem;
    max-width: 16rem;
  }

  .card-content strong {
    display: block;
    margin-bottom: 0.25rem;
  }

  .card-content p {
    margin: 0;
    font-size: 0.8125rem;
    color: var(--pug-color-text-secondary);
  }
</style>
