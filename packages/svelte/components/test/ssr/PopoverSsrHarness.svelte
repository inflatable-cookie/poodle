<script lang="ts">
  import type { PopoverTriggerState } from "@inflatable-cookie/poodle-core";

  import Button from "../../src/Button.svelte";
  import Popover from "../../src/Popover.svelte";

  // SSR counterpart of PopoverRetainedHarness.svelte: same two-mode shape, but
  // the interactive trigger is a real Poodle Button so the server HTML also
  // exercises Button's `controls` projection.
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
  {#if triggerIsInteractive}
    <Popover {open} {defaultOpen} {disabled} triggerIsInteractive>
      {#snippet trigger(state: PopoverTriggerState)}
        <Button
          ariaExpanded={state.expanded}
          controls={state.controls}
          disabled={state.disabled}
        >
          Open
        </Button>
      {/snippet}
      <button type="button" data-testid="surface-action">Surface action</button>
    </Popover>
  {:else}
    <Popover {open} {defaultOpen} {disabled}>
      {#snippet trigger()}
        Open
      {/snippet}
      <button type="button" data-testid="surface-action">Surface action</button>
    </Popover>
  {/if}
</div>
