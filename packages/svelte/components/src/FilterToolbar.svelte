<script lang="ts">
  import "@poodle/styles/filter-toolbar.css";
  import type { Snippet } from "svelte";

  import { default as CollapseToggle } from "./CollapseToggle.svelte";
  import { default as UiPresentationProvider } from "./UiPresentationProvider.svelte";
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
  <UiPresentationProvider
    sizeScale={size ?? $uiPresentation.sizeScale}
    density={density ?? $uiPresentation.density}
  >
    {#if collapsible && collapsed}
      <!-- Non-interactive container: the header holds interactive children
           (CollapseToggle, action buttons), so it must not be a <button> itself.
           CollapseToggle owns the accessible name and aria-expanded; the click
           handler here is a pointer convenience for the whole row. -->
      <div
        class="poodle-filter-toolbar__header poodle-filter-toolbar__header--button"
        onclick={handleHeaderClick}
      >
        <CollapseToggle
          {collapsed}
          ariaLabel={summaryText ? `Show filters. ${summaryText}` : "Show filters"}
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
      </div>
    {:else}
      {#if collapsible}
        <!-- See the collapsed branch: header is a container, not a control. -->
        <div
          class="poodle-filter-toolbar__header poodle-filter-toolbar__header--button poodle-filter-toolbar__header--clickable"
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
        </div>
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
  </UiPresentationProvider>
</div>

