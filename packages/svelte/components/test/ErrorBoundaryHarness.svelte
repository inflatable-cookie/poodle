<script lang="ts">
  import ErrorBoundary from "../src/ErrorBoundary.svelte";
  import ErrorBoundaryBomb from "./ErrorBoundaryBomb.svelte";

  let { shouldThrow, title = "Something went wrong", retryLabel = "Try again" } = $props<{
    shouldThrow: boolean;
    title?: string;
    retryLabel?: string;
  }>();
</script>

<ErrorBoundary {title} {retryLabel}>
  {#snippet children()}
    {#if shouldThrow}
      <ErrorBoundaryBomb />
    {:else}
      <span class="harness-ok">ok</span>
    {/if}
  {/snippet}
</ErrorBoundary>
