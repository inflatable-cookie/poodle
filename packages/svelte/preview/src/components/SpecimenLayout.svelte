<script lang="ts">
  import { Surface, Tabs, getUiPresentation, type TabItem } from "@inflatable-cookie/poodle-svelte";
  import type { Snippet } from "svelte";

  let {
    activeTab = "examples",
    bareVariants = false,
    variantDirection = "column",
    showSizes = true,
    showDensities = true,
    children,
    sizes,
    densities,
  }: {
    activeTab?: "examples" | "sizes" | "densities";
    bareVariants?: boolean;
    variantDirection?: "row" | "column";
    showSizes?: boolean;
    showDensities?: boolean;
    children?: Snippet;
    sizes?: Snippet<[size: (typeof controlSizes)[number]]>;
    densities?: Snippet<[density: (typeof controlDensities)[number]]>;
  } = $props();

  // An axis tab needs a renderer: without one the pane would be empty, which
  // is exactly what the catalogue must never advertise. `show*` may hide a
  // supplied renderer but cannot force a tabless pane.
  const tabs: TabItem[] = $derived([
    { value: "examples", label: "Examples" },
    ...(showSizes && sizes ? [{ value: "sizes", label: "Sizes" }] : []),
    ...(showDensities && densities ? [{ value: "densities", label: "Densities" }] : []),
  ]);

  // The preview reuses one layout across scene slugs (SceneSpecimen). When the
  // available tab set shrinks — e.g. Callout keeps Densities but Avatar does
  // not — the retained active tab would point at a vanished pane and render a
  // blank page. Normalize an invalid selection back to Examples whenever the
  // tab set changes.
  const tabValues = $derived(tabs.map((tab) => tab.value));
  $effect(() => {
    if (!tabValues.includes(activeTab)) {
      activeTab = "examples";
    }
  });

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;
  const controlDensities = ["compact", "default", "comfortable"] as const;

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
          {#each controlSizes as size}
            {@render sizes?.(size)}
          {/each}
        </div>
      {:else}
        <Surface tone="panel" border="subtle" padding="md">
          <div class="poodle-specimen-layout__variants" data-direction={variantDirection}>
            {#each controlSizes as size}
              {@render sizes?.(size)}
            {/each}
          </div>
        </Surface>
      {/if}
    {:else if activeTab === "densities" && showDensities}
      {#if bareVariants}
        <div class="poodle-specimen-layout__variants" data-direction={variantDirection}>
          {#each controlDensities as density}
            {@render densities?.(density)}
          {/each}
        </div>
      {:else}
        <Surface tone="panel" border="subtle" padding="md">
          <div class="poodle-specimen-layout__variants" data-direction={variantDirection}>
            {#each controlDensities as density}
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
