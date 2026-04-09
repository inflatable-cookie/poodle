<script lang="ts">
  import { Icon, CollapseToggle, getUiPresentation, resolveSemanticControlSize } from "@poodle/svelte-primitives";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "@poodle/svelte-primitives";

  export let ariaLabel = "Filters";
  export let summaryText: string | null = null;
  export let collapsible = true;
  export let collapsed = true;
  export let columns = 4;
  export let minItemWidth = "10rem";
  export let sticky = false;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "chrome";
  export let density: ControlDensity | null = null;

  const uiPresentation = getUiPresentation();

  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;

  function handleHeaderClick(e: MouseEvent) {
    if (!collapsible) return;
    const target = e.target as HTMLElement;
    if (target.closest('.filter-toolbar__actions') || target.closest('.collapse-toggle')) return;
    collapsed = !collapsed;
  }
</script>

<div
  class="filter-toolbar"
  data-sticky={sticky}
  data-collapsed={collapsible && collapsed}
  data-size={resolvedSize}
  data-density={resolvedDensity}
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
        collapsed={collapsed}
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
    <!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
    <div
      class="filter-toolbar__header"
      class:filter-toolbar__header--clickable={collapsible}
      on:click={handleHeaderClick}
    >
      {#if collapsible}
        <CollapseToggle
          collapsed={collapsed}
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
    gap: var(--poodle-space-stack-sm);
    margin-block-end: var(--poodle-space-stack-md);
    padding: var(--poodle-space-panel-y) var(--poodle-space-panel-x);
    border: 0.0625rem solid var(--poodle-color-border-subtle);
    border-radius: var(--poodle-radius-surface);
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 92%, transparent);
  }

  .filter-toolbar[data-sticky="true"] {
    box-shadow: var(--poodle-elevation-surface);
  }

  .filter-toolbar__header {
    display: flex;
    align-items: center;
    gap: var(--poodle-space-inline-sm);
  }

  .filter-toolbar__header--clickable {
    cursor: pointer;
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
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .filter-toolbar__summary {
    margin: 0;
    flex: 1;
    color: var(--poodle-color-text-secondary);
    font-size: var(--poodle-typography-label-size, 0.75rem);
    line-height: var(--poodle-typography-label-lineHeight, 1.4);
  }

  .filter-toolbar__actions {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    margin-left: auto;
  }

  .filter-toolbar__controls {
    display: grid;
    grid-template-columns: repeat(
      var(--ft-columns, 4),
      minmax(var(--ft-min-width, 10rem), 1fr)
    );
    gap: var(--poodle-space-inline-sm);
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
    gap: var(--poodle-space-inline-sm);
    align-items: center;
  }

  /* ── Size variants ──────────────────────────────────────────── */

  .filter-toolbar[data-size="xs"] .filter-toolbar__summary {
    font-size: 0.6875rem;
  }

  .filter-toolbar[data-size="sm"] .filter-toolbar__summary {
    font-size: 0.71875rem;
  }

  .filter-toolbar[data-size="lg"] .filter-toolbar__summary {
    font-size: 0.8125rem;
  }

  .filter-toolbar[data-size="xl"] .filter-toolbar__summary {
    font-size: 0.875rem;
  }

  /* Density variants */
  .filter-toolbar[data-density="compact"] { gap: 0.25rem; padding: 0.25rem; }
  .filter-toolbar[data-density="compact"] .filter-toolbar__controls { gap: 0.25rem; }
  .filter-toolbar[data-density="comfortable"] { gap: var(--poodle-space-inline-md); padding: 0.5rem; }
  .filter-toolbar[data-density="comfortable"] .filter-toolbar__controls { gap: var(--poodle-space-inline-md); }
</style>
