<script lang="ts">
  import type { Snippet } from "svelte";

  import CollapseToggle from "./CollapseToggle.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  interface Props {
    ariaLabel?: string;
    summaryText?: string | null;
    collapsible?: boolean;
    collapsed?: boolean;
    columns?: number;
    minItemWidth?: string;
    sticky?: boolean;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    children?: Snippet<[]>;
    summary?: Snippet<[]>;
    actions?: Snippet<[]>;
    secondary?: Snippet<[]>;
  }

  let {
    ariaLabel = "Filters",
    summaryText = null,
    collapsible = true,
    collapsed = $bindable(false),
    columns = 4,
    minItemWidth = "10rem",
    sticky = false,
    size = null,
    sizeRole = "chrome",
    density = null,
    children,
    summary,
    actions,
    secondary,
  }: Props = $props();

  const uiPresentation = getUiPresentation();

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);

  function handleHeaderClick(e: MouseEvent): void {
    if (!collapsible) return;

    const target = e.target as HTMLElement;
    if (target.closest(".poodle-filter-toolbar__actions") || target.closest(".poodle-collapse-toggle")) return;

    collapsed = !collapsed;
  }

</script>

<div
  class="poodle-filter-toolbar"
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
      class="poodle-filter-toolbar__header poodle-filter-toolbar__header--button"
      onclick={handleHeaderClick}
      aria-expanded="false"
      aria-label={summaryText ? `Show filters. ${summaryText}` : "Show filters"}
    >
      <CollapseToggle
        {collapsed}
        ariaLabel="Show filters"
        onToggle={(isCollapsed) => (collapsed = isCollapsed)}
      />

      {#if summary}
        <span class="poodle-filter-toolbar__summary">
          {@render summary()}
        </span>
      {:else if summaryText}
        <span class="poodle-filter-toolbar__summary">{summaryText}</span>
      {/if}

      {#if actions}
        <span class="poodle-filter-toolbar__actions">
          {@render actions()}
        </span>
      {/if}
    </button>
  {:else}
    {#if collapsible}
      <button
        type="button"
        class="poodle-filter-toolbar__header poodle-filter-toolbar__header--button poodle-filter-toolbar__header--clickable"
        aria-expanded={String(!collapsed)}
        onclick={handleHeaderClick}
      >
        <CollapseToggle
          {collapsed}
          ariaLabel={collapsed ? "Show filters" : "Hide filters"}
          onToggle={(isCollapsed) => (collapsed = isCollapsed)}
        />

        {#if summary}
          <span class="poodle-filter-toolbar__summary">
            {@render summary()}
          </span>
        {:else if summaryText}
          <span class="poodle-filter-toolbar__summary">{summaryText}</span>
        {/if}

        {#if actions}
          <span class="poodle-filter-toolbar__actions">
            {@render actions()}
          </span>
        {/if}
      </button>
    {:else}
      <div class="poodle-filter-toolbar__header">
        {#if summary}
          <div class="poodle-filter-toolbar__summary">
            {@render summary()}
          </div>
        {:else if summaryText}
          <p class="poodle-filter-toolbar__summary">{summaryText}</p>
        {/if}

        {#if actions}
          <div class="poodle-filter-toolbar__actions">
            {@render actions()}
          </div>
        {/if}
      </div>
    {/if}
  {/if}

  {#if !collapsible || !collapsed}
    <div
      class="poodle-filter-toolbar__controls"
      style:--ft-columns={columns}
      style:--ft-min-width={minItemWidth}
    >
      {@render children?.()}
    </div>
  {/if}

  {#if secondary}
    <div class="poodle-filter-toolbar__secondary">
      {@render secondary()}
    </div>
  {/if}
</div>

<style>
  .poodle-filter-toolbar {
    display: grid;
    gap: var(--poodle-space-stack-sm);
    padding: var(--poodle-space-panel-y) var(--poodle-space-panel-x);
    border: 0.0625rem solid var(--poodle-color-border-subtle);
    border-radius: var(--poodle-radius-surface);
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 92%, transparent);
  }

  .poodle-filter-toolbar[data-sticky="true"] {
    box-shadow: var(--poodle-elevation-surface);
  }

  .poodle-filter-toolbar__header {
    display: flex;
    align-items: center;
    gap: var(--poodle-space-inline-sm);
  }

  .poodle-filter-toolbar__header--clickable {
    cursor: pointer;
  }

  .poodle-filter-toolbar__header--button {
    width: 100%;
    padding: 0;
    border: 0;
    background: transparent;
    color: inherit;
    text-align: left;
    cursor: pointer;
  }

  .poodle-filter-toolbar__header--button:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .poodle-filter-toolbar__summary {
    margin: 0;
    flex: 1;
    color: var(--poodle-color-text-secondary);
    font-size: var(--poodle-typography-label-size, 0.75rem);
    line-height: var(--poodle-typography-label-lineHeight, 1.4);
  }

  .poodle-filter-toolbar__actions {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    margin-left: auto;
  }

  .poodle-filter-toolbar__controls {
    display: grid;
    grid-template-columns: repeat(
      auto-fit,
      minmax(min(var(--ft-min-width, 10rem), 100%), 1fr)
    );
    gap: var(--poodle-space-inline-sm);
    align-items: end;
  }

  :global(.poodle-filter-toolbar__controls > *) {
    min-width: 0;
  }

  @media (max-width: 640px) {
    .poodle-filter-toolbar__controls {
      grid-template-columns: 1fr;
    }
  }

  .poodle-filter-toolbar__secondary {
    display: flex;
    flex-wrap: wrap;
    gap: var(--poodle-space-inline-sm);
    align-items: center;
  }

  .poodle-filter-toolbar[data-size="xs"] .poodle-filter-toolbar__summary {
    font-size: 0.6875rem;
  }

  .poodle-filter-toolbar[data-size="sm"] .poodle-filter-toolbar__summary {
    font-size: 0.71875rem;
  }

  .poodle-filter-toolbar[data-size="lg"] .poodle-filter-toolbar__summary {
    font-size: 0.8125rem;
  }

  .poodle-filter-toolbar[data-size="xl"] .poodle-filter-toolbar__summary {
    font-size: 0.875rem;
  }

  .poodle-filter-toolbar[data-density="compact"] { gap: 0.25rem; padding-inline: 0.25rem; }
  .poodle-filter-toolbar[data-density="compact"] .poodle-filter-toolbar__controls { gap: 0.25rem; }
  .poodle-filter-toolbar[data-density="comfortable"] { gap: var(--poodle-space-inline-md); padding-inline: 0.5rem; }
  .poodle-filter-toolbar[data-density="comfortable"] .poodle-filter-toolbar__controls { gap: var(--poodle-space-inline-md); }
</style>
