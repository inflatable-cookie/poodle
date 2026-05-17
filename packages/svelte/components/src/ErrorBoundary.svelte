<script lang="ts">
  import type { Snippet } from "svelte";

  import { default as EmptyState } from "./EmptyState.svelte";
  import { default as Button } from "./Button.svelte";

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
    {#snippet actions()}
      <Button variant="secondary" onclick={reset}>{retryLabel}</Button>
    {/snippet}
  </EmptyState>
{:else}
  <svelte:boundary onerror={handleError}>
    {@render children()}
  </svelte:boundary>
{/if}
