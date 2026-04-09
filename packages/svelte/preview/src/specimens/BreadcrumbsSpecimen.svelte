<script lang="ts">
  import { Breadcrumbs, type BreadcrumbItem, type ControlDensity } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const densities: ControlDensity[] = ["compact", "default", "comfortable"];

  const basicItems: BreadcrumbItem[] = [
    { value: "home", label: "Home" },
    { value: "projects", label: "Projects" },
    { value: "poodle", label: "Poodle", current: true },
  ];

  const deepItems: BreadcrumbItem[] = [
    { value: "home", label: "Home" },
    { value: "workspace", label: "Workspace" },
    { value: "projects", label: "Projects" },
    { value: "poodle", label: "Poodle Design System" },
    { value: "primitives", label: "Primitives" },
    { value: "button", label: "Button", current: true },
  ];

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let lastNav = "";
</script>

<div class="specimen">
  <SpecimenGroup label="Basic">
    <Breadcrumbs
      items={basicItems}
      on:navigate={(e) => (lastNav = e.detail.value)}
    />
    {#if lastNav}
      <p>Navigated to: <strong>{lastNav}</strong></p>
    {/if}
  </SpecimenGroup>

  <SpecimenGroup label="Sizes">
    <div class="specimen__stack">
      {#each controlSizes as size}
        <Breadcrumbs items={basicItems} {size} />
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Densities">
    <div class="specimen__stack">
      {#each densities as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <Breadcrumbs items={basicItems} {density} />
        </div>
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Deep path">
    <Breadcrumbs items={deepItems} />
  </SpecimenGroup>

  <SpecimenGroup label="Collapsed (max 3 visible)">
    <Breadcrumbs items={deepItems} maxVisibleItems={3} />
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .specimen__row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .specimen__label {
    font-size: 0.75rem;
    font-family: var(--poodle-typography-code-family);
    color: var(--poodle-color-text-muted);
    min-width: 6rem;
  }

</style>
