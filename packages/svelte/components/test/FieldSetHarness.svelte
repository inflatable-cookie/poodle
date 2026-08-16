<script lang="ts">
  import FieldSet from "../src/FieldSet.svelte";
  import type { SpaceScale } from "../src/types";

  interface Props {
    legend?: string | null;
    description?: string | null;
    columns?: number;
    gap?: SpaceScale;
    span?: number | "full" | null;
  }

  let {
    legend = null,
    description = null,
    columns = 1,
    gap = "md",
    span = null,
  }: Props = $props();
</script>

<FieldSet {legend} {description} {columns} {gap} {span}>
  <button type="button" class="harness-field-a">Field A</button>
  <button type="button" class="harness-field-b">Field B</button>
</FieldSet>
