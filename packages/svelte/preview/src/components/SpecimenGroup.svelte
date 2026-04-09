<script lang="ts">
  import { Eyebrow } from "@poodle/svelte-primitives";

  export let label: string;
</script>

<div class="specimen-group">
  <Eyebrow>{label}</Eyebrow>
  <div class="specimen-group__content">
    <slot />
  </div>
</div>

<style>
  .specimen-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    padding-bottom: 1.25rem;
    border-bottom: 0.0625rem solid var(--poodle-color-border-subtle);
  }

  .specimen-group:last-child {
    border-bottom: 0;
    padding-bottom: 0;
  }

  .specimen-group__content {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
</style>
