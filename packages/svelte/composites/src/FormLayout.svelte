<script lang="ts">
  import { Callout, FormActions } from "@pug/svelte-primitives";

  export let columns = 2;
  export let error: string | null = null;
  export let success: string | null = null;
  export let fieldErrors: Record<string, string> | null = null;
  export let description: string | null = null;
</script>

<div class="form-layout">
  {#if description}
    <p class="form-layout__description">{description}</p>
  {/if}

  {#if error}
    <Callout tone="danger" message={error} />
  {/if}

  {#if success}
    <Callout tone="success" message={success} />
  {/if}

  {#if fieldErrors && Object.keys(fieldErrors).length > 0}
    <div class="form-layout__field-errors" role="alert" aria-live="polite">
      <p>Please fix the following errors:</p>
      <ul>
        {#each Object.entries(fieldErrors) as [field, message]}
          <li><strong>{field}</strong>: {message}</li>
        {/each}
      </ul>
    </div>
  {/if}

  <div class="form-layout__grid" style:--fl-columns={columns}>
    <slot />
  </div>

  {#if $$slots.actions}
    <div class="form-layout__actions">
      <FormActions>
        <slot name="actions" />
      </FormActions>
    </div>
  {/if}
</div>

<style>
  .form-layout {
    display: flex;
    flex-direction: column;
    gap: var(--pug-space-stack-md);
  }

  .form-layout__description {
    margin: 0;
    color: var(--pug-color-text-secondary);
    font-size: var(--pug-typography-body-size, 0.875rem);
    line-height: var(--pug-typography-body-lineHeight, 1.5);
  }

  .form-layout__field-errors {
    padding: var(--pug-space-panel-y, 0.75rem) var(--pug-space-panel-x, 1rem);
    border-radius: var(--pug-radius-surface, 0.5rem);
    background: color-mix(in srgb, var(--pug-color-status-danger) 8%, transparent);
    border: 0.0625rem solid color-mix(in srgb, var(--pug-color-status-danger) 40%, transparent);
    font-size: var(--pug-typography-label-size, 0.75rem);
  }

  .form-layout__field-errors p {
    margin: 0 0 0.25rem;
    font-weight: 600;
  }

  .form-layout__field-errors ul {
    margin: 0;
    padding-left: 1.25rem;
  }

  .form-layout__field-errors li {
    margin-bottom: 0.125rem;
  }

  .form-layout__grid {
    display: grid;
    grid-template-columns: repeat(var(--fl-columns, 2), 1fr);
    gap: var(--pug-space-stack-md) var(--pug-space-inline-md);
  }

  @media (max-width: 640px) {
    .form-layout__grid {
      grid-template-columns: 1fr;
    }
  }
</style>
