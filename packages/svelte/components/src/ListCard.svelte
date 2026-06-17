<script lang="ts">
  import { onMount, tick, type Snippet } from "svelte";

  import { menuNavigableItems } from "./internal";
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
  const resolvedContextMenuSize = $derived(resolveSemanticControlSize($uiPresentation.sizeScale, "chrome"));
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
  let contextMenuAdjustedPosition = $state<{ left: string; top: string } | null>(null);
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
  });

  onMount(() => {
    function handlePointerDown(event: MouseEvent): void {
      if (!contextMenuOpen) return;
      if (leadingElement?.contains(event.target as Node)) return;
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
    data-size={resolvedContextMenuSize}
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
    class="poodle-list-card__context-menu"
    data-size={resolvedSize}
    data-density={resolvedDensity}
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

<style>
  .poodle-list-card {
    --poodle-recipe-list-card-fill: color-mix(
      in srgb,
      var(--poodle-color-background-canvas) 88%,
      var(--poodle-color-text-primary)
    );
    --poodle-recipe-list-card-hover-fill: color-mix(
      in srgb,
      var(--poodle-color-background-canvas) 82%,
      var(--poodle-color-text-primary)
    );
    --poodle-list-card-gap: var(--poodle-space-inline-md);
    --poodle-list-card-padding-block: 0.625rem;
    --poodle-list-card-padding-inline: var(--poodle-space-panel-x);
    --poodle-list-card-density-gap-adjust: 0rem;
    --poodle-list-card-density-padding-block-adjust: 0rem;
    --poodle-list-card-density-padding-inline-adjust: 0rem;
    --poodle-list-card-leading-box-size: 2.25rem;
    --poodle-list-card-selection-indicator-size: 2.25rem;
    --poodle-list-card-leading-icon-size: 1.125rem;
    --poodle-list-card-leading-font-size: 0.875rem;
    --poodle-list-card-compact-gap: var(--poodle-space-inline-sm);
    --poodle-list-card-compact-padding-block: 0.5rem;
    --poodle-list-card-compact-padding-inline: 0.625rem;
    display: flex;
    align-items: center;
    gap: calc(var(--poodle-list-card-gap) + var(--poodle-list-card-density-gap-adjust));
    padding:
      calc(var(--poodle-list-card-padding-block) + var(--poodle-list-card-density-padding-block-adjust))
      calc(var(--poodle-list-card-padding-inline) + var(--poodle-list-card-density-padding-inline-adjust));
    border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 18%, transparent);
    border-radius: var(--poodle-radius-control);
    background: var(--poodle-treatment-list-card-fill, var(--poodle-recipe-list-card-fill));
    text-decoration: none;
    width: 100%;
    transition:
      background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      border-color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      box-shadow var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .poodle-list-card--has-sash {
    position: relative;
    overflow: hidden;
  }

  .poodle-list-card--interactive {
    cursor: pointer;
  }

  .poodle-list-card[data-reorder="true"]:not([data-disabled="true"]) {
    cursor: grab;
  }

  .poodle-list-card[data-reorder="true"]:not([data-disabled="true"]):active {
    cursor: grabbing;
  }

  .poodle-list-card--interactive:hover:not([data-disabled="true"]) {
    background: var(--poodle-treatment-list-card-hover-fill, var(--poodle-recipe-list-card-hover-fill));
    border-color: color-mix(in srgb, var(--poodle-color-border-default) 52%, transparent);
  }

  .poodle-list-card[data-selected="true"] {
    border-color: var(--poodle-color-accent-base);
    background: color-mix(in srgb, var(--poodle-recipe-list-card-fill) 84%, var(--poodle-color-accent-base) 16%);
    box-shadow:
      0 0 0 0.0625rem var(--poodle-color-accent-base),
      inset 0 0 0 0.0625rem color-mix(
        in srgb,
        var(--poodle-color-accent-base) 12%,
        transparent
      );
  }

  .poodle-list-card--interactive[data-selected="true"]:hover:not([data-disabled="true"]) {
    border-color: var(--poodle-color-accent-base);
  }

  .poodle-list-card[data-highlighted="true"] {
    border-color: color-mix(in srgb, var(--list-card-accent, var(--poodle-color-accent-base)) 34%, transparent);
    background:
      linear-gradient(
        90deg,
        color-mix(in srgb, var(--list-card-accent, var(--poodle-color-accent-base)) 10%, transparent),
        transparent 24%
      ),
      var(--poodle-treatment-list-card-fill, var(--poodle-recipe-list-card-fill));
    box-shadow:
      inset 0 0 0 0.0625rem color-mix(
        in srgb,
        var(--list-card-accent, var(--poodle-color-accent-base)) 12%,
        transparent
      );
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
    background: color-mix(in srgb, var(--poodle-recipe-list-card-fill) 32%, transparent);
    filter: grayscale(1);
    opacity: 0.72;
  }

  .poodle-list-card[data-not-live="true"]:hover:not([data-disabled="true"]) {
    border-color: var(--poodle-color-border-default);
    filter: grayscale(0);
    opacity: 1;
  }

  .poodle-list-card[data-layout="compact"] {
    gap: calc(var(--poodle-list-card-compact-gap) + var(--poodle-list-card-density-gap-adjust));
    padding:
      calc(var(--poodle-list-card-compact-padding-block) + var(--poodle-list-card-density-padding-block-adjust))
      calc(var(--poodle-list-card-compact-padding-inline) + var(--poodle-list-card-density-padding-inline-adjust));
    min-height: 3rem;
  }

  .poodle-list-card[data-leading-size="xs"] {
    --poodle-list-card-leading-box-size: 1.75rem;
    --poodle-list-card-selection-indicator-size: 1.75rem;
    --poodle-list-card-leading-icon-size: 0.875rem;
    --poodle-list-card-leading-font-size: 0.6875rem;
  }

  .poodle-list-card[data-leading-size="sm"] {
    --poodle-list-card-leading-box-size: 2rem;
    --poodle-list-card-selection-indicator-size: 2rem;
    --poodle-list-card-leading-icon-size: 1rem;
    --poodle-list-card-leading-font-size: 0.75rem;
  }

  .poodle-list-card[data-leading-size="md"] {
    --poodle-list-card-leading-box-size: 2.25rem;
    --poodle-list-card-selection-indicator-size: 2.25rem;
    --poodle-list-card-leading-icon-size: 1.125rem;
    --poodle-list-card-leading-font-size: 0.875rem;
  }

  .poodle-list-card[data-leading-size="lg"] {
    --poodle-list-card-leading-box-size: 2.75rem;
    --poodle-list-card-selection-indicator-size: 2.75rem;
    --poodle-list-card-leading-icon-size: 1.375rem;
    --poodle-list-card-leading-font-size: 1rem;
  }

  .poodle-list-card[data-leading-size="xl"] {
    --poodle-list-card-leading-box-size: 3rem;
    --poodle-list-card-selection-indicator-size: 3rem;
    --poodle-list-card-leading-icon-size: 1.5rem;
    --poodle-list-card-leading-font-size: 1.125rem;
  }

  .poodle-list-card[data-layout="compact"][data-leading-size="xs"] {
    --poodle-list-card-leading-box-size: 1.5rem;
    --poodle-list-card-selection-indicator-size: 1.5rem;
    --poodle-list-card-leading-icon-size: 0.75rem;
    --poodle-list-card-leading-font-size: 0.625rem;
  }

  .poodle-list-card[data-layout="compact"][data-leading-size="sm"] {
    --poodle-list-card-leading-box-size: 1.75rem;
    --poodle-list-card-selection-indicator-size: 1.75rem;
    --poodle-list-card-leading-icon-size: 0.875rem;
    --poodle-list-card-leading-font-size: 0.75rem;
  }

  .poodle-list-card[data-layout="compact"][data-leading-size="md"] {
    --poodle-list-card-leading-box-size: 1.875rem;
    --poodle-list-card-selection-indicator-size: 1.875rem;
    --poodle-list-card-leading-icon-size: 1rem;
    --poodle-list-card-leading-font-size: 0.8125rem;
  }

  .poodle-list-card[data-layout="compact"][data-leading-size="lg"] {
    --poodle-list-card-leading-box-size: 2.125rem;
    --poodle-list-card-selection-indicator-size: 2.125rem;
    --poodle-list-card-leading-icon-size: 1.125rem;
    --poodle-list-card-leading-font-size: 0.875rem;
  }

  .poodle-list-card[data-layout="compact"][data-leading-size="xl"] {
    --poodle-list-card-leading-box-size: 2.375rem;
    --poodle-list-card-selection-indicator-size: 2.375rem;
    --poodle-list-card-leading-icon-size: 1.25rem;
    --poodle-list-card-leading-font-size: 0.9375rem;
  }

  .poodle-list-card[data-layout="stacked"] {
    --poodle-list-card-padding-block: 0.75rem;
    --poodle-list-card-padding-inline: 0.75rem;
    --poodle-list-card-gap: 0.625rem;
    flex-direction: column;
    align-items: center;
    gap: calc(var(--poodle-list-card-gap) + var(--poodle-list-card-density-gap-adjust));
  }

  .poodle-list-card[data-layout="stacked"] .poodle-list-card__leading,
  .poodle-list-card[data-layout="stacked"] .poodle-list-card__selection-indicator,
  .poodle-list-card[data-layout="stacked"] .poodle-list-card__handle {
    align-self: center;
  }

  .poodle-list-card[data-layout="stacked"] .poodle-list-card__body {
    width: 100%;
    align-items: center;
    text-align: center;
    gap: 0.25rem;
  }

  .poodle-list-card[data-layout="stacked"] .poodle-list-card__header {
    width: 100%;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 0.25rem;
  }

  .poodle-list-card[data-layout="stacked"] .poodle-list-card__title {
    flex: 0 1 auto;
    text-align: center;
    text-wrap: balance;
    white-space: normal;
    display: -webkit-box;
    line-clamp: 2;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
  }

  .poodle-list-card[data-layout="stacked"] .poodle-list-card__subtitle {
    max-width: 26ch;
    text-align: center;
    text-wrap: balance;
    white-space: normal;
    display: -webkit-box;
    line-clamp: 2;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
  }

  .poodle-list-card[data-layout="stacked"] .poodle-list-card__header-accessories,
  .poodle-list-card[data-layout="stacked"] .poodle-list-card__footer {
    justify-content: center;
    flex-wrap: wrap;
  }

  .poodle-list-card[data-layout="stacked"] .poodle-list-card__footer {
    margin-top: 0.125rem;
  }

  .poodle-list-card[data-layout="compact"][data-size="xs"] {
    --poodle-list-card-compact-gap: 0.3125rem;
    --poodle-list-card-compact-padding-block: 0.25rem;
    --poodle-list-card-compact-padding-inline: 0.375rem;
    min-height: 2.25rem;
  }

  .poodle-list-card[data-layout="compact"][data-size="sm"] {
    --poodle-list-card-compact-gap: 0.375rem;
    --poodle-list-card-compact-padding-block: 0.3125rem;
    --poodle-list-card-compact-padding-inline: 0.5rem;
    min-height: 2.5rem;
  }

  .poodle-list-card[data-layout="compact"] .poodle-list-card__leading {
    width: var(--poodle-list-card-leading-box-size);
    height: var(--poodle-list-card-leading-box-size);
    font-size: var(--poodle-list-card-leading-font-size);
  }

  .poodle-list-card[data-layout="compact"] .poodle-list-card__body {
    gap: 0;
  }

  .poodle-list-card[data-layout="compact"] .poodle-list-card__title {
    font-size: 0.875rem;
  }

  .poodle-list-card[data-layout="compact"][data-size="xs"] .poodle-list-card__title {
    font-size: 0.75rem;
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

  .poodle-list-card__selection-indicator {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    width: var(--poodle-list-card-selection-indicator-size);
    height: var(--poodle-list-card-selection-indicator-size);
  }

  .poodle-list-card__selection-indicator--overlay {
    position: absolute;
    inset: 0;
    width: auto;
    height: auto;
    z-index: 1;
  }

  .poodle-list-card__leading-content {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;
    transition: opacity var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .poodle-list-card__leading-content :global(.poodle-icon),
  .poodle-list-card__leading-content :global(svg) {
    width: var(--poodle-list-card-leading-icon-size);
    height: var(--poodle-list-card-leading-icon-size);
  }

  .poodle-list-card__leading-content :global(img),
  .poodle-list-card__leading-content :global(picture) {
    width: 100%;
    height: 100%;
  }

  .poodle-list-card__leading-content :global(img) {
    display: block;
    object-fit: cover;
  }

  .poodle-list-card__leading-content :global(.poodle-media-thumbnail) {
    width: 100%;
    height: 100%;
    gap: 0;
  }

  .poodle-list-card__leading-content :global(.poodle-media-thumbnail__frame) {
    width: 100%;
    height: 100%;
  }

  .poodle-list-card__leading-content :global(.poodle-media-thumbnail[data-aspect-ratio] .poodle-media-thumbnail__frame) {
    aspect-ratio: auto;
  }

  .poodle-list-card__leading[data-selection-overlay="true"] .poodle-list-card__leading-content {
    opacity: 0;
  }

  .poodle-list-card__selection-box {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1rem;
    height: 1rem;
    border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-default) 88%, transparent);
    border-radius: 0.25rem;
    background: var(--poodle-color-background-surface);
    color: var(--poodle-color-text-inverse);
    transition:
      border-color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .poodle-list-card[data-selected="true"] .poodle-list-card__selection-box {
    border-color: var(--poodle-color-accent-base);
    background: var(--poodle-color-accent-base);
  }

  .poodle-list-card__selection-box svg {
    width: 0.75rem;
    height: 0.75rem;
  }

  .poodle-list-card__leading {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    width: var(--poodle-list-card-leading-box-size);
    height: var(--poodle-list-card-leading-box-size);
    overflow: hidden;
    border-radius: 999px;
    background: color-mix(in srgb, var(--list-card-accent, var(--poodle-color-accent-base)) 12%, transparent);
    color: var(--list-card-accent, var(--poodle-color-accent-base));
    font-size: var(--poodle-list-card-leading-font-size);
    font-weight: 600;
    transition:
      background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      box-shadow var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .poodle-list-card__leading-button {
    padding: 0;
    border: 0;
    appearance: none;
    text-align: inherit;
  }

  .poodle-list-card__leading[data-interactive="true"] {
    cursor: pointer;
  }

  .poodle-list-card__leading[data-interactive="true"]:hover {
    background: color-mix(in srgb, var(--list-card-accent, var(--poodle-color-accent-base)) 22%, transparent);
    box-shadow:
      inset 0 0 0 0.0625rem color-mix(in srgb, var(--list-card-accent, var(--poodle-color-accent-base)) 34%, transparent);
  }

  .poodle-list-card__leading[data-interactive="true"]:active {
    background: color-mix(in srgb, var(--list-card-accent, var(--poodle-color-accent-base)) 30%, transparent);
  }

  .poodle-list-card__leading[data-interactive="true"]:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .poodle-list-card[data-leading-shape="rounded-square"] .poodle-list-card__leading {
    border-radius: var(--poodle-radius-control);
  }

  .poodle-list-card[data-leading-shape="rounded-square"] .poodle-list-card__leading-content {
    --poodle-list-card-leading-icon-size: calc(var(--poodle-list-card-leading-icon-size) + 0.0625rem);
  }

  .poodle-list-card[data-leading-fill="solid"] .poodle-list-card__leading {
    background: var(--list-card-accent, var(--poodle-color-accent-base));
    color: #fff;
  }

  .poodle-list-card[data-leading-fill="solid"] .poodle-list-card__leading[data-interactive="true"]:hover {
    background: color-mix(in srgb, var(--list-card-accent, var(--poodle-color-accent-base)) 86%, var(--poodle-color-text-primary));
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

  .poodle-list-card__header-accessories {
    flex-shrink: 0;
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: var(--poodle-space-inline-sm);
  }

  .poodle-list-card__badges,
  .poodle-list-card__corner {
    display: inline-flex;
    align-items: center;
    gap: var(--poodle-space-inline-xs);
  }

  .poodle-list-card__corner {
    color: var(--poodle-color-text-tertiary);
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
    gap: var(--poodle-space-inline-sm);
    margin-top: 0.0625rem;
    font-size: 0.6875rem;
    line-height: 1.35;
  }

  .poodle-list-card__meta {
    flex-shrink: 0;
    font-size: 0.75rem;
    color: var(--poodle-color-text-secondary);
    font-variant-numeric: tabular-nums;
  }

  .poodle-list-card__utility-rail {
    display: flex;
    align-items: center;
    gap: var(--poodle-space-inline-sm);
    width: 100%;
    min-width: 0;
  }

  .poodle-list-card__utility-rail .poodle-list-card__actions,
  .poodle-list-card__utility-rail .poodle-list-card__trailing {
    margin-left: auto;
  }

  .poodle-list-card__utility-rail .poodle-list-card__meta + .poodle-list-card__actions,
  .poodle-list-card__utility-rail .poodle-list-card__meta + .poodle-list-card__trailing {
    margin-left: auto;
  }

  .poodle-list-card[data-layout="stacked"] .poodle-list-card__utility-rail {
    width: auto;
    justify-content: center;
    flex-wrap: wrap;
    margin-top: 0.125rem;
  }

  .poodle-list-card[data-layout="stacked"] .poodle-list-card__utility-rail .poodle-list-card__actions,
  .poodle-list-card[data-layout="stacked"] .poodle-list-card__utility-rail .poodle-list-card__trailing,
  .poodle-list-card[data-layout="stacked"] .poodle-list-card__utility-rail .poodle-list-card__meta + .poodle-list-card__actions,
  .poodle-list-card[data-layout="stacked"] .poodle-list-card__utility-rail .poodle-list-card__meta + .poodle-list-card__trailing {
    margin-left: 0;
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
  .poodle-list-card[data-size="xs"] {
    --poodle-list-card-padding-block: 0.375rem;
    --poodle-list-card-padding-inline: 0.5rem;
    --poodle-list-card-gap: 0.375rem;
  }
  .poodle-list-card[data-size="xs"] .poodle-list-card__title { font-size: 0.75rem; }
  .poodle-list-card[data-size="xs"] .poodle-list-card__subtitle { font-size: 0.625rem; }
  .poodle-list-card[data-size="xs"] .poodle-list-card__meta { font-size: 0.625rem; }
  .poodle-list-card[data-size="xs"] .poodle-list-card__selection-box { width: 0.875rem; height: 0.875rem; }
  .poodle-list-card[data-size="xs"] .poodle-list-card__selection-box svg { width: 0.625rem; height: 0.625rem; }

  .poodle-list-card[data-size="sm"] {
    --poodle-list-card-padding-block: 0.5rem;
    --poodle-list-card-padding-inline: 0.625rem;
    --poodle-list-card-gap: 0.5rem;
  }
  .poodle-list-card[data-size="sm"] .poodle-list-card__title { font-size: 0.8125rem; }
  .poodle-list-card[data-size="sm"] .poodle-list-card__subtitle { font-size: 0.6875rem; }
  .poodle-list-card[data-size="sm"] .poodle-list-card__meta { font-size: 0.6875rem; }
  .poodle-list-card[data-size="lg"] {
    --poodle-list-card-padding-block: 0.75rem;
    --poodle-list-card-padding-inline: 1rem;
    --poodle-list-card-gap: 0.875rem;
  }
  .poodle-list-card[data-size="lg"] .poodle-list-card__title { font-size: 1rem; }
  .poodle-list-card[data-size="lg"] .poodle-list-card__subtitle { font-size: 0.8125rem; }
  .poodle-list-card[data-size="lg"] .poodle-list-card__meta { font-size: 0.8125rem; }
  .poodle-list-card[data-size="lg"] .poodle-list-card__selection-box { width: 1.125rem; height: 1.125rem; }
  .poodle-list-card[data-size="lg"] .poodle-list-card__selection-box svg { width: 0.8125rem; height: 0.8125rem; }

  .poodle-list-card[data-size="xl"] {
    --poodle-list-card-padding-block: 0.875rem;
    --poodle-list-card-padding-inline: 1.125rem;
    --poodle-list-card-gap: 1rem;
  }
  .poodle-list-card[data-size="xl"] .poodle-list-card__title { font-size: 1.0625rem; }
  .poodle-list-card[data-size="xl"] .poodle-list-card__subtitle { font-size: 0.875rem; }
  .poodle-list-card[data-size="xl"] .poodle-list-card__meta { font-size: 0.875rem; }
  .poodle-list-card[data-size="xl"] .poodle-list-card__selection-box { width: 1.25rem; height: 1.25rem; }
  .poodle-list-card[data-size="xl"] .poodle-list-card__selection-box svg { width: 0.875rem; height: 0.875rem; }

  /* Density variants */
  .poodle-list-card[data-density="compact"] {
    --poodle-list-card-density-gap-adjust: -0.125rem;
    --poodle-list-card-density-padding-block-adjust: -0.125rem;
    --poodle-list-card-density-padding-inline-adjust: -0.125rem;
  }

  .poodle-list-card[data-density="default"] {
    --poodle-list-card-density-gap-adjust: 0rem;
    --poodle-list-card-density-padding-block-adjust: 0rem;
    --poodle-list-card-density-padding-inline-adjust: -0.0625rem;
  }

  .poodle-list-card[data-density="comfortable"] {
    --poodle-list-card-density-gap-adjust: 0.125rem;
    --poodle-list-card-density-padding-block-adjust: 0.125rem;
    --poodle-list-card-density-padding-inline-adjust: 0.0625rem;
  }

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

  .poodle-list-card__context-item[data-tone="danger"] {
    color: var(--poodle-color-status-danger);
  }

  .poodle-list-card__context-item[data-tone="danger"]:hover:not(:disabled),
  .poodle-list-card__context-item[data-tone="danger"]:focus-visible {
    background: color-mix(in srgb, var(--poodle-color-status-danger) 14%, transparent);
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

  .poodle-list-card__context-menu[data-size="xs"] .poodle-list-card__context-item { min-height: 1.5rem; font-size: 0.6875rem; }
  .poodle-list-card__context-menu[data-size="xs"] .poodle-list-card__context-meta { font-size: 0.5625rem; }
  .poodle-list-card__context-menu[data-size="sm"] .poodle-list-card__context-item { min-height: 1.75rem; font-size: 0.75rem; }
  .poodle-list-card__context-menu[data-size="sm"] .poodle-list-card__context-meta { font-size: 0.625rem; }
  .poodle-list-card__context-menu[data-size="lg"] .poodle-list-card__context-item { min-height: 2.75rem; font-size: 0.9375rem; }
  .poodle-list-card__context-menu[data-size="lg"] .poodle-list-card__context-meta { font-size: 0.75rem; }
  .poodle-list-card__context-menu[data-size="xl"] .poodle-list-card__context-item { min-height: 3.25rem; font-size: 1rem; }
  .poodle-list-card__context-menu[data-size="xl"] .poodle-list-card__context-meta { font-size: 0.8125rem; }

  .poodle-list-card__context-menu[data-density="compact"] .poodle-list-card__context-item { padding-inline: 0.375rem; }
  .poodle-list-card__context-menu[data-density="comfortable"] .poodle-list-card__context-item { padding-inline: 0.75rem; }
</style>
