<script lang="ts">
  import type { TabsFocusOnValueChange } from "@inflatable-cookie/poodle-core";

  import Tabs from "../src/Tabs.svelte";
  import type { TabItem } from "../src/types";

  const defaultItems: TabItem[] = [
    { value: "components", label: "Components" },
    { value: "preview", label: "Preview" },
    { value: "tree", label: "Tree" },
  ];

  const otherItems: TabItem[] = [
    { value: "other-a", label: "Other A" },
    { value: "other-b", label: "Other B" },
  ];

  interface Props {
    items?: TabItem[];
    focusOnValueChange?: TabsFocusOnValueChange;
  }

  let { items = defaultItems, focusOnValueChange = "preserve" }: Props = $props();

  let value = $state<string | null>("components");
  let alive = $state(true);

  function selectTree(): void {
    value = "tree";
  }

  function acceptOpen(): void {
    void Promise.resolve().then(() => {
      value = "tree";
    });
  }

  function supersede(): void {
    value = "preview";
    value = "tree";
  }

  function selectMissing(): void {
    value = "ghost";
  }

  function teardownAfterCapture(): void {
    value = "tree";
    queueMicrotask(() => {
      alive = false;
    });
  }
</script>

<button type="button" data-testid="outside">Outside</button>
<button type="button" data-testid="select-tree" onclick={selectTree}>Select Tree</button>
<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
<span role="presentation" data-testid="accept-open" onclick={acceptOpen}>Accept Open</span>
<button type="button" data-testid="supersede" onclick={supersede}>Supersede</button>
<button type="button" data-testid="select-missing" onclick={selectMissing}>Select missing</button>
<button type="button" data-testid="teardown" onclick={teardownAfterCapture}>Teardown</button>

{#if alive}
  <Tabs {items} {value} {focusOnValueChange} ariaLabel="Inspector" onValueChange={(next) => (value = next)}>
    {#snippet children(activeValue)}
      {#if activeValue === "components"}
        <button type="button" data-testid="list-card">ListCard row</button>
      {:else if activeValue === "tree"}
        <button type="button" data-testid="tree-return">Return to screen</button>
      {:else}
        <button type="button" data-testid="preview-panel">Preview body</button>
      {/if}
    {/snippet}
  </Tabs>
{/if}

<Tabs items={otherItems} value="other-a" focusOnValueChange="selected-tab" ariaLabel="Other">
  {#snippet children()}
    <button type="button" data-testid="other-panel">Other panel</button>
  {/snippet}
</Tabs>
