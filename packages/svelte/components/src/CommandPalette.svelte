<script lang="ts">
  import { onDestroy, tick } from "svelte";

  import { default as Icon } from "./Icon.svelte";
  import { default as TextInput } from "./TextInput.svelte";
  import { default as UiPresentationProvider } from "./UiPresentationProvider.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type {
    ControlDensity,
    ControlSize,
    SemanticControlSizeRole,
  } from "./types";

  import { default as ActionDiscoveryPanel } from "./ActionDiscoveryPanel.svelte";

  import type { CommandActionItem, DiscoveryState } from "./types";

  interface Props {
    open?: boolean;
    title?: string;
    description?: string | null;
    query?: string;
    items?: CommandActionItem[];
    state?: DiscoveryState;
    ariaLabel?: string | null;
    invocationHint?: string | null;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onQueryChange?: ((value: string) => void) | undefined;
    onCommandSelect?: ((id: string) => void) | undefined;
    onOpenChange?: ((open: boolean) => void) | undefined;
    onActiveChange?: ((id: string | null) => void) | undefined;
  }

  const uiPresentation = getUiPresentation();
  const queryInputId = "command-palette-query";
  const statusId = "command-palette-status";

  let {
    open = false,
    title = "Command palette",
    description = null,
    query = "",
    items = [],
    state: discoveryState = "ready",
    ariaLabel = null,
    invocationHint = null,
    size = null,
    sizeRole = "control",
    density = null,
    onQueryChange = undefined,
    onCommandSelect = undefined,
    onOpenChange = undefined,
    onActiveChange = undefined,
  }: Props = $props();

  let previousFocusedElement: HTMLElement | null = null;
  let activeId = $state<string | null>(null);
  let wasOpen = $state(false);
  let previousHtmlOverflow = $state("");
  let previousBodyOverflow = $state("");
  let panel = $state<ActionDiscoveryPanel | null>(null);

  let resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  let resolvedDensity = $derived(density ?? $uiPresentation.density);
  let currentQuery = $derived(query);
  let enabledItems = $derived(items.filter((item) => !item.disabled));
  let activeItem = $derived(enabledItems.find((item) => item.id === activeId) ?? null);
  let paletteStatus = $derived(
    discoveryState === "loading"
      ? "Loading commands."
      : discoveryState === "error"
        ? "Command palette unavailable."
        : discoveryState === "empty"
          ? "No commands are available in this workspace."
          : discoveryState === "no-results"
            ? `No commands match "${currentQuery}".`
            : `${enabledItems.length} command${enabledItems.length === 1 ? "" : "s"} available.${activeItem ? ` Active command: ${activeItem.title}.` : ""}`,
  );

  $effect(() => {
    if (open && !wasOpen) {
      previousFocusedElement = document.activeElement instanceof HTMLElement ? document.activeElement : null;
      previousHtmlOverflow = document.documentElement.style.overflow;
      previousBodyOverflow = document.body.style.overflow;
      document.documentElement.style.overflow = "hidden";
      document.body.style.overflow = "hidden";
      wasOpen = true;
      queueMicrotask(() => {
        void focusSearchInput();
      });
    }

    if (!open && wasOpen) {
      wasOpen = false;
      activeId = null;
      document.documentElement.style.overflow = previousHtmlOverflow;
      document.body.style.overflow = previousBodyOverflow;
      previousFocusedElement?.focus();
    }
  });

  async function focusSearchInput(): Promise<void> {
    await tick();
    const input = document.getElementById(queryInputId) as HTMLInputElement | null;
    input?.focus();
    if (enabledItems.length > 0) {
      activeId = enabledItems[0]?.id ?? null;
      onActiveChange?.(activeId);
    }
  }

  $effect(() => {
    if (open && enabledItems.length > 0 && (!activeId || !enabledItems.some((item) => item.id === activeId))) {
      activeId = enabledItems[0]?.id ?? null;
      onActiveChange?.(activeId);
    }
  });

  $effect(() => {
    if (open && enabledItems.length === 0 && activeId !== null) {
      activeId = null;
      onActiveChange?.(null);
    }
  });

  function close(): void {
    onOpenChange?.(false);
  }

  function trapFocus(event: KeyboardEvent): void {
    if (event.key !== "Tab") return;

    const focusableElements = Array.from(
      document.querySelectorAll<HTMLElement>(
        '.poodle-command-palette button:not([disabled]), .poodle-command-palette input:not([disabled]), .poodle-command-palette [tabindex]:not([tabindex="-1"])',
      ),
    ).filter((el) => !el.hasAttribute("disabled"));

    if (focusableElements.length === 0) return;

    const first = focusableElements[0];
    const last = focusableElements[focusableElements.length - 1];

    if (event.shiftKey && document.activeElement === first) {
      event.preventDefault();
      last.focus();
    } else if (!event.shiftKey && document.activeElement === last) {
      event.preventDefault();
      first.focus();
    }
  }

  function handleKeydown(event: KeyboardEvent): void {
    if (!open) return;

    trapFocus(event);

    if (event.key === "Escape") {
      event.preventDefault();
      close();
      return;
    }
    if (event.key === "ArrowDown") {
      event.preventDefault();
      panel?.moveActive(1);
      return;
    }
    if (event.key === "ArrowUp") {
      event.preventDefault();
      panel?.moveActive(-1);
      return;
    }
    if (event.key === "Home") {
      event.preventDefault();
      panel?.moveToBoundary("start");
      return;
    }
    if (event.key === "End") {
      event.preventDefault();
      panel?.moveToBoundary("end");
      return;
    }
    if (event.key === "Enter" && activeId) {
      event.preventDefault();
      onCommandSelect?.(activeId);
    }
  }

  onDestroy(() => {
    if (wasOpen) {
      document.documentElement.style.overflow = previousHtmlOverflow;
      document.body.style.overflow = previousBodyOverflow;
    }
    previousFocusedElement = null;
  });
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
  <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
  <div class="poodle-command-palette__overlay" aria-hidden="true" onclick={close}></div>
  <UiPresentationProvider sizeScale={resolvedSize} density={resolvedDensity}>
    <div
      class="poodle-command-palette"
      role="dialog"
      aria-modal="true"
      aria-label={ariaLabel ?? title}
      aria-describedby={description ? "command-palette-description" : undefined}
      data-size={resolvedSize}
      data-density={resolvedDensity}
    >
      <div class="poodle-command-palette__header">
        <div>
          <h3>{title}</h3>
          {#if description}
            <p id="command-palette-description">{description}</p>
          {/if}
        </div>
        <div class="poodle-command-palette__meta">
          {#if invocationHint}
            <span class="poodle-command-palette__hint">{invocationHint}</span>
          {/if}
          <button type="button" class="poodle-command-palette__close" aria-label="Close command palette" onclick={close}>
            <Icon name="x" />
          </button>
        </div>
      </div>

      <div class="poodle-command-palette__query">
        <TextInput
          id={queryInputId}
          type="search"
          value={currentQuery}
          ariaLabel="Search commands"
          describedBy={statusId}
          placeholder="Search commands, panels, and actions"
          onValueChange={(nextValue) => onQueryChange?.(nextValue)}
          onClear={() => onQueryChange?.("")}
          onCancel={close}
          onSubmit={() => {
            if (activeId) {
              onCommandSelect?.(activeId);
            }
          }}
        />
      </div>

      <p id={statusId} class="poodle-command-palette__status" role="status" aria-live="polite" aria-atomic="true">
        {paletteStatus}
      </p>

      <ActionDiscoveryPanel
        bind:this={panel}
        {items}
        state={discoveryState}
        bind:activeId
        ariaLabel="Command results"
        size={resolvedSize}
        density={resolvedDensity}
        onItemSelect={(id) => onCommandSelect?.(id)}
        onActiveChange={(id) => {
          activeId = id;
          onActiveChange?.(id);
        }}
      />
    </div>
  </UiPresentationProvider>
{/if}

<style>
  .poodle-command-palette__overlay {
    position: fixed;
    inset: 0;
    background: color-mix(in srgb, black 44%, transparent);
    backdrop-filter: blur(0.5rem);
    z-index: 40;
  }

  .poodle-command-palette {
    --poodle-command-palette-hint-height: 1.5rem;
    --poodle-command-palette-hint-x: 0.5rem;
    --poodle-command-palette-close-size: 1.75rem;
    position: fixed;
    top: 50%;
    left: 50%;
    display: grid;
    grid-template-rows: auto auto auto minmax(0, 1fr);
    gap: var(--poodle-space-stack-md);
    width: min(45rem, calc(100vw - 2rem));
    max-height: min(78vh, 52.5rem);
    min-height: 0;
    padding: var(--poodle-space-panel-y) var(--poodle-space-panel-x);
    border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-default) 42%, transparent);
    border-radius: calc(var(--poodle-radius-surface) + 0.125rem);
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 98%, transparent);
    box-shadow: var(--poodle-elevation-dialog);
    overflow: hidden;
    overscroll-behavior: contain;
    transform: translate(-50%, -50%);
    z-index: 41;
  }

  .poodle-command-palette[data-size="xs"] {
    --poodle-command-palette-hint-height: 1.25rem;
    --poodle-command-palette-hint-x: 0.375rem;
    --poodle-command-palette-close-size: 1.5rem;
  }

  .poodle-command-palette[data-size="sm"] {
    --poodle-command-palette-hint-height: 1.5rem;
    --poodle-command-palette-close-size: 1.75rem;
  }

  .poodle-command-palette[data-size="lg"] {
    --poodle-command-palette-hint-height: 1.75rem;
    --poodle-command-palette-hint-x: 0.625rem;
    --poodle-command-palette-close-size: 2rem;
  }

  .poodle-command-palette[data-size="xl"] {
    --poodle-command-palette-hint-height: 2rem;
    --poodle-command-palette-hint-x: 0.75rem;
    --poodle-command-palette-close-size: 2.25rem;
  }

  .poodle-command-palette__header {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    gap: var(--poodle-space-inline-md);
    align-items: start;
  }

  .poodle-command-palette__header h3,
  .poodle-command-palette__header p {
    margin: 0;
  }

  .poodle-command-palette__header h3 {
    font-size: 1.375rem;
    line-height: 1.2;
  }

  .poodle-command-palette__header p {
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.5;
  }

  .poodle-command-palette__meta {
    display: flex;
    gap: var(--poodle-space-inline-sm);
    align-items: center;
  }

  .poodle-command-palette__hint {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-height: var(--poodle-command-palette-hint-height);
    padding: 0 var(--poodle-command-palette-hint-x);
    border-radius: var(--poodle-radius-control);
    background: color-mix(in srgb, var(--poodle-color-background-surface) 76%, transparent);
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-code-family);
    font-size: var(--poodle-typography-label-size);
  }

  .poodle-command-palette__close {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: var(--poodle-command-palette-close-size);
    height: var(--poodle-command-palette-close-size);
    min-height: 0;
    border: 0;
    border-radius: calc(var(--poodle-radius-control) - 0.0625rem);
    background: transparent;
    color: var(--poodle-color-text-secondary);
    cursor: pointer;
    padding: 0;
    font: inherit;
  }

  .poodle-command-palette__close:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .poodle-command-palette__query {
    min-width: 0;
  }

  .poodle-command-palette__status {
    margin: 0;
    color: var(--poodle-color-text-secondary);
    font-size: 0.75rem;
    line-height: 1.4;
  }

  @media (max-width: 45rem) {
    .poodle-command-palette {
      width: calc(100vw - 1rem);
      padding: var(--poodle-space-panel-y) var(--poodle-space-panel-x-sm, 0.75rem);
    }

    .poodle-command-palette__header {
      grid-template-columns: 1fr;
    }

    .poodle-command-palette__meta {
      justify-content: space-between;
    }
  }
</style>
