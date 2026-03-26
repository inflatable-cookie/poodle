<script lang="ts">
  import { createEventDispatcher, tick } from "svelte";

  import { Icon } from "@poodle/svelte-primitives";

  import type { EditorBlock, BlockType, BlockTypeDefinition } from "./types";

  export let blocks: EditorBlock[] = [{ id: crypto.randomUUID(), type: "paragraph", content: "" }];
  export let blockTypes: BlockTypeDefinition[] = [
    { type: "paragraph", label: "Paragraph", icon: "file-text" },
    { type: "heading", label: "Heading", icon: "hash" },
    { type: "code", label: "Code", icon: "code" },
    { type: "quote", label: "Quote", icon: "bookmark" },
    { type: "list", label: "List", icon: "list" },
    { type: "image", label: "Image", icon: "image" },
    { type: "divider", label: "Divider", icon: "minus" },
  ];
  export let disabled = false;
  export let ariaLabel = "Block editor";

  const dispatch = createEventDispatcher<{
    change: { blocks: EditorBlock[] };
  }>();

  let activeBlockId: string | null = null;
  let showAddMenu = false;
  let addMenuIndex = -1;
  let addMenuAnchor: HTMLElement | null = null;
  let addMenuStyle = "";
  let dragSourceIndex: number | null = null;
  let dragOverIndex: number | null = null;

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
    showAddMenu = false;
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

  function openAddMenu(event: MouseEvent, index: number): void {
    addMenuIndex = index;
    addMenuAnchor = event.currentTarget as HTMLElement;
    const rect = addMenuAnchor.getBoundingClientRect();
    addMenuStyle = `top: ${rect.bottom + 4}px; left: ${rect.right}px; transform: translateX(-100%);`;
    showAddMenu = true;
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

<div class="block-editor" class:block-editor--disabled={disabled} aria-label={ariaLabel}>
  {#each blocks as block, index (block.id)}
    <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
    <div
      class="block-editor__block"
      class:active={activeBlockId === block.id}
      class:drag-over={dragOverIndex === index}
      class:dragging={dragSourceIndex === index}
      data-type={block.type}
      on:focusin={() => (activeBlockId = block.id)}
      on:dragover={(e) => handleDragOver(e, index)}
      on:dragleave={handleDragLeave}
      on:drop={(e) => handleDrop(e, index)}
      role="group"
      aria-label="{block.type} block"
    >
      <div class="block-editor__toolbar">
        <div class="block-editor__toolbar-left">
          <!-- svelte-ignore a11y-no-static-element-interactions -->
          <span
            class="block-editor__drag-grip"
            draggable="true"
            on:dragstart={(e) => handleDragStart(e, index)}
            on:dragend={handleDragEnd}
            title="Drag to reorder"
            aria-hidden="true"
          ><Icon name="grip-vertical" size="sm" /></span>

          <select
            class="block-editor__type-select"
            value={block.type}
            disabled={disabled}
            on:change={(e) => changeType(index, (e.currentTarget as HTMLSelectElement).value)}
            aria-label="Block type"
          >
            {#each blockTypes as bt}
              <option value={bt.type}>{bt.label}</option>
            {/each}
          </select>
        </div>

        <div class="block-editor__toolbar-right">
          <button
            type="button"
            class="block-editor__tool-btn"
            disabled={disabled || index === 0}
            on:click|stopPropagation={() => moveBlock(index, -1)}
            aria-label="Move up"
          ><Icon name="arrow-up" size="sm" /></button>
          <button
            type="button"
            class="block-editor__tool-btn"
            disabled={disabled || index === blocks.length - 1}
            on:click|stopPropagation={() => moveBlock(index, 1)}
            aria-label="Move down"
          ><Icon name="arrow-down" size="sm" /></button>
          <button
            type="button"
            class="block-editor__tool-btn block-editor__add-btn"
            title="Add block below"
            aria-label="Add block after this one"
            disabled={disabled}
            on:click|stopPropagation={(e) => openAddMenu(e, index)}
          ><Icon name="plus" size="sm" /></button>
          {#if blocks.length > 1}
            <button
              type="button"
              class="block-editor__tool-btn block-editor__remove-btn"
              disabled={disabled}
              on:click|stopPropagation={() => removeBlock(index)}
              aria-label="Remove block"
            ><Icon name="x" size="sm" /></button>
          {/if}
        </div>
      </div>

      <div class="block-editor__content">
        <slot
          name="block"
          {block}
          {index}
          disabled={disabled}
          update={(updates: Partial<EditorBlock>) => updateBlock(index, updates)}
        >
          <!-- Default rendering for built-in types -->
          {#if block.type === "divider"}
            <hr class="block-editor__divider" />
          {:else if block.type === "heading"}
            <input
              type="text"
              class="block-editor__input block-editor__input--heading"
              placeholder="Heading..."
              disabled={disabled}
              value={block.content}
              on:input={(e) => updateBlock(index, { content: (e.currentTarget as HTMLInputElement).value })}
            />
          {:else if block.type === "code"}
            <textarea
              class="block-editor__input block-editor__input--code"
              placeholder="Code..."
              disabled={disabled}
              value={block.content}
              on:input={(e) => updateBlock(index, { content: (e.currentTarget as HTMLTextAreaElement).value })}
              rows="3"
            ></textarea>
          {:else if block.type === "image"}
            <div class="block-editor__image-block">
              <input
                type="text"
                class="block-editor__input block-editor__input--image-url"
                placeholder="Image URL..."
                disabled={disabled}
                value={block.content}
                on:input={(e) => updateBlock(index, { content: (e.currentTarget as HTMLInputElement).value })}
              />
              {#if block.content}
                <div class="block-editor__image-preview">
                  <img src={block.content} alt="Block preview" />
                </div>
              {/if}
            </div>
          {:else if block.type === "quote"}
            <textarea
              class="block-editor__input block-editor__input--quote"
              placeholder="Quote..."
              disabled={disabled}
              value={block.content}
              on:input={(e) => updateBlock(index, { content: (e.currentTarget as HTMLTextAreaElement).value })}
              rows="2"
            ></textarea>
          {:else if block.type === "list"}
            <textarea
              class="block-editor__input block-editor__input--list"
              placeholder="List items (one per line)..."
              disabled={disabled}
              value={block.content}
              on:input={(e) => updateBlock(index, { content: (e.currentTarget as HTMLTextAreaElement).value })}
              rows="3"
            ></textarea>
          {:else}
            <textarea
              class="block-editor__input"
              placeholder="Type something..."
              disabled={disabled}
              value={block.content}
              on:input={(e) => updateBlock(index, { content: (e.currentTarget as HTMLTextAreaElement).value })}
              rows="1"
            ></textarea>
          {/if}
        </slot>
      </div>
    </div>
  {/each}

  {#if showAddMenu}
    <!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
    <div class="block-editor__add-overlay" on:click={() => (showAddMenu = false)}>
      <!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
      <div class="block-editor__add-menu" style={addMenuStyle} on:click|stopPropagation>
        {#each blockTypes as bt}
          <button
            type="button"
            class="block-editor__add-menu-item"
            on:click={() => addBlock(bt.type, addMenuIndex)}
          >
            <span class="block-editor__add-menu-icon"><Icon icon={bt.icon} size="sm" /></span>
            <span>{bt.label}</span>
          </button>
        {/each}
      </div>
    </div>
  {/if}
</div>

<style>
  .block-editor {
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-surface);
    background: var(--poodle-color-background-surface);
    padding: 0.75rem;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .block-editor--disabled {
    opacity: var(--poodle-state-opacity-disabled);
    pointer-events: none;
  }

  .block-editor__block {
    display: flex;
    flex-direction: column;
    border: none;
    border-radius: var(--poodle-radius-control);
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 42%, transparent);
    transition:
      background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      box-shadow var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .block-editor__block.active {
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 72%, transparent);
  }

  .block-editor__block.drag-over {
    box-shadow: 0 0 0 0.125rem var(--poodle-color-accent-base);
  }

  .block-editor__block.dragging {
    opacity: 0.4;
  }

  .block-editor__toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.25rem 0.375rem;
    border-bottom: none;
    background: transparent;
    border-radius: var(--poodle-radius-control) var(--poodle-radius-control) 0 0;
  }

  .block-editor__toolbar-left,
  .block-editor__toolbar-right {
    display: flex;
    align-items: center;
    gap: 0.125rem;
  }

  .block-editor__drag-grip {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1.25rem;
    height: 1.25rem;
    color: var(--poodle-color-text-tertiary);
    cursor: grab;
    border-radius: var(--poodle-radius-control);
    transition: background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .block-editor__drag-grip:hover {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 12%, transparent);
    color: var(--poodle-color-text-secondary);
  }

  .block-editor__drag-grip:active {
    cursor: grabbing;
  }

  .block-editor__tool-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1.25rem;
    height: 1.25rem;
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

  .block-editor__tool-btn:hover {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 16%, transparent);
    color: var(--poodle-color-text-primary);
  }

  .block-editor__tool-btn:disabled {
    opacity: 0.3;
    cursor: default;
  }

  .block-editor__tool-btn:disabled:hover {
    background: transparent;
    color: var(--poodle-color-text-tertiary);
  }

  .block-editor__remove-btn:hover:not(:disabled) {
    background: color-mix(in srgb, var(--poodle-color-status-danger) 16%, transparent);
    color: var(--poodle-color-status-danger);
  }

  .block-editor__type-select {
    padding: 0.0625rem 0.25rem;
    border: 0.0625rem solid var(--poodle-color-border-subtle);
    border-radius: var(--poodle-radius-control);
    background: transparent;
    color: var(--poodle-color-text-secondary);
    font: inherit;
    font-size: 0.6875rem;
    cursor: pointer;
  }

  .block-editor__content {
    padding: 0.375rem 0.5rem;
    min-height: 1.5rem;
  }

  /* Default block input styles — available when using built-in rendering */
  .block-editor__input {
    width: 100%;
    padding: 0.25rem 0.375rem;
    border: 0;
    background: transparent;
    color: var(--poodle-color-text-primary);
    font-family: var(--poodle-typography-body-family);
    font-size: 0.875rem;
    line-height: 1.6;
    outline: none;
    resize: vertical;
  }

  .block-editor__input--heading {
    font-size: 1.125rem;
    font-weight: 700;
  }

  .block-editor__input--code {
    font-family: var(--poodle-typography-code-family);
    font-size: 0.8125rem;
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 72%, transparent);
    border-radius: var(--poodle-radius-control);
    padding: 0.5rem;
  }

  .block-editor__input--quote {
    border-left: 0.1875rem solid var(--poodle-color-border-default);
    padding-left: 0.625rem;
    color: var(--poodle-color-text-secondary);
    font-style: italic;
  }

  .block-editor__input--list {
    padding-left: 1rem;
  }

  .block-editor__input--image-url {
    font-size: 0.75rem;
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-code-family);
  }

  .block-editor__divider {
    border: 0;
    border-top: 0.0625rem solid var(--poodle-color-border-subtle);
    margin: 0.5rem 0;
  }

  .block-editor__image-block {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
  }

  .block-editor__image-preview img {
    max-width: 100%;
    max-height: 16rem;
    border-radius: var(--poodle-radius-control);
    object-fit: contain;
  }

  .block-editor__add-overlay {
    position: fixed;
    inset: 0;
    z-index: var(--poodle-overlay-z-menu, 100);
  }

  .block-editor__add-menu {
    position: fixed;
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(6rem, 1fr));
    gap: 0.25rem;
    padding: 0.5rem;
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-surface);
    background: var(--poodle-color-background-elevated);
    box-shadow: var(--poodle-elevation-overlay);
    min-width: 16rem;
  }

  .block-editor__add-menu-item {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.5rem;
    border: 0;
    border-radius: var(--poodle-radius-control);
    background: transparent;
    color: var(--poodle-color-text-primary);
    cursor: pointer;
    font: inherit;
    font-size: 0.8125rem;
    text-align: left;
    transition: background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .block-editor__add-menu-item:hover {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 12%, transparent);
  }

  .block-editor__add-menu-icon {
    font-size: 0.875rem;
    width: 1.25rem;
    text-align: center;
  }
</style>
