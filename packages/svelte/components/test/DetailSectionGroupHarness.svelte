<script lang="ts">
  import DetailSectionGroup from "../src/DetailSectionGroup.svelte";

  let {
    layout = "grid" as "grid" | "stack",
    maxColumns = 4 as 2 | 3 | 4 | 5,
  } = $props<{
    layout?: "grid" | "stack";
    maxColumns?: 2 | 3 | 4 | 5;
  }>();
</script>

<DetailSectionGroup {layout} {maxColumns}>
  <section class="harness-child" aria-label="Project details">
    Project details
  </section>
  <section class="harness-child" aria-label="Billing">
    Billing
  </section>
</DetailSectionGroup>
