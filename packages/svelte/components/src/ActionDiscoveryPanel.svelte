<script lang="ts">
  import Eyebrow from "./Eyebrow.svelte";
  import ListCard from "./ListCard.svelte";
  import Skeleton from "./Skeleton.svelte";
  import UiPresentationProvider from "./UiPresentationProvider.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type {
    ControlDensity,
    ControlSize,
    SemanticControlSizeRole,
  } from "./types";
  import EmptyState from "./EmptyState.svelte";

  import type { CommandActionItem, DiscoveryState } from "./types";

  interface Props {
    items?: CommandActionItem[];
    state?: DiscoveryState;
    activeId?: string | null;
    ariaLabel?: string;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onItemSelect?: ((id: string) => void) | null;
    onActiveChange?: ((id: string | null) => void) | null;
  }

  let {
    items = [],
    state: discoveryState = "ready",
    activeId = $bindable<string | null>(null),
    ariaLabel = "Actions",
    size = null,
    sizeRole = "control",
    density = null,
    onItemSelect = null,
    onActiveChange = null,
  }: Props = $props();

  let itemElements = $state<Array<HTMLElement | null>>([]);
  const uiPresentation = getUiPresentation();

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const enabledItems = $derived(items.filter((item) => !item.disabled));
  const groupedItems = $derived.by(() =>
    items.reduce<Record<string, CommandActionItem[]>>((acc, item) => {
      const group = item.group ?? "Commands";
      acc[group] ??= [];
      acc[group].push(item);
      return acc;
    }, {}),
  );
  const groupEntries = $derived(Object.entries(groupedItems));

  export function moveActive(step: 1 | -1): void {
    if (enabledItems.length === 0) return;
    const idx = enabledItems.findIndex((item) => item.id === activeId);
    const next = idx === -1 ? 0 : (idx + step + enabledItems.length) % enabledItems.length;
    setActive(enabledItems[next]?.id ?? null);
  }

  export function moveToBoundary(direction: "start" | "end"): void {
    if (enabledItems.length === 0) return;
    setActive(direction === "start" ? enabledItems[0]?.id ?? null : enabledItems[enabledItems.length - 1]?.id ?? null);
  }

  export function selectActive(): void {
    if (activeId) onItemSelect?.(activeId);
  }

  export function getEnabledItems(): CommandActionItem[] {
    return enabledItems;
  }

  function setActive(id: string | null): void {
    activeId = id;
    onActiveChange?.(id);

    if (id) {
      const idx = enabledItems.findIndex((item) => item.id === id);
      if (idx >= 0) {
        queueMicrotask(() => {
          itemElements[idx]?.scrollIntoView({ block: "nearest" });
        });
      }
    }
  }
</script>

