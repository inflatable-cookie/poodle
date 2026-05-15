<script lang="ts">
  import type { Snippet } from "svelte";
  import Icon from "./Icon.svelte";
  import Popover from "./Popover.svelte";

  interface Props {
    label: string;
    description?: string | null;
    value?: string | number | null;
    emptyText?: string;
    truncateValue?: boolean;
    ariaLabel?: string | null;
    layout?: "inline" | "stacked";
    presentation?: "simple" | "surface";
    span?: "full" | "half" | null;
    valueContent?: Snippet;
    action?: Snippet;
    children?: Snippet;
  }

  let {
    label,
    description = null,
    value = null,
    emptyText = "—",
    truncateValue = false,
    ariaLabel = null,
    layout = "inline",
    presentation = "simple",
    span = null,
    valueContent,
    action,
    children,
  }: Props = $props();

  let renderedValue = $derived(value === null ? emptyText : String(value));
</script>

<div
  class="poodle-detail-item"
  data-layout={layout}
  data-presentation={presentation}
  data-span={span ?? undefined}
  aria-label={ariaLabel ?? undefined}
>
  <div class="poodle-detail-item__label-block">
    <dt class="poodle-detail-item__label">
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
    </dt>
  </div>

  <dd class:poodle-truncate={truncateValue} class="poodle-detail-item__value">
    {#if valueContent}
      {@render valueContent()}
    {:else if children}
      {@render children()}
    {:else}
      {renderedValue}
    {/if}
  </dd>

  {#if action}
    <div class="poodle-detail-item__action">
      {@render action()}
    </div>
  {/if}
</div>

<style>
  .poodle-detail-item {
    display: grid;
    gap: 0.25rem;
  }

  .poodle-detail-item[data-layout="inline"] {
    grid-template-columns: minmax(8rem, 11.25rem) minmax(0, 1fr) auto;
    gap: 0.25rem var(--poodle-space-inline-md);
    align-items: baseline;
  }

  .poodle-detail-item[data-layout="inline"] .poodle-detail-item__label-block {
    grid-row: 1;
  }

  .poodle-detail-item[data-layout="inline"] .poodle-detail-item__value {
    grid-row: 1;
  }

  .poodle-detail-item[data-layout="inline"] .poodle-detail-item__action {
    grid-row: 1;
  }

  .poodle-detail-item[data-span="full"] {
    grid-column: 1 / -1;
  }

  .poodle-detail-item__label,
  .poodle-detail-item__value {
    margin: 0;
  }

  .poodle-detail-item__label-block {
    display: flex;
    align-items: baseline;
    gap: var(--poodle-space-inline-sm);
    min-width: 0;
  }

  .poodle-detail-item__label {
    display: inline-flex;
    align-items: center;
    gap: 0.375rem;
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-label-family);
    font-size: var(--poodle-typography-label-size);
    line-height: var(--poodle-typography-label-lineHeight);
  }

  .poodle-detail-item__value {
    min-width: 0;
    color: var(--poodle-color-text-primary);
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-typography-body-size);
    line-height: var(--poodle-typography-body-lineHeight);
    word-break: break-word;
  }

  .poodle-detail-item__value.poodle-truncate {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .poodle-detail-item__info-trigger {
    display: inline-flex;
    align-items: center;
  }

  .poodle-detail-item__info-icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1.25em;
    height: 1.25em;
    cursor: pointer;
    flex-shrink: 0;
    border-radius: var(--poodle-radius-pill);
    background: color-mix(in srgb, var(--poodle-color-text-secondary) 14%, transparent);
    color: var(--poodle-color-text-secondary);
    transition: background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .poodle-detail-item__info-icon :global(svg) {
    width: 0.75em !important;
    height: 0.75em !important;
  }

  .poodle-detail-item__info-trigger:hover .poodle-detail-item__info-icon {
    background: color-mix(in srgb, var(--poodle-color-text-secondary) 26%, transparent);
    color: var(--poodle-color-text-primary);
  }

  .poodle-detail-item__info-content {
    margin: 0;
    color: var(--poodle-color-text-primary);
    font-family: var(--poodle-typography-body-family);
    font-size: 0.75rem;
    line-height: 1.5;
  }

  .poodle-detail-item__label :global(.poodle-popover__surface) {
    min-width: 10rem;
    max-width: 22rem;
    padding: 0.5rem 0.625rem;
  }

  .poodle-detail-item__label :global(.poodle-popover__trigger:focus-visible) {
    outline: none;
  }

  .poodle-detail-item__info-trigger:focus-visible .poodle-detail-item__info-icon {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.0625rem;
  }

  .poodle-detail-item[data-presentation="surface"] {
    grid-template-columns: 11.25rem minmax(0, 1fr) auto;
    gap: var(--poodle-space-inline-md);
    align-items: center;
    padding: 0.625rem var(--poodle-space-panel-x);
    border-radius: calc(var(--poodle-radius-surface) - 0.0625rem);
    background: color-mix(in srgb, var(--poodle-surface) 93%, var(--poodle-color-text-primary));
  }

  .poodle-detail-item[data-presentation="surface"][data-layout="stacked"] {
    grid-template-columns: minmax(0, 1fr) auto;
    align-items: start;
    gap: 0.1875rem;
  }

  .poodle-detail-item[data-presentation="surface"][data-layout="stacked"] .poodle-detail-item__label-block {
    grid-column: 1 / -1;
  }

  .poodle-detail-item[data-presentation="surface"][data-layout="stacked"] .poodle-detail-item__label {
    color: var(--poodle-color-text-tertiary);
    font-size: 0.75rem;
    line-height: 1.35;
  }

  .poodle-detail-item[data-presentation="surface"][data-layout="stacked"] .poodle-detail-item__value {
    font-size: 1rem;
    font-weight: 600;
  }

  @media (max-width: 45rem) {
    .poodle-detail-item[data-layout="inline"] {
      grid-template-columns: 1fr;
    }

    .poodle-detail-item[data-layout="inline"] .poodle-detail-item__label-block,
    .poodle-detail-item[data-layout="inline"] .poodle-detail-item__value,
    .poodle-detail-item[data-layout="inline"] .poodle-detail-item__action {
      grid-row: auto;
    }

    .poodle-detail-item[data-presentation="surface"] {
      grid-template-columns: 1fr;
    }

    .poodle-detail-item[data-presentation="surface"][data-layout="stacked"] {
      grid-template-columns: 1fr;
    }
  }
</style>
