<script lang="ts">
  import "@poodle/styles/form-actions.css";
  import type { Snippet } from "svelte";

  import { default as IconButton } from "./IconButton.svelte";
  import { default as Menu } from "./Menu.svelte";
  import { getUiPresentation } from "./presentation.ts";

  import type { ControlDensity, FormActionAlign, FormActionDangerItem, MenuItem } from "./types.ts";

  let {
    align = "end",
    density = null,
    showTopSeparation = true,
    showTopBorder = false,
    dangerItems = [],
    children = undefined,
    danger = undefined,
  }: {
    align?: FormActionAlign;
    density?: ControlDensity | null;
    showTopSeparation?: boolean;
    showTopBorder?: boolean;
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

<div
  class="poodle-form-actions"
  data-align={align}
  data-density={resolvedDensity}
  data-top-separation={showTopSeparation ? "true" : "false"}
  data-top-border={showTopBorder ? "true" : "false"}
>
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

