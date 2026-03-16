<script lang="ts">
  import { createEventDispatcher, onDestroy, tick } from "svelte";

  import { Callout, Icon, SearchField, Skeleton } from "@pug/svelte-primitives";
  import { EmptyState } from "@pug/svelte-composites";

  import type { CommandActionItem, DiscoveryState } from "./types";

  export let open = false;
  export let title = "Command palette";
  export let description: string | null = null;
  export let query = "";
  export let items: CommandActionItem[] = [];
  export let state: DiscoveryState = "ready";
  export let ariaLabel: string | null = null;
  export let invocationHint: string | null = null;

  const dispatch = createEventDispatcher<{
    queryChange: { value: string };
    commandSelect: { id: string };
    requestClose: void;
    activeChange: { id: string | null };
  }>();

  let previousFocusedElement: HTMLElement | null = null;
  let activeId: string | null = null;
  let wasOpen = false;
  let previousHtmlOverflow = "";
  let previousBodyOverflow = "";
  const queryInputId = "command-palette-query";
  const statusId = "command-palette-status";
  let itemButtons: Array<HTMLButtonElement | null> = [];

  $: enabledItems = items.filter((item) => !item.isDisabled);
  $: groupedItems = items.reduce<Record<string, CommandActionItem[]>>((accumulator, item) => {
    const group = item.group ?? "Commands";
    accumulator[group] ??= [];
    accumulator[group].push(item);
    return accumulator;
  }, {});
  $: groupEntries = Object.entries(groupedItems);
  $: if (open && !wasOpen) {
    previousFocusedElement = document.activeElement instanceof HTMLElement ? document.activeElement : null;
    previousHtmlOverflow = document.documentElement.style.overflow;
    previousBodyOverflow = document.body.style.overflow;
    document.documentElement.style.overflow = "hidden";
    document.body.style.overflow = "hidden";
    wasOpen = true;
    queueMicrotask(async () => {
      await tick();
      const input = document.getElementById(queryInputId) as HTMLInputElement | null;
      input?.focus();
      if (enabledItems.length > 0) {
        activeId = enabledItems[0]?.id ?? null;
        dispatch("activeChange", { id: activeId });
      }
    });
  }
  $: if (!open && wasOpen) {
    wasOpen = false;
    activeId = null;
    document.documentElement.style.overflow = previousHtmlOverflow;
    document.body.style.overflow = previousBodyOverflow;
    previousFocusedElement?.focus();
  }
  $: if (open && enabledItems.length > 0 && (!activeId || !enabledItems.some((item) => item.id === activeId))) {
    activeId = enabledItems[0]?.id ?? null;
    dispatch("activeChange", { id: activeId });
  }
  $: if (open && enabledItems.length === 0 && activeId !== null) {
    activeId = null;
    dispatch("activeChange", { id: null });
  }
  $: activeIndex = enabledItems.findIndex((item) => item.id === activeId);
  $: activeItem = enabledItems.find((item) => item.id === activeId) ?? null;
  $: paletteStatus =
    state === "loading"
      ? "Loading commands."
      : state === "error"
        ? "Command palette unavailable."
        : state === "empty"
          ? "No commands are available in this workspace."
          : state === "no-results"
            ? `No commands match "${query}".`
            : `${enabledItems.length} command${enabledItems.length === 1 ? "" : "s"} available.${activeItem ? ` Active command: ${activeItem.title}.` : ""}`;
  $: if (open && activeIndex >= 0) {
    queueMicrotask(() => {
      itemButtons[activeIndex]?.scrollIntoView({ block: "nearest" });
    });
  }

  function requestClose(): void {
    dispatch("requestClose");
  }

  function setActive(id: string | null): void {
    activeId = id;
    dispatch("activeChange", { id });
  }

  function moveActive(step: 1 | -1): void {
    if (enabledItems.length === 0) {
      return;
    }

    const currentIndex = enabledItems.findIndex((item) => item.id === activeId);
    const nextIndex =
      currentIndex === -1
        ? 0
        : (currentIndex + step + enabledItems.length) % enabledItems.length;
    setActive(enabledItems[nextIndex]?.id ?? null);
  }

  function moveToBoundary(direction: "start" | "end"): void {
    if (enabledItems.length === 0) {
      return;
    }

    setActive(direction === "start" ? enabledItems[0]?.id ?? null : enabledItems[enabledItems.length - 1]?.id ?? null);
  }

  function trapFocus(event: KeyboardEvent): void {
    if (event.key !== "Tab") {
      return;
    }

    const focusableElements = Array.from(
      document.querySelectorAll<HTMLElement>(
        '.command-palette button:not([disabled]), .command-palette input:not([disabled]), .command-palette [tabindex]:not([tabindex="-1"])',
      ),
    ).filter((element) => !element.hasAttribute("disabled"));

    if (focusableElements.length === 0) {
      return;
    }

    const firstElement = focusableElements[0];
    const lastElement = focusableElements[focusableElements.length - 1];

    if (event.shiftKey && document.activeElement === firstElement) {
      event.preventDefault();
      lastElement.focus();
      return;
    }

    if (!event.shiftKey && document.activeElement === lastElement) {
      event.preventDefault();
      firstElement.focus();
    }
  }

  function handleKeydown(event: KeyboardEvent): void {
    if (!open) {
      return;
    }

    trapFocus(event);

    if (event.key === "Escape") {
      event.preventDefault();
      requestClose();
      return;
    }

    if (event.key === "ArrowDown") {
      event.preventDefault();
      moveActive(1);
      return;
    }

    if (event.key === "ArrowUp") {
      event.preventDefault();
      moveActive(-1);
      return;
    }

    if (event.key === "Home") {
      event.preventDefault();
      moveToBoundary("start");
      return;
    }

    if (event.key === "End") {
      event.preventDefault();
      moveToBoundary("end");
      return;
    }

    if (event.key === "Enter" && activeId) {
      event.preventDefault();
      dispatch("commandSelect", { id: activeId });
    }
  }

  onDestroy(() => {
    document.documentElement.style.overflow = previousHtmlOverflow;
    document.body.style.overflow = previousBodyOverflow;
    previousFocusedElement = null;
  });
