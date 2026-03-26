<script lang="ts">
  import { afterUpdate, tick } from "svelte";

  import {
    Icon,
    UiPresentationProvider,
    getUiPresentation,
    resolveSemanticControlSize,
  } from "@poodle/svelte-primitives";
  import type {
    ControlDensity,
    ControlSize,
    SemanticControlSizeRole,
  } from "@poodle/svelte-primitives";

  import type { LogEntry, LogLevel } from "./types";

  export let entries: LogEntry[] = [];
  export let maxEntries: number = 500;
  export let autoScroll = true;
  export let filterLevel: LogLevel | null = null;
  export let filterText = "";
  export let ariaLabel = "Log output";
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;

  let scrollContainer: HTMLDivElement | null = null;
  let isUserScrolled = false;
  const uiPresentation = getUiPresentation();

  $: resolvedSize = size ?? resolveSemanticControlSize(uiPresentation?.sizeScale ?? "md", sizeRole);
  $: resolvedDensity = density ?? uiPresentation?.density ?? "default";

  $: displayEntries = entries
    .filter((e) => !filterLevel || e.level === filterLevel)
    .filter(
      (e) => !filterText || e.message.toLowerCase().includes(filterText.toLowerCase()),
    )
    .slice(-maxEntries);

  $: levelCounts = {
    info: entries.filter((e) => e.level === "info").length,
    warn: entries.filter((e) => e.level === "warn").length,
    error: entries.filter((e) => e.level === "error").length,
  };

  afterUpdate(() => {
    if (autoScroll && !isUserScrolled && scrollContainer) {
      scrollContainer.scrollTop = scrollContainer.scrollHeight;
    }
  });

  function handleScroll(): void {
    if (!scrollContainer) return;
    const { scrollTop, scrollHeight, clientHeight } = scrollContainer;
    isUserScrolled = scrollHeight - scrollTop - clientHeight > 32;
  }

  function scrollToBottom(): void {
    isUserScrolled = false;
    tick().then(() => {
      if (scrollContainer) {
        scrollContainer.scrollTop = scrollContainer.scrollHeight;
      }
    });
  }

  function formatTimestamp(ts: Date | string | number): string {
    const date = ts instanceof Date ? ts : new Date(ts);
    return date.toLocaleTimeString("en-US", { hour12: false, fractionalSecondDigits: 3 });
  }
</script>

