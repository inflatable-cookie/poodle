<script module lang="ts">
  let nextBlockEditorId = 0;
</script>

<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/block-editor.css";
  import {
    createDragDropController,
    type DragDropCommitResult,
    type DragSourceRegistration,
    type DropIntent,
    type DropPosition,
    type DropTargetRegistration,
  } from "@inflatable-cookie/poodle-core";
  import type { Snippet } from "svelte";
  import { tick } from "svelte";

  import { default as DragDropProvider } from "./DragDropProvider.svelte";
  import { default as Icon } from "./Icon.svelte";
  import { default as Select } from "./Select.svelte";
  import { default as UiPresentationProvider } from "./UiPresentationProvider.svelte";
  import {
    dragDropSnapshotStore,
    dragSourceAction,
    dropTargetAction,
    tryDragDrop,
  } from "./drag-drop-context";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type {
    BlockEditorMode,
    BlockTypeGroup,
    BlockTypeItems,
    ControlDensity,
    ControlSize,
    SemanticControlSizeRole,
    SelectItems,
    SelectOption,
    SelectOptionGroup,
  } from "./types";

  import type { EditorBlock, BlockType, BlockTypeDefinition } from "./types";

  interface TypePickerSnippetProps {
    block: EditorBlock;
    index: number;
    disabled: boolean;
    options: SelectOption[];
    groupedOptions: SelectItems;
    changeType: (type: BlockType) => void;
  }

  interface AddPickerSnippetProps {
    block: EditorBlock;
    index: number;
    disabled: boolean;
    options: SelectOption[];
    groupedOptions: SelectItems;
    addBlock: (type: BlockType) => void;
  }

  interface BlockSnippetProps {
    block: EditorBlock;
    index: number;
    disabled: boolean;
    update: (updates: Partial<EditorBlock>) => void;
  }

  interface Props {
    blocks?: EditorBlock[];
    blockTypes?: BlockTypeDefinition[];
    blockTypeItems?: BlockTypeItems | null;
    disabled?: boolean;
    ariaLabel?: string;
    mode?: BlockEditorMode;
    allowReorder?: boolean | null;
    allowAdd?: boolean | null;
    allowRemove?: boolean | null;
    allowTypeChange?: boolean | null;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onChange?: ((blocks: EditorBlock[]) => void) | null;
    typePicker?: Snippet<[TypePickerSnippetProps]>;
    addPicker?: Snippet<[AddPickerSnippetProps]>;
    block?: Snippet<[BlockSnippetProps]>;
  }

  let {
    blocks = [],
    blockTypes = [],
    blockTypeItems = null,
    disabled = false,
    ariaLabel = "Block editor",
    mode = "multi",
    allowReorder = null,
    allowAdd = null,
    allowRemove = null,
    allowTypeChange = null,
    size = null,
    sizeRole = "control",
    density = null,
    onChange = null,
    typePicker,
    addPicker,
    block,
  }: Props = $props();

  let activeBlockId = $state<string | null>(null);
  const uiPresentation = getUiPresentation();
  const editorId = ++nextBlockEditorId;

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const resolvedBlockTypeItems = $derived(blockTypeItems ?? blockTypes);
  const selectItems = $derived(toSelectItems(resolvedBlockTypeItems));
  const selectOptions = $derived(flattenSelectItems(selectItems));
  const canReorder = $derived(allowReorder ?? (mode === "multi"));
  const canAdd = $derived(allowAdd ?? (mode === "multi"));
  const canRemove = $derived(allowRemove ?? (mode === "multi"));
  const canTypeChange = $derived(allowTypeChange ?? true);

  function isBlockTypeGroupArray(value: BlockTypeItems): value is BlockTypeGroup[] {
    return value.length > 0 && "options" in value[0];
  }

  function toSelectItems(source: BlockTypeItems): SelectItems {
    if (isBlockTypeGroupArray(source)) {
      return source.map((group) => ({
        label: group.label,
        options: group.options.map((option) => ({
          value: option.type,
          label: option.label,
          icon: option.icon,
        })),
      }));
    }

    return source.map((option) => ({
      value: option.type,
      label: option.label,
      icon: option.icon,
    }));
  }

  function flattenSelectItems(items: SelectItems): SelectOption[] {
    if (items.length === 0) {
      return [];
    }

    if ("options" in items[0]) {
      return (items as SelectOptionGroup[]).flatMap((group) => group.options);
    }

    return items as SelectOption[];
  }

  function emitChange(nextBlocks: EditorBlock[]): void {
    onChange?.([...nextBlocks]);
  }

  function addBlockAfter(index: number, type: BlockType): void {
    const newBlock: EditorBlock = {
      id: crypto.randomUUID(),
      type,
      content: "",
      data: {},
    };
    const nextBlocks = [...blocks.slice(0, index + 1), newBlock, ...blocks.slice(index + 1)];
    emitChange(nextBlocks);

    tick().then(() => {
      activeBlockId = newBlock.id;
    });
  }

  function removeBlockAt(index: number): void {
    if (blocks.length <= 1) {
      return;
    }

    emitChange(blocks.filter((_, blockIndex) => blockIndex !== index));
  }

  /**
   * Join the nearest provider, or own a controller.
   *
   * Isolation does not come from the controller: it comes from the subject
   * family and the registration ids, so an editor that joined a shared
   * provider is still unreachable from a sibling editor holding the same block
   * ids.
   */
  const ambient = tryDragDrop();
  const ownDragController = ambient ? undefined : createDragDropController();
  const dragController = ambient?.controller ?? ownDragController!;
  const dragSource = dragSourceAction(dragController);
  const dropTarget = dropTargetAction(dragController);
  const dragSnapshot = dragDropSnapshotStore(dragController);

  /** A single block has nowhere to go, and a disabled editor moves nothing. */
  const canDragBlocks = $derived(canReorder && !disabled && blocks.length > 1);

  const subjectKind = `poodle.reorder-item:block-editor:${editorId}`;
  const registrationScope = `block-editor:${editorId}`;

  function sourceIdOf(id: string): string {
    return `${registrationScope}:source:${id}`;
  }

  function targetIdOf(id: string): string {
    return `${registrationScope}:target:${id}`;
  }

  function idOfTargetId(targetId: string): string {
    const prefix = `${registrationScope}:target:`;
    return targetId.startsWith(prefix) ? targetId.slice(prefix.length) : "";
  }

  function indexOfBlock(id: string): number {
    return blocks.findIndex((entry) => entry.id === id);
  }

  function sourceRegistration(blockItem: EditorBlock): DragSourceRegistration {
    return {
      sourceId: sourceIdOf(blockItem.id),
      subject: { kind: subjectKind, id: blockItem.id },
      allowedOperations: ["move"],
      label: `${blockItem.type} block`,
      disabled: disabled || !canReorder,
      // The grip is the handle; the block body stays an ordinary editing
      // surface, so a press in the textarea or a toolbar control never starts
      // a drag.
      handle: ".poodle-block-editor__drag-grip",
    };
  }

  function targetRegistration(blockItem: EditorBlock, index: number): DropTargetRegistration {
    return {
      targetId: targetIdOf(blockItem.id),
      acceptedKinds: [subjectKind],
      disabled: disabled || !canReorder,
      label: `${blockItem.type} block`,
      // One band per block: a block travelling down lands after its target and
      // one travelling up lands before it, so the dropped block ends up *at*
      // the block it was dropped on.
      resolvePosition: ({ subject }): DropPosition =>
        indexOfBlock(subject.id) < index ? "after" : "before",
      canDrop: (intent, subject) => {
        if (indexOfBlock(subject.id) < 0) {
          return { accepted: false, reason: "not this editor" };
        }
        return subject.id === blockItem.id
          ? { accepted: false, reason: "same block" }
          : { accepted: true, intent };
      },
      onDrop: handleDrop,
    };
  }

  /**
   * One accepted drop, one complete block order.
   *
   * Both indices are resolved again at commit: the host may have replaced
   * `blocks` while the pointer was down, and a stale index would move the
   * wrong block.
   */
  function handleDrop(intent: DropIntent): DragDropCommitResult {
    if (disabled || !canReorder) return { status: "rejected", reason: "not reorderable" };

    const from = indexOfBlock($dragSnapshot.session?.subject.id ?? "");
    const target = indexOfBlock(idOfTargetId(intent.targetId));
    if (from < 0 || target < 0 || from === target) {
      return { status: "rejected", reason: "missing block" };
    }

    const to =
      intent.position === "before"
        ? from < target
          ? target - 1
          : target
        : from < target
          ? target
          : target + 1;

    const nextBlocks = [...blocks];
    const [moved] = nextBlocks.splice(from, 1);
    nextBlocks.splice(to, 0, moved);
    emitChange(nextBlocks);
    return { status: "committed" };
  }

  /**
   * Move up / move down: the keyboard reorder route, run as a real session so
   * it shares eligibility, revalidation, and the single commit with a drop.
   */
  function moveBlock(index: number, direction: -1 | 1): void {
    if (disabled || !canReorder) return;

    const from = blocks[index];
    const target = blocks[index + direction];
    if (!from || !target) return;

    dragController.requestKeyboardDrop({
      sourceId: sourceIdOf(from.id),
      targetId: targetIdOf(target.id),
      position: direction === 1 ? "after" : "before",
    });
  }

  function updateBlock(index: number, updates: Partial<EditorBlock>): void {
    emitChange(blocks.map((blockItem, blockIndex) => (blockIndex === index ? { ...blockItem, ...updates } : blockItem)));
  }

  function changeType(index: number, type: BlockType): void {
    emitChange(blocks.map((blockItem, blockIndex) => (blockIndex === index ? { ...blockItem, type } : blockItem)));
  }

  function getTypePickerContext(blockItem: EditorBlock, index: number): TypePickerSnippetProps {
    return {
      block: blockItem,
      index,
      disabled,
      options: selectOptions,
      groupedOptions: selectItems,
      changeType: (type) => changeType(index, type),
    };
  }

  function getAddPickerContext(blockItem: EditorBlock, index: number): AddPickerSnippetProps {
    return {
      block: blockItem,
      index,
      disabled,
      options: selectOptions,
      groupedOptions: selectItems,
      addBlock: (type) => addBlockAfter(index, type),
    };
  }

  function getBlockContext(blockItem: EditorBlock, index: number): BlockSnippetProps {
    return {
      block: blockItem,
      index,
      disabled,
      update: (updates) => updateBlock(index, updates),
    };
  }

  function handleBlockInput(event: Event, index: number): void {
    const target = event.currentTarget;
    if (target instanceof HTMLTextAreaElement) {
      updateBlock(index, { content: target.value });
    }
  }

