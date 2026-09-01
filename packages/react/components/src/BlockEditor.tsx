import "@inflatable-cookie/poodle-core/styles/block-editor.css";

import { useId, useState, type ChangeEvent, type ReactNode } from "react";
import {
  createDragDropController,
  type DragDropCommitResult,
  type DropIntent,
} from "@inflatable-cookie/poodle-core";

import { BlockEditorBlock } from "./block-editor/BlockEditorBlock";
import { DragDropProvider, useOptionalDragDrop } from "./drag-drop";
import { Icon } from "./Icon";
import { Select } from "./Select";
import { UiPresentationProvider, resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type {
  BlockEditorMode,
  BlockType,
  BlockTypeDefinition,
  BlockTypeGroup,
  BlockTypeItems,
  ControlDensity,
  ControlSize,
  EditorBlock,
  SelectItems,
  SelectOption,
  SelectOptionGroup,
  SemanticControlSizeRole,
} from "./types";

export interface BlockEditorTypePickerProps {
  block: EditorBlock;
  index: number;
  disabled: boolean;
  options: SelectOption[];
  groupedOptions: SelectItems;
  changeType: (type: BlockType) => void;
}

export interface BlockEditorAddPickerProps {
  block: EditorBlock;
  index: number;
  disabled: boolean;
  options: SelectOption[];
  groupedOptions: SelectItems;
  addBlock: (type: BlockType) => void;
}

export interface BlockEditorBlockProps {
  block: EditorBlock;
  index: number;
  disabled: boolean;
  update: (updates: Partial<EditorBlock>) => void;
}

export interface BlockEditorProps {
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
  typePicker?: (props: BlockEditorTypePickerProps) => ReactNode;
  addPicker?: (props: BlockEditorAddPickerProps) => ReactNode;
  block?: (props: BlockEditorBlockProps) => ReactNode;
}

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

export function BlockEditor({
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
}: BlockEditorProps) {
  const uiPresentation = useUiPresentation();

  const [activeBlockId, setActiveBlockId] = useState<string | null>(null);
  const editorId = useId();

  /**
   * Join the nearest provider, or own a controller.
   *
   * Isolation does not come from the controller: it comes from the subject
   * family and the registration ids, so an editor that joined a shared
   * provider is still unreachable from a sibling editor holding the same block
   * ids.
   */
  const ambient = useOptionalDragDrop();
  const [ownDragController] = useState(() => (ambient ? null : createDragDropController()));
  const dragController = ambient?.controller ?? ownDragController!;

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const resolvedBlockTypeItems = blockTypeItems ?? blockTypes;
  const selectItems = toSelectItems(resolvedBlockTypeItems);
  const selectOptions = flattenSelectItems(selectItems);
  const canReorder = allowReorder ?? mode === "multi";
  const canAdd = allowAdd ?? mode === "multi";
  const canRemove = allowRemove ?? mode === "multi";
  const canTypeChange = allowTypeChange ?? true;

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
    setActiveBlockId(newBlock.id);
  }

  function removeBlockAt(index: number): void {
    if (blocks.length <= 1) {
      return;
    }

    emitChange(blocks.filter((_, blockIndex) => blockIndex !== index));
  }

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

  /**
   * One accepted drop, one complete block order.
   *
   * Both indices are resolved again at commit: the host may have replaced
   * `blocks` while the pointer was down, and a stale index would move the
   * wrong block.
   */
  function handleDrop(intent: DropIntent): DragDropCommitResult {
    if (disabled || !canReorder) return { status: "rejected", reason: "not reorderable" };

    const from = indexOfBlock(dragController.getSnapshot().session?.subject.id ?? "");
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

  function handleBlockInput(event: ChangeEvent<HTMLTextAreaElement>, index: number): void {
    updateBlock(index, { content: event.currentTarget.value });
  }

  const editor = (
    <div
      className={disabled ? "poodle-block-editor poodle-block-editor--disabled" : "poodle-block-editor"}
      data-size={resolvedSize}
      data-density={resolvedDensity}
      aria-label={ariaLabel}
    >
        {blocks.map((blockItem, index) => (
          <BlockEditorBlock
            key={blockItem.id}
            block={blockItem}
            index={index}
            active={activeBlockId === blockItem.id}
            disabled={disabled || !canReorder}
            subjectKind={subjectKind}
            sourceId={sourceIdOf(blockItem.id)}
            targetId={targetIdOf(blockItem.id)}
            indexOfBlock={indexOfBlock}
            onDrop={handleDrop}
            onActivate={() => setActiveBlockId(blockItem.id)}
          >
            <div className="poodle-block-editor__toolbar">
              <div className="poodle-block-editor__toolbar-left">
                <span
                  className="poodle-block-editor__drag-grip"
                  hidden={!canReorder}
                  title="Drag to reorder"
                  aria-hidden="true"
                >
                  <Icon name="grip-vertical" />
                </span>

                {canTypeChange
                  ? (typePicker?.({
                      block: blockItem,
                      index,
                      disabled,
                      options: selectOptions,
                      groupedOptions: selectItems,
                      changeType: (type) => changeType(index, type),
                    }) ?? (
                      <div
                        className={
                          canReorder
                            ? "poodle-block-editor__type-select"
                            : "poodle-block-editor__type-select poodle-block-editor__type-select--inset"
                        }
                      >
                        <Select
                          value={blockItem.type}
                          options={selectItems}
                          native={false}
                          variant="ghost"
                          menuMinWidth="10rem"
                          ariaLabel="Block type"
                          disabled={disabled}
                          onValueChange={(nextValue) => changeType(index, nextValue)}
                        />
                      </div>
                    ))
                  : null}
              </div>

              <div className="poodle-block-editor__toolbar-right">
                {canReorder ? (
                  <>
                    <button
                      type="button"
                      className="poodle-block-editor__tool-btn"
                      disabled={disabled || index === 0}
                      onClick={(event) => {
                        event.stopPropagation();
                        moveBlock(index, -1);
                      }}
                      aria-label="Move up"
                    >
                      <Icon name="arrow-up" />
                    </button>
                    <button
                      type="button"
                      className="poodle-block-editor__tool-btn"
                      disabled={disabled || index === blocks.length - 1}
                      onClick={(event) => {
                        event.stopPropagation();
                        moveBlock(index, 1);
                      }}
                      aria-label="Move down"
                    >
                      <Icon name="arrow-down" />
                    </button>
                  </>
                ) : null}
                {canAdd
                  ? (addPicker?.({
                      block: blockItem,
                      index,
                      disabled,
                      options: selectOptions,
                      groupedOptions: selectItems,
                      addBlock: (type) => addBlockAfter(index, type),
                    }) ?? (
                      <div className="poodle-block-editor__add-select">
                        <Select
                          value={null}
                          options={selectItems}
                          native={false}
                          variant="ghost"
                          menuMinWidth="10rem"
                          ariaLabel="Add block after this one"
                          disabled={disabled}
                          onValueChange={(nextValue) => addBlockAfter(index, nextValue)}
                          trigger={() => (
                            <span className="poodle-block-editor__tool-btn" aria-hidden="true">
                              <Icon name="plus" />
                            </span>
                          )}
                        />
                      </div>
                    ))
                  : null}
                {canRemove && blocks.length > 1 ? (
                  <button
                    type="button"
                    className="poodle-block-editor__tool-btn poodle-block-editor__remove-btn"
                    disabled={disabled}
                    onClick={(event) => {
                      event.stopPropagation();
                      removeBlockAt(index);
                    }}
                    aria-label="Remove block"
                  >
                    <Icon name="x" />
                  </button>
                ) : null}
              </div>
            </div>

            <div className="poodle-block-editor__content">
              {block?.({
                block: blockItem,
                index,
                disabled,
                update: (updates) => updateBlock(index, updates),
              }) ?? (
                <textarea
                  className="poodle-block-editor__input"
                  placeholder="Type something..."
                  disabled={disabled}
                  value={blockItem.content ?? ""}
                  onChange={(event) => handleBlockInput(event, index)}
                  rows={1}
                />
              )}
            </div>
          </BlockEditorBlock>
        ))}
    </div>
  );

  // An editor that joined a provider contributes registrations to it. One with
  // no provider owns a controller so it still reorders on its own.
  return (
    <UiPresentationProvider sizeScale={resolvedSize} density={resolvedDensity}>
      {ambient ? editor : <DragDropProvider controller={ownDragController!}>{editor}</DragDropProvider>}
    </UiPresentationProvider>
  );
}
