<script lang="ts">
  import type { Snippet } from "svelte";

  import { default as IconButton } from "./IconButton.svelte";
  import { default as Menu } from "./Menu.svelte";

  import type { FormActionAlign, FormActionDangerItem, MenuItem } from "./types";

  let {
    align = "end",
    dangerItems = [],
    children = undefined,
    danger = undefined,
  }: {
    align?: FormActionAlign;
    dangerItems?: FormActionDangerItem[];
    children?: Snippet;
    danger?: Snippet;
  } = $props();

  const hasDangerSlot = $derived(Boolean(danger));
  const hasDangerMenu = $derived(dangerItems.length > 0);
  const showResponsiveDangerSwap = $derived(hasDangerSlot && hasDangerMenu);
  const collapsedDangerItems = $derived(dangerItems.map<MenuItem>((item, index) => ({
    value: item.value ?? `${index}:${item.label}`,
    label: item.label,
    disabled: item.disabled === true,
  })));

  function handleDangerAction(value: string): void {
    const item = dangerItems.find((candidate, index) => (candidate.value ?? `${index}:${candidate.label}`) === value);
    item?.onSelect();
  }
</script>

<div class="poodle-form-actions" data-align={align}>
  {#if hasDangerSlot}
    <div class="poodle-form-actions__danger" data-mode={showResponsiveDangerSwap ? "responsive" : "inline"}>
      {@render danger?.()}
    </div>
  {/if}

  {#if hasDangerMenu}
    <div class="poodle-form-actions__danger-menu" data-visible={showResponsiveDangerSwap ? "responsive" : "always"}>
      <Menu items={collapsedDangerItems} ariaLabel="More actions" placement="top-end" onAction={handleDangerAction}>
        {#snippet trigger()}
          <span>
          <IconButton icon="ellipsis" ariaLabel="More actions" variant="ghost" sizeRole="chrome" />
          </span>
        {/snippet}
      </Menu>
    </div>
  {/if}

  {#if children}
    {@render children()}
  {/if}
</div>

<style>
  .poodle-form-actions {
    display: flex;
    flex-wrap: wrap;
    gap: var(--poodle-space-inline-md);
    align-items: center;
    padding-top: var(--poodle-space-stack-sm);
    container-type: inline-size;
  }

  .poodle-form-actions[data-align="start"] {
    justify-content: flex-start;
  }

  .poodle-form-actions[data-align="end"] {
    justify-content: flex-end;
  }

  .poodle-form-actions[data-align="between"] {
    justify-content: space-between;
  }

  .poodle-form-actions__danger {
    display: inline-flex;
    align-items: center;
    gap: var(--poodle-space-inline-md);
  }

  .poodle-form-actions__danger-menu {
    display: none;
    align-items: center;
  }

  .poodle-form-actions__danger-menu[data-visible="always"] {
    display: inline-flex;
  }

  @container (max-width: 31.25rem) {
    .poodle-form-actions__danger[data-mode="responsive"] {
      display: none;
    }

    .poodle-form-actions__danger-menu[data-visible="responsive"] {
      display: inline-flex;
    }
  }
</style>
