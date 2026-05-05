<script lang="ts">
  import { createEventDispatcher, onMount, tick } from "svelte";

  import { menuNavigableItems } from "./internal";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, MenuItem, SemanticControlSizeRole } from "./types";

  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;
  export let title: string;
  export let subtitle: string | null = null;
  export let meta: string | null = null;
  export let href: string | null = null;
  export let leadingShape: "circle" | "rounded-square" = "circle";
  export let leadingFill: "tint" | "solid" = "tint";
  export let accentColor: string | null = null;
  export let layout: "default" | "compact" = "default";
  export let interactive = false;
  export let disabled = false;
  export let selectable = false;
  export let selected = false;
  export let showReorderHandle = false;
  export let notLive = false;
  export let sash: string | null = null;
  export let sashColor: string | null = null;
  export let ariaLabel: string | null = null;
  export let contextMenuItems: MenuItem[] | null = null;
  export let contextMenuAriaLabel: string | null = null;
  export let onContextAction: ((value: string) => void) | null = null;

  const dispatch = createEventDispatcher<{
    click: MouseEvent;
    selectedChange: { selected: boolean };
  }>();

  const uiPresentation = getUiPresentation();

  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;
  $: isCompact = layout === "compact";
  $: isInteractive = Boolean(href) || interactive || selectable;
  $: actionableContextMenuItems = menuNavigableItems(contextMenuItems ?? []);
  $: hasContextMenu = (contextMenuItems?.length ?? 0) > 0;

  let rootElement: HTMLElement | null = null;
  let overlayElement: HTMLDivElement | null = null;
  let contextMenuOpen = false;
  let contextMenuAnchorPoint: { x: number; y: number } | null = null;
  let contextMenuAdjustedPosition: { left: string; top: string } | null = null;
  let contextMenuHighlightIndex = 0;
  let contextMenuItemElements: Array<HTMLButtonElement | null> = [];

  function handleClick(event: MouseEvent) {
    if (disabled) return;

    if (selectable) {
      event.preventDefault();
      dispatch("selectedChange", { selected: !selected });
      return;
    }

    if (interactive || href) {
      dispatch("click", event);
    }
  }

  function handleKeydown(event: KeyboardEvent) {
    if (disabled || href) return;

    if ((interactive || selectable) && (event.key === "Enter" || event.key === " ")) {
      event.preventDefault();
      if (selectable) {
        dispatch("selectedChange", { selected: !selected });
      } else {
        dispatch("click", new MouseEvent("click"));
      }
    }
  }

  function openContextMenuAt(x: number, y: number) {
    if (!hasContextMenu) return;
    contextMenuAnchorPoint = { x, y };
    contextMenuOpen = true;
  }

  function closeContextMenu() {
    contextMenuOpen = false;
    contextMenuHighlightIndex = 0;
  }

  function moveContextMenuHighlight(direction: 1 | -1): void {
    const count = actionableContextMenuItems.length;
    if (count === 0) return;

    let nextIndex = contextMenuHighlightIndex;
    for (let step = 0; step < count; step += 1) {
      nextIndex = (nextIndex + direction + count) % count;
      if (!actionableContextMenuItems[nextIndex]?.disabled) {
        contextMenuHighlightIndex = nextIndex;
        contextMenuItemElements[nextIndex]?.focus();
        return;
      }
    }
  }

  function activateContextMenuItem(item: MenuItem): void {
    if (item.disabled || item.kind === "separator") return;
    onContextAction?.(item.value);
    closeContextMenu();
  }

  function handleContextMenu(event: MouseEvent) {
    if (disabled || !hasContextMenu) return;
    event.preventDefault();
    event.stopPropagation();
    openContextMenuAt(event.clientX, event.clientY);
  }

  function handleContextMenuKeydown(event: KeyboardEvent) {
    if (disabled || !hasContextMenu) return;
    if (event.key === "ContextMenu" || (event.shiftKey && event.key === "F10")) {
      event.preventDefault();
      event.stopPropagation();
      const rect = rootElement?.getBoundingClientRect();
      if (!rect) return;
      openContextMenuAt(rect.left + 16, rect.top + 16);
    }
  }

  $: if (contextMenuOpen) {
    contextMenuAdjustedPosition = null;
    tick().then(() => {
      if (overlayElement && contextMenuAnchorPoint) {
        const rect = overlayElement.getBoundingClientRect();
        const vw = window.innerWidth;
        const vh = window.innerHeight;
        const pad = 8;
        let x = contextMenuAnchorPoint.x;
        let y = contextMenuAnchorPoint.y;

        if (x + rect.width > vw - pad) {
          x = Math.max(pad, x - rect.width);
        }

        if (y + rect.height > vh - pad) {
          y = Math.max(pad, vh - rect.height - pad);
        }

        contextMenuAdjustedPosition = { left: `${x}px`, top: `${y}px` };
      }

      contextMenuItemElements[contextMenuHighlightIndex]?.focus();
    });
  }

  onMount(() => {
    function handlePointerDown(event: MouseEvent): void {
      if (!contextMenuOpen) return;
      if (!overlayElement || !overlayElement.contains(event.target as Node)) {
        closeContextMenu();
      }
    }

    function handleDocumentKeydown(event: KeyboardEvent): void {
      if (event.key === "Escape" && contextMenuOpen) {
        event.preventDefault();
        closeContextMenu();
      }
    }

    document.addEventListener("mousedown", handlePointerDown);
    document.addEventListener("keydown", handleDocumentKeydown);

    return () => {
      document.removeEventListener("mousedown", handlePointerDown);
      document.removeEventListener("keydown", handleDocumentKeydown);
    };
  });
