<script lang="ts">
  export let label: string | null = null;
  export let ariaLabel: string | null = null;
</script>

<span class="poodle-meta-item" aria-label={ariaLabel ?? undefined}>
  {#if label}
    <span class="poodle-meta-item__label">{label}</span>
  {/if}
  <span class="poodle-meta-item__value">
    <slot />
  </span>
</span>

<style>
  .poodle-meta-item {
    display: inline-flex;
    align-items: center;
    flex-wrap: wrap;
    gap: 0.375rem;
    min-width: 0;
  }

  .poodle-meta-item__label {
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-label-family);
    font-size: 0.6875rem;
    font-weight: var(--poodle-typography-label-weight);
    letter-spacing: 0.08em;
    line-height: 1;
    text-transform: uppercase;
    white-space: nowrap;
  }

  .poodle-meta-item__value {
    display: inline-flex;
    align-items: center;
    gap: 0.375rem;
    min-width: 0;
    color: var(--poodle-color-text-primary);
    font-family: var(--poodle-typography-body-family);
    font-size: 0.875rem;
    line-height: 1.4;
  }

  .poodle-meta-item__value :global(*) {
    min-width: 0;
  }
</style>
