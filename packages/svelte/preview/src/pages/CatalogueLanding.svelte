<script lang="ts">
  import { Eyebrow } from "@poodle/svelte";
  import { componentsByTag, type ComponentEntry } from "../component-registry";
let { components = [] }: { components?: ComponentEntry[] } = $props();
  let groups = $derived(componentsByTag());</script>

<div class="poodle-catalogue-landing">
  <div class="poodle-catalogue-landing__header">
    <h2>Component catalogue</h2>
    <p>Browse the full Poodle component library. Each component handles accessibility, keyboard support, and theming.</p>
    <p class="poodle-catalogue-landing__count">{components.length} components</p>
  </div>

  {#each groups as group (group.tag)}
    <section class="poodle-catalogue-landing__section">
      <Eyebrow>{group.label}</Eyebrow>
      <div class="poodle-catalogue-landing__grid">
        {#each group.items as component (component.slug)}
          <a
            class="poodle-component-card"
            href="#components/{component.slug}"
          >
            <strong class="poodle-component-card__name">{component.displayName}</strong>
            <p class="poodle-component-card__description">{component.description}</p>
          </a>
        {/each}
      </div>
    </section>
  {/each}
</div>

<style>
  .poodle-catalogue-landing { padding: 1.5rem; }
  .poodle-catalogue-landing__header { margin-bottom: 2rem; }
  .poodle-catalogue-landing__header h2 { margin: 0 0 0.5rem; font-size: 1.5rem; font-weight: 700; color: var(--poodle-color-text-primary); }
  .poodle-catalogue-landing__header p { color: var(--poodle-color-text-secondary); font-size: 0.875rem; line-height: 1.5; margin: 0; }
  .poodle-catalogue-landing__count { margin-top: 0.25rem; font-size: 0.75rem; text-transform: uppercase; letter-spacing: 0.04em; }

  .poodle-catalogue-landing__section { margin-bottom: 1.5rem; }
  .poodle-catalogue-landing__grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(16rem, 1fr)); gap: 0.75rem; margin-top: 0.5rem; }

  .poodle-component-card { display: block; padding: 0.75rem 1rem; border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 60%, transparent); border-radius: var(--poodle-radius-surface); background: color-mix(in srgb, var(--poodle-color-background-panel) 90%, transparent); text-decoration: none; color: inherit; transition: border-color 0.15s, background 0.15s; }
  .poodle-component-card:hover { border-color: var(--poodle-color-border-default); background: var(--poodle-color-background-elevated); }
  .poodle-component-card__name { display: block; font-size: 0.875rem; font-weight: 600; color: var(--poodle-color-text-primary); margin-bottom: 0.25rem; }
  .poodle-component-card__description { font-size: 0.75rem; line-height: 1.45; color: var(--poodle-color-text-secondary); margin: 0; }
</style>