</script>

<svelte:window on:keydown={handleKeydown} />

{#if open}
  <div class="command-palette__overlay" aria-hidden="true" on:click={requestClose}></div>
  <div
    class="command-palette"
    role="dialog"
    aria-modal="true"
    aria-label={ariaLabel ?? title}
    aria-describedby={description ? "command-palette-description" : undefined}
  >
    <div class="command-palette__header">
      <div>
        <h3>{title}</h3>
        {#if description}
          <p id="command-palette-description">{description}</p>
        {/if}
      </div>
      <div class="command-palette__meta">
        {#if invocationHint}
          <span class="command-palette__hint">{invocationHint}</span>
        {/if}
        <button type="button" class="command-palette__close" aria-label="Close command palette" on:click={requestClose}>
          <Icon name="x" size="sm" />
        </button>
      </div>
    </div>

    <div class="command-palette__query">
      <SearchField
        id={queryInputId}
        value={query}
        ariaLabel="Search commands"
        describedBy={statusId}
        placeholder="Search commands, panels, and actions"
        on:valueChange={(event) => dispatch("queryChange", event.detail)}
        on:clear={() => dispatch("queryChange", { value: "" })}
        on:cancel={requestClose}
        on:submit={() => {
          if (activeId) {
            dispatch("commandSelect", { id: activeId });
          }
        }}
      />
    </div>

    <p id={statusId} class="command-palette__status" role="status" aria-live="polite" aria-atomic="true">
      {paletteStatus}
    </p>

    <div class="command-palette__results">
      {#if state === "loading"}
        <div class="command-palette__state">
          <Callout
            tone="info"
            title="Loading commands"
            message="Keep the query visible while grouped actions and recent commands resolve."
          />
          <div class="command-palette__skeletons" aria-hidden="true">
            {#each Array.from({ length: 5 }) as _}
              <div class="command-palette__skeleton-row">
                <Skeleton width="48%" />
                <Skeleton width="20%" />
              </div>
            {/each}
          </div>
        </div>
      {:else if state === "error"}
        <div class="command-palette__state">
          <Callout
            tone="danger"
            title="Command palette unavailable"
            message="Retry or fallback actions should remain visible when command discovery fails."
          />
          <EmptyState
            title="Commands could not be loaded"
            message="The shell should remain open long enough for recovery actions and error context to stay legible."
          />
        </div>
      {:else if state === "empty"}
        <EmptyState
          title="No commands available in this workspace"
          message="Empty command scopes should remain explicit rather than rendering a blank palette."
        />
      {:else if state === "no-results"}
        <EmptyState
          title="No commands match this query"
          message="Keep the query visible so users can widen or clear the search immediately."
          variant="search"
        />
      {:else}
        <div class="command-palette__group-list" role="list">
          {#each groupEntries as [group, groupItems]}
            <section class="command-palette__group" aria-labelledby={`palette-group-${group}`}>
              <h4 id={`palette-group-${group}`}>{group}</h4>
              <ul>
                {#each groupItems as item (item.id)}
                  <li>
                    <button
                      bind:this={itemButtons[enabledItems.findIndex((entry) => entry.id === item.id)]}
                      type="button"
                      class="command-palette__item"
                      class:command-palette__item--active={activeId === item.id}
                      disabled={item.isDisabled}
                      aria-current={activeId === item.id ? "true" : undefined}
                      aria-keyshortcuts={item.shortcut ?? undefined}
                      on:mouseenter={() => setActive(item.id)}
                      on:focus={() => setActive(item.id)}
                      on:click={() => dispatch("commandSelect", { id: item.id })}
                    >
                      <span class="command-palette__copy">
                        <strong>{item.title}</strong>
                        {#if item.description}
                          <small>{item.description}</small>
                        {/if}
                      </span>
                      <span class="command-palette__trailing">
                        {#if item.badge}
                          <span class="command-palette__badge">{item.badge}</span>
                        {/if}
                        {#if item.shortcut}
                          <kbd>{item.shortcut}</kbd>
                        {/if}
                      </span>
                    </button>
                  </li>
                {/each}
              </ul>
            </section>
          {/each}
        </div>
      {/if}
    </div>
  </div>
{/if}

<style>
  .command-palette__overlay {
    position: fixed;
    inset: 0;
    background: color-mix(in srgb, black 44%, transparent);
    backdrop-filter: blur(0.5rem);
    z-index: 40;
  }

  .command-palette {
    position: fixed;
    top: 50%;
    left: 50%;
    display: grid;
    grid-template-rows: auto auto auto minmax(0, 1fr);
    gap: var(--pug-space-stack-md);
    width: min(45rem, calc(100vw - 2rem));
    max-height: min(78vh, 52.5rem);
    min-height: 0;
    padding: var(--pug-space-panel-y) var(--pug-space-panel-x);
    border: 0.0625rem solid color-mix(in srgb, var(--pug-color-border-default) 42%, transparent);
    border-radius: calc(var(--pug-radius-surface) + 0.125rem);
    background: color-mix(in srgb, var(--pug-color-background-elevated) 98%, transparent);
    box-shadow: var(--pug-elevation-dialog);
    overflow: hidden;
    overscroll-behavior: contain;
    transform: translate(-50%, -50%);
    z-index: 41;
  }

  .command-palette__header {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    gap: var(--pug-space-inline-md);
    align-items: start;
  }

  .command-palette__header h3,
  .command-palette__header p {
    margin: 0;
  }

  .command-palette__header h3 {
    font-size: 1.375rem;
    line-height: 1.2;
  }

  .command-palette__header p {
    color: var(--pug-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.5;
  }

  .command-palette__meta {
    display: flex;
    gap: var(--pug-space-inline-sm);
    align-items: center;
  }

  .command-palette__hint,
  .command-palette__badge,
  .command-palette__item kbd {
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

  .command-palette__hint,
  .command-palette__item kbd {
    font-family: var(--pug-typography-code-family);
  }

  .command-palette__close {
    width: 1.75rem;
    height: 1.75rem;
    padding: 0;
    border: 0;
    border-radius: calc(var(--pug-radius-control) - 0.0625rem);
    background: color-mix(in srgb, var(--pug-color-background-surface) 62%, transparent);
    color: var(--pug-color-text-secondary);
    cursor: pointer;
    font: inherit;
  }

  .command-palette__close:hover {
    background: color-mix(in srgb, var(--pug-color-background-surface) 84%, transparent);
    color: var(--pug-color-text-primary);
  }

  .command-palette__results {
    display: grid;
    gap: var(--pug-space-stack-md);
    min-height: 0;
    overflow: auto;
    overscroll-behavior: contain;
  }

  .command-palette__status {
    margin: 0;
    color: var(--pug-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.5;
  }

  .command-palette__state,
  .command-palette__group-list,
  .command-palette__group,
  .command-palette__copy,
  .command-palette__skeletons {
    display: grid;
    gap: var(--pug-space-stack-sm);
  }

  .command-palette__group-list {
    min-height: 0;
    align-content: start;
    gap: 0.75rem;
  }

  .command-palette__group h4,
  .command-palette__copy strong,
  .command-palette__copy small {
    margin: 0;
  }

  .command-palette__group h4 {
    color: var(--pug-color-text-secondary);
    font-size: 0.75rem;
    font-weight: 600;
    letter-spacing: 0.08em;
    text-transform: uppercase;
  }

  .command-palette__group ul {
    display: grid;
    gap: 0.375rem;
    margin: 0;
    padding: 0;
    list-style: none;
  }

  .command-palette__item {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    gap: var(--pug-space-inline-md);
    align-items: center;
    width: 100%;
    padding: 0.6875rem 0.75rem;
    border: 0.0625rem solid transparent;
    border-radius: calc(var(--pug-radius-surface) - 0.125rem);
    background: color-mix(in srgb, var(--pug-color-background-panel) 92%, transparent);
    color: var(--pug-color-text-primary);
    text-align: left;
    cursor: pointer;
    font: inherit;
  }

  .command-palette__copy {
    gap: 0.1875rem;
  }

  .command-palette__copy strong {
    font-size: 0.875rem;
    line-height: 1.25;
  }

  .command-palette__item--active {
    border-color: transparent;
    background: color-mix(in srgb, var(--pug-color-accent-base) 18%, var(--pug-color-background-elevated));
    box-shadow: inset 0 0 0 0.0625rem color-mix(in srgb, var(--pug-color-accent-base) 22%, transparent);
  }

  .command-palette__item:focus-visible {
    outline: var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .command-palette__item:disabled {
    opacity: var(--pug-state-opacity-disabled);
    cursor: not-allowed;
  }

  .command-palette__copy small {
    color: var(--pug-color-text-secondary);
    font-size: 0.6875rem;
    line-height: 1.35;
  }

  .command-palette__trailing {
    display: flex;
    flex-wrap: wrap;
    gap: 0.375rem;
    align-items: center;
    justify-content: flex-end;
  }

  .command-palette__skeleton-row {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    gap: var(--pug-space-inline-md);
    padding: 0.875rem;
    border: 0.0625rem solid transparent;
    border-radius: calc(var(--pug-radius-surface) - 0.125rem);
    background: color-mix(in srgb, var(--pug-color-background-surface) 72%, transparent);
  }

  :global([data-theme="light"]) .command-palette {
    border-color: color-mix(in srgb, var(--pug-color-border-default) 24%, transparent);
    box-shadow:
      0 1.125rem 2.75rem rgba(49, 66, 85, 0.1),
      inset 0 0.0625rem 0 rgba(255, 255, 255, 0.72);
  }

  :global([data-theme="light"]) .command-palette__item,
  :global([data-theme="light"]) .command-palette__skeleton-row {
    background: color-mix(in srgb, var(--pug-color-background-elevated) 96%, var(--pug-color-background-panel));
    box-shadow: inset 0 0 0 0.0625rem color-mix(in srgb, var(--pug-color-border-subtle) 36%, transparent);
  }

  @media (max-width: 45rem) {
    .command-palette {
      width: min(100vw - 1.25rem, 45rem);
      max-height: calc(100vh - 1.25rem);
      padding: 1rem;
    }

    .command-palette__header,
    .command-palette__item,
    .command-palette__skeleton-row {
      grid-template-columns: 1fr;
    }

    .command-palette__trailing,
    .command-palette__meta {
      justify-content: flex-start;
    }
  }
</style>
