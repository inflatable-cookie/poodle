<script lang="ts">
  import Dialog from "../src/Dialog.svelte";

  let { initialFocus = "auto", bare = false }: { initialFocus?: "auto" | "none" | string; bare?: boolean } =
    $props();
</script>

<Dialog open title="Settings" showCloseButton {initialFocus} {bare}>
  <input data-testid="first-field" type="text" />
  <button type="button" data-testid="in-body-button">In body</button>
</Dialog>
