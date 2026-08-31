<script lang="ts">
  import DockRegion from "../src/DockRegion.svelte";
  import DragDropProvider from "../src/DragDropProvider.svelte";
  import type { DockEdge, PanelDragData, PanelTabItem } from "../src/types";

  /**
   * Two static dock regions, with or without one common provider.
   *
   * The whole point of the pair is the difference between those two mountings:
   * under one provider the regions share a controller and can resolve each
   * other's targets, and self-provided they cannot. Nothing else about them
   * differs.
   */
  interface Props {
    shared: boolean;
    items: PanelTabItem[];
    canAcceptPanel?: ((panelId: string, sourceEdge: DockEdge) => boolean) | null;
    onPanelDropA?: (payload: { panel: PanelDragData; targetEdge: DockEdge }) => void;
    onPanelDropB?: (payload: { panel: PanelDragData; targetEdge: DockEdge }) => void;
    onReorderA?: (order: string[]) => void;
  }

  let {
    shared,
    items,
    canAcceptPanel = null,
    onPanelDropA,
    onPanelDropB,
    onReorderA,
  }: Props = $props();
</script>

{#snippet pair()}
  <DockRegion
    sizing="static"
    edge="top"
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
    sizing="static"
    edge="top"
    dragZoneId="region:b"
    {items}
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