<UiPresentationProvider sizeScale={resolvedSize} density={resolvedDensity}>
  <div
    class="poodle-action-discovery-panel"
    role="listbox"
    aria-label={ariaLabel}
    data-size={resolvedSize}
    data-density={resolvedDensity}
  >
    {#if discoveryState === "loading"}
      <div class="poodle-action-discovery-panel__state">
        <div class="poodle-action-discovery-panel__skeletons" aria-hidden="true">
          {#each Array.from({ length: 5 }) as _}
            <div class="poodle-action-discovery-panel__skeleton-row">
              <Skeleton width="48%" />
              <Skeleton width="20%" />
            </div>
          {/each}
        </div>
      </div>
    {:else if discoveryState === "error"}
      <EmptyState
        title="Could not load actions"
        message="Actions could not be loaded. Try again."
      />
    {:else if discoveryState === "empty"}
      <EmptyState
        title="No actions available"
        message="No actions are available in this context."
      />
    {:else if discoveryState === "no-results"}
      <EmptyState
        title="No matching actions"
        message="No actions match the current search."
        variant="search"
      />
    {:else}
      {#each groupEntries as [group, groupItems]}
        <div class="poodle-action-discovery-panel__group">
          <Eyebrow>{group}</Eyebrow>
          <ul class="poodle-action-discovery-panel__list">
            {#each groupItems as item (item.id)}
              <li
                bind:this={itemElements[enabledItems.findIndex((enabledItem) => enabledItem.id === item.id)]}
                role="option"
                aria-selected={activeId === item.id}
              >
                <ListCard
                  title={item.title}
                  subtitle={item.description}
                  interactive={!item.disabled}
                  disabled={item.disabled ?? false}
                  ariaLabel={item.title}
                  onClick={() => onItemSelect?.(item.id)}
                  onmouseenter={() => setActive(item.id)}
                  onfocus={() => setActive(item.id)}
                >
                  {#snippet trailing()}
                    <span class="poodle-action-discovery-panel__trailing">
                      {#if item.badge}
                        <span class="poodle-action-discovery-panel__badge">{item.badge}</span>
                      {/if}
                      {#if item.shortcut}
                        <kbd class="poodle-action-discovery-panel__kbd">{item.shortcut}</kbd>
                      {/if}
                    </span>
                  {/snippet}
                </ListCard>
              </li>
            {/each}
          </ul>
        </div>
      {/each}
    {/if}
  </div>
</UiPresentationProvider>

<style>
  .poodle-action-discovery-panel {
    --poodle-action-discovery-stack-gap: 0.75rem;
    --poodle-action-discovery-group-gap: 0.375rem;
    --poodle-action-discovery-list-gap: 0.25rem;
    --poodle-action-discovery-chip-height: 1.375rem;
    --poodle-action-discovery-chip-x: 0.5rem;
    --poodle-action-discovery-chip-gap: 0.375rem;
    --poodle-action-discovery-chip-font-size: 0.6875rem;
    --poodle-action-discovery-skeleton-pad: 0.875rem;
    --poodle-action-discovery-row-y: 0.375rem;
    --poodle-action-discovery-row-x: 0.625rem;
    display: grid;
    gap: var(--poodle-action-discovery-stack-gap);
    min-height: 0;
    overflow: auto;
    overscroll-behavior: contain;
  }

  .poodle-action-discovery-panel :global(.poodle-list-card) {
    padding: var(--poodle-action-discovery-row-y) var(--poodle-action-discovery-row-x);
  }

  .poodle-action-discovery-panel[data-size="xs"] {
    --poodle-action-discovery-chip-height: 1.125rem;
    --poodle-action-discovery-chip-x: 0.375rem;
    --poodle-action-discovery-chip-font-size: 0.5625rem;
    --poodle-action-discovery-row-y: 0.25rem;
    --poodle-action-discovery-row-x: 0.5rem;
  }

  .poodle-action-discovery-panel[data-size="sm"] {
    --poodle-action-discovery-chip-height: 1.25rem;
    --poodle-action-discovery-chip-font-size: 0.625rem;
    --poodle-action-discovery-row-y: 0.3125rem;
    --poodle-action-discovery-row-x: 0.5rem;
  }

  .poodle-action-discovery-panel[data-size="lg"] {
    --poodle-action-discovery-chip-height: 1.5rem;
    --poodle-action-discovery-chip-x: 0.625rem;
    --poodle-action-discovery-chip-font-size: 0.75rem;
    --poodle-action-discovery-row-y: 0.5rem;
    --poodle-action-discovery-row-x: 0.75rem;
  }

  .poodle-action-discovery-panel[data-size="xl"] {
    --poodle-action-discovery-chip-height: 1.75rem;
    --poodle-action-discovery-chip-x: 0.75rem;
    --poodle-action-discovery-chip-font-size: 0.8125rem;
    --poodle-action-discovery-row-y: 0.625rem;
    --poodle-action-discovery-row-x: 0.875rem;
  }

  .poodle-action-discovery-panel[data-density="compact"] {
    --poodle-action-discovery-stack-gap: 0.5rem;
    --poodle-action-discovery-group-gap: 0.25rem;
    --poodle-action-discovery-list-gap: 0.1875rem;
    --poodle-action-discovery-chip-gap: 0.25rem;
    --poodle-action-discovery-skeleton-pad: 0.625rem;
    --poodle-action-discovery-row-y: 0.25rem;
    --poodle-action-discovery-row-x: 0.5rem;
  }

  .poodle-action-discovery-panel[data-density="comfortable"] {
    --poodle-action-discovery-stack-gap: 0.875rem;
    --poodle-action-discovery-group-gap: 0.5rem;
    --poodle-action-discovery-list-gap: 0.375rem;
    --poodle-action-discovery-chip-gap: 0.5rem;
    --poodle-action-discovery-skeleton-pad: 1rem;
    --poodle-action-discovery-row-y: 0.5rem;
    --poodle-action-discovery-row-x: 0.875rem;
  }

  .poodle-action-discovery-panel__group {
    display: grid;
    gap: var(--poodle-action-discovery-group-gap);
  }

  .poodle-action-discovery-panel__list {
    display: grid;
    gap: var(--poodle-action-discovery-list-gap);
    margin: 0;
    padding: 0;
    list-style: none;
  }

  .poodle-action-discovery-panel__list li[aria-selected="true"] :global(.poodle-list-card) {
    border-color: transparent;
    background: color-mix(in srgb, var(--poodle-color-accent-base) 18%, var(--poodle-color-background-elevated));
    box-shadow: inset 0 0 0 0.0625rem color-mix(in srgb, var(--poodle-color-accent-base) 22%, transparent);
  }

  .poodle-action-discovery-panel__trailing {
    display: flex;
    flex-wrap: wrap;
    gap: var(--poodle-action-discovery-chip-gap);
    align-items: center;
  }

  .poodle-action-discovery-panel__badge,
  .poodle-action-discovery-panel__kbd {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-height: var(--poodle-action-discovery-chip-height);
    padding: 0 var(--poodle-action-discovery-chip-x);
    border-radius: var(--poodle-radius-control);
    background: color-mix(in srgb, var(--poodle-color-background-surface) 76%, transparent);
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-label-family);
    font-size: var(--poodle-action-discovery-chip-font-size);
    font-weight: 600;
    letter-spacing: 0.03em;
    text-transform: uppercase;
    white-space: nowrap;
  }

  .poodle-action-discovery-panel__badge {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 16%, transparent);
    color: var(--poodle-color-accent-base);
  }

  .poodle-action-discovery-panel__kbd {
    font-family: var(--poodle-typography-code-family);
    letter-spacing: 0;
    text-transform: none;
  }

  .poodle-action-discovery-panel__state {
    min-height: 10rem;
    display: grid;
    place-items: center;
  }

  .poodle-action-discovery-panel__skeletons {
    display: grid;
    gap: var(--poodle-action-discovery-list-gap);
    width: 100%;
    padding: var(--poodle-action-discovery-skeleton-pad);
  }

  .poodle-action-discovery-panel__skeleton-row {
    display: flex;
    justify-content: space-between;
    gap: var(--poodle-action-discovery-chip-gap);
  }
</style>
