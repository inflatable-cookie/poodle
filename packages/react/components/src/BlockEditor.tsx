import "@inflatable-cookie/poodle-core/styles/block-editor.css";

import { useState, type ChangeEvent, type DragEvent as ReactDragEvent, type ReactNode } from "react";

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
  const [dragSourceIndex, setDragSourceIndex] = useState<number | null>(null);
  const [dragOverIndex, setDragOverIndex] = useState<number | null>(null);

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

  function moveBlock(index: number, direction: -1 | 1): void {
    const target = index + direction;
    if (target < 0 || target >= blocks.length) {
      return;
    }

    const nextBlocks = [...blocks];
    [nextBlocks[index], nextBlocks[target]] = [nextBlocks[target], nextBlocks[index]];
    emitChange(nextBlocks);
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

  function handleDragStart(event: ReactDragEvent, index: number): void {
    if (disabled || !canReorder || !event.dataTransfer) {
      return;
    }

    setDragSourceIndex(index);
    event.dataTransfer.effectAllowed = "move";
    event.dataTransfer.setData("text/plain", String(index));
  }

  function handleDragOver(event: ReactDragEvent, index: number): void {
    if (!canReorder) {
      setDragOverIndex(null);
      return;
    }

    if (dragSourceIndex === null || dragSourceIndex === index) {
      setDragOverIndex(null);
      return;
    }

    event.preventDefault();
    if (event.dataTransfer) {
      event.dataTransfer.dropEffect = "move";
    }
    setDragOverIndex(index);
  }

  function handleDragLeave(): void {
    if (!canReorder) {
      return;
    }

    setDragOverIndex(null);
  }

  function handleDrop(event: ReactDragEvent, targetIndex: number): void {
    if (!canReorder) {
      return;
    }

    event.preventDefault();
    if (dragSourceIndex === null || dragSourceIndex === targetIndex) {
      setDragSourceIndex(null);
      setDragOverIndex(null);
      return;
    }

    const nextBlocks = [...blocks];
    const [moved] = nextBlocks.splice(dragSourceIndex, 1);
    nextBlocks.splice(targetIndex, 0, moved);
    setDragSourceIndex(null);
    setDragOverIndex(null);
    emitChange(nextBlocks);
  }

  function handleDragEnd(): void {
    if (!canReorder) {
      return;
    }

    setDragSourceIndex(null);
    setDragOverIndex(null);
  }

  return (
    <UiPresentationProvider sizeScale={resolvedSize} density={resolvedDensity}>
      <div
        className={disabled ? "poodle-block-editor poodle-block-editor--disabled" : "poodle-block-editor"}
        data-size={resolvedSize}
        data-density={resolvedDensity}
        aria-label={ariaLabel}
      >
        {blocks.map((blockItem, index) => (
          <div
            key={blockItem.id}
            className={[
              "poodle-block-editor__block",
              activeBlockId === blockItem.id ? "poodle-active" : "",
              dragOverIndex === index ? "poodle-drag-over" : "",
              dragSourceIndex === index ? "poodle-dragging" : "",
            ]
              .filter(Boolean)
              .join(" ")}
            data-type={blockItem.type}
            onFocus={() => setActiveBlockId(blockItem.id)}
            onDragOver={(event) => handleDragOver(event, index)}
            onDragLeave={handleDragLeave}
            onDrop={(event) => handleDrop(event, index)}
            role="group"
            aria-label={`${blockItem.type} block`}
          >
            <div className="poodle-block-editor__toolbar">
              <div className="poodle-block-editor__toolbar-left">
                <span
                  className="poodle-block-editor__drag-grip"
                  draggable="true"
                  hidden={!canReorder}
                  onDragStart={(event) => handleDragStart(event, index)}
                  onDragEnd={handleDragEnd}
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
          </div>
        ))}
      </div>
    </UiPresentationProvider>
  );
}
