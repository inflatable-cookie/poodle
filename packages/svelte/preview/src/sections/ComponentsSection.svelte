<script lang="ts">
  import { SidebarNav } from "@poodle/svelte";
  import { allComponents, findComponent } from "../component-registry";
  import CatalogueLanding from "../pages/CatalogueLanding.svelte";
  import ComponentPage from "../pages/ComponentPage.svelte";
  import { specimenMap } from "../specimens/registry";

  export let activeComponent: string | undefined = undefined;

  $: entry = activeComponent ? findComponent(activeComponent) : undefined;
  $: specimen = entry?.slug ? specimenMap[entry.slug] ?? null : null;
  $: navGroups = [{
    id: "components",
    items: allComponents.map((component) => ({
      value: component.slug,
      label: component.displayName,
      href: `#components/${component.slug}`,
    })),
  }];
</script>

<div class="catalogue-layout">
  <div class="catalogue-sidebar">
    <SidebarNav
      ariaLabel="Components"
      groups={navGroups}
      value={activeComponent ?? null}
    />
  </div>

  <div class="catalogue-content">
    {#if entry}
      <ComponentPage {entry} specimenComponent={specimen} />
    {:else}
      <CatalogueLanding components={allComponents} />
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
    border-right: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 60%, transparent);
    overflow-y: auto;
  }

  .catalogue-content {
    flex: 1;
    min-width: 0;
    overflow-y: auto;
  }
</style>
