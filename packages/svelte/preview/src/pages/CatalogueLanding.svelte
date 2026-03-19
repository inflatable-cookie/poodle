<script lang="ts">
  import { Eyebrow } from "@pug/svelte-primitives";
  import type { ComponentEntry, ComponentTier } from "../component-registry";

  export let tier: ComponentTier;
  export let components: ComponentEntry[] = [];

  const tierConfig: Record<ComponentTier, { label: string; description: string; section: string }> = {
    primitive: {
      label: "Primitives",
      description: "Foundation-level building blocks. Each primitive handles one concern with full accessibility, keyboard, and theming support.",
      section: "primitives",
    },
    composite: {
      label: "Composites",
      description: "Higher-order components composed from primitives. Each composite handles a complete workflow pattern.",
      section: "composites",
    },
  };

  $: tierLabel = tierConfig[tier].label;
  $: tierDescription = tierConfig[tier].description;
  $: tierSection = tierConfig[tier].section;
</script>

<div class="catalogue-landing">
  <div class="catalogue-landing__header">
    <Eyebrow>{tierLabel}</Eyebrow>
    <h2>{tierLabel} catalogue</h2>
    <p>{tierDescription}</p>
    <p class="catalogue-landing__count">{components.length} components</p>
  </div>

  <div class="catalogue-landing__grid">
    {#each components as component (component.slug)}
      <a
        class="component-card"
        href="#{tierSection}/{component.slug}"
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
  .catalogue-landing {
    padding: 1.5rem;
  }

  .catalogue-landing__header {
    margin-bottom: 2rem;
  }

  .catalogue-landing__header h2 {
    margin: 0.25rem 0 0.5rem;
    font-size: 1.5rem;
    font-weight: 700;
    color: var(--pug-color-text-primary);
  }

  .catalogue-landing__header p {
    color: var(--pug-color-text-secondary);
    font-size: 0.875rem;
    line-height: 1.5;
  }

  .catalogue-landing__count {
    margin-top: 0.25rem;
    font-size: 0.75rem;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    color: var(--pug-color-text-secondary);
  }

  .catalogue-landing__grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(16rem, 1fr));
    gap: 0.75rem;
  }

  .component-card {
    display: block;
    padding: 1rem;
    border: 0.0625rem solid color-mix(in srgb, var(--pug-color-border-subtle) 60%, transparent);
    border-radius: var(--pug-radius-surface);
    background: color-mix(in srgb, var(--pug-color-background-panel) 90%, transparent);
    text-decoration: none;
    color: inherit;
    transition: border-color 0.15s, background 0.15s;
  }

  .component-card:hover {
    border-color: var(--pug-color-border-default);
    background: var(--pug-color-background-elevated);
  }

  .component-card__header {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 0.375rem;
  }

  .component-card__name {
    font-size: 0.9375rem;
    font-weight: 600;
    color: var(--pug-color-text-primary);
  }

  .component-card__description {
    font-size: 0.8125rem;
    line-height: 1.45;
    color: var(--pug-color-text-secondary);
    margin: 0;
  }
</style>
