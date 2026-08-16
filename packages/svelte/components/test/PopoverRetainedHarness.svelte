<script lang="ts">
  import Popover from "../src/Popover.svelte";

  interface Props {
    open?: boolean | null;
    defaultOpen?: boolean;
    disabled?: boolean;
    triggerIsInteractive?: boolean;
  }

  let {
    open = null,
    defaultOpen = false,
    disabled = false,
    triggerIsInteractive = false,
  }: Props = $props();
</script>

<div data-poodle-theme-root>
  <Popover {open} {defaultOpen} {disabled} {triggerIsInteractive}>
    {#snippet trigger()}
      {#if triggerIsInteractive}
        <button type="button" data-testid="inner-trigger">Open</button>
      {:else}
        Open
      {/if}
    {/snippet}
    <button type="button" data-testid="surface-action">Surface action</button>
  </Popover>
</div>
