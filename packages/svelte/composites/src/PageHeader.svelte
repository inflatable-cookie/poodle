<script lang="ts">
  export let title: string;
  export let subtitle: string | null = null;
  export let eyebrow: string | null = null;
  export let align: "start" | "between" = "between";
  export let ariaLabel: string | null = null;
</script>

<header class="page-header" data-align={align} aria-label={ariaLabel ?? undefined}>
  <div class="page-header__content">
    {#if $$slots.breadcrumbs}
      <div class="page-header__breadcrumbs">
        <slot name="breadcrumbs" />
      </div>
    {/if}

    <div class="page-header__title-block">
      {#if eyebrow}
        <p class="page-header__eyebrow">{eyebrow}</p>
      {/if}
      <h2 class="page-header__title">{title}</h2>
      {#if subtitle}
        <p class="page-header__subtitle">{subtitle}</p>
      {/if}
    </div>
  </div>

  {#if $$slots.actions}
    <div class="page-header__actions">
      <slot name="actions" />
    </div>
  {/if}
</header>

<style>
  .page-header {
    --poodle-recipe-page-header-padding-block-start: 0;
    --poodle-recipe-page-header-padding-inline: 0;
    --poodle-recipe-page-header-padding-block-end: calc(var(--poodle-space-stack-md) + 0.125rem);
    --poodle-recipe-page-header-fill: transparent;
    --poodle-recipe-page-header-border: transparent;
    --poodle-recipe-page-header-shadow: none;
    --poodle-recipe-page-header-radius: var(--poodle-radius-surface);
    display: grid;
    gap: var(--poodle-space-stack-md);
    align-items: end;
    padding:
      var(--poodle-recipe-page-header-padding-block-start)
      var(--poodle-recipe-page-header-padding-inline)
      var(--poodle-recipe-page-header-padding-block-end);
    border: 0.0625rem solid var(--poodle-recipe-page-header-border);
    border-radius: var(--poodle-recipe-page-header-radius);
    background: var(--poodle-recipe-page-header-fill);
    box-shadow: var(--poodle-recipe-page-header-shadow);
  }

  .page-header[data-align="between"] {
    grid-template-columns: minmax(0, 1fr) auto;
  }

  .page-header__content {
    display: grid;
    gap: var(--poodle-space-stack-md);
  }

  .page-header__title-block {
    display: grid;
    gap: var(--poodle-space-inline-sm);
  }

  .page-header__title,
  .page-header__subtitle,
  .page-header__eyebrow {
    margin: 0;
  }

  .page-header__title {
    font-family: var(--poodle-typography-heading-family);
    font-size: 1.75rem;
    line-height: 1.1;
    font-weight: 700;
  }

  .page-header__eyebrow {
    color: var(--poodle-color-text-secondary);
    font-size: 0.6875rem;
    font-weight: 600;
    letter-spacing: 0.12em;
    text-transform: uppercase;
  }

  .page-header__subtitle {
    color: var(--poodle-color-text-secondary);
    font-size: var(--poodle-typography-body-size);
    line-height: var(--poodle-typography-body-lineHeight);
  }

  .page-header__actions {
    display: flex;
    flex-wrap: wrap;
    gap: var(--poodle-space-inline-md);
    justify-content: flex-end;
    align-items: start;
  }

  @media (max-width: 45rem) {
    .page-header[data-align="between"] {
      grid-template-columns: 1fr;
    }
  }
</style>
