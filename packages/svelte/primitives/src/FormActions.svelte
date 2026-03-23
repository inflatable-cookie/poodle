<script lang="ts">
  import type { FormActionAlign } from "./types";

  export let align: FormActionAlign = "end";
</script>

<div class="form-actions" data-align={align}>
  <slot />
</div>

<style>
  .form-actions {
    display: flex;
    flex-wrap: wrap;
    gap: var(--poodle-space-inline-md);
    align-items: center;
    padding-top: var(--poodle-space-stack-sm);
  }

  .form-actions[data-align="start"] {
    justify-content: flex-start;
  }

  .form-actions[data-align="end"] {
    justify-content: flex-end;
  }

  .form-actions[data-align="between"] {
    justify-content: space-between;
  }
</style>
