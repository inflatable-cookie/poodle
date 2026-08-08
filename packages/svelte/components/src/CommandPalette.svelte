<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/command-palette.css";
  import { trapFocusKeydown } from "@inflatable-cookie/poodle-core";
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
    trapFocusKeydown(document.querySelector<HTMLElement>(".poodle-command-palette"), event);
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

