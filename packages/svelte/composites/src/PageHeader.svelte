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
    --flint-recipe-page-header-padding-block-start: 0;
    --flint-recipe-page-header-padding-inline: 0;
    --flint-recipe-page-header-padding-block-end: calc(var(--flint-space-stack-md) + 0.125rem);
    --flint-recipe-page-header-fill: transparent;
    --flint-recipe-page-header-border: transparent;
    --flint-recipe-page-header-shadow: none;
    --flint-recipe-page-header-radius: var(--flint-radius-surface);
    display: grid;
    gap: var(--flint-space-stack-md);
    align-items: end;
    padding:
      var(--flint-recipe-page-header-padding-block-start)
      var(--flint-recipe-page-header-padding-inline)
      var(--flint-recipe-page-header-padding-block-end);
    border: 0.0625rem solid var(--flint-recipe-page-header-border);
    border-radius: var(--flint-recipe-page-header-radius);
    background: var(--flint-recipe-page-header-fill);
    box-shadow: var(--flint-recipe-page-header-shadow);
  }

  .page-header[data-align="between"] {
    grid-template-columns: minmax(0, 1fr) auto;
  }

  .page-header__content {
    display: grid;
    gap: var(--flint-space-stack-md);
  }

  .page-header__title-block {
    display: grid;
    gap: var(--flint-space-inline-sm);
  }

  .page-header__title,
  .page-header__subtitle,
  .page-header__eyebrow {
    margin: 0;
  }

  .page-header__title {
    font-family: var(--flint-typography-heading-family);
    font-size: 1.75rem;
    line-height: 1.1;
    font-weight: 700;
  }

  .page-header__eyebrow {
    color: var(--flint-color-text-secondary);
    font-size: 0.6875rem;
    font-weight: 600;
    letter-spacing: 0.12em;
    text-transform: uppercase;
  }

  .page-header__subtitle {
    color: var(--flint-color-text-secondary);
    font-size: var(--flint-typography-body-size);
    line-height: var(--flint-typography-body-lineHeight);
  }

  .page-header__actions {
    display: flex;
    flex-wrap: wrap;
    gap: var(--flint-space-inline-md);
    justify-content: flex-end;
    align-items: start;
  }

  @media (max-width: 45rem) {
    .page-header[data-align="between"] {
      grid-template-columns: 1fr;
    }
  }
</style>
