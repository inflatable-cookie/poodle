<script lang="ts">
  import { SidebarNav } from "@poodle/svelte";
  import { allComponents, componentsByTag, findComponent, type ComponentEntry } from "../component-registry";
  import CatalogueLanding from "../pages/CatalogueLanding.svelte";
  import ComponentPage from "../pages/ComponentPage.svelte";
  import { specimenMap } from "../specimens/registry";

  export let activeComponent: string | undefined = undefined;
  export let search: string = "";

  let contentElement: HTMLDivElement;

  $: entry = activeComponent ? findComponent(activeComponent) : undefined;
  $: if (activeComponent && contentElement) {
    contentElement.scrollTop = 0;
  }
  $: specimen = entry?.slug ? specimenMap[entry.slug] ?? null : null;

  $: searchLower = search.trim().toLowerCase();
  $: filteredComponents = searchLower
    ? allComponents.filter((c) => c.displayName.toLowerCase().includes(searchLower) || c.description.toLowerCase().includes(searchLower))
    : allComponents;

  $: navGroups = componentsByTag()
    .map((group) => ({
      id: group.tag,
      label: group.label,
      items: group.items
        .filter((c) => !searchLower || c.displayName.toLowerCase().includes(searchLower) || c.description.toLowerCase().includes(searchLower))
        .map((component) => ({
          value: component.slug,
          label: component.displayName,
          href: `#components/${component.slug}`,
        })),
    }))
    .filter((group) => group.items.length > 0);
</script>

<div class="poodle-catalogue-layout">
  <div class="poodle-catalogue-sidebar">
    <SidebarNav
      ariaLabel="Components"
      groups={navGroups}
      value={activeComponent ?? null}
    />
  </div>

  <div class="poodle-catalogue-content" bind:this={contentElement}>
    {#if entry}
      <ComponentPage {entry} specimenComponent={specimen} />
    {:else}
      <CatalogueLanding components={filteredComponents} />
    {/if}
  </div>
</div>

<style>
  .poodle-catalogue-layout {
    display: flex;
    min-height: 0;
    height: 100%;
  }

  .poodle-catalogue-sidebar {
    width: 14rem;
    flex-shrink: 0;
    border-right: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 60%, transparent);
    overflow-y: auto;
  }

  .poodle-catalogue-content {
    flex: 1;
    min-width: 0;
    overflow-y: auto;
  }
</style>
