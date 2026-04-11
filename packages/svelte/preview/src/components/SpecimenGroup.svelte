<script lang="ts">
  import { Eyebrow, Surface } from "@poodle/svelte";

  export let label: string;
  /** When true, renders without Surface wrapper — for components that are surfaces themselves. */
  export let bare = false;
</script>

<div class="specimen-group">
  <Eyebrow>{label}</Eyebrow>
  {#if bare}
    <div class="specimen-group__content">
      <slot />
    </div>
  {:else}
    <Surface tone="panel" border="subtle" padding="md">
      <div class="specimen-group__content">
        <slot />
      </div>
    </Surface>
  {/if}
</div>

<style>
  .specimen-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    margin-bottom: 0.5rem;
  }

  .specimen-group__content {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
</style>
