<script lang="ts">
  import Callout from "./Callout.svelte";
  import FormActions from "./FormActions.svelte";

  export let columns = 6;
  export let error: string | null = null;
  export let success: string | null = null;
  export let fieldErrors: Record<string, string> | null = null;
  export let description: string | null = null;
</script>

<div class="poodle-form-layout">
  {#if description}
    <p class="poodle-form-layout__description">{description}</p>
  {/if}

  {#if error}
    <Callout tone="danger" message={error} />
  {/if}

  {#if success}
    <Callout tone="success" message={success} />
  {/if}

  {#if fieldErrors && Object.keys(fieldErrors).length > 0}
    <div class="poodle-form-layout__field-errors" role="alert" aria-live="polite">
      <p>Please fix the following errors:</p>
      <ul>
        {#each Object.entries(fieldErrors) as [field, message]}
          <li><strong>{field}</strong>: {message}</li>
        {/each}
      </ul>
    </div>
  {/if}

  <div class="poodle-form-layout__grid" style:--fl-columns={columns}>
    <slot />
  </div>

  {#if $$slots.actions}
    <div class="poodle-form-layout__actions">
      <FormActions>
        <slot name="actions" />
      </FormActions>
    </div>
  {/if}
</div>

<style>
  .poodle-form-layout {
    display: flex;
    flex-direction: column;
    gap: var(--poodle-space-stack-lg);
  }

  .poodle-form-layout__description {
    margin: 0;
    color: var(--poodle-color-text-secondary);
    font-size: var(--poodle-typography-body-size, 0.875rem);
    line-height: var(--poodle-typography-body-lineHeight, 1.5);
  }

  .poodle-form-layout__field-errors {
    padding: var(--poodle-space-panel-y, 0.75rem) var(--poodle-space-panel-x, 1rem);
    border-radius: var(--poodle-radius-surface, 0.5rem);
    background: color-mix(in srgb, var(--poodle-color-status-danger) 8%, transparent);
    border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-status-danger) 40%, transparent);
    font-size: var(--poodle-typography-label-size, 0.75rem);
  }

  .poodle-form-layout__field-errors p {
    margin: 0 0 0.25rem;
    font-weight: 600;
  }

  .poodle-form-layout__field-errors ul {
    margin: 0;
    padding-left: 1.25rem;
  }

  .poodle-form-layout__field-errors li {
    margin-bottom: 0.125rem;
  }

  .poodle-form-layout {
    container-type: inline-size;
  }

  .poodle-form-layout__grid {
    display: grid;
    grid-template-columns: repeat(var(--fl-columns, 6), 1fr);
    gap: var(--poodle-space-stack-lg) var(--poodle-space-inline-md);
  }

  @container (max-width: 600px) {
    .poodle-form-layout__grid {
      grid-template-columns: repeat(2, 1fr);
    }

    .poodle-form-layout__grid :global([style*="grid-column"]) {
      grid-column: span 1 !important;
    }

    .poodle-form-layout__grid :global([style*="span 6"]),
    .poodle-form-layout__grid :global([style*="span -1"]) {
      grid-column: 1 / -1 !important;
    }
  }

  @container (max-width: 480px) {
    .poodle-form-layout__grid {
      grid-template-columns: 1fr;
    }
  }
</style>
