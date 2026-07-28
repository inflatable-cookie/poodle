<script module lang="ts">
  let nextTabsId = 0;
</script>

<script lang="ts">
  import "@poodle/styles/tabs.css";
  import { onDestroy, onMount, tick, type Snippet } from "svelte";

  import {
    tabsKeydownEvent,
    tabsTransition,
    type TabsContext as HeadlessTabsContext,
    type TabsEvent as HeadlessTabsEvent,
  } from "@poodle/headless";

  import { anchored } from "./anchored";
  import { default as Button } from "./Button.svelte";
  import { default as Icon } from "./Icon.svelte";
  import { default as Menu } from "./Menu.svelte";
  import { default as Pill } from "./Pill.svelte";
  import { firstEnabledIndex } from "./internal";
  import {
    handleDragStart as startDrag,
    handleDragOver as overDrag,
    handleDrop as dropDrag,
  } from "./tabs-reorder";
  import {
    getUiPresentation,
    resolveSemanticControlSize,
    resolveSupportingVisualSize,
  } from "./presentation";

  import type {
    ControlDensity,
    ControlSize,
    Orientation,
    SemanticControlSizeRole,
    TabActivationMode,
    TabItem,
    TabVariant,
  } from "./types";

  interface Props {
    value?: string | null;
    defaultValue?: string | null;
    items?: TabItem[];
    variant?: TabVariant;
    orientation?: Orientation;
    activationMode?: TabActivationMode;
    bordered?: boolean;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    collapseWhenOverflow?: boolean;
    fullWidth?: boolean;
    collapseLabel?: string | null;
    reorderable?: boolean;
    ariaLabel?: string | null;
    showTooltips?: boolean;
    historyKey?: string | null;
    onValueChange?: ((value: string) => void) | undefined;
    onReorder?: ((items: string[]) => void) | undefined;
    onClose?: ((value: string) => void) | undefined;
    onDragStart?: ((value: string, event: DragEvent) => void) | undefined;
    onDragEnd?: ((value: string, event: DragEvent) => void) | undefined;
    children?: Snippet<[string]>;
    actions?: Snippet<[]>;
  }

  let {
    value = $bindable<string | null>(null),
    defaultValue = null,
    items = [],
    variant = "text",
    orientation = "horizontal",
    activationMode = "automatic",
    bordered = true,
    size = null,
    sizeRole = "chrome",
    density = null,
    collapseWhenOverflow = false,
    fullWidth = false,
    collapseLabel = null,
    reorderable = false,
    ariaLabel = null,
    showTooltips = false,
    historyKey = null,
    onValueChange = undefined,
    onReorder = undefined,
    onClose = undefined,
    onDragStart = undefined,
    onDragEnd = undefined,
    children,
    actions,
  }: Props = $props();

  const tabsId = ++nextTabsId;
  const isBrowser = typeof window !== "undefined";
  const uiPresentation = getUiPresentation();
  let tabElements = $state<Array<HTMLButtonElement | null>>([]);
  let rootElement = $state<HTMLDivElement | null>(null);
  let measureListElement = $state<HTMLDivElement | null>(null);
  let uncontrolledValue = $state<string | null>(null);
  let seededDefaultValue = $state(false);
  let focusIndex = $state(0);
  let renderedItems = $state<TabItem[]>([]);
  let lastItemsSignature = $state("");
  let lastSyncedValue = $state<string | null>(null);
  let tooltipIndex = $state<number | null>(null);
  let tooltipTimer = $state<ReturnType<typeof setTimeout> | null>(null);
  let dragSourceIndex = $state<number | null>(null);
  let dragSourceValue = $state<string | null>(null);
  let dropTargetIndex = $state<number | null>(null);
  let collapsedByOverflow = $state(false);
  let historyReady = $state(false);

  function getItemsSignature(nextItems: TabItem[]): string {
    return JSON.stringify(
      nextItems.map((item) => ({
        value: item.value,
        label: item.label,
        icon: item.icon ?? null,
        disabled: item.disabled ?? false,
        closable: item.closable ?? false,
        count: item.count ?? null,
        separator: item.separator ?? false,
      })),
    );
  }

  $effect.pre(() => {
    const itemsSignature = getItemsSignature(items);
    if (itemsSignature === lastItemsSignature) {
      return;
    }

    lastItemsSignature = itemsSignature;
    renderedItems = items;
  });

  $effect.pre(() => {
    if (!seededDefaultValue) {
      uncontrolledValue = defaultValue;
      seededDefaultValue = true;
    }
  });

  const isControlled = $derived(value !== null);
  const currentValue = $derived(
    (isControlled ? value : uncontrolledValue) ??
      renderedItems[firstEnabledIndex(renderedItems)]?.value ??
      null,
  );
  const selectedIndex = $derived(renderedItems.findIndex((item) => item.value === currentValue));
  const hasPanel = $derived(children !== undefined);
  const isVertical = $derived(orientation === "vertical");
  const hasTooltips = $derived(isVertical || showTooltips);
  const canCollapse = $derived(collapseWhenOverflow && !isVertical);
  const resolvedVariant = $derived(variant === "underline" ? "text" : variant);
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const resolvedIconSize = $derived(resolveSupportingVisualSize(resolvedSize));
  const selectedItem = $derived(renderedItems.find((item) => item.value === currentValue) ?? null);
  const collapseTriggerLabel = $derived(collapseLabel ?? selectedItem?.label ?? "Sections");
  const collapsedMenuItems = $derived(
    renderedItems.map((item) => ({
      value: item.value,
      label: item.count === undefined ? item.label : `${item.label} (${item.count})`,
      disabled: item.disabled,
      kind: "radio" as const,
      checked: item.value === currentValue,
    })),
  );

  $effect(() => {
    if (selectedIndex >= 0) {
      focusIndex = selectedIndex;
    }
  });

  async function evaluateCollapsedOverflow(): Promise<void> {
    if (!canCollapse) {
      collapsedByOverflow = false;
      return;
    }

    await tick();

    if (!rootElement || !measureListElement) {
      return;
    }

    const naturalWidth = measureListElement.getBoundingClientRect().width;
    const availableWidth = rootElement.getBoundingClientRect().width;
    collapsedByOverflow = naturalWidth > availableWidth + 1;
  }

  function handleViewportChange(): void {
    void evaluateCollapsedOverflow();
  }

  // ── Tooltip (vertical icon-only mode) ──

  function scheduleTooltip(index: number): void {
    clearTooltip();
    tooltipTimer = setTimeout(() => (tooltipIndex = index), 300);
  }

  function dismissTooltip(): void {
    clearTooltip();
    tooltipIndex = null;
  }

  function clearTooltip(): void {
    if (tooltipTimer) {
      clearTimeout(tooltipTimer);
      tooltipTimer = null;
    }
  }

  onDestroy(() => clearTooltip());

  // Environment paths (URL history restore) apply values directly and keep
  // the old unknown-value semantics; user interactions go through the machine
  // via send().
  function setValue(nextValue: string): void {
    if (!isControlled) {
      uncontrolledValue = nextValue;
    } else {
      value = nextValue;
    }

    onValueChange?.(nextValue);
  }

  const machineContext = $derived<HeadlessTabsContext<TabItem>>({
    items: renderedItems,
    value: currentValue,
    focusIndex,
    activationMode,
    reorderable,
  });

  function send(event: HeadlessTabsEvent): void {
    const result = tabsTransition(machineContext, event);

    if (result.context.items !== machineContext.items) {
      renderedItems = result.context.items;
    }

    if (result.context.focusIndex !== focusIndex) {
      focusIndex = result.context.focusIndex;
    }

    for (const effect of result.effects) {
      switch (effect.type) {
        case "emitValueChange": {
          if (!isControlled) {
            uncontrolledValue = effect.value;
          } else {
            value = effect.value;
          }

          onValueChange?.(effect.value);
          break;
        }
        case "focusTab": {
          const index = effect.index;
          tick().then(() => tabElements[index]?.focus());
          break;
        }
        case "emitReorder": {
          onReorder?.(effect.order);
          break;
        }
        case "emitClose": {
          onClose?.(effect.value);
          break;
        }
      }
    }
  }

  function replaceUrlTabParam(nextValue: string): void {
    if (!isBrowser || !historyKey) return;
    const url = new URL(window.location.href);
    const defaultValue = renderedItems[firstEnabledIndex(renderedItems)]?.value ?? null;
    if (defaultValue && nextValue === defaultValue) {
      url.searchParams.delete(historyKey);
    } else {
      url.searchParams.set(historyKey, nextValue);
    }
    window.history.replaceState(window.history.state, "", url);
  }

  onMount(() => {
    const resizeObserver =
      typeof ResizeObserver === "undefined"
        ? null
        : new ResizeObserver(() => {
            void evaluateCollapsedOverflow();
          });

    if (resizeObserver) {
      if (rootElement) resizeObserver.observe(rootElement);
      if (measureListElement) resizeObserver.observe(measureListElement);
    }

    let handlePopState: (() => void) | null = null;
    if (isBrowser && historyKey) {
      const urlValue = new URL(window.location.href).searchParams.get(historyKey);
      if (urlValue) {
        setValue(urlValue);
        lastSyncedValue = urlValue;
      } else if (currentValue) {
        replaceUrlTabParam(currentValue);
        lastSyncedValue = currentValue;
      }

      handlePopState = () => {
        const nextValue = new URL(window.location.href).searchParams.get(historyKey);
        if (nextValue && nextValue !== currentValue) {
          setValue(nextValue);
          lastSyncedValue = nextValue;
          return;
        }

        if (!nextValue) {
          const fallbackValue = renderedItems[firstEnabledIndex(renderedItems)]?.value ?? null;
          if (fallbackValue && fallbackValue !== currentValue) {
            setValue(fallbackValue);
            lastSyncedValue = fallbackValue;
          }
        }
      };

      window.addEventListener("popstate", handlePopState);
    }
    
    void tick().then(() => {
      historyReady = true;
    });

    return () => {
      resizeObserver?.disconnect();
      if (handlePopState) {
        window.removeEventListener("popstate", handlePopState);
      }
    };
  });

  onMount(() => {
    window.addEventListener("resize", handleViewportChange);
    return () => window.removeEventListener("resize", handleViewportChange);
  });

  $effect(() => {
    if (!historyReady) return;
    if (isBrowser && historyKey && currentValue && currentValue !== lastSyncedValue) {
      replaceUrlTabParam(currentValue);
      lastSyncedValue = currentValue;
    }
  });

  $effect(() => {
    void currentValue;
    void renderedItems;
    void resolvedDensity;
    void resolvedSize;
    void resolvedVariant;
    void canCollapse;
    void actions;
    void evaluateCollapsedOverflow();
  });

  // ── Reorder (native drag session also powers cross-region/window moves) ──

  function resetDrag(): void {
    dragSourceIndex = null;
    dragSourceValue = null;
    dropTargetIndex = null;
  }

  function handleDragStart(event: DragEvent, index: number): void {
    const result = startDrag(event, index, reorderable);
    if (result.dragSourceIndex !== null) {
      dragSourceIndex = result.dragSourceIndex;
      dragSourceValue = renderedItems[index].value;
      onDragStart?.(dragSourceValue, event);
    }
  }

  function handleDragOver(event: DragEvent, index: number): void {
    const result = overDrag(event, index, dragSourceIndex);
    if (result.dropTargetIndex !== null) {
      dropTargetIndex = result.dropTargetIndex;
    }
  }

  function handleDragLeave(): void {
    dropTargetIndex = null;
  }

  function handleDrop(event: DragEvent, index: number): void {
    const result = dropDrag(event, index, dragSourceIndex);
    if (result.fromIndex !== null && result.toIndex !== null) {
      send({ type: "REORDER", fromIndex: result.fromIndex, toIndex: result.toIndex });
    }
  }

  function handleDragEnd(event: DragEvent): void {
    if (dragSourceValue) {
      onDragEnd?.(dragSourceValue, event);
    }
    resetDrag();
  }

  function handleKeydown(event: KeyboardEvent, index: number): void {
    const item = renderedItems[index];

    if (event.key === "Delete" && item?.closable) {
      event.preventDefault();
      send({ type: "CLOSE", value: item.value });
      return;
    }

    const machineEvent = tabsKeydownEvent(
      event.key,
      event.altKey,
      orientation,
      { reorderable, activationMode },
      index,
    );

    if (machineEvent) {
      event.preventDefault();
      send(machineEvent);
    }
  }
