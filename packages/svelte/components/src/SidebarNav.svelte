<script module lang="ts">
  let nextSidebarNavId = 0;
</script>

<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/sidebar-nav.css";
  import type {
    ControlDensity,
    ControlSize,
    SemanticControlSizeRole,
  } from "./types";

  import type { SidebarNavGroup, SidebarNavItem } from "./types";
  import ContextMenu from "./ContextMenu.svelte";

  interface Props {
    groups?: SidebarNavGroup[];
    value?: string | null;
    ariaLabel?: string | null;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onValueChange?: ((value: string) => void) | undefined;
    onContextAction?: ((itemValue: string, actionValue: string) => void) | undefined;
  }

  let {
    groups = [],
    value = $bindable<string | null>(null),
    ariaLabel = null,
    size = null,
    sizeRole = "chrome",
    density = null,
    onValueChange = undefined,
    onContextAction = undefined,
  }: Props = $props();

  const sidebarNavId = ++nextSidebarNavId;

  const visibleGroups = $derived(groups.filter((group) => group.items.length > 0));

  let contextMenuOpen = $state(false);
  let contextMenuAnchor = $state<{ x: number; y: number } | null>(null);
  let contextMenuItemValue = $state<string | null>(null);

  const contextMenuHost = $derived(
    visibleGroups
      .flatMap((group) => group.items)
      .find((item) => item.value === contextMenuItemValue) ?? null,
  );
  const contextMenuItems = $derived(contextMenuHost?.contextMenuItems ?? []);
  const contextMenuAriaLabel = $derived(
    contextMenuHost?.contextMenuAriaLabel ??
      (contextMenuHost ? `${contextMenuHost.label} actions` : null),
  );

  function handleItemActivation(item: SidebarNavItem): void {
    if (item.disabled) return;
    value = item.value;
    onValueChange?.(item.value);
  }

  function itemHasContextMenu(item: SidebarNavItem): boolean {
    return !item.disabled && (item.contextMenuItems?.length ?? 0) > 0;
  }

  function openContextMenu(item: SidebarNavItem, x: number, y: number): void {
    if (!itemHasContextMenu(item)) return;
    contextMenuItemValue = item.value;
    contextMenuAnchor = { x, y };
    contextMenuOpen = true;
  }

  function handleItemContextMenu(item: SidebarNavItem, event: MouseEvent): void {
    if (!itemHasContextMenu(item)) return;
    event.preventDefault();
    event.stopPropagation();
    openContextMenu(item, event.clientX, event.clientY);
  }

  function handleItemKeydown(item: SidebarNavItem, event: KeyboardEvent): void {
    if (!itemHasContextMenu(item)) return;
    if (event.key !== "ContextMenu" && !(event.shiftKey && event.key === "F10")) {
      return;
    }

    event.preventDefault();
    event.stopPropagation();
    const target = event.currentTarget;
    if (!(target instanceof HTMLElement)) return;
    const rect = target.getBoundingClientRect();
    openContextMenu(item, rect.left + 16, rect.top + 16);
  }

  function handleContextAction(actionValue: string): void {
    if (contextMenuItemValue == null) return;
    onContextAction?.(contextMenuItemValue, actionValue);
  }
</script>

<nav
  class="poodle-sidebar-nav"
  data-size={size ?? undefined}
  data-density={density ?? undefined}
  data-size-role={sizeRole}
  aria-label={ariaLabel ?? undefined}
>
  {#snippet itemContent(item: SidebarNavItem, endLabelId: string)}
    {#if item.endLabel}
      <span class="poodle-sidebar-nav__label">{item.label}</span>
      <span class="poodle-sidebar-nav__end-label" id={endLabelId} aria-hidden="true">{item.endLabel}</span>
    {:else}
      {item.label}
    {/if}
  {/snippet}

  {#each visibleGroups as group, groupIndex (group.id)}
    <section
      class="poodle-sidebar-nav__group"
      data-separated={visibleGroups.length > 1}
      aria-label={group.label ?? undefined}
    >
      {#if group.label}
        <h2 class="poodle-sidebar-nav__group-title" title={group.label}>{group.label}</h2>
      {/if}

      <ul class="poodle-sidebar-nav__list">
        {#each group.items as item, itemIndex (item.value)}
          {@const endLabelId = `poodle-sidebar-nav-${sidebarNavId}-${groupIndex}-${itemIndex}-end-label`}
          {@const hasMenu = itemHasContextMenu(item)}
          <li>
            {#if item.href && !item.disabled}
              <a
                class="poodle-sidebar-nav__item"
                class:poodle-sidebar-nav__item--active={item.value === value}
                href={item.href}
                aria-describedby={item.endLabel ? endLabelId : undefined}
                data-end-label={item.endLabel ? "true" : undefined}
                aria-current={item.value === value ? "page" : undefined}
                onclick={() => handleItemActivation(item)}
                oncontextmenu={hasMenu ? (event) => handleItemContextMenu(item, event) : undefined}
                onkeydown={hasMenu ? (event) => handleItemKeydown(item, event) : undefined}
              >
                {@render itemContent(item, endLabelId)}
              </a>
            {:else}
              <button
                type="button"
                class="poodle-sidebar-nav__item"
                class:poodle-sidebar-nav__item--active={item.value === value}
                aria-current={item.value === value ? "page" : undefined}
                disabled={item.disabled}
                aria-describedby={item.endLabel ? endLabelId : undefined}
                data-end-label={item.endLabel ? "true" : undefined}
                onclick={() => handleItemActivation(item)}
                oncontextmenu={hasMenu ? (event) => handleItemContextMenu(item, event) : undefined}
                onkeydown={hasMenu ? (event) => handleItemKeydown(item, event) : undefined}
              >
                {@render itemContent(item, endLabelId)}
              </button>
            {/if}
          </li>
        {/each}
      </ul>
    </section>
  {/each}

  <ContextMenu
    trigger={false}
    items={contextMenuItems}
    bind:open={contextMenuOpen}
    anchorPoint={contextMenuAnchor}
    ariaLabel={contextMenuAriaLabel}
    size={size}
    sizeRole={sizeRole}
    density={density}
    onAction={handleContextAction}
  />
</nav>
