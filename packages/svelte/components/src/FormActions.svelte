<script lang="ts">
  import type { Snippet } from "svelte";

  import { default as IconButton } from "./IconButton.svelte";
  import { default as Menu } from "./Menu.svelte";
  import { getUiPresentation } from "./presentation";

  import type { ControlDensity, FormActionAlign, FormActionDangerItem, MenuItem } from "./types";

  let {
    align = "end",
    density = null,
    dangerItems = [],
    children = undefined,
    danger = undefined,
  }: {
    align?: FormActionAlign;
    density?: ControlDensity | null;
    dangerItems?: FormActionDangerItem[];
    children?: Snippet;
    danger?: Snippet;
  } = $props();

  const uiPresentation = getUiPresentation();

  const resolvedDensity = $derived(density ?? $uiPresentation.density);
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

<div class="poodle-form-actions" data-align={align} data-density={resolvedDensity}>
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
    --poodle-form-actions-gap: var(--poodle-space-inline-md);
    --poodle-form-actions-padding-top: var(--poodle-space-stack-sm);
    display: flex;
    flex-wrap: wrap;
    gap: var(--poodle-form-actions-gap);
    align-items: center;
    padding-top: var(--poodle-form-actions-padding-top);
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
    gap: var(--poodle-form-actions-gap);
  }

  .poodle-form-actions__danger-menu {
    display: none;
    align-items: center;
  }

  .poodle-form-actions[data-density="compact"] {
    --poodle-form-actions-gap: 0.5rem;
    --poodle-form-actions-padding-top: 0.375rem;
  }

  .poodle-form-actions[data-density="default"] {
    --poodle-form-actions-gap: var(--poodle-space-inline-md);
    --poodle-form-actions-padding-top: var(--poodle-space-stack-sm);
  }

  .poodle-form-actions[data-density="comfortable"] {
    --poodle-form-actions-gap: 0.875rem;
    --poodle-form-actions-padding-top: 0.75rem;
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
