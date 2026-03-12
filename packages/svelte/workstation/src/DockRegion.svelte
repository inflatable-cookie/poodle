<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { EmptyState } from "@pug/svelte-composites";

  import PanelHeader from "./PanelHeader.svelte";
  import PanelSurface from "./PanelSurface.svelte";
  import PanelTabs from "./PanelTabs.svelte";

  import type { DockEdge, PanelTabItem } from "./types";

  export let edge: DockEdge = "left";
  export let isCollapsed = false;
  export let tabsPlacement: "edge" | "top" = "edge";
  export let items: PanelTabItem[] = [];
  export let value: string | null = null;
  export let ariaLabel: string | null = null;

  const dispatch = createEventDispatcher<{
    valueChange: { value: string };
    collapsedChange: { collapsed: boolean };
    requestContextMenu: { value: string | null };
    requestClose: { value: string };
    reorder: { items: string[] };
  }>();

  $: activeItem = items.find((item) => item.value === value) ?? items[0] ?? null;
</script>

<section
  class="dock-region"
  data-edge={edge}
  data-collapsed={isCollapsed}
  data-tabs-placement={tabsPlacement}
  aria-label={ariaLabel ?? `${edge} dock`}
>
  <PanelSurface
    title={activeItem?.label ?? `${edge} dock`}
    hasHeader={true}
    bodyPadding="none"
    isActive={!isCollapsed && items.length > 0}
  >
    <div slot="header">
      <PanelHeader title={activeItem?.label ?? `${edge} dock`} isActive={!isCollapsed} isCollapsible={true}>
        <div slot="tabs">
          <PanelTabs
            items={items}
            {value}
            ariaLabel={`${edge} dock panels`}
            on:valueChange={(event) => dispatch("valueChange", event.detail)}
            on:reorder={(event) => dispatch("reorder", event.detail)}
            on:requestContextMenu={(event) => dispatch("requestContextMenu", event.detail)}
            on:requestClose={(event) => dispatch("requestClose", event.detail)}
          />
        </div>
        <div slot="collapse">
          <button
            type="button"
            class="dock-region__collapse"
            aria-label={isCollapsed ? `Expand ${edge} dock` : `Collapse ${edge} dock`}
            aria-pressed={isCollapsed}
            on:click={() => dispatch("collapsedChange", { collapsed: !isCollapsed })}
          >
            {isCollapsed ? "⇢" : "⇠"}
          </button>
        </div>
      </PanelHeader>
    </div>

    {#if items.length === 0}
      <div class="dock-region__empty">
        <slot name="empty">
          <EmptyState
            title="No dock panels"
            message="Dock regions should keep an explicit empty posture instead of collapsing into unlabeled chrome."
          />
        </slot>
      </div>
    {:else if isCollapsed}
      <div class="dock-region__collapsed">
        <slot name="collapsed" activeItem={activeItem}>
          <span>{activeItem?.label ?? `${edge} dock`}</span>
        </slot>
      </div>
    {:else}
      <div class="dock-region__body">
        <slot activeItem={activeItem} />
      </div>
    {/if}
  </PanelSurface>
</section>

<style>
  .dock-region {
    min-width: 0;
    min-height: 0;
  }

  .dock-region__body,
  .dock-region__collapsed,
  .dock-region__empty {
    min-height: 0;
  }

  .dock-region__body {
    height: 100%;
  }

  .dock-region__collapsed {
    display: flex;
    align-items: center;
    justify-content: flex-start;
    min-height: 160px;
    padding: 16px 14px;
    color: var(--pug-color-text-secondary);
    font-size: 13px;
    line-height: 1.5;
  }

  .dock-region__collapse {
    min-height: 28px;
    min-width: 28px;
    padding: 0;
    border: 0;
    border-radius: calc(var(--pug-radius-control) - 1px);
    background: color-mix(in srgb, var(--pug-color-background-surface) 62%, transparent);
    color: var(--pug-color-text-secondary);
    cursor: pointer;
    font: inherit;
  }

  .dock-region__collapse:hover {
    background: color-mix(in srgb, var(--pug-color-background-surface) 82%, transparent);
    color: var(--pug-color-text-primary);
  }

  .dock-region__collapse:focus-visible {
    outline: var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing);
    outline-offset: 2px;
  }
</style>
