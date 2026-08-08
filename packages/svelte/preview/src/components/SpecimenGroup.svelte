<script lang="ts">
  import { Eyebrow, Surface } from "@inflatable-cookie/poodle-svelte";
  let {
    label,
    /** When true, renders without Surface wrapper — for components that are surfaces themselves. */
    bare = false,
    children,
  }: { label: string; bare?: boolean; children?: import("svelte").Snippet } = $props();
</script>

<div class="poodle-specimen-group">
  <Eyebrow>{label}</Eyebrow>
  {#if bare}
    <div class="poodle-specimen-group__content">
      {@render children?.()}
    </div>
  {:else}
    <Surface tone="panel" border="subtle" padding="md">
      <div class="poodle-specimen-group__content">
        {@render children?.()}
      </div>
    </Surface>
  {/if}
</div>

<style>
  .poodle-specimen-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    margin-bottom: 0.5rem;
  }

  .poodle-specimen-group__content {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
</style>
