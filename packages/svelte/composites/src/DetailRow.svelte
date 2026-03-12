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
    grid-template-columns: minmax(0, 180px) minmax(0, 1fr) auto;
    gap: var(--pug-space-inline-md);
    align-items: start;
    padding: 12px 14px;
    border-radius: calc(var(--pug-radius-surface) - 1px);
    background: color-mix(in srgb, var(--pug-color-background-surface) 52%, transparent);
  }

  .detail-row__label,
  .detail-row__description,
  .detail-row__value {
    margin: 0;
  }

  .detail-row__label-block {
    display: grid;
    gap: 4px;
  }

  .detail-row__label {
    color: var(--pug-color-text-secondary);
    font-family: var(--pug-typography-label-family);
    font-size: var(--pug-typography-label-size);
    line-height: var(--pug-typography-label-lineHeight);
  }

  .detail-row__description {
    color: var(--pug-color-text-secondary);
    font-size: 13px;
    line-height: 1.5;
  }

  .detail-row__value {
    color: var(--pug-color-text-primary);
    font-family: var(--pug-typography-body-family);
    font-size: var(--pug-typography-body-size);
    line-height: var(--pug-typography-body-lineHeight);
  }

  .detail-row__value.truncate {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  @media (max-width: 720px) {
    .detail-row {
      grid-template-columns: 1fr;
    }
  }
</style>
