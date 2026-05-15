<script lang="ts">
  import { Surface, Tabs, getUiPresentation, type TabItem } from "@poodle/svelte";

  export let activeTab: "examples" | "sizes" | "densities" = "examples";
  /** When true, size/density variants render without a Surface wrapper. */
  export let bareVariants = false;
  /** Layout direction for size/density variants. */
  export let variantDirection: "row" | "column" = "column";

  const tabs: TabItem[] = [
    { value: "examples", label: "Examples" },
    { value: "sizes", label: "Sizes" },
    { value: "densities", label: "Densities" },
  ];

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;
  const densities = ["compact", "default", "comfortable"] as const;

  const uiPresentation = getUiPresentation();
</script>

<div class="poodle-specimen-layout">
  <Tabs
    value={activeTab}
    items={tabs}
    variant="text"
    ariaLabel="Specimen view"
    onValueChange={(value) => (activeTab = value as typeof activeTab)}
  />

  <div class="poodle-specimen-layout__content">
    {#if activeTab === "examples"}
      <slot />
    {:else if activeTab === "sizes"}
      {#if bareVariants}
        <div class="poodle-specimen-layout__variants" data-direction={variantDirection}>
          {#each controlSizes as size}
            <slot name="sizes" {size} />
          {/each}
        </div>
      {:else}
        <Surface tone="panel" border="subtle" padding="md">
          <div class="poodle-specimen-layout__variants" data-direction={variantDirection}>
            {#each controlSizes as size}
              <slot name="sizes" {size} />
            {/each}
          </div>
        </Surface>
      {/if}
    {:else if activeTab === "densities"}
      {#if bareVariants}
        <div class="poodle-specimen-layout__variants" data-direction={variantDirection}>
          {#each densities as density}
            <slot name="densities" {density} />
          {/each}
        </div>
      {:else}
        <Surface tone="panel" border="subtle" padding="md">
          <div class="poodle-specimen-layout__variants" data-direction={variantDirection}>
            {#each densities as density}
              <slot name="densities" {density} />
            {/each}
          </div>
        </Surface>
      {/if}
    {/if}
  </div>
</div>

<style>
  .poodle-specimen-layout {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-specimen-layout__content {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-specimen-layout__variants {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 1rem;
  }

  .poodle-specimen-layout__variants[data-direction="row"] {
    flex-direction: row;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.75rem;
  }
</style>