</script>

<div
  bind:this={rootElement}
  class="poodle-tabs"
  data-variant={resolvedVariant}
  data-bordered={bordered}
  data-orientation={orientation}
  data-size={resolvedSize}
  data-density={resolvedDensity}
  data-collapsed={collapsedByOverflow || undefined}
  data-full-width={fullWidth || undefined}
>
  {#if canCollapse}
    <div class="poodle-tabs__measure-shell" aria-hidden="true">
      <div bind:this={measureListElement} class="poodle-tabs__list poodle-tabs__list--measure">
        {#each renderedItems as item, index (item.value)}
          <div class="poodle-tabs__item" role="presentation" data-selected={currentValue === item.value}>
            <span class="poodle-tabs__tab">
              {#if item.icon}
                <Icon icon={item.icon} size={resolvedIconSize} />
              {/if}
              <span class="poodle-tabs__label">{item.label}</span>
              {#if item.count !== undefined}
                <Pill
                  tone="neutral"
                  appearance="badge"
                  size={resolvedIconSize}
                  muted
                  adaptiveWidth
                  ariaLabel={`${item.count}`}
                >
                  {item.count}
                </Pill>
              {/if}
            </span>

            {#if item.closable}
              <span class="poodle-tabs__close" aria-hidden="true">
                <Icon name="x" size={resolvedIconSize} />
              </span>
            {/if}
          </div>
          {#if item.separator && index < renderedItems.length - 1}
            <span class="poodle-tabs__separator" aria-hidden="true"></span>
          {/if}
        {/each}
      </div>
    </div>
  {/if}

  {#if collapsedByOverflow}
    <div class="poodle-tabs__collapsed">
      <Menu
        items={collapsedMenuItems}
        ariaLabel={ariaLabel ?? "Sections"}
        triggerAriaLabel={ariaLabel ?? "Sections"}
        size={resolvedSize}
        density={resolvedDensity}
        onAction={(value) => send({ type: "SELECT", value })}
      >
        {#snippet trigger()}
          <Button
            type="button"
            variant="secondary"
            size={resolvedSize}
            density={resolvedDensity}
            leadingIcon="menu"
            chevron
            ariaLabel={ariaLabel ?? collapseTriggerLabel}
          >
            {collapseTriggerLabel}
          </Button>
        {/snippet}
      </Menu>

      {#if actions}
        <div class="poodle-tabs__actions">
          {@render actions()}
        </div>
      {/if}
    </div>
  {:else}
    <div
      class="poodle-tabs__list"
      role="tablist"
      aria-label={ariaLabel ?? undefined}
      aria-orientation={orientation}
    >
      {#each renderedItems as item, index (item.value)}
        <div
          class="poodle-tabs__item"
          role="presentation"
          data-selected={currentValue === item.value}
          data-drag-source={dragSourceIndex === index || undefined}
          data-drop-target={dropTargetIndex === index && dropTargetIndex !== dragSourceIndex || undefined}
          draggable={reorderable && !item.disabled}
          ondragstart={(event) => handleDragStart(event, index)}
          ondragover={(event) => handleDragOver(event, index)}
          ondragleave={handleDragLeave}
          ondrop={(event) => handleDrop(event, index)}
          ondragend={handleDragEnd}
          onmouseenter={() => hasTooltips && scheduleTooltip(index)}
          onmouseleave={() => hasTooltips && dismissTooltip()}
        >
          <button
            bind:this={tabElements[index]}
            type="button"
            class="poodle-tabs__tab"
            disabled={item.disabled === true}
            id={`poodle-tab-${tabsId}-${item.value}`}
            role="tab"
            tabindex={focusIndex === index ? 0 : -1}
            aria-selected={currentValue === item.value ? "true" : "false"}
            aria-controls={hasPanel ? `poodle-tabpanel-${tabsId}-${item.value}` : undefined}
            onfocus={() => { focusIndex = index; if (isVertical) scheduleTooltip(index); }}
            onblur={() => hasTooltips && dismissTooltip()}
            onpointerdown={(event) => {
              if (
                reorderable &&
                event.button === 0 &&
                item.disabled !== true &&
                currentValue !== item.value
              ) {
                send({ type: "SELECT", value: item.value });
              }
            }}
            onclick={() => send({ type: "SELECT", value: item.value })}
            onkeydown={(event) => {
              if (event.key === "Escape" && hasTooltips) dismissTooltip();
              handleKeydown(event, index);
            }}
          >
            {#if item.icon}
              <Icon icon={item.icon} size={resolvedIconSize} />
            {/if}
            <span class="poodle-tabs__label">{item.label}</span>
            {#if item.count !== undefined}
              <Pill
                tone="neutral"
                appearance="badge"
                size={resolvedIconSize}
                muted
                adaptiveWidth
                ariaLabel={`${item.count}`}
              >
                {item.count}
              </Pill>
            {/if}
          </button>

          {#if item.closable}
            <button
              type="button"
              class="poodle-tabs__close"
              aria-label={`Close ${item.label}`}
              onclick={(event) => {
                event.stopPropagation();
                send({ type: "CLOSE", value: item.value });
              }}
            >
              <Icon name="x" size={resolvedIconSize} />
            </button>
          {/if}

          {#if hasTooltips && tooltipIndex === index}
            <span
              use:anchored={{
                anchor: tabElements[index],
                placement: isVertical ? "right" : "bottom",
                offset: 6,
              }}
              class="poodle-tabs__tooltip"
              data-placement={isVertical ? "right" : "bottom"}
              role="tooltip"
            >
              {item.label}
            </span>
          {/if}

        </div>
        {#if item.separator && index < renderedItems.length - 1}
          <span class="poodle-tabs__separator" aria-hidden="true"></span>
        {/if}
      {/each}

      {#if actions}
        <div class="poodle-tabs__actions">
          {@render actions()}
        </div>
      {/if}
    </div>
  {/if}

  {#if hasPanel && currentValue}
    <div
      class="poodle-tabs__panel"
      id={`poodle-tabpanel-${tabsId}-${currentValue}`}
      role="tabpanel"
      tabindex="0"
      aria-labelledby={`poodle-tab-${tabsId}-${currentValue}`}
    >
      {@render children?.(currentValue)}
    </div>
  {/if}
</div>
