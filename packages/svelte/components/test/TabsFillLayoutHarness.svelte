<script lang="ts">
  /**
   * Harness for the Tabs fill-layout seam tests: Tabs only renders its panel
   * when a `children(activeValue)` snippet is provided, and snippets cannot
   * be built from plain TypeScript tests.
   */
  import { default as Tabs } from "../src/Tabs.svelte";

  let {
    layout = "auto",
    hostHeight = "300px",
    hostDisplay = "block",
    panelScroll = "panel",
    panelPadding = null,
  }: {
    layout?: "auto" | "fill";
    hostHeight?: string;
    hostDisplay?: "block" | "flex";
    panelScroll?: "panel" | "content";
    panelPadding?: string | null;
  } = $props();

  const items = [
    { value: "mix", label: "Mix" },
    { value: "master", label: "Master" },
  ];
</script>

<div
  data-testid="fill-host"
  style="display: {hostDisplay}; flex-direction: column; height: {hostHeight};{panelPadding !== null
    ? ` --poodle-tabs-panel-padding: ${panelPadding};`
    : ''}"
>
  <Tabs {layout} {panelScroll} ariaLabel="Fill layout" {items}>
    {#snippet children(value)}
      {#if panelScroll === "content"}
        <div data-testid="content-scroll" style="max-height: 150px; overflow: auto; min-height: 0">
          <p data-testid="panel-content">Panel for {value}</p>
          {#each { length: 40 } as _, row (row)}
            <div data-testid="panel-row">Fill panel row {row}</div>
          {/each}
        </div>
      {:else}
        <p data-testid="panel-content">Panel for {value}</p>
        {#each { length: 40 } as _, row (row)}
          <div data-testid="panel-row">Fill panel row {row}</div>
        {/each}
      {/if}
    {/snippet}
  </Tabs>
</div>
