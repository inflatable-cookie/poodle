<script lang="ts">
  import { Eyebrow } from "@inflatable-cookie/poodle-svelte";
  import { componentsBySection, type CatalogueNavEntry } from "../catalogue-nav";
  import type { ComponentEntry } from "../component-registry";

  let { components = [] }: { components?: ComponentEntry[] } = $props();
  let groups = $derived(componentsBySection(components as CatalogueNavEntry[]));
</script>

<div class="poodle-catalogue-landing">
  <div class="poodle-catalogue-landing__header">
    <h2>Component catalogue</h2>
    <p>Browse the full Poodle component library. Each component handles accessibility, keyboard support, and theming.</p>
    <p class="poodle-catalogue-landing__count">{components.length} components</p>
  </div>

  {#each groups as section (section.id)}
    <section class="poodle-catalogue-landing__section" data-catalogue-section={section.id}>
      <Eyebrow>{section.label}</Eyebrow>
      {#each section.families as family (family.id)}
        <div class="poodle-catalogue-landing__family" data-catalogue-family={family.id}>
          <h3 class="poodle-catalogue-landing__family-title">
            {family.label}
            <span class="poodle-catalogue-landing__family-count">{family.items.length}</span>
          </h3>
          <div class="poodle-catalogue-landing__grid">
            {#each family.items as component (component.slug)}
              <a class="poodle-component-card" href="#components/{component.slug}">
                <strong class="poodle-component-card__name">{component.displayName}</strong>
                <p class="poodle-component-card__description">{component.description}</p>
                <p class="poodle-component-card__crumb">{component.familyLabel} · {component.kindLabel}</p>
              </a>
            {/each}
          </div>
        </div>
      {/each}
    </section>
  {/each}
</div>
