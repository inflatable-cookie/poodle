<script lang="ts">
  import "@poodle/styles/action-discovery-panel.css";
  import { default as Eyebrow } from "./Eyebrow.svelte";
  import { default as ListCard } from "./ListCard.svelte";
  import { default as Skeleton } from "./Skeleton.svelte";
  import { default as UiPresentationProvider } from "./UiPresentationProvider.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type {
    ControlDensity,
    ControlSize,
    SemanticControlSizeRole,
  } from "./types";
  import { default as EmptyState } from "./EmptyState.svelte";

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
                onmouseenter={() => setActive(item.id)}
                onfocus={() => setActive(item.id)}
              >
                <ListCard
                  title={item.title}
                  subtitle={item.description}
                  interactive={!item.disabled}
                  disabled={item.disabled ?? false}
                  ariaLabel={item.title}
                  onClick={() => onItemSelect?.(item.id)}
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

