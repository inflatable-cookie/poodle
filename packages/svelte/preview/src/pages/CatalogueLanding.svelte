<script lang="ts">
  import { Eyebrow } from "@poodle/svelte";
  import type { ComponentEntry } from "../component-registry";

  export let components: ComponentEntry[] = [];
</script>

<div class="catalogue-landing">
  <div class="catalogue-landing__header">
    <Eyebrow>Components</Eyebrow>
    <h2>Component catalogue</h2>
    <p>Browse the full Poodle component library. Each component handles accessibility, keyboard support, and theming.</p>
    <p class="catalogue-landing__count">{components.length} components</p>
  </div>

  <div class="catalogue-landing__grid">
    {#each components as component (component.slug)}
      <a
        class="component-card"
        href="#components/{component.slug}"
      >
        <div class="component-card__header">
          <strong class="component-card__name">{component.displayName}</strong>
        </div>
        <p class="component-card__description">{component.description}</p>
      </a>
    {/each}
  </div>
</div>

<style>
  .catalogue-landing { padding: 1.5rem; }
  .catalogue-landing__header { margin-bottom: 2rem; }
  .catalogue-landing__header h2 { margin: 0.25rem 0 0.5rem; font-size: 1.5rem; font-weight: 700; color: var(--poodle-color-text-primary); }
  .catalogue-landing__header p { color: var(--poodle-color-text-secondary); font-size: 0.875rem; line-height: 1.5; }
  .catalogue-landing__count { margin-top: 0.25rem; font-size: 0.75rem; text-transform: uppercase; letter-spacing: 0.04em; color: var(--poodle-color-text-secondary); }
  .catalogue-landing__grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(16rem, 1fr)); gap: 0.75rem; }
  .component-card { display: block; padding: 1rem; border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 60%, transparent); border-radius: var(--poodle-radius-surface); background: color-mix(in srgb, var(--poodle-color-background-panel) 90%, transparent); text-decoration: none; color: inherit; transition: border-color 0.15s, background 0.15s; }
  .component-card:hover { border-color: var(--poodle-color-border-default); background: var(--poodle-color-background-elevated); }
  .component-card__header { display: flex; align-items: center; gap: 0.5rem; margin-bottom: 0.375rem; }
  .component-card__name { font-size: 0.9375rem; font-weight: 600; color: var(--poodle-color-text-primary); }
  .component-card__description { font-size: 0.8125rem; line-height: 1.45; color: var(--poodle-color-text-secondary); margin: 0; }
</style>
