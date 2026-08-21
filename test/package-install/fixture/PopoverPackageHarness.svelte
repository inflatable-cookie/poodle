<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/button.css";
  import "@inflatable-cookie/poodle-core/styles/popover.css";
  import {
    Button,
    Popover,
    type PopoverTriggerState,
  } from "@inflatable-cookie/poodle-svelte";
</script>

<Popover triggerIsInteractive ariaLabel="Packed popover">
  {#snippet trigger(state: PopoverTriggerState)}
    <Button
      ariaExpanded={state.expanded}
      controls={state.controls}
      disabled={state.disabled}
    >
      Packed trigger
    </Button>
  {/snippet}
  <p>Packed surface content</p>
</Popover>
