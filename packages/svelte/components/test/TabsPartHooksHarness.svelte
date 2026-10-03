<script lang="ts">
  /**
   * Harness for the Tabs `data-part` hook tests: Tabs only renders its panel
   * when a `children(activeValue)` snippet and its actions when an `actions`
   * snippet are provided, and snippets cannot be built from plain TypeScript
   * tests.
   */
  import { default as Tabs } from "../src/Tabs.svelte";

  let {
    collapseWhenOverflow = false,
    showTooltips = false,
  }: {
    collapseWhenOverflow?: boolean;
    showTooltips?: boolean;
  } = $props();

  const items = [
    { value: "mix", label: "Mix" },
    { value: "master", label: "Master", separator: true },
    { value: "notes", label: "Notes", closable: true },
  ];
</script>

<Tabs
  {items}
  defaultValue="mix"
  activeEdge="underline"
  ariaLabel="Part hooks"
  {collapseWhenOverflow}
  {showTooltips}
>
  {#snippet actions()}
    <button type="button">Add</button>
  {/snippet}
  {#snippet children(value)}
    <p>Panel for {value}</p>
  {/snippet}
</Tabs>
