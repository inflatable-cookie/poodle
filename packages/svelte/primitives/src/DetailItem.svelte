<script lang="ts">
  export let label: string;
  export let description: string | null = null;
  export let value: string | number | null = null;
  export let emptyText = "—";
  export let truncateValue = false;
  export let ariaLabel: string | null = null;
  export let layout: "inline" | "stacked" = "inline";
  export let presentation: "simple" | "surface" = "simple";
  export let span: "full" | "half" | null = null;
</script>

<div
  class="detail-item"
  data-layout={layout}
  data-presentation={presentation}
  data-span={span ?? undefined}
  aria-label={ariaLabel ?? undefined}
>
  <div class="detail-item__label-block">
    <dt class="detail-item__label">{label}</dt>
    {#if description}
      <p class="detail-item__description">{description}</p>
    {/if}
  </div>

  <dd class:truncate={truncateValue} class="detail-item__value">
    {#if $$slots.value}
      <slot name="value" />
    {:else if $$slots.default}
      <slot />
    {:else}
      {value === null ? emptyText : String(value)}
    {/if}
  </dd>

  {#if $$slots.action}
    <div class="detail-item__action">
      <slot name="action" />
    </div>
  {/if}
</div>

<style>
  .detail-item {
    display: grid;
    gap: 0.25rem;
  }

  .detail-item[data-span="full"] {
    grid-column: 1 / -1;
  }

  .detail-item__label,
  .detail-item__description,
  .detail-item__value {
    margin: 0;
  }

  .detail-item__label-block {
    display: grid;
    gap: var(--poodle-space-inline-sm);
    min-width: 0;
  }

  .detail-item__label {
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-label-family);
    font-size: var(--poodle-typography-label-size);
    line-height: var(--poodle-typography-label-lineHeight);
  }

  .detail-item__description {
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.5;
  }

  .detail-item__value {
    min-width: 0;
    color: var(--poodle-color-text-primary);
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-typography-body-size);
    line-height: var(--poodle-typography-body-lineHeight);
    word-break: break-word;
  }

  .detail-item__value.truncate {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .detail-item[data-presentation="surface"] {
    grid-template-columns: 11.25rem minmax(0, 1fr) auto;
    gap: var(--poodle-space-inline-md);
    align-items: center;
    padding: var(--poodle-space-panel-y) var(--poodle-space-panel-x);
    border-radius: calc(var(--poodle-radius-surface) - 0.0625rem);
    background: color-mix(in srgb, var(--poodle-surface) 93%, var(--poodle-color-text-primary));
  }

  .detail-item[data-presentation="surface"][data-layout="stacked"] {
    grid-template-columns: minmax(0, 1fr) auto;
    align-items: start;
  }

  .detail-item[data-presentation="surface"][data-layout="stacked"] .detail-item__label-block {
    grid-column: 1 / -1;
    gap: 0.25rem;
  }

  .detail-item[data-presentation="surface"][data-layout="stacked"] .detail-item__value {
    font-size: 1rem;
    font-weight: 600;
  }

  @media (max-width: 45rem) {
    .detail-item[data-presentation="surface"] {
      grid-template-columns: 1fr;
    }

    .detail-item[data-presentation="surface"][data-layout="stacked"] {
      grid-template-columns: 1fr;
    }
  }
</style>
