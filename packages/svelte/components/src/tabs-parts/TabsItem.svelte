<script lang="ts">
  import type {
    CrossWindowDragSourceBridge,
    DragDropCommitResult,
    DragSourceRegistration,
    DropIntent,
    DropPosition,
    DropTargetRegistration,
  } from "@inflatable-cookie/poodle-core";

  import { default as Icon } from "../Icon.svelte";
  import { default as Pill } from "../Pill.svelte";
  import { anchored } from "../anchored";
  import { useDragDrop } from "../drag-drop";
  import type { ControlSize, TabItem } from "../types";

  /**
   * One interactive tab.
   *
   * It exists as its own component because the drag substrate is consumed
   * through context, and a component cannot consume a provider it renders
   * itself. The measure list has no drag behaviour and stays inline in Tabs.
   */
  interface Props {
    item: TabItem;
    tabsId: number;
    subjectKind: string;
    selected: boolean;
    focused: boolean;
    hasPanel: boolean;
    hasTooltips: boolean;
    isVertical: boolean;
    tooltipOpen: boolean;
    reorderable: boolean;
    iconSize: ControlSize;
    crossWindowSourceBridge?: CrossWindowDragSourceBridge;
    /** Whether a subject id belongs to this strip at all. */
    ownsValue: (value: string) => boolean;
    sourceId: string;
    targetId: string;
    onDrop: (intent: DropIntent) => DragDropCommitResult;
    onElement: (element: HTMLButtonElement | null) => void;
    onSelect: () => void;
    onClose: () => void;
    onKeydown: (event: KeyboardEvent) => void;
    onFocus: () => void;
    onBlur: () => void;
    onEnter: () => void;
    onLeave: () => void;
    anchorElement: HTMLButtonElement | null;
  }

  let {
    item,
    tabsId,
    subjectKind,
    selected,
    focused,
    hasPanel,
    hasTooltips,
    isVertical,
    tooltipOpen,
    reorderable,
    iconSize,
    crossWindowSourceBridge,
    ownsValue,
    sourceId,
    targetId,
    onDrop,
    onElement,
    onSelect,
    onClose,
    onKeydown,
    onFocus,
    onBlur,
    onEnter,
    onLeave,
    anchorElement,
  }: Props = $props();

  const { dragSource, dropTarget, snapshot } = useDragDrop();

  /** A disabled tab cannot be picked up. It is still a place to put one. */
  const canDrag = $derived(reorderable && item.disabled !== true);
  const dragging = $derived(
    $snapshot.sourceId === sourceId &&
      ($snapshot.phase === "dragging" || $snapshot.phase === "dropping"),
  );
  const isDropTarget = $derived(
    $snapshot.targetId === targetId &&
      $snapshot.targetPosture === "accepted" &&
      $snapshot.sourceId !== sourceId,
  );

  const sourceRegistration = $derived<DragSourceRegistration>({
    sourceId,
    subject: { kind: subjectKind, id: item.value },
    allowedOperations: ["move"],
    label: item.label,
    disabled: !canDrag,
    crossWindowSourceBridge,
  });

  /**
   * Contract: the band rule reads the fraction of this tab's own bounds along
   * the strip axis. The origin-facing half is `before`, the trailing half is
   * `after`, so dragging over a sibling and back toward origin is a no-op
   * rather than a swap.
   */
  const targetRegistration = $derived<DropTargetRegistration>({
    targetId,
    acceptedKinds: [subjectKind],
    disabled: !reorderable,
    label: item.label,
    resolvePosition: ({ x, y, rect }): DropPosition =>
      isVertical
        ? y < rect.top + rect.height / 2
          ? "before"
          : "after"
        : x < rect.left + rect.width / 2
          ? "before"
          : "after",
    canDrop: (intent, subject) => {
      // A shared family means another surface's subject can reach this target.
      // Refusing it *here*, during eligibility, is what lets arbitration
      // discard this tab and hand the drop to an eligible ancestor composite.
      // Claiming it and rejecting at commit would swallow the drop instead.
      if (!ownsValue(subject.id)) {
        return { accepted: false, reason: "not this tab set" };
      }
      return subject.id === item.value
        ? { accepted: false, reason: "same tab" }
        : { accepted: true, intent };
    },
    onDrop,
  });
</script>

<div
  class="poodle-tabs__item"
  role="presentation"
  data-selected={selected}
  data-reorderable={canDrag || undefined}
  data-drag-source={dragging || undefined}
  data-drop-target={isDropTarget || undefined}
  onmouseenter={onEnter}
  onmouseleave={onLeave}
  use:dropTarget={targetRegistration}
>
  <button
    bind:this={
      () => anchorElement,
      (element) => onElement(element as HTMLButtonElement | null)
    }
    type="button"
    class="poodle-tabs__tab"
    disabled={item.disabled === true}
    id={`poodle-tab-${tabsId}-${item.value}`}
    data-value={item.value}
    role="tab"
    tabindex={focused ? 0 : -1}
    aria-selected={selected ? "true" : "false"}
    aria-controls={hasPanel ? `poodle-tabpanel-${tabsId}-${item.value}` : undefined}
    onfocus={onFocus}
    onblur={onBlur}
    onclick={onSelect}
    onkeydown={onKeydown}
    use:dragSource={sourceRegistration}
  >
    {#if item.icon}
      <Icon icon={item.icon} size={iconSize} />
    {/if}
    <span class="poodle-tabs__label">{item.label}</span>
    {#if item.count !== undefined}
      <Pill
        tone="neutral"
        appearance="badge"
        size={iconSize}
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
        onClose();
      }}
    >
      <Icon name="x" size={iconSize} />
    </button>
  {/if}

  {#if hasTooltips && tooltipOpen}
    <span
      use:anchored={{
        anchor: anchorElement,
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
