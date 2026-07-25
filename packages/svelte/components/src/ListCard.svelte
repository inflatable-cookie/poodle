<script lang="ts">
  import "@poodle/styles/list-card.css";
  import { tick, type Snippet } from "svelte";

  import { registerDismissLayer, pointAnchor } from "@poodle/headless";

  import { menuNavigableItems } from "./internal";
  import { anchored } from "./anchored";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, MenuItem, SemanticControlSizeRole } from "./types";

  interface Props {
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    title: string;
    subtitle?: string | null;
    meta?: string | null;
    href?: string | null;
    leadingShape?: "circle" | "rounded-square";
    leadingFill?: "tint" | "solid";
    leadingSizeOffset?: number;
    accentColor?: string | null;
    layout?: "default" | "compact" | "stacked";
    interactive?: boolean;
    disabled?: boolean;
    selectable?: boolean;
    selected?: boolean;
    highlighted?: boolean;
    selectionIndicator?: "none" | "checkbox";
    showReorderHandle?: boolean;
    notLive?: boolean;
    sash?: string | null;
    sashColor?: string | null;
    ariaLabel?: string | null;
    contextMenuItems?: MenuItem[] | null;
    contextMenuAriaLabel?: string | null;
    contextMenuTrigger?: "context" | "leading";
    onClick?: ((event: MouseEvent) => void) | null;
    onSelectedChange?: ((selected: boolean) => void) | null;
    onContextAction?: ((value: string) => void) | null;
    titleContent?: Snippet<[]>;
    subtitleContent?: Snippet<[]>;
    metaContent?: Snippet<[]>;
    sashContent?: Snippet<[]>;
    leading?: Snippet<[]>;
    badges?: Snippet<[]>;
    corner?: Snippet<[]>;
    footer?: Snippet<[]>;
    actions?: Snippet<[]>;
    trailing?: Snippet<[]>;
  }

  let {
    size = null,
    sizeRole = "control",
    density = null,
    title,
    subtitle = null,
    meta = null,
    href = null,
    leadingShape = "circle",
    leadingFill = "tint",
    leadingSizeOffset = 0,
    accentColor = null,
    layout = "default",
    interactive = false,
    disabled = false,
    selectable = false,
    selected = false,
    highlighted = false,
    selectionIndicator = "none",
    showReorderHandle = false,
    notLive = false,
    sash = null,
    sashColor = null,
    ariaLabel = null,
    contextMenuItems = null,
    contextMenuAriaLabel = null,
    contextMenuTrigger = "context",
    onClick = null,
    onSelectedChange = null,
    onContextAction = null,
    titleContent,
    subtitleContent,
    metaContent,
    sashContent,
    leading,
    badges,
    corner,
    footer,
    actions,
    trailing,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  const controlSizes: ControlSize[] = ["xs", "sm", "md", "lg", "xl"];

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const resolvedLeadingSize = $derived(offsetControlSize(resolvedSize, leadingSizeOffset));
  const isCompact = $derived(layout === "compact");
  const isStacked = $derived(layout === "stacked");
  const showMeta = $derived(!trailing && (meta || metaContent) && !isCompact);
  const showActions = $derived(!trailing && Boolean(actions));
  const showUtilityRail = $derived(
    isStacked && (Boolean(trailing) || showMeta || showActions)
  );
  const isInteractive = $derived(Boolean(href) || interactive || selectable);
  const showSelectionIndicator = $derived(selectable && selectionIndicator === "checkbox");
  const showSelectionOverlay = $derived(showSelectionIndicator && Boolean(leading));
  const actionableContextMenuItems = $derived(menuNavigableItems(contextMenuItems ?? []));
  const hasContextMenu = $derived((contextMenuItems?.length ?? 0) > 0);
  const useLeadingContextMenu = $derived(contextMenuTrigger === "leading" && hasContextMenu && !selectable);

  let rootElement = $state<HTMLElement | null>(null);
  let leadingElement = $state<HTMLElement | null>(null);
  let overlayElement = $state<HTMLDivElement | null>(null);
  let contextMenuOpen = $state(false);
  let contextMenuAnchorPoint = $state<{ x: number; y: number } | null>(null);
  let contextMenuHighlightIndex = $state(0);
  let contextMenuItemElements = $state<Array<HTMLButtonElement | null>>([]);

  function offsetControlSize(size: ControlSize, offset: number): ControlSize {
    const baseIndex = controlSizes.indexOf(size);
    const nextIndex = Math.max(0, Math.min(controlSizes.length - 1, baseIndex + Math.round(offset)));
    return controlSizes[nextIndex] ?? size;
  }

  function handleClick(event: MouseEvent) {
    if (disabled) return;

    if (selectable) {
      event.preventDefault();
      onSelectedChange?.(!selected);
      return;
    }

    if (interactive || href) {
      onClick?.(event);
    }
  }

  function handleKeydown(event: KeyboardEvent) {
    if (disabled || href) return;

    if ((interactive || selectable) && (event.key === "Enter" || event.key === " ")) {
      event.preventDefault();
      if (selectable) {
        onSelectedChange?.(!selected);
      } else {
        onClick?.(new MouseEvent("click"));
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
    if (useLeadingContextMenu) return;
    event.preventDefault();
    event.stopPropagation();
    openContextMenuAt(event.clientX, event.clientY);
  }

  function toggleContextMenuFromLeading(event: MouseEvent | KeyboardEvent) {
    if (disabled || !useLeadingContextMenu || !leadingElement) return;
    event.preventDefault();
    event.stopPropagation();

    if (contextMenuOpen) {
      closeContextMenu();
      return;
    }

    const rect = leadingElement.getBoundingClientRect();
    openContextMenuAt(rect.left + rect.width / 2, rect.bottom + 4);
  }

  function handleContextMenuKeydown(event: KeyboardEvent) {
    if (disabled || !hasContextMenu) return;
    if (event.key === "ContextMenu" || (event.shiftKey && event.key === "F10")) {
      event.preventDefault();
      event.stopPropagation();
      if (useLeadingContextMenu) {
        toggleContextMenuFromLeading(event);
        return;
      }
      const rect = rootElement?.getBoundingClientRect();
      if (!rect) return;
      openContextMenuAt(rect.left + 16, rect.top + 16);
    }
  }

  function handleRootKeydown(event: KeyboardEvent) {
    handleKeydown(event);
    handleContextMenuKeydown(event);
  }

  $effect(() => {
    if (!contextMenuOpen) {
      return;
    }

    tick().then(() => {
      contextMenuItemElements[contextMenuHighlightIndex]?.focus();
    });
  });

  $effect(() => {
    if (!contextMenuOpen) {
      return;
    }

    return registerDismissLayer({
      // The overlay and the leading trigger area count as inside.
      contains: (target) =>
        (overlayElement?.contains(target) ?? false) || (leadingElement?.contains(target) ?? false),
      dismissOnOutsideInteract: true,
      onDismiss: () => closeContextMenu(),
    });
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
    data-leading-size={resolvedLeadingSize}
    data-layout={layout}
    data-selected={selected}
    data-highlighted={highlighted}
    data-reorder={showReorderHandle}
    aria-label={ariaLabel ?? title}
    class:poodle-list-card--has-sash={!!sash}
    style={[
      accentColor ? `--list-card-accent: ${accentColor}` : '',
      sashColor ? `--list-card-sash: ${sashColor}` : '',
    ].filter(Boolean).join('; ') || undefined}
    onclick={handleClick}
    oncontextmenu={handleContextMenu}
    onkeydown={handleContextMenuKeydown}
  >
    {#if sashContent || sash}
      <span class="poodle-list-card__sash" aria-label={sash ?? undefined}>
        {#if sashContent}
          {@render sashContent()}
        {:else if sash}
          {sash}
        {/if}
      </span>
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

    {#if leading}
      {#if useLeadingContextMenu && !disabled}
        <button
          bind:this={leadingElement}
          type="button"
          class="poodle-list-card__leading poodle-list-card__leading-button"
          data-interactive={true}
          data-selection-overlay={showSelectionOverlay}
          aria-label={contextMenuAriaLabel ?? `${title} actions`}
          onclick={toggleContextMenuFromLeading}
          onkeydown={(event) => {
            if (event.key === "Enter" || event.key === " ") {
              toggleContextMenuFromLeading(event);
            }
          }}
        >
          <span class="poodle-list-card__leading-content" aria-hidden={showSelectionOverlay ? "true" : undefined}>
            {@render leading()}
          </span>
          {#if showSelectionOverlay}
            <span class="poodle-list-card__selection-indicator poodle-list-card__selection-indicator--overlay" aria-hidden="true">
              <span class="poodle-list-card__selection-box">
                {#if selected}
                  <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M3.5 8.25l2.75 2.75L12.5 4.75" />
                  </svg>
                {/if}
              </span>
            </span>
          {/if}
        </button>
      {:else}
        <span
          bind:this={leadingElement}
          class="poodle-list-card__leading"
          data-interactive={useLeadingContextMenu}
          data-selection-overlay={showSelectionOverlay}
        >
          <span class="poodle-list-card__leading-content" aria-hidden={showSelectionOverlay ? "true" : undefined}>
            {@render leading()}
          </span>
          {#if showSelectionOverlay}
            <span class="poodle-list-card__selection-indicator poodle-list-card__selection-indicator--overlay" aria-hidden="true">
              <span class="poodle-list-card__selection-box">
                {#if selected}
                  <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M3.5 8.25l2.75 2.75L12.5 4.75" />
                  </svg>
                {/if}
              </span>
            </span>
          {/if}
        </span>
      {/if}
    {:else if showSelectionIndicator}
      <span class="poodle-list-card__selection-indicator" aria-hidden="true">
        <span class="poodle-list-card__selection-box">
          {#if selected}
            <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M3.5 8.25l2.75 2.75L12.5 4.75" />
            </svg>
          {/if}
        </span>
      </span>
    {/if}

    <div class="poodle-list-card__body">
      <div class="poodle-list-card__header">
        <span class="poodle-list-card__title">
          {#if titleContent}
            {@render titleContent()}
          {:else}
            {title}
          {/if}
        </span>
        {#if badges || corner}
          <span class="poodle-list-card__header-accessories">
            {#if badges}
              <span class="poodle-list-card__badges">
                {@render badges()}
              </span>
            {/if}
            {#if corner}
              <span class="poodle-list-card__corner">
                {@render corner()}
              </span>
            {/if}
          </span>
        {/if}
      </div>
      {#if subtitleContent}
        <span class="poodle-list-card__subtitle">
          {@render subtitleContent()}
        </span>
      {:else if subtitle}
        <span class="poodle-list-card__subtitle">{subtitle}</span>
      {/if}
      {#if footer}
        <div class="poodle-list-card__footer">
          {@render footer()}
        </div>
      {/if}
    </div>

    {#if showUtilityRail}
      <div class="poodle-list-card__utility-rail">
        {#if showMeta}
          <span class="poodle-list-card__meta">
            {#if metaContent}
              {@render metaContent()}
            {:else}
              {meta}
            {/if}
          </span>
        {/if}

        {#if showActions}
          <span class="poodle-list-card__actions">
            {#if actions}
              {@render actions()}
            {/if}
          </span>
        {/if}

        {#if trailing}
          <span class="poodle-list-card__trailing">
            {@render trailing()}
          </span>
        {/if}
      </div>
    {:else}
      {#if showMeta}
        <span class="poodle-list-card__meta">
          {#if metaContent}
            {@render metaContent()}
          {:else}
            {meta}
          {/if}
        </span>
      {/if}

      {#if showActions}
        <span class="poodle-list-card__actions">
          {#if actions}
            {@render actions()}
          {/if}
        </span>
      {/if}

      {#if trailing}
        <span class="poodle-list-card__trailing">
          {@render trailing()}
        </span>
      {/if}
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
    data-leading-size={resolvedLeadingSize}
    data-layout={layout}
    data-selected={selected}
    data-highlighted={highlighted}
    data-reorder={showReorderHandle}
    role={isInteractive ? (selectable ? "button" : "button") : undefined}
    aria-pressed={selectable ? selected : undefined}
    tabindex={isInteractive && !disabled ? 0 : -1}
    aria-label={ariaLabel ?? title}
    class:poodle-list-card--has-sash={!!sash}
    style={[
      accentColor ? `--list-card-accent: ${accentColor}` : '',
      sashColor ? `--list-card-sash: ${sashColor}` : '',
    ].filter(Boolean).join('; ') || undefined}
    onclick={handleClick}
    onkeydown={handleRootKeydown}
    oncontextmenu={handleContextMenu}
  >
    {#if sashContent || sash}
      <span class="poodle-list-card__sash" aria-label={sash ?? undefined}>
        {#if sashContent}
          {@render sashContent()}
        {:else if sash}
          {sash}
        {/if}
      </span>
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

    {#if leading}
      {#if useLeadingContextMenu && !disabled}
        <button
          bind:this={leadingElement}
          type="button"
          class="poodle-list-card__leading poodle-list-card__leading-button"
          data-interactive={true}
          data-selection-overlay={showSelectionOverlay}
          aria-label={contextMenuAriaLabel ?? `${title} actions`}
          onclick={toggleContextMenuFromLeading}
          onkeydown={(event) => {
            if (event.key === "Enter" || event.key === " ") {
              toggleContextMenuFromLeading(event);
            }
          }}
        >
          <span class="poodle-list-card__leading-content" aria-hidden={showSelectionOverlay ? "true" : undefined}>
            {@render leading()}
          </span>
          {#if showSelectionOverlay}
            <span class="poodle-list-card__selection-indicator poodle-list-card__selection-indicator--overlay" aria-hidden="true">
              <span class="poodle-list-card__selection-box">
                {#if selected}
                  <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M3.5 8.25l2.75 2.75L12.5 4.75" />
                  </svg>
                {/if}
              </span>
            </span>
          {/if}
        </button>
      {:else}
        <span
          bind:this={leadingElement}
          class="poodle-list-card__leading"
          data-interactive={useLeadingContextMenu}
          data-selection-overlay={showSelectionOverlay}
        >
          <span class="poodle-list-card__leading-content" aria-hidden={showSelectionOverlay ? "true" : undefined}>
            {@render leading()}
          </span>
          {#if showSelectionOverlay}
            <span class="poodle-list-card__selection-indicator poodle-list-card__selection-indicator--overlay" aria-hidden="true">
              <span class="poodle-list-card__selection-box">
                {#if selected}
                  <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M3.5 8.25l2.75 2.75L12.5 4.75" />
                  </svg>
                {/if}
              </span>
            </span>
          {/if}
        </span>
      {/if}
    {:else if showSelectionIndicator}
      <span class="poodle-list-card__selection-indicator" aria-hidden="true">
        <span class="poodle-list-card__selection-box">
          {#if selected}
            <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M3.5 8.25l2.75 2.75L12.5 4.75" />
            </svg>
          {/if}
        </span>
      </span>
    {/if}

    <div class="poodle-list-card__body">
      <div class="poodle-list-card__header">
        <span class="poodle-list-card__title">
          {#if titleContent}
            {@render titleContent()}
          {:else}
            {title}
          {/if}
        </span>
        {#if badges || corner}
          <span class="poodle-list-card__header-accessories">
            {#if badges}
              <span class="poodle-list-card__badges">
                {@render badges()}
              </span>
            {/if}
            {#if corner}
              <span class="poodle-list-card__corner">
                {@render corner()}
              </span>
            {/if}
          </span>
        {/if}
      </div>
      {#if subtitleContent}
        <span class="poodle-list-card__subtitle">
          {@render subtitleContent()}
        </span>
      {:else if subtitle}
        <span class="poodle-list-card__subtitle">{subtitle}</span>
      {/if}
      {#if footer}
        <div class="poodle-list-card__footer">
          {@render footer()}
        </div>
      {/if}
    </div>

    {#if showUtilityRail}
      <div class="poodle-list-card__utility-rail">
        {#if showMeta}
          <span class="poodle-list-card__meta">
            {#if metaContent}
              {@render metaContent()}
            {:else}
              {meta}
            {/if}
          </span>
        {/if}

        {#if showActions}
          <span class="poodle-list-card__actions">
            {#if actions}
              {@render actions()}
            {/if}
          </span>
        {/if}

        {#if trailing}
          <span class="poodle-list-card__trailing">
            {@render trailing()}
          </span>
        {/if}
      </div>
    {:else}
      {#if showMeta}
        <span class="poodle-list-card__meta">
          {#if metaContent}
            {@render metaContent()}
          {:else}
            {meta}
          {/if}
        </span>
      {/if}

      {#if showActions}
        <span class="poodle-list-card__actions">
          {#if actions}
            {@render actions()}
          {/if}
        </span>
      {/if}

      {#if trailing}
        <span class="poodle-list-card__trailing">
          {@render trailing()}
        </span>
      {/if}
    {/if}
  </div>
{/if}

{#if contextMenuOpen && hasContextMenu && contextMenuAnchorPoint}
  <div
    bind:this={overlayElement}
    use:anchored={{
      // A right-click has no element behind it, so the menu anchors to the
      // point itself and the shared resolver handles the edge flipping.
      anchor: pointAnchor(contextMenuAnchorPoint.x, contextMenuAnchorPoint.y, rootElement),
      placement: "bottom-start",
      offset: 0,
    }}
    class="poodle-list-card__context-menu"
    data-size={resolvedSize}
    data-density={resolvedDensity}
    role="menu"
    aria-label={contextMenuAriaLabel ?? undefined}
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
          data-tone={item.tone ?? "default"}
          role={item.kind === "checkbox" || item.kind === "radio" ? `menuitem${item.kind}` : "menuitem"}
          aria-checked={item.kind === "checkbox" || item.kind === "radio" ? (item.checked ? "true" : "false") : undefined}
          onclick={() => activateContextMenuItem(item)}
          onkeydown={(event) => {
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

