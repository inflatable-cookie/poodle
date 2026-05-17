<script lang="ts">
  import { Surface, Tabs, getUiPresentation, type TabItem } from "@poodle/svelte";

  let {
    activeTab = "examples",
    bareVariants = false,
    variantDirection = "column",
    showSizes = true,
    showDensities = true,
  }: {
    activeTab?: "examples" | "sizes" | "densities";
    bareVariants?: boolean;
    variantDirection?: "row" | "column";
    showSizes?: boolean;
    showDensities?: boolean;
  } = $props();

  const tabs: TabItem[] = $derived([
    { value: "examples", label: "Examples" },
    ...(showSizes ? [{ value: "sizes", label: "Sizes" }] : []),
    ...(showDensities ? [{ value: "densities", label: "Densities" }] : []),
  ]);

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
    {:else if activeTab === "sizes" && showSizes}
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
    {:else if activeTab === "densities" && showDensities}
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
