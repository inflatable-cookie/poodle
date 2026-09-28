<script lang="ts">
  // Focused harness: a Select nested in a Popover. The Select listbox
  // portals to the theme root, so Tab from an open listbox never bubbles
  // through the popover surface — the popover's document-level trap must
  // still hold it.
  import Popover from "../src/Popover.svelte";
  import Select from "../src/Select.svelte";

  import type { SelectItems } from "../src/types";

  interface Props {
    open?: boolean | null;
    defaultOpen?: boolean;
    options?: SelectItems;
  }

  let {
    open = null,
    defaultOpen = true,
    options = [
      { value: "alpha", label: "Alpha" },
      { value: "beta", label: "Beta" },
    ],
  }: Props = $props();
</script>

<div data-poodle-theme-root>
  <Popover {open} {defaultOpen}>
    {#snippet trigger()}
      Open
    {/snippet}
    <button type="button" data-testid="surface-action">Surface action</button>
    <Select {options} native={false} ariaLabel="Pick" />
  </Popover>
</div>
