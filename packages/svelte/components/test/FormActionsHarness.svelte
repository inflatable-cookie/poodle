<script lang="ts">
  import FormActions from "../src/FormActions.svelte";
  import { createRawSnippet } from "svelte";
  import type { FormActionDangerItem } from "../src/types";

  interface Props {
    align?: "start" | "end" | "between";
    density?: "compact" | "default" | "comfortable" | null;
    showTopSeparation?: boolean;
    showTopBorder?: boolean;
    dangerItems?: FormActionDangerItem[];
    showDanger?: boolean;
  }

  let {
    align = "end",
    density = null,
    showTopSeparation = true,
    showTopBorder = false,
    dangerItems = [],
    showDanger = false,
  }: Props = $props();

  // Raw snippets keep the branded Snippet type through the conditional prop
  // ternary (plain `{#snippet}` declarations widen it away), and they
  // materialize real markup under the test DOM.
  const dangerSnippet = createRawSnippet(() => ({
    render: () => `<button type="button" class="harness-danger">Delete</button>`,
  }));
</script>

<FormActions
  {align}
  {density}
  {showTopSeparation}
  {showTopBorder}
  {dangerItems}
  danger={showDanger ? dangerSnippet : undefined}
>
  <button type="button">Cancel</button>
  <button type="button">Save changes</button>
</FormActions>
