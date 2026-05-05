<script lang="ts">
  import { createEventDispatcher } from "svelte";

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

  export let items: CommandActionItem[] = [];
  export let state: DiscoveryState = "ready";
  export let activeId: string | null = null;
  export let ariaLabel = "Actions";
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;

  const dispatch = createEventDispatcher<{
    itemSelect: { id: string };
    activeChange: { id: string | null };
  }>();

  let itemElements: Array<HTMLElement | null> = [];
  const uiPresentation = getUiPresentation();

  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;

  $: enabledItems = items.filter((item) => !item.disabled);
  $: groupedItems = items.reduce<Record<string, CommandActionItem[]>>((acc, item) => {
    const group = item.group ?? "Commands";
    acc[group] ??= [];
    acc[group].push(item);
    return acc;
  }, {});
  $: groupEntries = Object.entries(groupedItems);

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
    if (activeId) dispatch("itemSelect", { id: activeId });
  }

  export function getEnabledItems(): CommandActionItem[] {
    return enabledItems;
  }

  function setActive(id: string | null): void {
    activeId = id;
    dispatch("activeChange", { id });

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
    {#if state === "loading"}
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
    {:else if state === "error"}
      <EmptyState
        title="Could not load actions"
        message="Actions could not be loaded. Try again."
      />
    {:else if state === "empty"}
      <EmptyState
        title="No actions available"
        message="No actions are available in this context."
      />
    {:else if state === "no-results"}
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
                bind:this={itemElements[enabledItems.findIndex((e) => e.id === item.id)]}
                role="option"
                aria-selected={activeId === item.id}
              >
                <ListCard
                  title={item.title}
                  subtitle={item.description}
                  interactive={!item.disabled}
                  disabled={item.disabled ?? false}
                  ariaLabel={item.title}
                  on:click={() => dispatch("itemSelect", { id: item.id })}
                  on:mouseenter={() => setActive(item.id)}
                  on:focus={() => setActive(item.id)}
                >
                  <svelte:fragment slot="trailing">
                    <span class="poodle-action-discovery-panel__trailing">
                      {#if item.badge}
                        <span class="poodle-action-discovery-panel__badge">{item.badge}</span>
                      {/if}
                      {#if item.shortcut}
                        <kbd class="poodle-action-discovery-panel__kbd">{item.shortcut}</kbd>
                      {/if}
                    </span>
                  </svelte:fragment>
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
    font-size: var(--poodle-action-discovery-chip-font-size);
  }

  .poodle-action-discovery-panel__kbd {
    font-family: var(--poodle-typography-code-family);
  }

  .poodle-action-discovery-panel__state {
    display: grid;
    gap: var(--poodle-space-stack-sm);
  }

  .poodle-action-discovery-panel__skeletons {
    display: grid;
    gap: var(--poodle-space-stack-sm);
  }

  .poodle-action-discovery-panel__skeleton-row {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    gap: var(--poodle-space-inline-md);
    padding: var(--poodle-action-discovery-skeleton-pad);
    border-radius: calc(var(--poodle-radius-surface) - 0.125rem);
    background: color-mix(in srgb, var(--poodle-color-background-surface) 72%, transparent);
  }
</style>
