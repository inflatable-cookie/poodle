<script lang="ts">
  export let title: string | null = null;
  export let subtitle: string | null = null;
  export let isDragRegion = false;
  export let ariaLabel: string | null = null;
</script>

<header
  class="app-header"
  data-drag-region={isDragRegion}
  aria-label={ariaLabel ?? title ?? undefined}
>
  <div class="app-header__identity">
    {#if $$slots.identity}
      <slot name="identity" />
    {:else if title}
      <div class="app-header__title-group">
        <strong>{title}</strong>
        {#if subtitle}
          <span class="app-header__subtitle">{subtitle}</span>
        {/if}
      </div>
    {/if}
  </div>

  {#if $$slots.actions}
    <div class="app-header__actions">
      <slot name="actions" />
    </div>
  {/if}

  {#if $$slots.utility}
    <div class="app-header__utility">
      <slot name="utility" />
    </div>
  {/if}
</header>

<style>
  .app-header {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto auto;
    gap: var(--flint-space-inline-md);
    align-items: center;
    min-height: var(--flint-size-panel-header);
    padding: var(--flint-space-control-y) var(--flint-space-panel-x);
    border-bottom: 0.0625rem solid var(--flint-color-border-subtle);
    background: color-mix(in srgb, var(--flint-color-background-panel) 94%, transparent);
    overflow: visible;
  }

  .app-header__identity,
  .app-header__actions,
  .app-header__utility {
    display: flex;
    align-items: center;
    gap: var(--flint-space-inline-sm);
    min-width: 0;
  }

  .app-header__title-group {
    display: flex;
    align-items: baseline;
    gap: var(--flint-space-inline-sm);
    min-width: 0;
  }

  .app-header__identity strong {
    font-size: 0.9375rem;
    line-height: 1.2;
    white-space: nowrap;
  }

  .app-header__subtitle {
    color: var(--flint-color-text-secondary);
    font-size: 0.75rem;
    line-height: 1.2;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .app-header__utility {
    justify-content: flex-end;
  }

  @media (max-width: 45rem) {
    .app-header {
      grid-template-columns: 1fr;
    }

    .app-header__utility {
      justify-content: flex-start;
    }
  }
</style>
