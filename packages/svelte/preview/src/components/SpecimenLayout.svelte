<script lang="ts">
  import { Surface, Tabs, getUiPresentation, type TabItem } from "@inflatable-cookie/poodle-svelte";
  import type { Snippet } from "svelte";

  const DEFAULT_CONTROL_SIZES = ["xs", "sm", "md", "lg", "xl"] as const;
  const DEFAULT_CONTROL_DENSITIES = ["compact", "default", "comfortable"] as const;

  let {
    activeTab = "examples",
    bareVariants = false,
    variantDirection = "column",
    showSizes = true,
    showDensities = true,
    children,
    sizes,
    densities,
    sizeValues,
    densityValues,
  }: {
    activeTab?: "examples" | "sizes" | "densities";
    bareVariants?: boolean;
    variantDirection?: "row" | "column";
    showSizes?: boolean;
    showDensities?: boolean;
    children?: Snippet;
    sizes?: Snippet<[size: string]>;
    densities?: Snippet<[density: string]>;
    sizeValues?: readonly string[];
    densityValues?: readonly string[];
  } = $props();

  const tabs: TabItem[] = $derived([
    { value: "examples", label: "Examples" },
    ...(showSizes && sizes ? [{ value: "sizes", label: "Sizes" }] : []),
    ...(showDensities && densities ? [{ value: "densities", label: "Densities" }] : []),
  ]);

  const tabValues = $derived(tabs.map((tab) => tab.value));
  $effect(() => {
    if (!tabValues.includes(activeTab)) {
      activeTab = "examples";
    }
  });

  const resolvedSizeValues = $derived(sizeValues ?? DEFAULT_CONTROL_SIZES);
  const resolvedDensityValues = $derived(densityValues ?? DEFAULT_CONTROL_DENSITIES);

  const uiPresentation = getUiPresentation();
</script>

<div class="poodle-specimen-layout">
  <Tabs
    value={activeTab}
    items={tabs}
    variant="card"
    ariaLabel="Specimen view"
    onValueChange={(value) => (activeTab = value as typeof activeTab)}
  />

  <div class="poodle-specimen-layout__content">
    {#if activeTab === "examples"}
      {@render children?.()}
    {:else if activeTab === "sizes" && showSizes}
      {#if bareVariants}
        <div class="poodle-specimen-layout__variants" data-direction={variantDirection}>
          {#each resolvedSizeValues as size}
            {@render sizes?.(size)}
          {/each}
        </div>
      {:else}
        <Surface tone="panel" border="subtle" padding="md">
          <div class="poodle-specimen-layout__variants" data-direction={variantDirection}>
            {#each resolvedSizeValues as size}
              {@render sizes?.(size)}
            {/each}
          </div>
        </Surface>
      {/if}
    {:else if activeTab === "densities" && showDensities}
      {#if bareVariants}
        <div class="poodle-specimen-layout__variants" data-direction={variantDirection}>
          {#each resolvedDensityValues as density}
            {@render densities?.(density)}
          {/each}
        </div>
      {:else}
        <Surface tone="panel" border="subtle" padding="md">
          <div class="poodle-specimen-layout__variants" data-direction={variantDirection}>
            {#each resolvedDensityValues as density}
              {@render densities?.(density)}
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
