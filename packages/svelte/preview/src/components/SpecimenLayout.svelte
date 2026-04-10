<script lang="ts">
  import { Eyebrow, Surface, Tabs, UiPresentationProvider, getUiPresentation, type TabItem } from "@poodle/svelte-primitives";

  export let activeTab: "examples" | "sizes" | "densities" = "examples";
  /** When true, each size/density variant is wrapped in a Surface card. */
  export let surfaceVariants = false;

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
          <UiPresentationProvider sizeScale={size} density={$uiPresentation.density}>
            {#if surfaceVariants}
              <Surface tone="panel" border="subtle" padding="md">
                <Eyebrow>{size}</Eyebrow>
                <div class="specimen-layout__demo">
                  <slot name="sizes" {size} />
                </div>
              </Surface>
            {:else}
              <div class="specimen-layout__variant">
                <Eyebrow>{size}</Eyebrow>
                <div class="specimen-layout__demo">
                  <slot name="sizes" {size} />
                </div>
              </div>
            {/if}
          </UiPresentationProvider>
        {/each}
      </div>
    {:else if activeTab === "densities"}
      <div class="specimen-layout__grid">
        {#each densities as density}
          <UiPresentationProvider sizeScale={$uiPresentation.sizeScale} {density}>
            {#if surfaceVariants}
              <Surface tone="panel" border="subtle" padding="md">
                <Eyebrow>{density}</Eyebrow>
                <div class="specimen-layout__demo">
                  <slot name="densities" {density} />
                </div>
              </Surface>
            {:else}
              <div class="specimen-layout__variant">
                <Eyebrow>{density}</Eyebrow>
                <div class="specimen-layout__demo">
                  <slot name="densities" {density} />
                </div>
              </div>
            {/if}
          </UiPresentationProvider>
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

  .specimen-layout__demo {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 0.5rem;
    margin-top: 0.25rem;
  }
</style>
