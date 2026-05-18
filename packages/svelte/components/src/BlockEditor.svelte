<script lang="ts">
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

<style>
  .poodle-block-editor {
    --poodle-block-editor-shell-x: 0.75rem;
    --poodle-block-editor-shell-y: 0.75rem;
    --poodle-block-editor-stack-gap: 0.5rem;
    --poodle-block-editor-toolbar-y: 0.25rem;
    --poodle-block-editor-toolbar-x: 0.375rem;
    --poodle-block-editor-toolbar-gap: 0.125rem;
    --poodle-block-editor-control-size: 1.5rem;
    --poodle-block-editor-content-x: 0.5rem;
    --poodle-block-editor-content-y: 0.375rem;
    --poodle-block-editor-input-x: 0.375rem;
    --poodle-block-editor-input-y: 0.25rem;
    background: var(--poodle-color-background-surface);
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: var(--poodle-block-editor-stack-gap);
  }

  .poodle-block-editor[data-size="xs"] {
    --poodle-block-editor-control-size: 1.25rem;
  }

  .poodle-block-editor[data-size="sm"] {
    --poodle-block-editor-control-size: 1.5rem;
  }

  .poodle-block-editor[data-size="md"] {
    --poodle-block-editor-control-size: 1.75rem;
  }

  .poodle-block-editor[data-size="lg"] {
    --poodle-block-editor-control-size: 2rem;
  }

  .poodle-block-editor[data-size="xl"] {
    --poodle-block-editor-control-size: 2.25rem;
  }

  .poodle-block-editor[data-density="compact"] {
    --poodle-block-editor-shell-x: 0.625rem;
    --poodle-block-editor-shell-y: 0.625rem;
    --poodle-block-editor-stack-gap: 0.375rem;
    --poodle-block-editor-toolbar-y: 0.1875rem;
    --poodle-block-editor-toolbar-x: 0.25rem;
    --poodle-block-editor-content-x: 0.375rem;
    --poodle-block-editor-content-y: 0.25rem;
    --poodle-block-editor-input-x: 0.25rem;
    --poodle-block-editor-input-y: 0.1875rem;
  }

  .poodle-block-editor[data-density="comfortable"] {
    --poodle-block-editor-shell-x: 1rem;
    --poodle-block-editor-shell-y: 1rem;
    --poodle-block-editor-stack-gap: 0.625rem;
    --poodle-block-editor-toolbar-y: 0.3125rem;
    --poodle-block-editor-toolbar-x: 0.5rem;
    --poodle-block-editor-content-x: 0.625rem;
    --poodle-block-editor-content-y: 0.5rem;
    --poodle-block-editor-input-x: 0.5rem;
    --poodle-block-editor-input-y: 0.3125rem;
  }

  .poodle-block-editor--disabled {
    opacity: var(--poodle-state-opacity-disabled);
    pointer-events: none;
  }

  .poodle-block-editor__block {
    display: flex;
    flex-direction: column;
    border: none;
    border-radius: var(--poodle-radius-control);
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 42%, transparent);
    transition:
      background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      box-shadow var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .poodle-block-editor__block.poodle-active {
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 72%, transparent);
  }

  .poodle-block-editor__block.poodle-drag-over {
    box-shadow: 0 0 0 0.125rem var(--poodle-color-accent-base);
  }

  .poodle-block-editor__block.poodle-dragging {
    opacity: 0.4;
  }

  .poodle-block-editor__toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--poodle-block-editor-toolbar-y) var(--poodle-block-editor-toolbar-x);
    background: transparent;
    border-radius: var(--poodle-radius-control) var(--poodle-radius-control) 0 0;
  }

  .poodle-block-editor__toolbar-left,
  .poodle-block-editor__toolbar-right {
    display: flex;
    align-items: center;
    gap: var(--poodle-block-editor-toolbar-gap);
  }

  .poodle-block-editor__drag-grip {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: var(--poodle-block-editor-control-size);
    height: var(--poodle-block-editor-control-size);
    color: var(--poodle-color-text-tertiary);
    cursor: grab;
    border-radius: var(--poodle-radius-control);
    transition: background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .poodle-block-editor__drag-grip:hover {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 12%, transparent);
    color: var(--poodle-color-text-secondary);
  }

  .poodle-block-editor__drag-grip:active {
    cursor: grabbing;
  }

  .poodle-block-editor__drag-grip[hidden] {
    display: none;
  }

  .poodle-block-editor__type-select {
    flex-shrink: 0;
  }

  .poodle-block-editor__type-select--inset {
    --poodle-block-editor-type-picker-inset: calc(
      var(--poodle-block-editor-content-x) + var(--poodle-block-editor-input-x) -
        var(--poodle-block-editor-toolbar-x)
    );
    margin-left: var(--poodle-block-editor-type-picker-inset);
  }

  .poodle-block-editor__type-select :global(.poodle-select[data-variant="ghost"]) {
    min-height: var(--poodle-block-editor-control-size);
  }

  .poodle-block-editor__type-select :global(.poodle-select[data-variant="ghost"] .poodle-select__trigger) {
    min-height: var(--poodle-block-editor-control-size);
    line-height: var(--poodle-typography-body-lineHeight);
  }

  .poodle-block-editor__type-select :global(.poodle-select[data-variant="ghost"] .poodle-select__trigger-content) {
    min-height: var(--poodle-block-editor-control-size);
    align-items: center;
  }

  .poodle-block-editor__type-select :global(.poodle-select[data-variant="ghost"] .poodle-select__value) {
    display: inline-flex;
    align-items: center;
    min-height: var(--poodle-block-editor-control-size);
  }

  .poodle-block-editor__add-select {
    flex-shrink: 0;
  }

  .poodle-block-editor__tool-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: var(--poodle-block-editor-control-size);
    height: var(--poodle-block-editor-control-size);
    padding: 0;
    border: 0;
    border-radius: var(--poodle-radius-control);
    background: transparent;
    color: var(--poodle-color-text-tertiary);
    cursor: pointer;
    font-size: 0.75rem;
    line-height: 1;
    transition: background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .poodle-block-editor__tool-btn:hover {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 16%, transparent);
    color: var(--poodle-color-text-primary);
  }

  .poodle-block-editor__tool-btn:disabled {
    opacity: 0.3;
    cursor: default;
  }

  .poodle-block-editor__tool-btn:disabled:hover {
    background: transparent;
    color: var(--poodle-color-text-tertiary);
  }

  .poodle-block-editor__remove-btn:hover:not(:disabled) {
    background: color-mix(in srgb, var(--poodle-color-status-danger) 16%, transparent);
    color: var(--poodle-color-status-danger);
  }

  .poodle-block-editor__content {
    padding: var(--poodle-block-editor-content-y) var(--poodle-block-editor-content-x);
    min-height: 1.5rem;
  }

  .poodle-block-editor__input {
    width: 100%;
    padding: var(--poodle-block-editor-input-y) var(--poodle-block-editor-input-x);
    border: 0;
    background: transparent;
    color: var(--poodle-color-text-primary);
    font-family: var(--poodle-typography-body-family);
    font-size: 0.875rem;
    line-height: 1.6;
    outline: none;
    resize: vertical;
  }
</style>
