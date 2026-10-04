<script lang="ts">
  import ContextMenu from "../src/ContextMenu.svelte";
  import type { MenuItem } from "../src/types";

  let invokerPresent = $state(true);

  const items: MenuItem[] = [{ value: "rename", label: "Rename" }];
</script>

<button type="button" data-testid="remove-invoker" onclick={() => (invokerPresent = false)}>
  Remove invoker
</button>

<ContextMenu {items} dismissOnOutsideInteract={false}>
  {#if invokerPresent}
    <button type="button" data-testid="invoker">Invoker</button>
  {:else}
    <span>Invoker removed</span>
  {/if}
</ContextMenu>
