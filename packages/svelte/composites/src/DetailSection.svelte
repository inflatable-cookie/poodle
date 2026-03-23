<script lang="ts">
  export let title: string | null = null;
  export let description: string | null = null;
  export let isSeparated = true;
  export let ariaLabel: string | null = null;
</script>

<section class="detail-section" data-separated={isSeparated} aria-label={ariaLabel ?? undefined}>
  {#if title || description || $$slots.actions}
    <div class="detail-section__header">
      <div class="detail-section__title-block">
        {#if title}
          <h3 class="detail-section__title">{title}</h3>
        {/if}
        {#if description}
          <p class="detail-section__description">{description}</p>
        {/if}
      </div>
      {#if $$slots.actions}
        <div class="detail-section__actions">
          <slot name="actions" />
        </div>
      {/if}
    </div>
  {/if}
  <div class="detail-section__body">
    <slot />
  </div>
</section>

<style>
  .detail-section {
    display: grid;
    gap: calc(var(--flint-space-stack-md) + 0.125rem);
    padding-top: calc(var(--flint-space-stack-md) + 0.125rem);
  }

  .detail-section[data-separated="true"] {
    border-top: 0;
  }

  .detail-section__header {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
    gap: var(--flint-space-inline-md);
    align-items: start;
  }

  .detail-section__title-block {
    display: grid;
    gap: 0.375rem;
  }

  .detail-section__title,
  .detail-section__description {
    margin: 0;
  }

  .detail-section__title {
    font-family: var(--flint-typography-heading-family);
    font-size: 1.125rem;
    line-height: 1.2;
  }

  .detail-section__description {
    color: var(--flint-color-text-secondary);
    font-size: var(--flint-typography-body-size);
    line-height: var(--flint-typography-body-lineHeight);
  }

  .detail-section__body {
    display: grid;
    gap: var(--flint-space-stack-sm);
  }
</style>
