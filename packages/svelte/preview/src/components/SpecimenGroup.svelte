<script lang="ts">
  import { Eyebrow, Surface } from "@poodle/svelte-primitives";

  export let label: string;
  /** When true, renders without Surface wrapper — for components that are surfaces themselves. */
  export let bare = false;
</script>

{#if bare}
  <div class="specimen-group">
    <Eyebrow>{label}</Eyebrow>
    <div class="specimen-group__content">
      <slot />
    </div>
  </div>
{:else}
  <Surface tone="panel" border="subtle" padding="md">
    <Eyebrow>{label}</Eyebrow>
    <div class="specimen-group__content">
      <slot />
    </div>
  </Surface>
{/if}

<style>
  .specimen-group__content {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    margin-top: 0.5rem;
  }
</style>
