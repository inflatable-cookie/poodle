<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/block-editor.css";
  import type { Snippet } from "svelte";
  import { tick } from "svelte";

  import { default as Icon } from "./Icon.svelte";
  import { default as Select } from "./Select.svelte";
  import { default as UiPresentationProvider } from "./UiPresentationProvider.svelte";
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
  let dragSourceIndex = $state<number | null>(null);
  let dragOverIndex = $state<number | null>(null);
  const uiPresentation = getUiPresentation();

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

  function handleDragStart(event: DragEvent, index: number): void {
    if (disabled || !canReorder || !event.dataTransfer) {
      return;
    }

    dragSourceIndex = index;
    event.dataTransfer.effectAllowed = "move";
    event.dataTransfer.setData("text/plain", String(index));
  }

  function handleDragOver(event: DragEvent, index: number): void {
    if (!canReorder) {
      dragOverIndex = null;
      return;
    }

    if (dragSourceIndex === null || dragSourceIndex === index) {
      dragOverIndex = null;
      return;
    }

    event.preventDefault();
    if (event.dataTransfer) {
      event.dataTransfer.dropEffect = "move";
    }
    dragOverIndex = index;
  }

  function handleDragLeave(): void {
    if (!canReorder) {
      return;
    }

    dragOverIndex = null;
  }

  function handleDrop(event: DragEvent, targetIndex: number): void {
    if (!canReorder) {
      return;
    }

    event.preventDefault();
    if (dragSourceIndex === null || dragSourceIndex === targetIndex) {
      dragSourceIndex = null;
      dragOverIndex = null;
      return;
    }

    const nextBlocks = [...blocks];
    const [moved] = nextBlocks.splice(dragSourceIndex, 1);
    nextBlocks.splice(targetIndex, 0, moved);
    dragSourceIndex = null;
    dragOverIndex = null;
    emitChange(nextBlocks);
  }

  function handleDragEnd(): void {
    if (!canReorder) {
      return;
    }

    dragSourceIndex = null;
    dragOverIndex = null;
  }
</script>

<UiPresentationProvider sizeScale={resolvedSize} density={resolvedDensity}>
  <div
    class="poodle-block-editor"
    class:poodle-block-editor--disabled={disabled}
    data-size={resolvedSize}
    data-density={resolvedDensity}
    role="group"
    aria-label={ariaLabel}
  >
    {#each blocks as blockItem, index (blockItem.id)}
      <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
      <div
        class="poodle-block-editor__block"
        class:poodle-active={activeBlockId === blockItem.id}
        class:poodle-drag-over={dragOverIndex === index}
        class:poodle-dragging={dragSourceIndex === index}
        data-type={blockItem.type}
        onfocusin={() => (activeBlockId = blockItem.id)}
        ondragover={(event) => handleDragOver(event, index)}
        ondragleave={handleDragLeave}
        ondrop={(event) => handleDrop(event, index)}
        role="group"
        aria-label={`${blockItem.type} block`}
      >
        <div class="poodle-block-editor__toolbar">
          <div class="poodle-block-editor__toolbar-left">
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <span
              class="poodle-block-editor__drag-grip"
              draggable="true"
              hidden={!canReorder}
              ondragstart={(event) => handleDragStart(event, index)}
              ondragend={handleDragEnd}
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
</UiPresentationProvider>

