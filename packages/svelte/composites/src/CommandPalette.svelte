<script lang="ts">
  import { createEventDispatcher, onDestroy, tick } from "svelte";

  import {
    Icon,
    SearchField,
    UiPresentationProvider,
    getUiPresentation,
    resolveSemanticControlSize,
  } from "@poodle/svelte-primitives";
  import type {
    ControlDensity,
    ControlSize,
    SemanticControlSizeRole,
  } from "@poodle/svelte-primitives";

  import ActionDiscoveryPanel from "./ActionDiscoveryPanel.svelte";

  import type { CommandActionItem, DiscoveryState } from "./types";

  export let open = false;
  export let title = "Command palette";
  export let description: string | null = null;
  export let query = "";
  export let items: CommandActionItem[] = [];
  export let state: DiscoveryState = "ready";
  export let ariaLabel: string | null = null;
  export let invocationHint: string | null = null;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;

  const dispatch = createEventDispatcher<{
    queryChange: { value: string };
    commandSelect: { id: string };
    openChange: { open: boolean };
    activeChange: { id: string | null };
  }>();

  let previousFocusedElement: HTMLElement | null = null;
  let activeId: string | null = null;
  let wasOpen = false;
  let previousHtmlOverflow = "";
  let previousBodyOverflow = "";
  let panel: ActionDiscoveryPanel;
  const uiPresentation = getUiPresentation();
  const queryInputId = "command-palette-query";
  const statusId = "command-palette-status";

  $: resolvedSize = size ?? resolveSemanticControlSize(uiPresentation?.sizeScale ?? "md", sizeRole);
  $: resolvedDensity = density ?? uiPresentation?.density ?? "default";

  $: enabledItems = items.filter((item) => !item.disabled);
  $: if (open && !wasOpen) {
    previousFocusedElement = document.activeElement instanceof HTMLElement ? document.activeElement : null;
    previousHtmlOverflow = document.documentElement.style.overflow;
    previousBodyOverflow = document.body.style.overflow;
    document.documentElement.style.overflow = "hidden";
    document.body.style.overflow = "hidden";
    wasOpen = true;
    queueMicrotask(async () => {
      await tick();
      const input = document.getElementById(queryInputId) as HTMLInputElement | null;
      input?.focus();
      if (enabledItems.length > 0) {
        activeId = enabledItems[0]?.id ?? null;
        dispatch("activeChange", { id: activeId });
      }
    });
  }
  $: if (!open && wasOpen) {
    wasOpen = false;
    activeId = null;
    document.documentElement.style.overflow = previousHtmlOverflow;
    document.body.style.overflow = previousBodyOverflow;
    previousFocusedElement?.focus();
  }
  $: if (open && enabledItems.length > 0 && (!activeId || !enabledItems.some((item) => item.id === activeId))) {
    activeId = enabledItems[0]?.id ?? null;
    dispatch("activeChange", { id: activeId });
  }
  $: if (open && enabledItems.length === 0 && activeId !== null) {
    activeId = null;
    dispatch("activeChange", { id: null });
  }
  $: activeItem = enabledItems.find((item) => item.id === activeId) ?? null;
  $: paletteStatus =
    state === "loading"
      ? "Loading commands."
      : state === "error"
        ? "Command palette unavailable."
        : state === "empty"
          ? "No commands are available in this workspace."
          : state === "no-results"
            ? `No commands match "${query}".`
            : `${enabledItems.length} command${enabledItems.length === 1 ? "" : "s"} available.${activeItem ? ` Active command: ${activeItem.title}.` : ""}`;

  function close(): void {
    open = false;
    dispatch("openChange", { open: false });
  }

  function trapFocus(event: KeyboardEvent): void {
    if (event.key !== "Tab") return;

    const focusableElements = Array.from(
      document.querySelectorAll<HTMLElement>(
        '.command-palette button:not([disabled]), .command-palette input:not([disabled]), .command-palette [tabindex]:not([tabindex="-1"])',
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
      dispatch("commandSelect", { id: activeId });
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

<svelte:window on:keydown={handleKeydown} />

{#if open}
  <!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
  <div class="command-palette__overlay" aria-hidden="true" on:click={close}></div>
  <UiPresentationProvider sizeScale={resolvedSize} density={resolvedDensity}>
    <div
      class="command-palette"
      role="dialog"
      aria-modal="true"
      aria-label={ariaLabel ?? title}
      aria-describedby={description ? "command-palette-description" : undefined}
      data-size={resolvedSize}
      data-density={resolvedDensity}
    >
      <div class="command-palette__header">
        <div>
          <h3>{title}</h3>
          {#if description}
            <p id="command-palette-description">{description}</p>
          {/if}
        </div>
        <div class="command-palette__meta">
          {#if invocationHint}
            <span class="command-palette__hint">{invocationHint}</span>
          {/if}
          <button type="button" class="command-palette__close" aria-label="Close command palette" on:click={close}>
            <Icon name="x" />
          </button>
        </div>
      </div>

      <div class="command-palette__query">
        <SearchField
          id={queryInputId}
          value={query}
          ariaLabel="Search commands"
          describedBy={statusId}
          placeholder="Search commands, panels, and actions"
          on:valueChange={(event) => dispatch("queryChange", event.detail)}
          on:clear={() => dispatch("queryChange", { value: "" })}
          on:cancel={close}
          on:submit={() => {
            if (activeId) {
              dispatch("commandSelect", { id: activeId });
            }
          }}
        />
      </div>

      <p id={statusId} class="command-palette__status" role="status" aria-live="polite" aria-atomic="true">
        {paletteStatus}
      </p>

      <ActionDiscoveryPanel
        bind:this={panel}
        {items}
        {state}
        bind:activeId
        ariaLabel="Command results"
        size={resolvedSize}
        density={resolvedDensity}
        on:itemSelect={(e) => dispatch("commandSelect", e.detail)}
        on:activeChange={(e) => {
          activeId = e.detail.id;
          dispatch("activeChange", e.detail);
        }}
      />
    </div>
  </UiPresentationProvider>
{/if}

<style>
  .command-palette__overlay {
    position: fixed;
    inset: 0;
    background: color-mix(in srgb, black 44%, transparent);
    backdrop-filter: blur(0.5rem);
    z-index: 40;
  }

  .command-palette {
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

  .command-palette[data-size="xs"] {
    --poodle-command-palette-hint-height: 1.25rem;
    --poodle-command-palette-hint-x: 0.375rem;
    --poodle-command-palette-close-size: 1.5rem;
  }

  .command-palette[data-size="sm"] {
    --poodle-command-palette-hint-height: 1.5rem;
    --poodle-command-palette-close-size: 1.75rem;
  }

  .command-palette[data-size="lg"] {
    --poodle-command-palette-hint-height: 1.75rem;
    --poodle-command-palette-hint-x: 0.625rem;
    --poodle-command-palette-close-size: 2rem;
  }

  .command-palette[data-size="xl"] {
    --poodle-command-palette-hint-height: 2rem;
    --poodle-command-palette-hint-x: 0.75rem;
    --poodle-command-palette-close-size: 2.25rem;
  }

  .command-palette__header {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    gap: var(--poodle-space-inline-md);
    align-items: start;
  }

  .command-palette__header h3,
  .command-palette__header p {
    margin: 0;
  }

  .command-palette__header h3 {
    font-size: 1.375rem;
    line-height: 1.2;
  }

  .command-palette__header p {
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.5;
  }

  .command-palette__meta {
    display: flex;
    gap: var(--poodle-space-inline-sm);
    align-items: center;
  }

  .command-palette__hint {
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

  .command-palette__close {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: var(--poodle-command-palette-close-size);
    height: var(--poodle-command-palette-close-size);
    padding: 0;
    border: 0;
    border-radius: calc(var(--poodle-radius-control) - 0.0625rem);
    background: color-mix(in srgb, var(--poodle-color-background-surface) 62%, transparent);
    color: var(--poodle-color-text-secondary);
    cursor: pointer;
    font: inherit;
  }

  .command-palette__close:hover {
    background: color-mix(in srgb, var(--poodle-color-background-surface) 84%, transparent);
    color: var(--poodle-color-text-primary);
  }

  .command-palette__close:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .command-palette__status {
    margin: 0;
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.5;
  }

  :global([data-theme="light"]) .command-palette {
    border-color: color-mix(in srgb, var(--poodle-color-border-default) 24%, transparent);
    box-shadow:
      0 1.125rem 2.75rem rgba(49, 66, 85, 0.1),
      inset 0 0.0625rem 0 rgba(255, 255, 255, 0.72);
  }

  @media (max-width: 45rem) {
    .command-palette {
      width: min(100vw - 1.25rem, 45rem);
      max-height: calc(100vh - 1.25rem);
      padding: 1rem;
    }

    .command-palette__header {
      grid-template-columns: 1fr;
    }

    .command-palette__meta {
      justify-content: flex-start;
    }
  }
</style>