</script>

{#if href && !disabled && !selectable}
  <a
    bind:this={rootElement}
    class="poodle-list-card"
    class:poodle-list-card--interactive={isInteractive}
    href={href}
    data-size={resolvedSize}
    data-density={resolvedDensity}
    data-disabled={disabled}
    data-not-live={notLive}
    data-leading-shape={leadingShape}
    data-leading-fill={leadingFill}
    data-layout={layout}
    data-selected={selected}
    aria-label={ariaLabel ?? title}
    class:poodle-list-card--has-sash={!!sash}
    style={[
      accentColor ? `--list-card-accent: ${accentColor}` : '',
      sashColor ? `--list-card-sash: ${sashColor}` : '',
    ].filter(Boolean).join('; ') || undefined}
    on:click={handleClick}
    on:contextmenu={handleContextMenu}
    on:keydown={handleContextMenuKeydown}
  >
    {#if sash}
      <span class="poodle-list-card__sash" aria-label={sash}>{sash}</span>
    {/if}

    {#if showReorderHandle}
      <span class="poodle-list-card__handle" aria-hidden="true">
        <svg viewBox="0 0 16 16" fill="currentColor">
          <circle cx="5" cy="4" r="1.1"></circle>
          <circle cx="5" cy="8" r="1.1"></circle>
          <circle cx="5" cy="12" r="1.1"></circle>
          <circle cx="11" cy="4" r="1.1"></circle>
          <circle cx="11" cy="8" r="1.1"></circle>
          <circle cx="11" cy="12" r="1.1"></circle>
        </svg>
      </span>
    {/if}

    {#if $$slots.leading}
      <span class="poodle-list-card__leading">
        <slot name="leading" />
      </span>
    {/if}

    <div class="poodle-list-card__body">
      <div class="poodle-list-card__header">
        <span class="poodle-list-card__title">
          <slot name="title">{title}</slot>
        </span>
        {#if $$slots.badges}
          <span class="poodle-list-card__badges">
            <slot name="badges" />
          </span>
        {/if}
      </div>
      {#if subtitle}
        <span class="poodle-list-card__subtitle">{subtitle}</span>
      {/if}
      {#if $$slots.footer}
        <div class="poodle-list-card__footer">
          <slot name="footer" />
        </div>
      {/if}
    </div>

    {#if meta && !isCompact}
      <span class="poodle-list-card__meta">{meta}</span>
    {/if}

    {#if $$slots.actions}
      <span class="poodle-list-card__actions">
        <slot name="actions" />
      </span>
    {/if}

    {#if $$slots.trailing}
      <span class="poodle-list-card__trailing">
        <slot name="trailing" />
      </span>
    {/if}
  </a>
{:else}
  <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
  <div
    bind:this={rootElement}
    class="poodle-list-card"
    class:poodle-list-card--interactive={isInteractive}
    data-size={resolvedSize}
    data-density={resolvedDensity}
    data-disabled={disabled}
    data-not-live={notLive}
    data-leading-shape={leadingShape}
    data-leading-fill={leadingFill}
    data-layout={layout}
    data-selected={selected}
    role={isInteractive ? (selectable ? "button" : "button") : undefined}
    aria-pressed={selectable ? selected : undefined}
    tabindex={isInteractive && !disabled ? 0 : -1}
    aria-label={ariaLabel ?? title}
    class:poodle-list-card--has-sash={!!sash}
    style={[
      accentColor ? `--list-card-accent: ${accentColor}` : '',
      sashColor ? `--list-card-sash: ${sashColor}` : '',
    ].filter(Boolean).join('; ') || undefined}
    on:click={handleClick}
    on:keydown={handleKeydown}
    on:contextmenu={handleContextMenu}
    on:keydown={handleContextMenuKeydown}
  >
    {#if sash}
      <span class="poodle-list-card__sash" aria-label={sash}>{sash}</span>
    {/if}

    {#if showReorderHandle}
      <span class="poodle-list-card__handle" aria-hidden="true">
        <svg viewBox="0 0 16 16" fill="currentColor">
          <circle cx="5" cy="4" r="1.1"></circle>
          <circle cx="5" cy="8" r="1.1"></circle>
          <circle cx="5" cy="12" r="1.1"></circle>
          <circle cx="11" cy="4" r="1.1"></circle>
          <circle cx="11" cy="8" r="1.1"></circle>
          <circle cx="11" cy="12" r="1.1"></circle>
        </svg>
      </span>
    {/if}

    {#if $$slots.leading}
      <span class="poodle-list-card__leading">
        <slot name="leading" />
      </span>
    {/if}

    <div class="poodle-list-card__body">
      <div class="poodle-list-card__header">
        <span class="poodle-list-card__title">
          <slot name="title">{title}</slot>
        </span>
        {#if $$slots.badges}
          <span class="poodle-list-card__badges">
            <slot name="badges" />
          </span>
        {/if}
      </div>
      {#if subtitle}
        <span class="poodle-list-card__subtitle">{subtitle}</span>
      {/if}
      {#if $$slots.footer}
        <div class="poodle-list-card__footer">
          <slot name="footer" />
        </div>
      {/if}
    </div>

    {#if meta && !isCompact}
      <span class="poodle-list-card__meta">{meta}</span>
    {/if}

    {#if $$slots.actions}
      <span class="poodle-list-card__actions">
        <slot name="actions" />
      </span>
    {/if}

    {#if $$slots.trailing}
      <span class="poodle-list-card__trailing">
        <slot name="trailing" />
      </span>
    {/if}
  </div>
{/if}

{#if contextMenuOpen && hasContextMenu && contextMenuAnchorPoint}
  <div
    bind:this={overlayElement}
    class="poodle-list-card__context-menu"
    role="menu"
    aria-label={contextMenuAriaLabel ?? undefined}
    style={contextMenuAdjustedPosition
      ? `left: ${contextMenuAdjustedPosition.left}; top: ${contextMenuAdjustedPosition.top};`
      : `left: ${contextMenuAnchorPoint.x}px; top: ${contextMenuAnchorPoint.y}px; visibility: hidden;`}
  >
    {#each contextMenuItems ?? [] as item (item.value)}
      {#if item.kind === "separator"}
        <div class="poodle-list-card__context-separator" role="separator"></div>
      {:else}
        <button
          bind:this={contextMenuItemElements[actionableContextMenuItems.findIndex((candidate) => candidate.value === item.value)]}
          type="button"
          class="poodle-list-card__context-item"
          disabled={item.disabled === true}
          role={item.kind === "checkbox" || item.kind === "radio" ? `menuitem${item.kind}` : "menuitem"}
          aria-checked={item.kind === "checkbox" || item.kind === "radio" ? (item.checked ? "true" : "false") : undefined}
          on:click={() => activateContextMenuItem(item)}
          on:keydown={(event) => {
            if (event.key === "ArrowDown") {
              event.preventDefault();
              moveContextMenuHighlight(1);
            }

            if (event.key === "ArrowUp") {
              event.preventDefault();
              moveContextMenuHighlight(-1);
            }

            if (event.key === "Home") {
              event.preventDefault();
              contextMenuHighlightIndex = 0;
              contextMenuItemElements[0]?.focus();
            }

            if (event.key === "End") {
              event.preventDefault();
              contextMenuHighlightIndex = actionableContextMenuItems.length - 1;
              contextMenuItemElements[actionableContextMenuItems.length - 1]?.focus();
            }

            if (event.key === "Enter" || event.key === " ") {
              event.preventDefault();
              activateContextMenuItem(item);
            }
          }}
        >
          <span>{item.label}</span>

          {#if item.checked}
            <span class="poodle-list-card__context-meta" aria-hidden="true">✓</span>
          {:else if item.shortcutLabel}
            <span class="poodle-list-card__context-meta" aria-hidden="true">{item.shortcutLabel}</span>
          {/if}
        </button>
      {/if}
    {/each}
  </div>
{/if}

<style>
  .poodle-list-card {
    display: flex;
    align-items: center;
    gap: var(--poodle-space-inline-md);
    padding: 0.625rem var(--poodle-space-panel-x);
    border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 18%, transparent);
    border-radius: var(--poodle-radius-control);
    background: color-mix(in srgb, var(--poodle-surface) 88%, var(--poodle-color-text-primary));
    text-decoration: none;
    width: 100%;
    transition:
      background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      border-color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .poodle-list-card--has-sash {
    position: relative;
    overflow: hidden;
  }

  .poodle-list-card--interactive {
    cursor: pointer;
  }

  .poodle-list-card--interactive:hover:not([data-disabled="true"]) {
    background: color-mix(in srgb, var(--poodle-surface) 82%, var(--poodle-color-text-primary));
    border-color: color-mix(in srgb, var(--poodle-color-border-default) 52%, transparent);
  }

  .poodle-list-card[data-selected="true"] {
    border-color: var(--list-card-accent, var(--poodle-color-accent-base));
    box-shadow:
      0 0 0 0.0625rem var(--list-card-accent, var(--poodle-color-accent-base)),
      inset 0 0 0 0.0625rem color-mix(
        in srgb,
        var(--list-card-accent, var(--poodle-color-accent-base)) 12%,
        transparent
      );
  }

  .poodle-list-card--interactive[data-selected="true"]:hover:not([data-disabled="true"]) {
    border-color: var(--list-card-accent, var(--poodle-color-accent-base));
  }

  .poodle-list-card:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: -0.0625rem;
  }

  .poodle-list-card[data-disabled="true"] {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  .poodle-list-card[data-not-live="true"] {
    border: 0.1875rem dashed color-mix(in srgb, var(--poodle-color-border-default) 72%, transparent);
    background: color-mix(in srgb, var(--poodle-surface) 32%, transparent);
    filter: grayscale(1);
    opacity: 0.72;
  }

  .poodle-list-card[data-not-live="true"]:hover:not([data-disabled="true"]) {
    border-color: var(--poodle-color-border-default);
    filter: grayscale(0);
    opacity: 1;
  }

  .poodle-list-card[data-layout="compact"] {
    gap: var(--poodle-space-inline-sm);
    padding: 0.5rem 0.625rem;
    min-height: 3rem;
  }

  .poodle-list-card[data-layout="compact"] .poodle-list-card__leading {
    width: 1.5rem;
    height: 1.5rem;
    font-size: 0.75rem;
  }

  .poodle-list-card[data-layout="compact"][data-leading-shape="rounded-square"] .poodle-list-card__leading {
    width: 1.75rem;
    height: 1.75rem;
  }

  .poodle-list-card[data-layout="compact"] .poodle-list-card__body {
    gap: 0;
  }

  .poodle-list-card[data-layout="compact"] .poodle-list-card__title {
    font-size: 0.875rem;
  }

  .poodle-list-card[data-layout="compact"] .poodle-list-card__subtitle,
  .poodle-list-card[data-layout="compact"] .poodle-list-card__footer {
    display: none;
  }

  .poodle-list-card__handle {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    width: 1rem;
    height: 1rem;
    color: var(--poodle-color-text-secondary);
    opacity: 0.8;
  }

  .poodle-list-card__handle svg {
    width: 100%;
    height: 100%;
  }

  .poodle-list-card__leading {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    width: 2rem;
    height: 2rem;
    overflow: hidden;
    border-radius: 999px;
    background: color-mix(in srgb, var(--list-card-accent, var(--poodle-color-accent-base)) 12%, transparent);
    color: var(--list-card-accent, var(--poodle-color-accent-base));
    font-size: 0.875rem;
    font-weight: 600;
  }

  .poodle-list-card[data-leading-shape="rounded-square"] .poodle-list-card__leading {
    width: 2.75rem;
    height: 2.75rem;
    border-radius: var(--poodle-radius-control);
  }

  .poodle-list-card[data-leading-fill="solid"] .poodle-list-card__leading {
    background: var(--list-card-accent, var(--poodle-color-accent-base));
    color: #fff;
  }

  .poodle-list-card__body {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 0.0625rem;
  }

  .poodle-list-card__header {
    display: flex;
    align-items: baseline;
    gap: var(--poodle-space-inline-sm);
  }

  .poodle-list-card__title {
    flex: 1;
    min-width: 0;
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-typography-body-size);
    font-weight: 500;
    color: var(--poodle-color-text-primary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .poodle-list-card__badges {
    flex-shrink: 0;
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: var(--poodle-space-inline-sm);
  }

  .poodle-list-card__subtitle {
    font-size: 0.75rem;
    color: var(--poodle-color-text-secondary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .poodle-list-card__footer {
    display: flex;
    align-items: center;
    gap: var(--poodle-space-inline-md);
    margin-top: 0.125rem;
  }

  .poodle-list-card__meta {
    flex-shrink: 0;
    font-size: 0.75rem;
    color: var(--poodle-color-text-secondary);
    font-variant-numeric: tabular-nums;
  }

  .poodle-list-card__trailing {
    display: flex;
    align-items: center;
    flex-shrink: 0;
  }

  .poodle-list-card__actions {
    display: flex;
    align-items: center;
    flex-shrink: 0;
  }

  /* Size variants */
  .poodle-list-card[data-size="xs"] { padding: 0.375rem 0.5rem; gap: 0.375rem; }
  .poodle-list-card[data-size="xs"] .poodle-list-card__title { font-size: 0.75rem; }
  .poodle-list-card[data-size="xs"] .poodle-list-card__subtitle { font-size: 0.625rem; }
  .poodle-list-card[data-size="xs"] .poodle-list-card__meta { font-size: 0.625rem; }
  .poodle-list-card[data-size="xs"] .poodle-list-card__leading { width: 1.5rem; height: 1.5rem; font-size: 0.6875rem; }

  .poodle-list-card[data-size="sm"] { padding: 0.5rem 0.625rem; gap: 0.5rem; }
  .poodle-list-card[data-size="sm"] .poodle-list-card__title { font-size: 0.8125rem; }
  .poodle-list-card[data-size="sm"] .poodle-list-card__subtitle { font-size: 0.6875rem; }
  .poodle-list-card[data-size="sm"] .poodle-list-card__meta { font-size: 0.6875rem; }
  .poodle-list-card[data-size="sm"] .poodle-list-card__leading { width: 1.75rem; height: 1.75rem; font-size: 0.75rem; }

  .poodle-list-card[data-size="lg"] { padding: 0.75rem 1rem; gap: 0.875rem; }
  .poodle-list-card[data-size="lg"] .poodle-list-card__title { font-size: 1rem; }
  .poodle-list-card[data-size="lg"] .poodle-list-card__subtitle { font-size: 0.8125rem; }
  .poodle-list-card[data-size="lg"] .poodle-list-card__meta { font-size: 0.8125rem; }
  .poodle-list-card[data-size="lg"] .poodle-list-card__leading { width: 2.5rem; height: 2.5rem; font-size: 1rem; }

  .poodle-list-card[data-size="xl"] { padding: 0.875rem 1.125rem; gap: 1rem; }
  .poodle-list-card[data-size="xl"] .poodle-list-card__title { font-size: 1.0625rem; }
  .poodle-list-card[data-size="xl"] .poodle-list-card__subtitle { font-size: 0.875rem; }
  .poodle-list-card[data-size="xl"] .poodle-list-card__meta { font-size: 0.875rem; }
  .poodle-list-card[data-size="xl"] .poodle-list-card__leading { width: 2.75rem; height: 2.75rem; font-size: 1.125rem; }

  /* Density variants */
  .poodle-list-card[data-density="compact"] { padding-inline: 0.5rem; }
  .poodle-list-card[data-density="comfortable"] { padding-inline: 1.125rem; }

  .poodle-list-card__sash {
    position: absolute;
    top: 0.34375rem;
    left: -2.25rem;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 6rem;
    padding: 0.125rem 0;
    background: var(--list-card-sash, var(--poodle-color-positive-base, #22c55e));
    color: #fff;
    font-size: 0.5625rem;
    font-weight: 700;
    text-transform: uppercase;
    line-height: 1;
    transform: rotate(-45deg);
    pointer-events: none;
    z-index: 1;
  }

  .poodle-list-card__context-menu {
    position: fixed;
    z-index: var(--poodle-overlay-z-menu);
    min-width: 14rem;
    padding: 0.25rem;
    border: 0.0625rem solid var(
      --poodle-treatment-surface-elevated-border,
      color-mix(in srgb, var(--poodle-color-border-default) 72%, transparent)
    );
    border-radius: var(--poodle-treatment-surface-elevated-radius, var(--poodle-radius-surface));
    background: var(
      --poodle-treatment-surface-elevated-fill,
      color-mix(in srgb, var(--poodle-color-background-elevated) 98%, var(--poodle-color-background-panel))
    );
    box-shadow: var(--poodle-treatment-surface-elevated-shadow, var(--poodle-elevation-overlay));
  }

  .poodle-list-card__context-item {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    align-items: center;
    width: 100%;
    min-height: var(--poodle-size-control-height);
    padding: var(--poodle-space-control-y) var(--poodle-space-control-x);
    border: 0;
    border-radius: calc(var(--poodle-radius-control) - 0.125rem);
    background: transparent;
    color: var(--poodle-color-text-primary);
    cursor: pointer;
    font: inherit;
    font-size: var(--poodle-typography-body-size);
    text-align: left;
  }

  .poodle-list-card__context-item:hover:not(:disabled),
  .poodle-list-card__context-item:focus-visible {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 16%, transparent);
    outline: none;
  }

  .poodle-list-card__context-item:disabled {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  .poodle-list-card__context-meta {
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-code-family);
    font-size: 0.6875rem;
  }

  .poodle-list-card__context-separator {
    width: 100%;
    height: 0.0625rem;
    margin: 0.25rem 0;
    background: color-mix(in srgb, var(--poodle-color-border-subtle) 72%, transparent);
  }
</style>
