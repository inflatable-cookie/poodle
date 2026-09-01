<script lang="ts">
  import DockRegion from "../src/DockRegion.svelte";
  import DragDropProvider from "../src/DragDropProvider.svelte";
  import type { DockEdge, DockPanelDropPayload, DockSizing, PanelTabItem } from "../src/types";

  /**
   * Two dock regions, with or without one common provider.
   *
   * The whole point of the pair is the difference between those two mountings:
   * under one provider the regions share a controller and can resolve each
   * other's targets, and self-provided they cannot. Nothing else about them
   * differs.
   */
  interface Props {
    shared: boolean;
    items: PanelTabItem[];
    itemsB?: PanelTabItem[];
    sizing?: DockSizing;
    canAcceptPanel?: ((panelId: string, sourceEdge: DockEdge) => boolean) | null;
    onPanelDropA?: (payload: DockPanelDropPayload) => void;
    onPanelDropB?: (payload: DockPanelDropPayload) => void;
    onReorderA?: (order: string[]) => void;
    edge?: DockEdge;
  }

  let {
    shared,
    items,
    itemsB = items,
    sizing = "static",
    canAcceptPanel = null,
    onPanelDropA,
    onPanelDropB,
    onReorderA,
    edge = "top",
  }: Props = $props();
</script>

{#snippet pair()}
  <DockRegion
    {sizing}
    {edge}
    dragZoneId="region:a"
    {items}
    {canAcceptPanel}
    onPanelDrop={onPanelDropA}
    onReorder={onReorderA}
  >
    {#snippet panel(item)}
      <span data-panel={item.value}>{item.label}</span>
    {/snippet}
  </DockRegion>
  <DockRegion
    {sizing}
    {edge}
    dragZoneId="region:b"
    items={itemsB}
    {canAcceptPanel}
    onPanelDrop={onPanelDropB}
  >
    {#snippet panel(item)}
      <span data-panel={item.value}>{item.label}</span>
    {/snippet}
  </DockRegion>
{/snippet}

{#if shared}
  <DragDropProvider>
    {@render pair()}
  </DragDropProvider>
{:else}
  {@render pair()}
{/if}
