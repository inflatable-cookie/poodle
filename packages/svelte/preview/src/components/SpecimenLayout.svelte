<script lang="ts">
  import { Tabs, UiPresentationProvider, getUiPresentation, type TabItem } from "@poodle/svelte-primitives";

  export let activeTab: "examples" | "sizes" | "densities" = "examples";

  const tabs: TabItem[] = [
    { value: "examples", label: "Examples" },
    { value: "sizes", label: "Sizes" },
    { value: "densities", label: "Densities" },
  ];

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;
  const densities = ["compact", "default", "comfortable"] as const;

  const uiPresentation = getUiPresentation();
</script>

<div class="specimen-layout">
  <Tabs
    value={activeTab}
    items={tabs}
    variant="text"
    ariaLabel="Specimen view"
    on:valueChange={(e) => (activeTab = e.detail.value as typeof activeTab)}
  />

  <div class="specimen-layout__content">
    {#if activeTab === "examples"}
      <slot />
    {:else if activeTab === "sizes"}
      <div class="specimen-layout__grid">
        {#each controlSizes as size}
          <div class="specimen-layout__variant">
            <span class="specimen-layout__label">{size}</span>
            <UiPresentationProvider sizeScale={size} density={$uiPresentation.density}>
              <div class="specimen-layout__demo">
                <slot name="sizes" {size} />
              </div>
            </UiPresentationProvider>
          </div>
        {/each}
      </div>
    {:else if activeTab === "densities"}
      <div class="specimen-layout__grid">
        {#each densities as density}
          <div class="specimen-layout__variant">
            <span class="specimen-layout__label">{density}</span>
            <UiPresentationProvider sizeScale={$uiPresentation.sizeScale} {density}>
              <div class="specimen-layout__demo">
                <slot name="densities" {density} />
              </div>
            </UiPresentationProvider>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>

<style>
  .specimen-layout {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .specimen-layout__content {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .specimen-layout__grid {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .specimen-layout__variant {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
  }

  .specimen-layout__label {
    font-size: 0.6875rem;
    font-family: var(--poodle-typography-code-family);
    color: var(--poodle-color-text-tertiary);
    text-transform: uppercase;
    letter-spacing: 0.08em;
  }

  .specimen-layout__demo {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 0.5rem;
  }
</style>
