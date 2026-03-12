<script lang="ts">
  export let title: string | null = null;
  export let isActive = false;
  export let isCollapsible = false;
  export let ariaLabel: string | null = null;
</script>

<header
  class="panel-header"
  data-active={isActive}
  aria-label={ariaLabel ?? title ?? undefined}
>
  <div class="panel-header__identity">
    {#if $$slots.title}
      <slot name="title" />
    {:else if title}
      <strong>{title}</strong>
    {/if}
  </div>

  {#if $$slots.tabs}
    <div class="panel-header__tabs">
      <slot name="tabs" />
    </div>
  {/if}

  <div class="panel-header__actions">
    {#if $$slots.actions}
      <slot name="actions" />
    {/if}
    {#if isCollapsible && $$slots.collapse}
      <slot name="collapse" />
    {/if}
  </div>
</header>

<style>
  .panel-header {
    display: grid;
    grid-template-columns: auto minmax(0, 1fr) auto;
    gap: var(--pug-space-inline-md);
    align-items: center;
    padding: 10px 12px;
    border-bottom: 1px solid var(--pug-color-border-subtle);
    background: color-mix(in srgb, var(--pug-color-background-panel) 94%, transparent);
  }

  .panel-header[data-active="true"] {
    background: color-mix(in srgb, var(--pug-color-accent-base) 8%, var(--pug-color-background-panel));
  }

  .panel-header__identity,
  .panel-header__tabs,
  .panel-header__actions {
    display: flex;
    align-items: center;
    gap: var(--pug-space-inline-sm);
    min-width: 0;
  }

  .panel-header__identity strong {
    font-size: 14px;
    line-height: 1.2;
  }

  .panel-header__actions {
    justify-content: flex-end;
  }

  @media (max-width: 720px) {
    .panel-header {
      grid-template-columns: 1fr;
    }
  }
</style>
