<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { Eyebrow, ListCard, Skeleton } from "@pug/svelte-primitives";
  import EmptyState from "./EmptyState.svelte";

  import type { CommandActionItem, DiscoveryState } from "./types";

  export let items: CommandActionItem[] = [];
  export let state: DiscoveryState = "ready";
  export let activeId: string | null = null;
  export let ariaLabel = "Actions";

  const dispatch = createEventDispatcher<{
    itemSelect: { id: string };
    activeChange: { id: string | null };
  }>();

  let itemElements: Array<HTMLElement | null> = [];

  $: enabledItems = items.filter((item) => !item.isDisabled);
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

<div class="action-discovery-panel" role="listbox" aria-label={ariaLabel}>
  {#if state === "loading"}
    <div class="action-discovery-panel__state">
      <div class="action-discovery-panel__skeletons" aria-hidden="true">
        {#each Array.from({ length: 5 }) as _}
          <div class="action-discovery-panel__skeleton-row">
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
      <div class="action-discovery-panel__group">
        <Eyebrow>{group}</Eyebrow>
        <ul class="action-discovery-panel__list">
          {#each groupItems as item (item.id)}
            <li
              bind:this={itemElements[enabledItems.findIndex((e) => e.id === item.id)]}
              role="option"
              aria-selected={activeId === item.id}
            >
              <ListCard
                title={item.title}
                subtitle={item.description}
                isInteractive={!item.isDisabled}
                isDisabled={item.isDisabled ?? false}
                ariaLabel={item.title}
                on:click={() => dispatch("itemSelect", { id: item.id })}
                on:mouseenter={() => setActive(item.id)}
                on:focus={() => setActive(item.id)}
              >
                <svelte:fragment slot="trailing">
                  <span class="action-discovery-panel__trailing">
                    {#if item.badge}
                      <span class="action-discovery-panel__badge">{item.badge}</span>
                    {/if}
                    {#if item.shortcut}
                      <kbd class="action-discovery-panel__kbd">{item.shortcut}</kbd>
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

<style>
  .action-discovery-panel {
    display: grid;
    gap: 0.75rem;
    min-height: 0;
    overflow: auto;
    overscroll-behavior: contain;
  }

  .action-discovery-panel__group {
    display: grid;
    gap: 0.375rem;
  }

  .action-discovery-panel__list {
    display: grid;
    gap: 0.25rem;
    margin: 0;
    padding: 0;
    list-style: none;
  }

  .action-discovery-panel__list li[aria-selected="true"] :global(.list-card) {
    border-color: transparent;
    background: color-mix(in srgb, var(--pug-color-accent-base) 18%, var(--pug-color-background-elevated));
    box-shadow: inset 0 0 0 0.0625rem color-mix(in srgb, var(--pug-color-accent-base) 22%, transparent);
  }

  .action-discovery-panel__trailing {
    display: flex;
    flex-wrap: wrap;
    gap: 0.375rem;
    align-items: center;
  }

  .action-discovery-panel__badge,
  .action-discovery-panel__kbd {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-height: 1.5rem;
    padding: 0 0.5rem;
    border-radius: var(--pug-radius-control);
    background: color-mix(in srgb, var(--pug-color-background-surface) 76%, transparent);
    color: var(--pug-color-text-secondary);
    font-size: 0.75rem;
  }

  .action-discovery-panel__kbd {
    font-family: var(--pug-typography-code-family);
  }

  .action-discovery-panel__state {
    display: grid;
    gap: var(--pug-space-stack-sm);
  }

  .action-discovery-panel__skeletons {
    display: grid;
    gap: var(--pug-space-stack-sm);
  }

  .action-discovery-panel__skeleton-row {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    gap: var(--pug-space-inline-md);
    padding: 0.875rem;
    border-radius: calc(var(--pug-radius-surface) - 0.125rem);
    background: color-mix(in srgb, var(--pug-color-background-surface) 72%, transparent);
  }
</style>
