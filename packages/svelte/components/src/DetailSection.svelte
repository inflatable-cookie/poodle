<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    title?: string | null;
    description?: string | null;
    separated?: boolean;
    ariaLabel?: string | null;
    columns?: 1 | 2 | 3;
    actions?: Snippet;
    children?: Snippet;
  }

  let {
    title = null,
    description = null,
    separated = true,
    ariaLabel = null,
    columns = 1,
    actions,
    children,
  }: Props = $props();
</script>

<section
  class="poodle-detail-section"
  data-separated={separated}
  data-columns={columns}
  aria-label={ariaLabel ?? undefined}
>
  {#if title || description || actions}
    <div class="poodle-detail-section__header">
      <div class="poodle-detail-section__title-block">
        {#if title}
          <h3 class="poodle-detail-section__title">{title}</h3>
        {/if}
        {#if description}
          <p class="poodle-detail-section__description">{description}</p>
        {/if}
      </div>
      {#if actions}
        <div class="poodle-detail-section__actions">
          {@render actions()}
        </div>
      {/if}
    </div>
  {/if}
  <div class="poodle-detail-section__body">
    {@render children?.()}
  </div>
</section>

<style>
  .poodle-detail-section {
    display: grid;
    gap: calc(var(--poodle-space-stack-md) + 0.125rem);
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
