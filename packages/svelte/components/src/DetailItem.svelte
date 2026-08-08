<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/detail-item.css";
  import type { Snippet } from "svelte";
  import { default as Icon } from "./Icon.svelte";
  import { default as Popover } from "./Popover.svelte";
  import { getUiPresentation } from "./presentation";
  import type { ControlDensity } from "./types";

  interface Props {
    density?: ControlDensity | null;
    label: string;
    description?: string | null;
    value?: string | number | null;
    emptyText?: string;
    truncateValue?: boolean;
    ariaLabel?: string | null;
    layout?: "inline" | "stacked";
    presentation?: "simple" | "surface";
    span?: "full" | "half" | 1 | 2 | 3 | 4 | null;
    valueContent?: Snippet;
    action?: Snippet;
    children?: Snippet;
  }

  let {
    density = null,
    label,
    description = null,
    value = null,
    emptyText = "—",
    truncateValue = false,
    ariaLabel = null,
    layout = "inline",
    presentation = "surface",
    span = null,
    valueContent,
    action,
    children,
  }: Props = $props();

  const uiPresentation = getUiPresentation();

  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const resolvedLayout = $derived(layout === "stacked" ? "stacked" : "inline");
  let renderedValue = $derived(value === null ? emptyText : String(value));
</script>

<div
  class="poodle-detail-item"
  data-density={resolvedDensity}
  data-layout={resolvedLayout}
  data-presentation={presentation}
  data-span={span ?? undefined}
  aria-label={ariaLabel ?? undefined}
>
  <div class="poodle-detail-item__label-block">
    <span class="poodle-detail-item__label">
      {label}
      {#if description}
        <Popover placement="top" offset={6} ariaLabel="More information">
          {#snippet trigger()}
            <span class="poodle-detail-item__info-trigger">
              <span class="poodle-detail-item__info-icon" aria-label="More information">
                <Icon name="info" />
              </span>
            </span>
          {/snippet}
          <p class="poodle-detail-item__info-content">{description}</p>
        </Popover>
      {/if}
    </span>
  </div>

  <div class:poodle-truncate={truncateValue} class="poodle-detail-item__value">
    {#if valueContent}
      {@render valueContent()}
    {:else if children}
      {@render children()}
    {:else}
      {renderedValue}
    {/if}
  </div>

  {#if action}
    <div class="poodle-detail-item__action">
      {@render action()}
    </div>
  {/if}
</div>

