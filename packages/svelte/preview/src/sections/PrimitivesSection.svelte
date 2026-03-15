<script lang="ts">
  import { primitiveComponents, findComponent } from "../component-registry";
  import CatalogueLanding from "../pages/CatalogueLanding.svelte";
  import ComponentPage from "../pages/ComponentPage.svelte";
  import { specimenMap } from "../specimens/registry";

  export let activeComponent: string | undefined = undefined;

  $: entry = activeComponent ? findComponent(activeComponent, "primitive") : undefined;
  $: specimen = entry?.slug ? specimenMap[entry.slug] ?? null : null;
</script>

<div class="catalogue-layout">
  <nav class="catalogue-sidebar" aria-label="Primitive components">
    <ul class="catalogue-sidebar__list">
      {#each primitiveComponents as component (component.slug)}
        <li>
          <a
            class="catalogue-sidebar__link"
            class:catalogue-sidebar__link--active={activeComponent === component.slug}
            href="#primitives/{component.slug}"
          >
            {component.displayName}
          </a>
        </li>
      {/each}
    </ul>
  </nav>

  <div class="catalogue-content">
    {#if entry}
      <ComponentPage {entry} specimenComponent={specimen} />
    {:else}
      <CatalogueLanding tier="primitive" components={primitiveComponents} />
    {/if}
  </div>
</div>

<style>
  .catalogue-layout {
    display: flex;
    min-height: 0;
    height: 100%;
  }

  .catalogue-sidebar {
    width: 14rem;
    flex-shrink: 0;
    border-right: 0.0625rem solid color-mix(in srgb, var(--pug-color-border-subtle) 60%, transparent);
    overflow-y: auto;
    padding: 0.75rem 0;
  }

  .catalogue-sidebar__list {
    list-style: none;
    margin: 0;
    padding: 0;
  }

  .catalogue-sidebar__link {
    display: block;
    padding: 0.375rem 1rem;
    font-size: 0.8125rem;
    color: var(--pug-color-text-secondary);
    text-decoration: none;
    border-left: 0.125rem solid transparent;
    transition: color 0.1s, border-color 0.1s, background 0.1s;
  }

  .catalogue-sidebar__link:hover {
    color: var(--pug-color-text-primary);
    background: color-mix(in srgb, var(--pug-color-background-elevated) 50%, transparent);
  }

  .catalogue-sidebar__link--active {
    color: var(--pug-color-text-primary);
    font-weight: 600;
    border-left-color: var(--pug-color-accent-base);
    background: color-mix(in srgb, var(--pug-color-accent-base) 8%, transparent);
  }

  .catalogue-content {
    flex: 1;
    min-width: 0;
    overflow-y: auto;
  }
</style>
