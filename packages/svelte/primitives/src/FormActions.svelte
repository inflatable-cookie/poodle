<script lang="ts">
  import IconButton from "./IconButton.svelte";
  import Menu from "./Menu.svelte";

  import type { FormActionAlign, FormActionDangerItem, MenuItem } from "./types";

  export let align: FormActionAlign = "end";
  export let dangerItems: FormActionDangerItem[] = [];

  $: hasDangerSlot = Boolean($$slots.danger);
  $: hasDangerMenu = dangerItems.length > 0;
  $: showResponsiveDangerSwap = hasDangerSlot && hasDangerMenu;
  $: collapsedDangerItems = dangerItems.map<MenuItem>((item, index) => ({
    value: item.value ?? `${index}:${item.label}`,
    label: item.label,
    disabled: item.disabled === true,
  }));

  function handleDangerAction(value: string): void {
    const item = dangerItems.find((candidate, index) => (candidate.value ?? `${index}:${candidate.label}`) === value);
    item?.onSelect();
  }
</script>

<div class="form-actions" data-align={align}>
  <slot />

  {#if hasDangerSlot}
    <div class="form-actions__danger" data-mode={showResponsiveDangerSwap ? "responsive" : "inline"}>
      <slot name="danger" />
    </div>
  {/if}

  {#if hasDangerMenu}
    <div class="form-actions__danger-menu" data-visible={showResponsiveDangerSwap ? "responsive" : "always"}>
      <Menu items={collapsedDangerItems} ariaLabel="More actions" placement="top-end" on:action={(event) => handleDangerAction(event.detail.value)}>
        <span slot="trigger">
          <IconButton icon="ellipsis" ariaLabel="More actions" variant="ghost" sizeRole="chrome" />
        </span>
      </Menu>
    </div>
  {/if}
</div>

<style>
  .form-actions {
    display: flex;
    flex-wrap: wrap;
    gap: var(--poodle-space-inline-md);
    align-items: center;
    padding-top: var(--poodle-space-stack-sm);
    container-type: inline-size;
  }

  .form-actions[data-align="start"] {
    justify-content: flex-start;
  }

  .form-actions[data-align="end"] {
    justify-content: flex-end;
  }

  .form-actions[data-align="between"] {
    justify-content: space-between;
  }

  .form-actions__danger {
    display: inline-flex;
    align-items: center;
    gap: var(--poodle-space-inline-md);
  }

  .form-actions__danger-menu {
    display: none;
    align-items: center;
  }

  .form-actions__danger-menu[data-visible="always"] {
    display: inline-flex;
  }

  @container (max-width: 31.25rem) {
    .form-actions__danger[data-mode="responsive"] {
      display: none;
    }

    .form-actions__danger-menu[data-visible="responsive"] {
      display: inline-flex;
    }
  }
</style>
