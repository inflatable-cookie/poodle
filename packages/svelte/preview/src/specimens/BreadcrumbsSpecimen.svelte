<script lang="ts">
  import { Breadcrumbs, type BreadcrumbItem } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

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

  let lastNav = "";
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

  <SpecimenGroup label="Deep path">
    <Breadcrumbs items={deepItems} />
  </SpecimenGroup>

  <SpecimenGroup label="Collapsed (max 3 visible)">
    <Breadcrumbs items={deepItems} maxVisibleItems={3} />
  </SpecimenGroup>

  <svelte:fragment slot="sizes" let:size>
    <Breadcrumbs items={basicItems} {size} />
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <Breadcrumbs items={basicItems} {density} />
  </svelte:fragment>
</SpecimenLayout>
