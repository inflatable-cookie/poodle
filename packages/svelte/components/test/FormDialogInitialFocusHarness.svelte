<script lang="ts">
  import FormDialog from "../src/FormDialog.svelte";

  let { initialFocus = "auto" }: { initialFocus?: "auto" | "none" | string } = $props();
</script>

<FormDialog open title="Edit" {initialFocus}>
  {#snippet body(submitting)}
    <input data-testid="form-first-field" type="text" disabled={submitting} />
    <input data-testid="form-second-field" type="text" disabled={submitting} />
  {/snippet}
</FormDialog>
