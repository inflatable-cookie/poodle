<script lang="ts">
  export let label: string;
  export let description: string | null = null;
  export let value: string | null = null;
  export let truncateValue = false;
  export let ariaLabel: string | null = null;
</script>

<div class="detail-row" aria-label={ariaLabel ?? undefined}>
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
    gap: var(--pug-space-inline-md);
    align-items: center;
    padding: var(--pug-space-panel-y) var(--pug-space-panel-x);
    border-radius: calc(var(--pug-radius-surface) - 0.0625rem);
    background: color-mix(in srgb, var(--pug-surface) 93%, var(--pug-color-text-primary));
  }

  .detail-row__label,
  .detail-row__description,
  .detail-row__value {
    margin: 0;
  }

  .detail-row__label-block {
    display: grid;
    gap: var(--pug-space-inline-sm);
    min-width: 0;
  }

  .detail-row__label {
    color: var(--pug-color-text-secondary);
    font-family: var(--pug-typography-label-family);
    font-size: var(--pug-typography-label-size);
    line-height: var(--pug-typography-label-lineHeight);
  }

  .detail-row__description {
    color: var(--pug-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.5;
  }

  .detail-row__value {
    color: var(--pug-color-text-primary);
    font-family: var(--pug-typography-body-family);
    font-size: var(--pug-typography-body-size);
    line-height: var(--pug-typography-body-lineHeight);
    min-width: 0;
  }

  .detail-row__value.truncate {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  @media (max-width: 45rem) {
    .detail-row {
      grid-template-columns: 1fr;
    }
  }
</style>
