<script lang="ts">
  import type { PopoverTriggerState } from "@inflatable-cookie/poodle-core";

  import Popover from "../src/Popover.svelte";

  interface Props {
    open?: boolean | null;
    defaultOpen?: boolean;
    disabled?: boolean;
    triggerIsInteractive?: boolean;
    trapFocus?: boolean;
    onOpenChange?: (open: boolean) => void;
  }

  let {
    open = null,
    defaultOpen = false,
    disabled = false,
    triggerIsInteractive = false,
    trapFocus = false,
    onOpenChange = undefined,
  }: Props = $props();
</script>

<div data-poodle-theme-root>
  {#if triggerIsInteractive}
    <Popover {open} {defaultOpen} {disabled} {trapFocus} triggerIsInteractive {onOpenChange}>
      {#snippet trigger(state: PopoverTriggerState)}
        <button
          type="button"
          data-testid="inner-trigger"
          aria-expanded={state.expanded}
          aria-controls={state.controls ?? undefined}
          disabled={state.disabled}
        >
          Open
        </button>
      {/snippet}
      <button type="button" data-testid="surface-action">Surface action</button>
      {#if trapFocus}
        <button type="button" data-testid="surface-next">Next action</button>
      {/if}
    </Popover>
  {:else}
    <Popover {open} {defaultOpen} {disabled} {trapFocus}>
      {#snippet trigger()}
        Open
      {/snippet}
      <button type="button" data-testid="surface-action">Surface action</button>
      {#if trapFocus}
        <button type="button" data-testid="surface-next">Next action</button>
      {/if}
    </Popover>
  {/if}
</div>
