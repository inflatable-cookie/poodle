<script lang="ts">
  import { createEventDispatcher, tick } from "svelte";

  import Icon from "./Icon.svelte";
  import Select from "./Select.svelte";
  import UiPresentationProvider from "./UiPresentationProvider.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type {
    ControlDensity,
    ControlSize,
    SemanticControlSizeRole,
    SelectOption,
  } from "./types";

  import type { EditorBlock, BlockType, BlockTypeDefinition } from "./types";

  export let blocks: EditorBlock[] = [];
  export let blockTypes: BlockTypeDefinition[] = [];
  export let disabled = false;
  export let ariaLabel = "Block editor";
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;

  const dispatch = createEventDispatcher<{
    change: { blocks: EditorBlock[] };
  }>();

  let activeBlockId: string | null = null;
  let dragSourceIndex: number | null = null;
  let dragOverIndex: number | null = null;
  const uiPresentation = getUiPresentation();

  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;
  $: selectOptions = blockTypes.map<SelectOption>((bt) => ({
    value: bt.type,
    label: bt.label,
    icon: bt.icon,
  }));

  function emitChange(): void {
    dispatch("change", { blocks: [...blocks] });
  }

  function addBlock(type: BlockType, afterIndex: number): void {
    const newBlock: EditorBlock = {
      id: crypto.randomUUID(),
      type,
      content: "",
    };
    blocks = [...blocks.slice(0, afterIndex + 1), newBlock, ...blocks.slice(afterIndex + 1)];
    emitChange();

    tick().then(() => {
      activeBlockId = newBlock.id;
    });
  }

  function removeBlock(index: number): void {
    if (blocks.length <= 1) return;
    blocks = blocks.filter((_, i) => i !== index);
    emitChange();
  }

  function moveBlock(index: number, direction: -1 | 1): void {
    const target = index + direction;
    if (target < 0 || target >= blocks.length) return;
    const copy = [...blocks];
    [copy[index], copy[target]] = [copy[target], copy[index]];
    blocks = copy;
    emitChange();
  }

  function updateBlock(index: number, updates: Partial<EditorBlock>): void {
    blocks[index] = { ...blocks[index], ...updates };
    blocks = [...blocks];
    emitChange();
  }

  function changeType(index: number, type: BlockType): void {
    blocks[index] = { ...blocks[index], type };
    blocks = [...blocks];
    emitChange();
  }

  function handleDragStart(event: DragEvent, index: number): void {
    if (disabled || !event.dataTransfer) return;
    dragSourceIndex = index;
    event.dataTransfer.effectAllowed = "move";
    event.dataTransfer.setData("text/plain", String(index));
  }

  function handleDragOver(event: DragEvent, index: number): void {
    if (dragSourceIndex === null || dragSourceIndex === index) {
      dragOverIndex = null;
      return;
    }
    event.preventDefault();
    if (event.dataTransfer) event.dataTransfer.dropEffect = "move";
    dragOverIndex = index;
  }

  function handleDragLeave(): void {
    dragOverIndex = null;
  }

  function handleDrop(event: DragEvent, targetIndex: number): void {
    event.preventDefault();
    if (dragSourceIndex === null || dragSourceIndex === targetIndex) {
      dragSourceIndex = null;
      dragOverIndex = null;
      return;
    }
    const copy = [...blocks];
    const [moved] = copy.splice(dragSourceIndex, 1);
    copy.splice(targetIndex, 0, moved);
    blocks = copy;
    dragSourceIndex = null;
    dragOverIndex = null;
    emitChange();
  }

  function handleDragEnd(): void {
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
    {#each blocks as block, index (block.id)}
      <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
      <div
        class="poodle-block-editor__block"
        class:poodle-active={activeBlockId === block.id}
        class:poodle-drag-over={dragOverIndex === index}
        class:poodle-dragging={dragSourceIndex === index}
        data-type={block.type}
        on:focusin={() => (activeBlockId = block.id)}
        on:dragover={(e) => handleDragOver(e, index)}
        on:dragleave={handleDragLeave}
        on:drop={(e) => handleDrop(e, index)}
        role="group"
        aria-label={`${block.type} block`}
      >
        <div class="poodle-block-editor__toolbar">
          <div class="poodle-block-editor__toolbar-left">
            <!-- svelte-ignore a11y-no-static-element-interactions -->
            <span
              class="poodle-block-editor__drag-grip"
              draggable="true"
              on:dragstart={(e) => handleDragStart(e, index)}
              on:dragend={handleDragEnd}
              title="Drag to reorder"
              aria-hidden="true"
            ><Icon name="grip-vertical" /></span>

            <div class="poodle-block-editor__type-select">
              <Select
                value={block.type}
                options={selectOptions}
                native={false}
                variant="ghost"
                menuMinWidth="10rem"
                ariaLabel="Block type"
                {disabled}
                on:valueChange={(e) => changeType(index, e.detail.value)}
              />
            </div>
          </div>

          <div class="poodle-block-editor__toolbar-right">
            <button
              type="button"
              class="poodle-block-editor__tool-btn"
              disabled={disabled || index === 0}
              on:click|stopPropagation={() => moveBlock(index, -1)}
              aria-label="Move up"
            ><Icon name="arrow-up" /></button>
            <button
              type="button"
              class="poodle-block-editor__tool-btn"
              disabled={disabled || index === blocks.length - 1}
              on:click|stopPropagation={() => moveBlock(index, 1)}
              aria-label="Move down"
            ><Icon name="arrow-down" /></button>
            <div class="poodle-block-editor__add-select">
              <Select
                value={null}
                options={selectOptions}
                native={false}
                variant="ghost"
                menuMinWidth="10rem"
                ariaLabel="Add block after this one"
                {disabled}
                on:valueChange={(e) => addBlock(e.detail.value, index)}
              >
                <svelte:fragment slot="trigger">
                  <span class="poodle-block-editor__tool-btn" aria-hidden="true">
                    <Icon name="plus" />
                  </span>
                </svelte:fragment>
              </Select>
            </div>
            {#if blocks.length > 1}
              <button
                type="button"
                class="poodle-block-editor__tool-btn poodle-block-editor__remove-btn"
                disabled={disabled}
                on:click|stopPropagation={() => removeBlock(index)}
                aria-label="Remove block"
              ><Icon name="x" /></button>
            {/if}
          </div>
        </div>

        <div class="poodle-block-editor__content">
          <slot
            name="block"
            {block}
            {index}
            disabled={disabled}
            update={(updates: Partial<EditorBlock>) => updateBlock(index, updates)}
          >
            <textarea
              class="poodle-block-editor__input"
              placeholder="Type something..."
              disabled={disabled}
              value={block.content}
              on:input={(e) => updateBlock(index, { content: (e.currentTarget as HTMLTextAreaElement).value })}
              rows="1"
            ></textarea>
          </slot>
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
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-surface);
    background: var(--poodle-color-background-surface);
    padding: var(--poodle-block-editor-shell-y) var(--poodle-block-editor-shell-x);
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

  .poodle-block-editor__type-select {
    flex-shrink: 0;
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

  /* Minimal fallback input (used when no block slot is provided) */
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
