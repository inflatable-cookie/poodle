<script module lang="ts">
  let nextTabsId = 0;
</script>

<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/tabs.css";
  import { onDestroy, onMount, tick, type Snippet } from "svelte";

  import {
    createDragDropController,
    tabIndicatorBox,
    tabsKeydownEvent,
    tabsTransition,
    type CrossWindowDragSourceBridge,
    type DragDropCommitResult,
    type DropIntent,
    type TabsContext as HeadlessTabsContext,
    type TabsEvent as HeadlessTabsEvent,
  } from "@inflatable-cookie/poodle-core";
  import { default as Button } from "./Button.svelte";
  import { default as DragDropProvider } from "./DragDropProvider.svelte";
  import { default as Icon } from "./Icon.svelte";
  import { default as Menu } from "./Menu.svelte";
  import { default as Pill } from "./Pill.svelte";
  import { firstEnabledIndex } from "./internal";
  import { useMotionReady } from "./motion-ready.svelte";
  import { default as TabsItem } from "./tabs-parts/TabsItem.svelte";
  import { getTabsForeignInsert } from "./tabs-foreign-insert";
  import { default as TabsKeyboardTargets } from "./tabs-parts/TabsKeyboardTargets.svelte";
  import { tryDragDrop } from "./drag-drop-context";
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
    TabItem,
  } from "./types";

  /**
   * The contracted Tabs surface. `items` takes the framework's `TabItem`
   * shape; everything else is the contract's own vocabulary.
   *
   * Contract: `docs/contracts/components/tabs.md`. The Rust counterpart is
   * `poodle_specs::TabsSpec`.
   */
  interface Props {
    value?: string | null;
    defaultValue?: string | null;
    items?: TabItem[];
    variant?: "card" | "pill" | "block";
    activeEdge?: "none" | "outline" | "underline";
    activeFill?: "none" | "tint" | "solid";
    orientation?: Orientation;
    activationMode?: "automatic" | "manual";
    bordered?: boolean;
    fullWidth?: boolean;
    reorderable?: boolean;
    ariaLabel?: string | null;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    /**
     * Selection edge on the active tab: `"none"` draws no edge, `"outline"`
     * draws the accent border around the active item (the former `card`
     * variant's selected-border value), `"underline"` draws the accent edge
     * along the inline-end side (the former `strip` variant's indicator).
     * The edge axis is an enum, so outline and underline cannot both apply.
     */
    /**
     * Selection treatment on the active tab: `"none"` draws no fill (the
     * edge and the selected text colour carry selection alone), `"tint"` is
     * the accent-tinted fill; `"solid"` fills the tab with `accent-base` and
     * switches the foreground to `text-inverse` for contrast.
     */
    collapseWhenOverflow?: boolean;
    /**
     * What to do as the strip stops fitting.
     *
     * `"collapse"` is today's behaviour: one threshold, then the whole strip
     * becomes a menu. `"shed"` gives up decoration first — see `shed`.
     */
    overflowStrategy?: "collapse" | "shed";
    /**
     * Which parts to give up, in order, before collapsing.
     *
     * Icons first by default: an icon usually repeats what the label already
     * says, where a count carries information the label does not. Labels are
     * never shed, so no tab becomes an unnamed glyph.
     */
    shed?: ("icon" | "count")[];
    collapseLabel?: string | null;
    showTooltips?: boolean;
    historyKey?: string | null;
    onValueChange?: ((value: string) => void) | undefined;
    onReorder?: ((items: string[]) => void) | undefined;
    onClose?: ((value: string) => void) | undefined;
    /**
     * Host preparation for a tab that may be moved to another window.
     *
     * The whole cross-window seam, in one semantic prop: preparation runs on
     * the pre-drag gesture, the host owns the transaction, and only an opaque
     * receipt leaves the window. Tabs keeps its local reorder either way.
     */
    crossWindowSourceBridge?: CrossWindowDragSourceBridge;
    /**
     * The semantic drag family this strip belongs to.
     *
     * `null` mints a family scoped to this Tabs instance, so two ordinary tab
     * sets sharing one provider are never eligible for each other. An owning
     * composite passes an explicit kind to put the strip in a shared family —
     * without taking over reorder, which stays Tabs' own.
     */
    dragSubjectKind?: string | null;
    children?: Snippet<[string]>;
    actions?: Snippet<[]>;
  }

  let {
    value = $bindable<string | null>(null),
    defaultValue = null,
    items = [],
    variant = "card",
    activeEdge = "none",
    activeFill = "tint",
    orientation = "horizontal",
    activationMode = "automatic",
    bordered = false,
    size = null,
    sizeRole = "chrome",
    density = null,
    collapseWhenOverflow = false,
    overflowStrategy = "collapse",
    shed = ["icon", "count"],
    fullWidth = false,
    collapseLabel = null,
    reorderable = false,
    ariaLabel = null,
    showTooltips = false,
    historyKey = null,
    onValueChange = undefined,
    onReorder = undefined,
    onClose = undefined,
    crossWindowSourceBridge = undefined,
    dragSubjectKind = null,
    children,
    actions,
  }: Props = $props();

  const tabsId = ++nextTabsId;
  const foreignInsert = getTabsForeignInsert();
  const isBrowser = typeof window !== "undefined";
  const uiPresentation = getUiPresentation();
  let tabElements = $state<Record<string, HTMLButtonElement | null>>({});
  let listElement = $state<HTMLDivElement | null>(null);
  let indicatorBox = $state<{ left: number; top: number; width: number; height: number } | null>(null);
  let indicatorSnap = $state(false);
  const motionReady = useMotionReady();
  let rootElement = $state<HTMLDivElement | null>(null);
  let measureListElement = $state<HTMLDivElement | null>(null);
  /** How many entries of `shed` are currently given up. */
  let shedCount = $state(0);
  /**
   * Re-entrancy guard for the measurement pass.
   *
   * The ResizeObserver watches the measure list, and deciding a level means
   * changing that list's width twice — once to shed, once to restore. Without
   * this the observer fires mid-measurement and re-enters, and the transient
   * states leak to the screen: narrowing past the icon threshold flashed the
   * collapsed menu before settling on shed icons.
   */
  let measuring = false;
  let measuringFrame: number | null = null;
  let destroyed = false;
  let uncontrolledValue = $state<string | null>(null);
  let seededDefaultValue = $state(false);
  let focusIndex = $state(0);
  let renderedItems = $state<TabItem[]>([]);
  let lastItemsSignature = $state("");
  let lastSyncedValue = $state<string | null>(null);
  let tooltipIndex = $state<number | null>(null);
  let tooltipTimer = $state<ReturnType<typeof setTimeout> | null>(null);
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
  const isVertical = $derived(orientation === "vertical");

  let indicatorSnapFrame = 0;

  function measureIndicator(snap: boolean): void {
    if (activeEdge !== "underline" || !listElement) {
      indicatorBox = null;
      return;
    }
    if (snap) {
      indicatorSnap = true;
    }
    indicatorBox = tabIndicatorBox(
      listElement,
      tabElements[renderedItems[selectedIndex]?.value ?? ""] ?? null,
      isVertical ? "vertical" : "horizontal",
    );
    if (snap) {
      cancelAnimationFrame(indicatorSnapFrame);
      indicatorSnapFrame = requestAnimationFrame(() => {
        indicatorSnap = false;
      });
    }
  }

  $effect(() => {
    currentValue;
    orientation;
    activeEdge;
    measureIndicator(false);
  });

  $effect(() => {
    if (!listElement) {
      return;
    }
    const observer = new ResizeObserver(() => measureIndicator(true));
    observer.observe(listElement);
    const selected = tabElements[renderedItems[selectedIndex]?.value ?? ""];
    if (selected) {
      observer.observe(selected);
    }
    return () => {
      cancelAnimationFrame(indicatorSnapFrame);
      observer.disconnect();
    };
  });
  const hasPanel = $derived(children !== undefined);
  const hasTooltips = $derived(isVertical || showTooltips);
  const canCollapse = $derived(collapseWhenOverflow && !isVertical);
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

  /**
   * Walk the ladder: full → shed the first part → shed the second → collapse.
   *
   * Every measurement is taken from the hidden measure list, which stays at
   * full fidelity at rest. The shed attribute is set on it and removed inside
   * this function, so no paint sees it and — more importantly — the input to
   * the calculation never depends on its own output. Measuring the *real*
   * strip instead would oscillate at every boundary: shedding icons makes it
   * narrower, which says icons fit, which puts them back.
   */
  async function evaluateCollapsedOverflow(): Promise<void> {
    const isShedding = overflowStrategy === "shed" && !isVertical;

    if (!canCollapse && !isShedding) {
      collapsedByOverflow = false;
      shedCount = 0;
      return;
    }

    if (measuring) {
      return;
    }

    await tick();

    const root = rootElement;
    const measureList = measureListElement;
    if (destroyed || !root || !measureList) {
      return;
    }

    measuring = true;
    const availableWidth = root.getBoundingClientRect().width;
    const fits = (level: number): boolean => {
      measureList.dataset.shed = shed.slice(0, level).join(" ");
      const width = measureList.getBoundingClientRect().width;
      return width <= availableWidth + 1;
    };

    try {
      if (!isShedding) {
        collapsedByOverflow = !fits(0);
        shedCount = 0;
        return;
      }

      // The richest level that fits wins; the loop starts at 0 so a strip that
      // fits keeps everything and nothing changes by default.
      for (let level = 0; level <= shed.length; level += 1) {
        if (fits(level)) {
          shedCount = level;
          collapsedByOverflow = false;
          return;
        }
      }

      // Nothing fit even stripped bare. Collapse if the consumer allowed it;
      // otherwise stay fully shed and overflow as the strip does today.
      collapsedByOverflow = canCollapse;
      // Once collapsed there is no strip to shed, and leaving the state set
      // hides the icon on the menu's own trigger. Parts return with the menu.
      shedCount = canCollapse ? 0 : shed.length;
    } finally {
      delete measureList.dataset.shed;
      // Released after a frame: the observer fires asynchronously, so clearing
      // it synchronously would let the restore-width notification through and
      // re-enter anyway.
      if (destroyed) {
        measuring = false;
        return;
      }
      measuringFrame = requestAnimationFrame(() => {
        measuringFrame = null;
        measuring = false;
      });
    }
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

  onDestroy(() => {
    destroyed = true;
    clearTooltip();
    if (measuringFrame !== null) {
      cancelAnimationFrame(measuringFrame);
      measuringFrame = null;
    }
    measuring = false;
  });

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
          // The button follows the tab's value across keyed reorder; index slots do not.
          const value = result.context.items[effect.index]?.value;
          tick().then(() => {
            if (value) tabElements[value]?.focus();
          });
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
    void variant;
    void canCollapse;
    void actions;
    void evaluateCollapsedOverflow();
  });

  // ── Reorder (shared drag substrate; local and cross-window are one session) ──

  /**
   * Join the nearest provider, or own a controller.
   *
   * Joining is what lets an owning composite arbitrate a tab drop against its
   * own targets. Isolation does not depend on the controller: it comes from
   * the subject family and the registration ids, so a Tabs that joined a
   * shared provider is still unreachable from an ordinary sibling strip.
   */
  const ambient = tryDragDrop();
  const ownDragController = ambient ? undefined : createDragDropController();
  const dragController = ambient?.controller ?? ownDragController!;

  /**
   * The semantic family, and the registration namespace, are different things.
   *
   * `subjectKind` is what other surfaces can match on — shared when a
   * composite says so. `sourceId` / `targetId` are always scoped to this
   * instance, because two strips in one ambient controller may legitimately
   * hold the same tab values and duplicate live ids are an error, not
   * last-writer-wins.
   */
  const subjectKind = $derived(dragSubjectKind ?? `poodle.reorder-item:tabs:${tabsId}`);
  const registrationScope = `tabs:${tabsId}`;

  function sourceIdOf(value: string): string {
    return `${registrationScope}:source:${value}`;
  }

  function targetIdOf(value: string): string {
    return `${registrationScope}:target:${value}`;
  }

  function valueOfTargetId(targetId: string): string {
    const prefix = `${registrationScope}:target:`;
    return targetId.startsWith(prefix) ? targetId.slice(prefix.length) : "";
  }

  function indexOfValue(value: string): number {
    return renderedItems.findIndex((item) => item.value === value);
  }

  /** Whether a subject belongs to this strip at all. */
  function ownsValue(value: string): boolean {
    return indexOfValue(value) >= 0;
  }

  /**
   * Turn one revalidated intent into the machine's reorder.
   *
   * `before`/`after` are relative to the target's own position, and
   * `applyReorder` splices into the *shortened* array, so the index shifts by
   * one when the tab is moving forward.
   */
  function handleDrop(intent: DropIntent): DragDropCommitResult {
    const subjectId = dragController.getSnapshot().session?.subject.id ?? "";
    const from = indexOfValue(subjectId);
    const target = indexOfValue(valueOfTargetId(intent.targetId));
    const foreign = foreignInsert;
    if (from < 0) {
      if (!foreign || target < 0) {
        return { status: "rejected", reason: "same tab" };
      }
      return foreign.commit(subjectId, intent.position === "after" ? target + 1 : target);
    }
    if (target < 0 || from === target) {
      return { status: "rejected", reason: "same tab" };
    }

    const to =
      intent.position === "before"
        ? from < target
          ? target - 1
          : target
        : from < target
          ? target
          : target + 1;

    if (from === to) {
      return { status: "rejected", reason: "same tab" };
    }

    send({ type: "REORDER", fromIndex: from, toIndex: to });
    return { status: "committed" };
  }

  /** Alt+Arrow: the established one-keystroke move, run as a real session. */
  function moveTab(fromIndex: number, direction: -1 | 1): void {
    const from = renderedItems[fromIndex];
    const target = renderedItems[fromIndex + direction];
    if (!from || !target) return;
    dragController.requestKeyboardDrop({
      sourceId: sourceIdOf(from.value),
      targetId: targetIdOf(target.value),
      position: direction === 1 ? "after" : "before",
    });
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

    if (!machineEvent) return;
    event.preventDefault();

    // The reorder step is the one machine event the substrate owns: it must
    // reach the same session, announcement, and revalidation a pointer drop
    // does rather than mutating the order behind the controller's back.
    if (machineEvent.type === "REORDER_STEP") {
      moveTab(machineEvent.fromIndex ?? index, machineEvent.direction);
      return;
    }

    send(machineEvent);
  }
