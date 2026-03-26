<script lang="ts">
  export let label: string;
  export let description: string | null = null;
  export let value: string | null = null;
  export let truncateValue = false;
  export let ariaLabel: string | null = null;
  export let layout: "inline" | "stacked" = "inline";
</script>

<div class="detail-row" data-layout={layout} aria-label={ariaLabel ?? undefined}>
  <div class="detail-row__label-block">
    <dt class="detail-row__label">{label}</dt>
    {#if description}
      <p class="detail-row__description">{description}</p>
    {/if}
  </div>

  <dd class:truncate={truncateValue} class="detail-row__value">
    {#if $$slots.value}
      <slot name="value" />
    {:else}
      {value ?? "—"}
    {/if}
  </dd>

  {#if $$slots.action}
    <div class="detail-row__action">
      <slot name="action" />
    </div>
  {/if}
</div>

<style>
  .detail-row {
    display: grid;
    grid-template-columns: 11.25rem minmax(0, 1fr) auto;
    gap: var(--poodle-space-inline-md);
    align-items: center;
    padding: var(--poodle-space-panel-y) var(--poodle-space-panel-x);
    border-radius: calc(var(--poodle-radius-surface) - 0.0625rem);
    background: color-mix(in srgb, var(--poodle-surface) 93%, var(--poodle-color-text-primary));
  }

  .detail-row__label,
  .detail-row__description,
  .detail-row__value {
    margin: 0;
  }

  .detail-row__label-block {
    display: grid;
    gap: var(--poodle-space-inline-sm);
    min-width: 0;
  }

  .detail-row__label {
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-label-family);
    font-size: var(--poodle-typography-label-size);
    line-height: var(--poodle-typography-label-lineHeight);
  }

  .detail-row__description {
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.5;
  }

  .detail-row__value {
    color: var(--poodle-color-text-primary);
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-typography-body-size);
    line-height: var(--poodle-typography-body-lineHeight);
    min-width: 0;
  }

  .detail-row__value.truncate {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .detail-row[data-layout="stacked"] {
    grid-template-columns: minmax(0, 1fr) auto;
    align-items: start;
  }

  .detail-row[data-layout="stacked"] .detail-row__label-block {
    grid-column: 1 / -1;
    gap: 0.25rem;
  }

  .detail-row[data-layout="stacked"] .detail-row__value {
    font-size: 1rem;
    font-weight: 600;
  }

  @media (max-width: 45rem) {
    .detail-row {
      grid-template-columns: 1fr;
    }

    .detail-row[data-layout="stacked"] {
      grid-template-columns: 1fr;
    }
  }
</style>
