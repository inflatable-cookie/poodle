<script lang="ts">
  import { Breadcrumbs, type BreadcrumbItem } from "@poodle/svelte-primitives";
  import { Eyebrow } from "@poodle/svelte-primitives";

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
    <Eyebrow>Sizes</Eyebrow>
    <div class="specimen__stack">
      {#each controlSizes as size}
        <Breadcrumbs items={basicItems} {size} />
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Densities</Eyebrow>
    <div class="specimen__stack">
      {#each ["compact", "default", "comfortable"] as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <Breadcrumbs items={basicItems} {density} />
        </div>
      {/each}
    </div>
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

  .specimen__group p {
    margin: 0;
    font-size: 0.8125rem;
    color: var(--poodle-color-text-secondary);
  }
</style>
