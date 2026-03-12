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
    padding: 0.875rem var(--pug-space-panel-x);
    border-bottom: 0.0625rem solid color-mix(in srgb, var(--pug-color-border-subtle) 72%, transparent);
    background: color-mix(in srgb, var(--pug-color-background-surface) 74%, var(--pug-color-background-panel));
  }

  .project-header__identity {
    display: grid;
    gap: 0.375rem;
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
    font-size: 1.125rem;
    line-height: 1.2;
  }

  .project-header__identity p {
    color: var(--pug-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.5;
  }

  .project-header__dirty {
    padding: 0.25rem 0.5rem;
    border-radius: calc(var(--pug-radius-control) - 0.0625rem);
    background: color-mix(in srgb, var(--pug-color-status-warning) 18%, var(--pug-color-background-surface));
    color: var(--pug-color-status-warning);
    font-size: 0.75rem;
    font-weight: 600;
  }

  @media (max-width: 45rem) {
    .project-header {
      grid-template-columns: 1fr;
    }
  }
</style>
