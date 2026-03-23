<script lang="ts">
  import { Icon, CollapseToggle } from "@pug/svelte-primitives";

  export let ariaLabel = "Filters";
  export let summaryText: string | null = null;
  export let collapsible = false;
  export let collapsed = false;
  export let columns = 4;
  export let minItemWidth = "10rem";
  export let isSticky = false;

  function handleHeaderClick(e: MouseEvent) {
    if (!collapsible || !collapsed) return;
    const target = e.target as HTMLElement;
    if (target.closest('.filter-toolbar__actions') || target.closest('.collapse-toggle')) return;
    collapsed = false;
  }
</script>

<div
  class="filter-toolbar"
  data-sticky={isSticky}
  data-collapsed={collapsible && collapsed}
  role="toolbar"
  aria-label={ariaLabel}
>
  {#if collapsible && collapsed}
    <button
      type="button"
      class="filter-toolbar__header filter-toolbar__header--button"
      on:click={handleHeaderClick}
      aria-expanded="false"
      aria-label={summaryText ? `Show filters. ${summaryText}` : "Show filters"}
    >
      <CollapseToggle
        isCollapsed={collapsed}
        ariaLabel="Show filters"
        on:toggle={(e) => (collapsed = e.detail.isCollapsed)}
      />

      {#if summaryText}
        <span class="filter-toolbar__summary">{summaryText}</span>
      {/if}

      {#if $$slots.actions}
        <span class="filter-toolbar__actions">
          <slot name="actions" />
        </span>
      {/if}
    </button>
  {:else}
    <div class="filter-toolbar__header">
      {#if collapsible}
        <CollapseToggle
          isCollapsed={collapsed}
          ariaLabel={collapsed ? "Show filters" : "Hide filters"}
          on:toggle={(e) => (collapsed = e.detail.isCollapsed)}
        />
      {/if}

      {#if summaryText}
        <p class="filter-toolbar__summary">{summaryText}</p>
      {/if}

      {#if $$slots.actions}
        <div class="filter-toolbar__actions">
          <slot name="actions" />
        </div>
      {/if}
    </div>
  {/if}

  {#if !collapsible || !collapsed}
    <div
      class="filter-toolbar__controls"
      style:--ft-columns={columns}
      style:--ft-min-width={minItemWidth}
    >
      <slot />
    </div>
  {/if}

  {#if $$slots.secondary}
    <div class="filter-toolbar__secondary">
      <slot name="secondary" />
    </div>
  {/if}
</div>

<style>
  .filter-toolbar {
    display: grid;
    gap: var(--pug-space-stack-sm);
    padding: var(--pug-space-panel-y) var(--pug-space-panel-x);
    border: 0.0625rem solid var(--pug-color-border-subtle);
    border-radius: var(--pug-radius-surface);
    background: color-mix(in srgb, var(--pug-color-background-elevated) 92%, transparent);
  }

  .filter-toolbar[data-sticky="true"] {
    box-shadow: var(--pug-elevation-surface);
  }

  .filter-toolbar__header {
    display: flex;
    align-items: center;
    gap: var(--pug-space-inline-sm);
  }

  .filter-toolbar__header--button {
    width: 100%;
    padding: 0;
    border: 0;
    background: transparent;
    color: inherit;
    text-align: left;
    cursor: pointer;
  }

  .filter-toolbar__header--button:focus-visible {
    outline: var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .filter-toolbar__summary {
    margin: 0;
    flex: 1;
    color: var(--pug-color-text-secondary);
    font-size: var(--pug-typography-label-size, 0.75rem);
    line-height: var(--pug-typography-label-lineHeight, 1.4);
  }

  .filter-toolbar__actions {
    display: flex;
    align-items: center;
    gap: var(--pug-space-inline-xs, 0.25rem);
    margin-left: auto;
  }

  .filter-toolbar__controls {
    display: grid;
    grid-template-columns: repeat(
      var(--ft-columns, 4),
      minmax(var(--ft-min-width, 10rem), 1fr)
    );
    gap: var(--pug-space-inline-sm);
    align-items: end;
  }

  @media (max-width: 960px) {
    .filter-toolbar__controls {
      grid-template-columns: repeat(2, minmax(var(--ft-min-width, 10rem), 1fr));
    }
  }

  @media (max-width: 640px) {
    .filter-toolbar__controls {
      grid-template-columns: 1fr;
    }
  }

  .filter-toolbar__secondary {
    display: flex;
    flex-wrap: wrap;
    gap: var(--pug-space-inline-sm);
    align-items: center;
  }
</style>
