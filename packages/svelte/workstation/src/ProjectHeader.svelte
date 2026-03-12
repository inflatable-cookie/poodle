<script lang="ts">
  export let title: string;
  export let subtitle: string | null = null;
  export let isDirty = false;
  export let ariaLabel: string | null = null;
</script>

<section class="project-header" aria-label={ariaLabel ?? title}>
  <div class="project-header__identity">
    <div class="project-header__title-row">
      <h3>{title}</h3>
      {#if isDirty}
        <span class="project-header__dirty">Unsaved</span>
      {/if}
    </div>
    {#if subtitle}
      <p>{subtitle}</p>
    {/if}
  </div>

  {#if $$slots.actions}
    <div class="project-header__actions">
      <slot name="actions" />
    </div>
  {/if}

  {#if $$slots.status}
    <div class="project-header__status">
      <slot name="status" />
    </div>
  {/if}
</section>

<style>
  .project-header {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto auto;
    gap: var(--pug-space-inline-md);
    align-items: center;
    padding: 14px var(--pug-space-panel-x);
    border-bottom: 1px solid color-mix(in srgb, var(--pug-color-border-subtle) 72%, transparent);
    background: color-mix(in srgb, var(--pug-color-background-surface) 74%, var(--pug-color-background-panel));
  }

  .project-header__identity {
    display: grid;
    gap: 6px;
    min-width: 0;
  }

  .project-header__title-row,
  .project-header__actions,
  .project-header__status {
    display: flex;
    align-items: center;
    gap: var(--pug-space-inline-sm);
  }

  .project-header__title-row h3,
  .project-header__identity p {
    margin: 0;
  }

  .project-header__title-row h3 {
    font-size: 18px;
    line-height: 1.2;
  }

  .project-header__identity p {
    color: var(--pug-color-text-secondary);
    font-size: 13px;
    line-height: 1.5;
  }

  .project-header__dirty {
    padding: 4px 8px;
    border-radius: calc(var(--pug-radius-control) - 1px);
    background: color-mix(in srgb, var(--pug-color-status-warning) 18%, var(--pug-color-background-surface));
    color: var(--pug-color-status-warning);
    font-size: 12px;
    font-weight: 600;
  }

  @media (max-width: 720px) {
    .project-header {
      grid-template-columns: 1fr;
    }
  }
</style>