</script>

{#snippet editor()}
  <div
    class="poodle-block-editor"
    class:poodle-block-editor--disabled={disabled}
    data-size={resolvedSize}
    data-density={resolvedDensity}
    role="group"
    aria-label={ariaLabel}
  >
    {#each blocks as blockItem, index (blockItem.id)}
      <div
        class="poodle-block-editor__block"
        class:poodle-active={activeBlockId === blockItem.id}
        class:poodle-drag-over={$dragSnapshot.targetId === targetIdOf(blockItem.id) &&
          $dragSnapshot.targetPosture === "accepted"}
        class:poodle-dragging={$dragSnapshot.sourceId === sourceIdOf(blockItem.id) &&
          ($dragSnapshot.phase === "dragging" || $dragSnapshot.phase === "dropping")}
        data-type={blockItem.type}
        onfocusin={() => (activeBlockId = blockItem.id)}
        role="group"
        aria-label={`${blockItem.type} block`}
        use:dragSource={canDragBlocks ? sourceRegistration(blockItem) : null}
        use:dropTarget={canDragBlocks ? targetRegistration(blockItem, index) : null}
      >
        <div class="poodle-block-editor__toolbar">
          <div class="poodle-block-editor__toolbar-left">
            <span
              class="poodle-block-editor__drag-grip"
              hidden={!canReorder}
              title="Drag to reorder"
              aria-hidden="true"
            ><Icon name="grip-vertical" /></span>

            {#if canTypeChange}
              {#if typePicker}
                {@render typePicker(getTypePickerContext(blockItem, index))}
              {:else}
                <div class="poodle-block-editor__type-select" class:poodle-block-editor__type-select--inset={!canReorder}>
                  <Select
                    value={blockItem.type}
                    options={selectItems}
                    native={false}
                    variant="ghost"
                    menuMinWidth="10rem"
                    ariaLabel="Block type"
                    {disabled}
                    onValueChange={(nextValue) => changeType(index, nextValue)}
                  />
                </div>
              {/if}
            {/if}
          </div>

          <div class="poodle-block-editor__toolbar-right">
            {#if canReorder}
              <button
                type="button"
                class="poodle-block-editor__tool-btn"
                disabled={disabled || index === 0}
                onclick={(event) => {
                  event.stopPropagation();
                  moveBlock(index, -1);
                }}
                aria-label="Move up"
              ><Icon name="arrow-up" /></button>
              <button
                type="button"
                class="poodle-block-editor__tool-btn"
                disabled={disabled || index === blocks.length - 1}
                onclick={(event) => {
                  event.stopPropagation();
                  moveBlock(index, 1);
                }}
                aria-label="Move down"
              ><Icon name="arrow-down" /></button>
            {/if}
            {#if canAdd}
              {#if addPicker}
                {@render addPicker(getAddPickerContext(blockItem, index))}
              {:else}
                <div class="poodle-block-editor__add-select">
                  <Select
                    value={null}
                    options={selectItems}
                    native={false}
                    variant="ghost"
                    menuMinWidth="10rem"
                    ariaLabel="Add block after this one"
                    {disabled}
                    onValueChange={(nextValue) => addBlockAfter(index, nextValue)}
                  >
                    {#snippet trigger()}
                      <span class="poodle-block-editor__tool-btn" aria-hidden="true">
                        <Icon name="plus" />
                      </span>
                    {/snippet}
                  </Select>
                </div>
              {/if}
            {/if}
            {#if canRemove && blocks.length > 1}
              <button
                type="button"
                class="poodle-block-editor__tool-btn poodle-block-editor__remove-btn"
                disabled={disabled}
                onclick={(event) => {
                  event.stopPropagation();
                  removeBlockAt(index);
                }}
                aria-label="Remove block"
              ><Icon name="x" /></button>
            {/if}
          </div>
        </div>

        <div class="poodle-block-editor__content">
          {#if block}
            {@render block(getBlockContext(blockItem, index))}
          {:else}
            <textarea
              class="poodle-block-editor__input"
              placeholder="Type something..."
              disabled={disabled}
              value={blockItem.content ?? ""}
              oninput={(event) => handleBlockInput(event, index)}
              rows="1"
            ></textarea>
          {/if}
        </div>
      </div>
    {/each}
  </div>
{/snippet}

<UiPresentationProvider sizeScale={resolvedSize} density={resolvedDensity}>
  {#if ambient}
    {@render editor()}
  {:else}
    <DragDropProvider controller={ownDragController}>
      {@render editor()}
    </DragDropProvider>
  {/if}
</UiPresentationProvider>

