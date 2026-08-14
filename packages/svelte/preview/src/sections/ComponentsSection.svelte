<script lang="ts">
  import { Icon, SidebarNav } from "@inflatable-cookie/poodle-svelte";
  import { allComponents, findComponent, type ComponentEntry } from "../component-registry";
  import {
    componentsBySection,
    isFamilyDisclosed,
    matchesCatalogueSearch,
  } from "../catalogue-nav";
  import CatalogueLanding from "../pages/CatalogueLanding.svelte";
  import ComponentPage from "../pages/ComponentPage.svelte";
  import { specimenMap } from "../specimens/registry";

  let { activeComponent = undefined, search = "" }: { activeComponent?: string | undefined; search?: string } = $props();
  let contentElement: HTMLDivElement | undefined = $state();
  let userExpanded = $state<string[]>([]);

  let entry = $derived(activeComponent ? findComponent(activeComponent) : undefined);
  $effect(() => {
    if (activeComponent && contentElement) {
      contentElement.scrollTop = 0;
    }
  });
  let specimen = $derived(entry?.slug ? specimenMap[entry.slug] ?? null : null);
  let searchLower = $derived(search.trim());
  let searchActive = $derived(searchLower.length > 0);
  let filteredComponents = $derived(
    searchActive
      ? allComponents.filter((component) => matchesCatalogueSearch(component, searchLower))
      : allComponents,
  );
  let expandedSet = $derived(new Set(userExpanded));
  let sectionGroups = $derived(componentsBySection(allComponents));
  let searchResults = $derived(filteredComponents);

  function toggleFamily(familyId: string): void {
    userExpanded = userExpanded.includes(familyId)
      ? userExpanded.filter((id) => id !== familyId)
      : [...userExpanded, familyId];
  }

  function familyOpen(familyId: (typeof sectionGroups)[number]["families"][number]["id"]): boolean {
    return isFamilyDisclosed(familyId, activeComponent, expandedSet, allComponents);
  }

  function navItems(items: ComponentEntry[]) {
    return items.map((component) => ({
      value: component.slug,
      label: component.displayName,
      href: `#components/${component.slug}`,
    }));
  }
</script>

<div class="poodle-catalogue-layout">
  <div class="poodle-catalogue-sidebar">
    {#if searchActive}
      <div class="poodle-catalogue-search" data-catalogue-search="true">
        {#if searchResults.length === 0}
          <p class="poodle-catalogue-search__empty">No matching components.</p>
        {:else}
          {#each searchResults as component (component.slug)}
            <a
              class="poodle-catalogue-search__item"
              href="#components/{component.slug}"
              aria-current={component.slug === activeComponent ? "page" : undefined}
              data-catalogue-result={component.slug}
            >
              <span class="poodle-catalogue-search__name">{component.displayName}</span>
              <span class="poodle-catalogue-search__crumb">{component.familyLabel} · {component.kindLabel}</span>
            </a>
          {/each}
        {/if}
      </div>
    {:else}
      <nav class="poodle-catalogue-nav" aria-label="Components">
        {#each sectionGroups as section (section.id)}
          <div class="poodle-catalogue-nav__section" data-catalogue-section={section.id}>
            <h2 class="poodle-catalogue-nav__section-title">{section.label}</h2>
            {#each section.families as family (family.id)}
              {@const open = familyOpen(family.id)}
              <div class="poodle-catalogue-family" data-catalogue-family={family.id} data-open={open || undefined}>
                <button
                  type="button"
                  class="poodle-catalogue-family__trigger"
                  aria-expanded={open}
                  onclick={() => toggleFamily(family.id)}
                >
                  <Icon name={open ? "chevron-down" : "chevron-right"} size="sm" />
                  <span class="poodle-catalogue-family__label">{family.label}</span>
                  <span class="poodle-catalogue-family__count">{family.items.length}</span>
                </button>
                {#if open}
                  <div class="poodle-catalogue-family__items">
                    <SidebarNav
                      ariaLabel={family.label}
                      groups={[{ id: family.id, items: navItems(family.items) }]}
                      value={activeComponent ?? null}
                    />
                  </div>
                {/if}
              </div>
            {/each}
          </div>
        {/each}
      </nav>
    {/if}
  </div>

  <div class="poodle-catalogue-content" bind:this={contentElement}>
    {#if entry}
      <ComponentPage {entry} specimenComponent={specimen} />
    {:else}
      <CatalogueLanding components={filteredComponents} />
    {/if}
  </div>
</div>
