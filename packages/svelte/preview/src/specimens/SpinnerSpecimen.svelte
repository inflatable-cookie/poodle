<script lang="ts">
  import { Eyebrow, Spinner } from "@poodle/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Ring</Eyebrow>
    <div class="specimen__row">
      <Spinner variant="ring" size="sm" />
      <Spinner variant="ring" size="md" />
      <Spinner variant="ring" size="lg" />
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>CLI grid</Eyebrow>
    <div class="specimen__row">
      <Spinner variant="grid" size="sm" tone="muted" />
      <Spinner variant="grid" size="md" tone="accent" />
      <Spinner variant="grid" size="lg" />
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Context tones</Eyebrow>
    <div class="specimen__row specimen__row--tones">
      <span class="specimen__chip specimen__chip--inverse"><Spinner variant="ring" size="md" tone="current" /></span>
      <span class="specimen__chip"><Spinner variant="ring" size="md" tone="accent" /></span>
      <span class="specimen__chip"><Spinner variant="grid" size="md" tone="muted" /></span>
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
    align-items: center;
    gap: 1rem;
  }

  .specimen__row--tones {
    gap: 0.75rem;
  }

  .specimen__chip {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 2.5rem;
    min-height: 2.5rem;
    padding: 0.5rem;
    border: 1px solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-control);
    background: var(--poodle-color-background-surface);
    color: var(--poodle-color-text-primary);
  }

  .specimen__chip--inverse {
    background: var(--poodle-color-text-primary);
    color: var(--poodle-color-text-inverse);
  }
</style>