</script>

{#snippet strip()}
<TabsKeyboardTargets
  items={renderedItems}
  {reorderable}
  {subjectKind}
  {targetIdOf}
  {ownsValue}
  onDrop={handleDrop}
/>
<div
  bind:this={rootElement}
  class="poodle-tabs"
  data-variant={variant}
  data-bordered={bordered}
  data-active-edge={activeEdge}
  data-active-fill={activeFill}
  data-orientation={orientation}
  data-size={resolvedSize}
  data-density={resolvedDensity}
  data-collapsed={collapsedByOverflow || undefined}
  data-shed={shedCount > 0 ? shed.slice(0, shedCount).join(" ") : undefined}
  data-full-width={fullWidth || undefined}
  data-motion-ready={motionReady.ready}
  data-indicator-snap={indicatorSnap}
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
      bind:this={listElement}
      class="poodle-tabs__list"
      role="tablist"
      aria-label={ariaLabel ?? undefined}
      aria-orientation={orientation}
    >
      {#each renderedItems as item, index (item.value)}
        <TabsItem
          {item}
          {index}
          {tabsId}
          {subjectKind}
          {reorderable}
          {hasPanel}
          {hasTooltips}
          {isVertical}
          {crossWindowSourceBridge}
          {indexOfValue}
          {ownsValue}
          acceptsForeign={foreignInsert !== null}
          sourceId={sourceIdOf(item.value)}
          targetId={targetIdOf(item.value)}
          selected={currentValue === item.value}
          focused={focusIndex === index}
          tooltipOpen={tooltipIndex === index}
          iconSize={resolvedIconSize}
          anchorElement={tabElements[item.value] ?? null}
          onDrop={handleDrop}
          onElement={(element) => (tabElements[item.value] = element)}
          onSelect={() => send({ type: "SELECT", value: item.value })}
          onClose={() => send({ type: "CLOSE", value: item.value })}
          onFocus={() => {
            focusIndex = index;
            if (isVertical) scheduleTooltip(index);
          }}
          onBlur={() => hasTooltips && dismissTooltip()}
          onEnter={() => hasTooltips && scheduleTooltip(index)}
          onLeave={() => hasTooltips && dismissTooltip()}
          onKeydown={(event) => {
            if (event.key === "Escape" && hasTooltips) dismissTooltip();
            handleKeydown(event, index);
          }}
        />
        {#if item.separator && index < renderedItems.length - 1}
          <span class="poodle-tabs__separator" aria-hidden="true"></span>
        {/if}
      {/each}

      {#if activeEdge === "underline" && indicatorBox}
        <span
          class="poodle-tabs__indicator"
          aria-hidden="true"
          style="left: {indicatorBox.left}px; top: {indicatorBox.top}px; width: {indicatorBox.width}px; height: {indicatorBox.height}px"
        ></span>
      {/if}

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
      data-value={currentValue}
      role="tabpanel"
      tabindex="0"
      aria-labelledby={`poodle-tab-${tabsId}-${currentValue}`}
    >
      {@render children?.(currentValue)}
    </div>
  {/if}
</div>
{/snippet}

{#if ambient}
  {@render strip()}
{:else}
  <DragDropProvider controller={ownDragController}>
    {@render strip()}
  </DragDropProvider>
{/if}
