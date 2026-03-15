<script lang="ts">
  import { onMount, afterUpdate, tick } from "svelte";

  import { Icon } from "@pug/svelte-primitives";

  import type { LogEntry, LogLevel } from "./types";

  export let entries: LogEntry[] = [];
  export let maxEntries: number = 500;
  export let autoScroll = true;
  export let filterLevel: LogLevel | null = null;
  export let filterText = "";
  export let ariaLabel = "Log output";

  let scrollContainer: HTMLDivElement | null = null;
  let isUserScrolled = false;

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

<div class="log-list" role="log" aria-label={ariaLabel}>
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
      <Icon name="arrow-down" size="sm" /> New entries
    </button>
  {/if}
</div>

<style>
  .log-list {
    display: flex;
    flex-direction: column;
    border: 0.0625rem solid var(--pug-color-border-subtle);
    border-radius: var(--pug-radius-surface);
    background: var(--pug-color-background-panel);
    overflow: hidden;
    position: relative;
  }

  .log-list__toolbar {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.375rem 0.5rem;
    border-bottom: 0.0625rem solid var(--pug-color-border-subtle);
    background: color-mix(in srgb, var(--pug-color-background-elevated) 92%, transparent);
    flex-wrap: wrap;
  }

  .log-list__filters {
    display: flex;
    gap: 0.25rem;
  }

  .log-list__filter-btn {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
    padding: 0.1875rem 0.5rem;
    border: 0.0625rem solid var(--pug-color-border-default);
    border-radius: var(--pug-radius-control);
    background: transparent;
    color: var(--pug-color-text-secondary);
    cursor: pointer;
    font: inherit;
    font-size: 0.6875rem;
    font-family: var(--pug-typography-code-family);
    line-height: 1;
    transition: background var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard);
  }

  .log-list__filter-btn:hover {
    background: color-mix(in srgb, var(--pug-color-background-elevated) 72%, transparent);
  }

  .log-list__filter-btn.active {
    background: color-mix(in srgb, var(--pug-color-accent-base) 16%, transparent);
    border-color: color-mix(in srgb, var(--pug-color-accent-base) 42%, transparent);
    color: var(--pug-color-text-primary);
  }

  .log-list__filter-btn--warn.active {
    background: color-mix(in srgb, var(--pug-color-status-warning, #eab308) 16%, transparent);
    border-color: color-mix(in srgb, var(--pug-color-status-warning, #eab308) 42%, transparent);
  }

  .log-list__filter-btn--error.active {
    background: color-mix(in srgb, var(--pug-color-status-danger, #ef4444) 16%, transparent);
    border-color: color-mix(in srgb, var(--pug-color-status-danger, #ef4444) 42%, transparent);
  }

  .log-list__count {
    opacity: 0.7;
    font-size: 0.625rem;
  }

  .log-list__search {
    flex: 1;
    min-width: 8rem;
    padding: 0.1875rem 0.5rem;
    border: 0.0625rem solid var(--pug-color-border-default);
    border-radius: var(--pug-radius-control);
    background: var(--pug-color-background-surface);
    color: var(--pug-color-text-primary);
    font: inherit;
    font-size: 0.6875rem;
    font-family: var(--pug-typography-code-family);
    outline: none;
  }

  .log-list__search:focus {
    border-color: var(--pug-color-accent-focusRing);
  }

  .log-list__scroll {
    max-height: 20rem;
    overflow-y: auto;
    font-family: var(--pug-typography-code-family);
    font-size: 0.75rem;
    line-height: 1.6;
  }

  .log-list__entry {
    display: flex;
    gap: 0.625rem;
    padding: 0.125rem 0.5rem;
    border-bottom: 0.0625rem solid color-mix(in srgb, var(--pug-color-border-subtle) 42%, transparent);
  }

  .log-list__entry:hover {
    background: color-mix(in srgb, var(--pug-color-background-elevated) 42%, transparent);
  }

  .log-list__entry[data-level="warn"] {
    background: color-mix(in srgb, var(--pug-color-status-warning, #eab308) 6%, transparent);
  }

  .log-list__entry[data-level="error"] {
    background: color-mix(in srgb, var(--pug-color-status-danger, #ef4444) 8%, transparent);
  }

  .log-list__ts {
    color: var(--pug-color-text-tertiary);
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
    color: var(--pug-color-accent-base, #6366f1);
  }

  .log-list__entry[data-level="warn"] .log-list__level {
    color: var(--pug-color-status-warning, #eab308);
  }

  .log-list__entry[data-level="error"] .log-list__level {
    color: var(--pug-color-status-danger, #ef4444);
  }

  .log-list__msg {
    flex: 1;
    min-width: 0;
    word-break: break-word;
    color: var(--pug-color-text-primary);
  }

  .log-list__empty {
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 4rem;
    color: var(--pug-color-text-tertiary);
    font-size: 0.8125rem;
  }

  .log-list__scroll-btn {
    position: absolute;
    bottom: 0.5rem;
    left: 50%;
    transform: translateX(-50%);
    padding: 0.25rem 0.75rem;
    border: 0.0625rem solid var(--pug-color-border-default);
    border-radius: 999rem;
    background: var(--pug-color-background-elevated);
    color: var(--pug-color-accent-base);
    cursor: pointer;
    font: inherit;
    font-size: 0.6875rem;
    box-shadow: var(--pug-elevation-overlay);
    transition: background var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard);
  }

  .log-list__scroll-btn:hover {
    background: color-mix(in srgb, var(--pug-color-accent-base) 12%, transparent);
  }
</style>
