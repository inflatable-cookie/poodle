<script lang="ts" generics="T extends { id: string; label?: string }">
  import type {
    DragDropCommitResult,
    DragSession,
    DragSourceRegistration,
    DragTerminalOutcome,
    DropIntent,
    DropTargetRegistration,
  } from "@inflatable-cookie/poodle-core";
  import type { Snippet } from "svelte";

  import { default as IconButton } from "../IconButton.svelte";
  import { useDragDrop } from "../drag-drop";
  import type { ControlDensity, ControlSize } from "../types";

  interface Props {
    item: T;
    index: number;
    total: number;
    reorderable: boolean;
    embeddedHandle: boolean;
    isUnavailable: boolean;
    showRemove: boolean;
    lastMoved: boolean;
    resolvedSize: ControlSize;
    resolvedDensity: ControlDensity;
    itemSnippet?: Snippet<[T]>;
    onDrop: (intent: DropIntent) => DragDropCommitResult;
    onDragStart: (session: DragSession) => void;
    onDragEnd: (outcome: DragTerminalOutcome) => void;
    onRemove: (id: string) => void;
    onIdleKeydown: (event: KeyboardEvent, index: number) => void;
  }

  let {
    item,
    index,
    total,
    reorderable,
    embeddedHandle,
    isUnavailable,
    showRemove,
    lastMoved,
    resolvedSize,
    resolvedDensity,
    itemSnippet,
    onDrop,
    onDragStart,
    onDragEnd,
    onRemove,
    onIdleKeydown,
  }: Props = $props();

  const { dragSource, dropTarget, snapshot } = useDragDrop();
  const canDrag = $derived(reorderable && !isUnavailable);
  const dragging = $derived(
    $snapshot.sourceId === item.id &&
      ($snapshot.phase === "dragging" || $snapshot.phase === "dropping"),
  );
  const isDropTarget = $derived(
    $snapshot.targetId === item.id &&
      $snapshot.targetPosture === "accepted" &&
      $snapshot.sourceId !== item.id,
  );
  const grabbed = $derived(dragging && $snapshot.inputKind === "keyboard");

  const sourceRegistration = $derived<DragSourceRegistration>({
    sourceId: item.id,
    subject: { kind: "poodle.editable-list", id: item.id },
    allowedOperations: ["move"],
    label: item.label ?? item.id,
    disabled: !canDrag,
    handle: canDrag && !embeddedHandle ? ".poodle-editable-list__handle" : undefined,
    onDragStart,
    onDragEnd,
  });

  const targetRegistration = $derived<DropTargetRegistration>({
    targetId: item.id,
    acceptedKinds: ["poodle.editable-list"],
    disabled: !canDrag,
    label: item.label ?? item.id,
    resolvePosition: (input) => (input.y < input.rect.top + input.rect.height / 2 ? "before" : "after"),
    canDrop: (intent, subject) =>
      subject.id === intent.targetId ? { accepted: false, reason: "self" } : { accepted: true, intent },
    onDrop,
  });
</script>

<li
  class="poodle-editable-list__item"
  class:poodle-editable-list__item--dragging={dragging}
  class:poodle-editable-list__item--drop-target={isDropTarget}
  class:poodle-editable-list__item--grabbed={grabbed}
  class:poodle-editable-list__item--last-moved={lastMoved}
  class:poodle-editable-list__item--embedded-handle={embeddedHandle}
  role="option"
  tabindex={isUnavailable ? -1 : 0}
  aria-selected="false"
  aria-label={`Reorder ${item.label ?? item.id}. Position ${index + 1} of ${total}. Press space to grab, then arrow keys to move.`}
  data-reorder-index={index}
  use:dragSource={sourceRegistration}
  use:dropTarget={targetRegistration}
  onkeydown={(event) => onIdleKeydown(event, index)}
>
  {#if reorderable && !embeddedHandle}
    <span class="poodle-editable-list__handle" aria-hidden="true">
      <svg viewBox="0 0 16 16" fill="currentColor">
        <circle cx="5" cy="4" r="1.25" />
        <circle cx="11" cy="4" r="1.25" />
        <circle cx="5" cy="8" r="1.25" />
        <circle cx="11" cy="8" r="1.25" />
        <circle cx="5" cy="12" r="1.25" />
        <circle cx="11" cy="12" r="1.25" />
      </svg>
    </span>
  {/if}
  <span class="poodle-editable-list__content">
    {#if itemSnippet}
      {@render itemSnippet(item)}
    {:else}
      {item.label ?? item.id}
    {/if}
  </span>
  {#if showRemove}
    <div class="poodle-editable-list__remove poodle-editable-list__remove--danger-on-hover">
      <IconButton
        icon="x"
        variant="ghost"
        size={resolvedSize}
        sizeRole="chrome"
        density={resolvedDensity}
        disabled={isUnavailable}
        ariaLabel={`Remove ${item.label ?? item.id}`}
        onClick={(event) => {
          event.stopPropagation();
          onRemove(item.id);
        }}
      />
    </div>
  {/if}
</li>
