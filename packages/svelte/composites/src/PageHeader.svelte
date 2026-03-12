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
    display: grid;
    gap: var(--pug-space-stack-md);
    align-items: end;
    padding-bottom: calc(var(--pug-space-stack-md) + 2px);
  }

  .page-header[data-align="between"] {
    grid-template-columns: minmax(0, 1fr) auto;
  }

  .page-header__content {
    display: grid;
    gap: var(--pug-space-stack-md);
  }

  .page-header__title-block {
    display: grid;
    gap: 6px;
  }

  .page-header__title,
  .page-header__subtitle,
  .page-header__eyebrow {
    margin: 0;
  }

  .page-header__title {
    font-family: var(--pug-typography-heading-family);
    font-size: 28px;
    line-height: 1.1;
    font-weight: 700;
  }

  .page-header__eyebrow {
    color: var(--pug-color-text-secondary);
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 0.12em;
    text-transform: uppercase;
  }

  .page-header__subtitle {
    color: var(--pug-color-text-secondary);
    font-size: var(--pug-typography-body-size);
    line-height: var(--pug-typography-body-lineHeight);
  }

  .page-header__actions {
    display: flex;
    flex-wrap: wrap;
    gap: var(--pug-space-inline-md);
    justify-content: flex-end;
    align-items: start;
  }

  @media (max-width: 720px) {
    .page-header[data-align="between"] {
      grid-template-columns: 1fr;
    }
  }
</style>
