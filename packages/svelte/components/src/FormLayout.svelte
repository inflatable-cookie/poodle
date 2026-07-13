<script lang="ts">
  import "@poodle/styles/form-layout.css";
  import type { Snippet } from "svelte";

  import { default as Callout } from "./Callout.svelte";
  import { default as FormActions } from "./FormActions.svelte";

  interface Props {
    columns?: number;
    error?: string | null;
    success?: string | null;
    fieldErrors?: Record<string, string> | null;
    description?: string | null;
    actions?: Snippet;
    children?: Snippet;
  }

  let {
    columns = 6,
    error = null,
    success = null,
    fieldErrors = null,
    description = null,
    actions,
    children,
  }: Props = $props();

  const hasFieldErrors = $derived(Boolean(fieldErrors && Object.keys(fieldErrors).length > 0));
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

  {#if hasFieldErrors}
    <div class="poodle-form-layout__field-errors" role="alert" aria-live="polite">
      <p>Please fix the following errors:</p>
      <ul>
        {#each Object.entries(fieldErrors ?? {}) as [field, message]}
          <li><strong>{field}</strong>: {message}</li>
        {/each}
      </ul>
    </div>
  {/if}

  <div class="poodle-form-layout__grid" style:--fl-columns={columns}>
    {@render children?.()}
  </div>

  {#if actions}
    <div class="poodle-form-layout__actions">
      <FormActions>
        {@render actions()}
      </FormActions>
    </div>
  {/if}
</div>

