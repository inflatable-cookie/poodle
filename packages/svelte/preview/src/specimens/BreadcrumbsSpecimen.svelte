<script lang="ts">
  import { Breadcrumbs, type BreadcrumbItem } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  const basicItems: BreadcrumbItem[] = [
    { value: "home", label: "Home" },
    { value: "projects", label: "Projects" },
    { value: "poodle", label: "Poodle", current: true },
  ];

  // Icon presentation is per item: a named glyph beside the label, or a
  // visually icon-only root that is still announced as "Home".
  const iconItems: BreadcrumbItem[] = [
    { value: "home", label: "Home", icon: "home", iconOnly: true },
    { value: "projects", label: "Projects", icon: "folder" },
    { value: "poodle", label: "Poodle", icon: "package", current: true },
  ];

  const deepItems: BreadcrumbItem[] = [
    { value: "home", label: "Home" },
    { value: "workspace", label: "Workspace" },
    { value: "projects", label: "Projects" },
    { value: "poodle", label: "Poodle Design System" },
    { value: "primitives", label: "Primitives" },
    { value: "button", label: "Button", current: true },
  ];

  let lastNav = $state("");
</script>

<SpecimenLayout>
  <SpecimenGroup label="Basic">
    <Breadcrumbs
      items={basicItems}
      onNavigate={(value) => (lastNav = value)}
    />
    {#if lastNav}
      <p>Navigated to: <strong>{lastNav}</strong></p>
    {/if}
  </SpecimenGroup>

  <SpecimenGroup label="Icons">
    <Breadcrumbs items={iconItems} />
  </SpecimenGroup>

  <SpecimenGroup label="Deep path">
    <Breadcrumbs items={deepItems} />
  </SpecimenGroup>

  <SpecimenGroup label="Collapsed (max 3 visible)">
    <Breadcrumbs items={deepItems} maxVisibleItems={3} />
  </SpecimenGroup>

  {#snippet sizes(size)}
    <Breadcrumbs items={basicItems} {size} />
  {/snippet}

  {#snippet densities(density)}
    <Breadcrumbs items={basicItems} {density} />
  {/snippet}
</SpecimenLayout>
