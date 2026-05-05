<script lang="ts">
  export let title: string | null = null;
  export let description: string | null = null;
  export let separated = true;
  export let ariaLabel: string | null = null;
  export let columns: 1 | 2 | 3 = 1;
</script>

<section
  class="poodle-detail-section"
  data-separated={separated}
  data-columns={columns}
  aria-label={ariaLabel ?? undefined}
>
  {#if title || description || $$slots.actions}
    <div class="poodle-detail-section__header">
      <div class="poodle-detail-section__title-block">
        {#if title}
          <h3 class="poodle-detail-section__title">{title}</h3>
        {/if}
        {#if description}
          <p class="poodle-detail-section__description">{description}</p>
        {/if}
      </div>
      {#if $$slots.actions}
        <div class="poodle-detail-section__actions">
          <slot name="actions" />
        </div>
      {/if}
    </div>
  {/if}
  <div class="poodle-detail-section__body">
    <slot />
  </div>
</section>

<style>
  .poodle-detail-section {
    display: grid;
    gap: calc(var(--poodle-space-stack-md) + 0.125rem);
    padding-top: calc(var(--poodle-space-stack-md) + 0.125rem);
  }

  .poodle-detail-section[data-separated="true"] {
    border-top: 0;
  }

  .poodle-detail-section__header {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
    gap: var(--poodle-space-inline-md);
    align-items: start;
  }

  .poodle-detail-section__title-block {
    display: grid;
    gap: 0.375rem;
  }

  .poodle-detail-section__title,
  .poodle-detail-section__description {
    margin: 0;
  }

  .poodle-detail-section__title {
    font-family: var(--poodle-typography-heading-family);
    font-size: 1.125rem;
    line-height: 1.2;
  }

  .poodle-detail-section__description {
    color: var(--poodle-color-text-secondary);
    font-size: var(--poodle-typography-body-size);
    line-height: var(--poodle-typography-body-lineHeight);
  }

  .poodle-detail-section__body {
    display: grid;
    gap: var(--poodle-space-stack-sm);
  }

  .poodle-detail-section[data-columns="2"] .poodle-detail-section__body {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }

  .poodle-detail-section[data-columns="3"] .poodle-detail-section__body {
    grid-template-columns: repeat(3, minmax(0, 1fr));
  }

  @media (max-width: 60rem) {
    .poodle-detail-section[data-columns="2"] .poodle-detail-section__body,
    .poodle-detail-section[data-columns="3"] .poodle-detail-section__body {
      grid-template-columns: 1fr;
    }
  }
</style>
