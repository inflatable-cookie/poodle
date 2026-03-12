<script lang="ts">
  export let summaryText: string | null = null;
  export let ariaLabel = "Filters";
  export let isSticky = false;
</script>

<div
  class="filter-toolbar"
  data-sticky={isSticky}
  role="toolbar"
  aria-label={ariaLabel}
>
  <div class="filter-toolbar__primary">
    <slot />
  </div>

  {#if summaryText}
    <p class="filter-toolbar__summary">{summaryText}</p>
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
    gap: var(--pug-space-stack-sm);
    padding: var(--pug-space-panel-y) var(--pug-space-panel-x);
    border: 0.0625rem solid var(--pug-color-border-subtle);
    border-radius: var(--pug-radius-surface);
    background: color-mix(in srgb, var(--pug-color-background-elevated) 92%, transparent);
  }

  .filter-toolbar[data-sticky="true"] {
    box-shadow: var(--pug-elevation-surface);
  }

  .filter-toolbar__primary,
  .filter-toolbar__secondary {
    display: flex;
    flex-wrap: wrap;
    gap: var(--pug-space-inline-sm);
    align-items: center;
  }

  .filter-toolbar__summary {
    margin: 0;
    color: var(--pug-color-text-secondary);
    font-family: var(--pug-typography-body-family);
    font-size: var(--pug-typography-body-size);
    line-height: var(--pug-typography-body-lineHeight);
  }
</style>
