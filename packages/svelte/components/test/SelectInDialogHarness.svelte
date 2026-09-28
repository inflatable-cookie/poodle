<script lang="ts">
  // Real-path harness: a Select composed inside a Dialog, driven by keyboard
  // events on its trigger — focus never lands on a listbox option, because
  // options are tabindex=-1 and the highlight moves by aria-activedescendant.
  import Dialog from "../src/Dialog.svelte";
  import Select from "../src/Select.svelte";

  import type { SelectItems } from "../src/types";

  interface Props {
    options?: SelectItems;
    onValueChange?: ((value: string) => void) | undefined;
  }

  let {
    options = [
      { value: "alpha", label: "Alpha" },
      { value: "beta", label: "Beta" },
    ],
    onValueChange = undefined,
  }: Props = $props();
</script>

<div data-poodle-theme-root>
  <Dialog open title="Pick one">
    <Select {options} native={false} ariaLabel="Pick" {onValueChange} />
  </Dialog>
</div>
