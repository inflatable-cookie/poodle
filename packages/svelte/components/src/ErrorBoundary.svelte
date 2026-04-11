<script lang="ts">
  import type { Snippet } from "svelte";

  import EmptyState from "./EmptyState.svelte";
  import Button from "./Button.svelte";

  interface Props {
    children: Snippet;
    title?: string;
    retryLabel?: string;
  }

  let {
    children,
    title = "Something went wrong",
    retryLabel = "Try again",
  }: Props = $props();

  let currentError = $state<Error | null>(null);

  function handleError(error: unknown) {
    currentError = error instanceof Error ? error : new Error(String(error));
  }

  function reset() {
    currentError = null;
  }
</script>

{#if currentError}
  <EmptyState {title} message={currentError.message}>
    <Button slot="actions" variant="secondary" onclick={reset}>{retryLabel}</Button>
  </EmptyState>
{:else}
  <svelte:boundary onerror={handleError}>
    {@render children()}
  </svelte:boundary>
{/if}
