<script lang="ts">
  import { Surface, Tabs, UiPresentationProvider, getUiPresentation, type TabItem } from "@poodle/svelte-primitives";

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
      <Surface tone="panel" border="subtle" padding="md">
        <div class="specimen-layout__variants">
          {#each controlSizes as size}
            <UiPresentationProvider sizeScale={size} density={$uiPresentation.density}>
              <slot name="sizes" {size} />
            </UiPresentationProvider>
          {/each}
        </div>
      </Surface>
    {:else if activeTab === "densities"}
      <Surface tone="panel" border="subtle" padding="md">
        <div class="specimen-layout__variants">
          {#each densities as density}
            <UiPresentationProvider sizeScale={$uiPresentation.sizeScale} {density}>
              <slot name="densities" {density} />
            </UiPresentationProvider>
          {/each}
        </div>
      </Surface>
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

  .specimen-layout__variants {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 0.75rem;
  }
</style>
