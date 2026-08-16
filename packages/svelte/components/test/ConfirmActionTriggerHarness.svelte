<script lang="ts">
  import ConfirmAction from "../src/ConfirmAction.svelte";

  let { title = "Clear all?" } = $props<{ title?: string }>();
</script>

<ConfirmAction {title}>
  {#snippet trigger()}
    <button type="button">Clear filters</button>
  {/snippet}
</ConfirmAction>
