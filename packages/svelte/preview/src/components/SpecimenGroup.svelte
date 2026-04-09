<script lang="ts">
  import { Eyebrow, Surface } from "@poodle/svelte-primitives";

  export let label: string;
</script>

<Surface tone="panel" border="subtle" padding="md">
  <Eyebrow>{label}</Eyebrow>
  <div class="specimen-group__content">
    <slot />
  </div>
</Surface>

<style>
  .specimen-group__content {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    margin-top: 0.5rem;
  }
</style>