<UiPresentationProvider sizeScale={resolvedSize} density={resolvedDensity}>
  <div
    class="log-list"
    data-size={resolvedSize}
    data-density={resolvedDensity}
    role="log"
    aria-label={ariaLabel}
  >
    <div class="log-list__toolbar">
      <div class="log-list__filters">
        <button
          type="button"
          class="log-list__filter-btn"
          class:active={filterLevel === null}
          on:click={() => (filterLevel = null)}
        >
          All <span class="log-list__count">{entries.length}</span>
        </button>
        <button
          type="button"
          class="log-list__filter-btn log-list__filter-btn--info"
          class:active={filterLevel === "info"}
          on:click={() => (filterLevel = filterLevel === "info" ? null : "info")}
        >
          Info <span class="log-list__count">{levelCounts.info}</span>
        </button>
        <button
          type="button"
          class="log-list__filter-btn log-list__filter-btn--warn"
          class:active={filterLevel === "warn"}
          on:click={() => (filterLevel = filterLevel === "warn" ? null : "warn")}
        >
          Warn <span class="log-list__count">{levelCounts.warn}</span>
        </button>
        <button
          type="button"
          class="log-list__filter-btn log-list__filter-btn--error"
          class:active={filterLevel === "error"}
          on:click={() => (filterLevel = filterLevel === "error" ? null : "error")}
        >
          Error <span class="log-list__count">{levelCounts.error}</span>
        </button>
      </div>
      <input
        type="text"
        class="log-list__search"
        placeholder="Filter logs..."
        bind:value={filterText}
        aria-label="Filter log messages"
      />
    </div>

    <div
      class="log-list__scroll"
      bind:this={scrollContainer}
      on:scroll={handleScroll}
    >
      {#if displayEntries.length === 0}
        <div class="log-list__empty">No log entries{filterLevel || filterText ? " match filters" : ""}</div>
      {:else}
        {#each displayEntries as entry (entry.id ?? `${entry.timestamp}-${entry.message}`)}
          <div class="log-list__entry" data-level={entry.level}>
            <time class="log-list__ts">{formatTimestamp(entry.timestamp)}</time>
            <span class="log-list__level">{entry.level.toUpperCase()}</span>
            <span class="log-list__msg">{entry.message}</span>
          </div>
        {/each}
      {/if}
    </div>

    {#if isUserScrolled && autoScroll}
      <button
        type="button"
        class="log-list__scroll-btn"
        on:click={scrollToBottom}
        aria-label="Scroll to latest"
      >
        <Icon name="arrow-down" /> New entries
      </button>
    {/if}
  </div>
</UiPresentationProvider>

<style>
  .log-list {
    --poodle-log-list-toolbar-y: 0.375rem;
    --poodle-log-list-toolbar-x: 0.5rem;
    --poodle-log-list-filter-gap: 0.25rem;
    --poodle-log-list-filter-x: 0.5rem;
    --poodle-log-list-filter-y: 0.1875rem;
    --poodle-log-list-entry-x: 0.5rem;
    --poodle-log-list-entry-y: 0.125rem;
    --poodle-log-list-entry-gap: 0.625rem;
    --poodle-log-list-scroll-button-x: 0.75rem;
    --poodle-log-list-scroll-button-y: 0.25rem;
    display: flex;
    flex-direction: column;
    border: 0.0625rem solid var(--poodle-color-border-subtle);
    border-radius: var(--poodle-radius-surface);
    background: var(--poodle-color-background-panel);
    overflow: hidden;
    position: relative;
  }

  .log-list[data-size="xs"] {
    --poodle-log-list-filter-x: 0.375rem;
    --poodle-log-list-entry-gap: 0.5rem;
  }

  .log-list[data-size="lg"] {
    --poodle-log-list-filter-x: 0.625rem;
    --poodle-log-list-scroll-button-x: 0.875rem;
  }

  .log-list[data-size="xl"] {
    --poodle-log-list-filter-x: 0.75rem;
    --poodle-log-list-scroll-button-x: 1rem;
  }

  .log-list[data-density="compact"] {
    --poodle-log-list-toolbar-y: 0.25rem;
    --poodle-log-list-toolbar-x: 0.375rem;
    --poodle-log-list-filter-gap: 0.1875rem;
    --poodle-log-list-filter-y: 0.125rem;
    --poodle-log-list-entry-y: 0.0625rem;
    --poodle-log-list-scroll-button-y: 0.1875rem;
  }

  .log-list[data-density="comfortable"] {
    --poodle-log-list-toolbar-y: 0.5rem;
    --poodle-log-list-toolbar-x: 0.625rem;
    --poodle-log-list-filter-gap: 0.375rem;
    --poodle-log-list-filter-y: 0.25rem;
    --poodle-log-list-entry-y: 0.1875rem;
    --poodle-log-list-scroll-button-y: 0.3125rem;
  }

  .log-list__toolbar {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: var(--poodle-log-list-toolbar-y) var(--poodle-log-list-toolbar-x);
    border-bottom: 0.0625rem solid var(--poodle-color-border-subtle);
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 92%, transparent);
    flex-wrap: wrap;
  }

  .log-list__filters {
    display: flex;
    gap: var(--poodle-log-list-filter-gap);
  }

  .log-list__filter-btn {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
    min-height: var(--poodle-size-control-height);
    padding: var(--poodle-log-list-filter-y) var(--poodle-log-list-filter-x);
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-control);
    background: transparent;
    color: var(--poodle-color-text-secondary);
    cursor: pointer;
    font: inherit;
    font-size: var(--poodle-typography-label-size);
    font-family: var(--poodle-typography-code-family);
    line-height: 1;
    transition: background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .log-list__filter-btn:hover {
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 72%, transparent);
  }

  .log-list__filter-btn.active {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 16%, transparent);
    border-color: color-mix(in srgb, var(--poodle-color-accent-base) 42%, transparent);
    color: var(--poodle-color-text-primary);
  }

  .log-list__filter-btn--warn.active {
    background: color-mix(in srgb, var(--poodle-color-status-warning, #eab308) 16%, transparent);
    border-color: color-mix(in srgb, var(--poodle-color-status-warning, #eab308) 42%, transparent);
  }

  .log-list__filter-btn--error.active {
    background: color-mix(in srgb, var(--poodle-color-status-danger, #ef4444) 16%, transparent);
    border-color: color-mix(in srgb, var(--poodle-color-status-danger, #ef4444) 42%, transparent);
  }

  .log-list__count {
    opacity: 0.7;
    font-size: 0.625rem;
  }

  .log-list__search {
    flex: 1;
    min-width: 8rem;
    min-height: var(--poodle-size-control-height);
    padding: var(--poodle-log-list-filter-y) var(--poodle-log-list-filter-x);
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-control);
    background: var(--poodle-color-background-surface);
    color: var(--poodle-color-text-primary);
    font: inherit;
    font-size: var(--poodle-typography-label-size);
    font-family: var(--poodle-typography-code-family);
    outline: none;
  }

  .log-list__search:focus {
    border-color: var(--poodle-color-accent-focusRing);
  }

  .log-list__scroll {
    max-height: 20rem;
    overflow-y: auto;
    font-family: var(--poodle-typography-code-family);
    font-size: 0.75rem;
    line-height: 1.6;
  }

  .log-list__entry {
    display: flex;
    gap: var(--poodle-log-list-entry-gap);
    padding: var(--poodle-log-list-entry-y) var(--poodle-log-list-entry-x);
    border-bottom: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 42%, transparent);
  }

  .log-list__entry:hover {
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 42%, transparent);
  }

  .log-list__entry[data-level="warn"] {
    background: color-mix(in srgb, var(--poodle-color-status-warning, #eab308) 6%, transparent);
  }

  .log-list__entry[data-level="error"] {
    background: color-mix(in srgb, var(--poodle-color-status-danger, #ef4444) 8%, transparent);
  }

  .log-list__ts {
    color: var(--poodle-color-text-tertiary);
    flex-shrink: 0;
    white-space: nowrap;
  }

  .log-list__level {
    flex-shrink: 0;
    width: 3rem;
    text-align: right;
    font-weight: 600;
  }

  .log-list__entry[data-level="info"] .log-list__level {
    color: var(--poodle-color-accent-base, #6366f1);
  }

  .log-list__entry[data-level="warn"] .log-list__level {
    color: var(--poodle-color-status-warning, #eab308);
  }

  .log-list__entry[data-level="error"] .log-list__level {
    color: var(--poodle-color-status-danger, #ef4444);
  }

  .log-list__msg {
    flex: 1;
    min-width: 0;
    word-break: break-word;
    color: var(--poodle-color-text-primary);
  }

  .log-list__empty {
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 4rem;
    color: var(--poodle-color-text-tertiary);
    font-size: 0.8125rem;
  }

  .log-list__scroll-btn {
    position: absolute;
    bottom: 0.5rem;
    left: 50%;
    transform: translateX(-50%);
    padding: var(--poodle-log-list-scroll-button-y) var(--poodle-log-list-scroll-button-x);
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: 999rem;
    background: var(--poodle-color-background-elevated);
    color: var(--poodle-color-accent-base);
    cursor: pointer;
    font: inherit;
    font-size: var(--poodle-typography-label-size);
    box-shadow: var(--poodle-elevation-overlay);
    transition: background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .log-list__scroll-btn:hover {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 12%, transparent);
  }
</style>
