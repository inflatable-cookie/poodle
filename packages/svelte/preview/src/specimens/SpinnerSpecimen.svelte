<script lang="ts">
  import { Eyebrow, Spinner } from "@poodle/svelte-primitives";
  import type { ControlDensity } from "@poodle/svelte-primitives";

  const densities: ControlDensity[] = ["compact", "default", "comfortable"];

  const spinnerSizes = ["xs", "sm", "md", "lg", "xl"] as const;
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Ring</Eyebrow>
    <div class="specimen__row">
      {#each spinnerSizes as size}
        <Spinner variant="ring" {size} />
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>CLI grid</Eyebrow>
    <div class="specimen__row">
      {#each spinnerSizes as size, index}
        <Spinner variant="grid" {size} tone={index === 0 ? "muted" : index === 2 ? "accent" : "current"} />
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Densities</Eyebrow>
    <div class="specimen__stack">
      {#each densities as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <Spinner variant="ring" {density} />
          <Spinner variant="grid" {density} />
        </div>
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Context tones</Eyebrow>
    <div class="specimen__row specimen__row--tones">
      <span class="specimen__chip specimen__chip--inverse"><Spinner variant="ring" tone="current" /></span>
      <span class="specimen__chip"><Spinner variant="ring" tone="accent" /></span>
      <span class="specimen__chip"><Spinner variant="grid" tone="muted" /></span>
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

  .specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .specimen__label {
    font-size: 0.75rem;
    font-family: var(--poodle-typography-code-family);
    color: var(--poodle-color-text-muted);
    min-width: 6rem;
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
