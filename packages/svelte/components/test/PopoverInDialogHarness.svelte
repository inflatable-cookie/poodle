<script lang="ts">
  import Dialog from "../src/Dialog.svelte";
  import Popover from "../src/Popover.svelte";

  let popoverOpen = $state(false);
</script>

<div data-poodle-theme-root style="z-index: 0;">
  <Dialog open title="Settings">
    <Popover bind:open={popoverOpen}>
      {#snippet trigger()}
        <button type="button" onclick={() => (popoverOpen = true)}>Open popover</button>
      {/snippet}
      <p>Popover body</p>
    </Popover>
  </Dialog>
</div>
