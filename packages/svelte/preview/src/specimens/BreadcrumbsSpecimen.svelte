<script lang="ts">
  import { Breadcrumbs, type BreadcrumbItem } from "@flint/svelte-primitives";
  import { Eyebrow } from "@flint/svelte-primitives";

  const basicItems: BreadcrumbItem[] = [
    { value: "home", label: "Home" },
    { value: "projects", label: "Projects" },
    { value: "flint", label: "Flint", isCurrent: true },
  ];

  const deepItems: BreadcrumbItem[] = [
    { value: "home", label: "Home" },
    { value: "workspace", label: "Workspace" },
    { value: "projects", label: "Projects" },
    { value: "flint", label: "Flint Design System" },
    { value: "primitives", label: "Primitives" },
    { value: "button", label: "Button", isCurrent: true },
  ];

  let lastNav = "";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Basic</Eyebrow>
    <Breadcrumbs
      items={basicItems}
      on:navigate={(e) => (lastNav = e.detail.value)}
    />
    {#if lastNav}
      <p>Navigated to: <strong>{lastNav}</strong></p>
    {/if}
  </div>

  <div class="specimen__group">
    <Eyebrow>Deep path</Eyebrow>
    <Breadcrumbs items={deepItems} />
  </div>

  <div class="specimen__group">
    <Eyebrow>Collapsed (max 3 visible)</Eyebrow>
    <Breadcrumbs items={deepItems} maxVisibleItems={3} />
  </div>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .specimen__group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .specimen__group p {
    margin: 0;
    font-size: 0.8125rem;
    color: var(--flint-color-text-secondary);
  }
</style>
