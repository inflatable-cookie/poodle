<script lang="ts">
  import { createEventDispatcher, tick } from "svelte";

  import { Icon } from "@pug/svelte-primitives";

  import type { EditorBlock, BlockType } from "./types";

  export let blocks: EditorBlock[] = [{ id: crypto.randomUUID(), type: "paragraph", content: "" }];
  export let isDisabled = false;
  export let ariaLabel = "Block editor";

  const dispatch = createEventDispatcher<{
    change: { blocks: EditorBlock[] };
  }>();

  const blockTypes: { type: BlockType; label: string; icon: string }[] = [
    { type: "paragraph", label: "Paragraph", icon: "file-text" },
    { type: "heading", label: "Heading", icon: "hash" },
    { type: "code", label: "Code", icon: "code" },
    { type: "quote", label: "Quote", icon: "bookmark" },
    { type: "list", label: "List", icon: "list" },
    { type: "image", label: "Image", icon: "image" },
    { type: "divider", label: "Divider", icon: "minus" },
  ];

  let activeBlockId: string | null = null;
  let showAddMenu = false;
  let addMenuIndex = -1;

  function emitChange(): void {
    dispatch("change", { blocks: [...blocks] });
  }

  function addBlock(type: BlockType, afterIndex: number): void {
    const newBlock: EditorBlock = {
      id: crypto.randomUUID(),
      type,
      content: type === "divider" ? "" : "",
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

  function updateContent(index: number, content: string): void {
    blocks[index] = { ...blocks[index], content };
    blocks = [...blocks];
    emitChange();
  }

  function changeType(index: number, type: BlockType): void {
    blocks[index] = { ...blocks[index], type };
    blocks = [...blocks];
    emitChange();
  }

  function openAddMenu(index: number): void {
    addMenuIndex = index;
    showAddMenu = true;
  }

  function handleKeydown(event: KeyboardEvent, index: number): void {
    if (event.key === "Enter" && !event.shiftKey && blocks[index].type === "paragraph") {
      event.preventDefault();
      addBlock("paragraph", index);
    }
    if (event.key === "Backspace" && blocks[index].content === "" && blocks.length > 1) {
      event.preventDefault();
      removeBlock(index);
    }
  }
</script>

<div class="block-editor" class:block-editor--disabled={isDisabled} aria-label={ariaLabel}>
  {#each blocks as block, index (block.id)}
    <div
      class="block-editor__block"
      data-type={block.type}
      class:active={activeBlockId === block.id}
      on:click={() => (activeBlockId = block.id)}
      on:keydown={() => {}}
      role="group"
      aria-label="{block.type} block"
    >
      <div class="block-editor__gutter">
        <button
          type="button"
          class="block-editor__add-btn"
          title="Add block below"
          aria-label="Add block after this one"
          disabled={isDisabled}
          on:click|stopPropagation={() => openAddMenu(index)}
        ><Icon name="plus" size="sm" /></button>
        <div class="block-editor__drag-handle" aria-hidden="true">
          <button
            type="button"
            class="block-editor__move-btn"
            disabled={isDisabled || index === 0}
            on:click|stopPropagation={() => moveBlock(index, -1)}
            aria-label="Move up"
          ><Icon name="arrow-up" size="sm" /></button>
          <button
            type="button"
            class="block-editor__move-btn"
            disabled={isDisabled || index === blocks.length - 1}
            on:click|stopPropagation={() => moveBlock(index, 1)}
            aria-label="Move down"
          ><Icon name="arrow-down" size="sm" /></button>
        </div>
      </div>

      <div class="block-editor__content">
        {#if block.type === "divider"}
          <hr class="block-editor__divider" />
        {:else if block.type === "heading"}
          <input
            type="text"
            class="block-editor__heading-input"
            placeholder="Heading..."
            disabled={isDisabled}
            value={block.content}
            on:input={(e) => updateContent(index, (e.currentTarget as HTMLInputElement).value)}
            on:keydown={(e) => handleKeydown(e, index)}
          />
        {:else if block.type === "code"}
          <textarea
            class="block-editor__code-input"
            placeholder="Code..."
            disabled={isDisabled}
            value={block.content}
            on:input={(e) => updateContent(index, (e.currentTarget as HTMLTextAreaElement).value)}
            rows="3"
          ></textarea>
        {:else if block.type === "image"}
          <div class="block-editor__image-block">
            <input
              type="text"
              class="block-editor__image-url"
              placeholder="Image URL..."
              disabled={isDisabled}
              value={block.content}
              on:input={(e) => updateContent(index, (e.currentTarget as HTMLInputElement).value)}
            />
            {#if block.content}
              <div class="block-editor__image-preview">
                <img src={block.content} alt="Block image" />
              </div>
            {/if}
          </div>
        {:else if block.type === "quote"}
          <textarea
            class="block-editor__quote-input"
            placeholder="Quote..."
            disabled={isDisabled}
            value={block.content}
            on:input={(e) => updateContent(index, (e.currentTarget as HTMLTextAreaElement).value)}
            on:keydown={(e) => handleKeydown(e, index)}
            rows="2"
          ></textarea>
        {:else if block.type === "list"}
          <textarea
            class="block-editor__list-input"
            placeholder="List items (one per line)..."
            disabled={isDisabled}
            value={block.content}
            on:input={(e) => updateContent(index, (e.currentTarget as HTMLTextAreaElement).value)}
            rows="3"
          ></textarea>
        {:else}
          <textarea
            class="block-editor__paragraph-input"
            placeholder="Type something..."
            disabled={isDisabled}
            value={block.content}
            on:input={(e) => updateContent(index, (e.currentTarget as HTMLTextAreaElement).value)}
            on:keydown={(e) => handleKeydown(e, index)}
            rows="1"
          ></textarea>
        {/if}
      </div>

      <div class="block-editor__actions">
        <select
          class="block-editor__type-select"
          value={block.type}
          disabled={isDisabled}
          on:change={(e) => changeType(index, (e.currentTarget as HTMLSelectElement).value as BlockType)}
          aria-label="Block type"
        >
          {#each blockTypes as bt}
            <option value={bt.type}>{bt.icon} {bt.label}</option>
          {/each}
        </select>
        {#if blocks.length > 1}
          <button
            type="button"
            class="block-editor__remove-btn"
            disabled={isDisabled}
            on:click|stopPropagation={() => removeBlock(index)}
            aria-label="Remove block"
          ><Icon name="x" size="sm" /></button>
        {/if}
      </div>
    </div>
  {/each}

  {#if showAddMenu}
    <!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
    <div class="block-editor__add-overlay" on:click={() => (showAddMenu = false)}>
      <!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
      <div class="block-editor__add-menu" on:click|stopPropagation>
        {#each blockTypes as bt}
          <button
            type="button"
            class="block-editor__add-menu-item"
            on:click={() => addBlock(bt.type, addMenuIndex)}
          >
            <span class="block-editor__add-menu-icon"><Icon name={bt.icon} size="sm" /></span>
            <span>{bt.label}</span>
          </button>
        {/each}
      </div>
    </div>
  {/if}
</div>

<style>
  .block-editor {
    border: 0.0625rem solid var(--pug-color-border-default);
    border-radius: var(--pug-radius-surface);
    background: var(--pug-color-background-surface);
    padding: 0.75rem;
  }

  .block-editor--disabled {
    opacity: var(--pug-state-opacity-disabled);
    pointer-events: none;
  }

  .block-editor__block {
    display: flex;
    gap: 0.375rem;
    padding: 0.375rem 0;
    border-radius: var(--pug-radius-control);
    transition: background var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard);
  }

  .block-editor__block:hover,
  .block-editor__block.active {
    background: color-mix(in srgb, var(--pug-color-background-elevated) 42%, transparent);
  }

  .block-editor__gutter {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.125rem;
    padding-top: 0.125rem;
    opacity: 0;
    transition: opacity var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard);
  }

  .block-editor__block:hover .block-editor__gutter,
  .block-editor__block.active .block-editor__gutter {
    opacity: 1;
  }

  .block-editor__add-btn,
  .block-editor__move-btn,
  .block-editor__remove-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1.25rem;
    height: 1.25rem;
    padding: 0;
    border: 0;
    border-radius: var(--pug-radius-control);
    background: transparent;
    color: var(--pug-color-text-tertiary);
    cursor: pointer;
    font-size: 0.75rem;
    line-height: 1;
    transition: background var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard);
  }

  .block-editor__add-btn:hover,
  .block-editor__move-btn:hover,
  .block-editor__remove-btn:hover {
    background: color-mix(in srgb, var(--pug-color-accent-base) 16%, transparent);
    color: var(--pug-color-text-primary);
  }

  .block-editor__remove-btn:hover {
    background: color-mix(in srgb, var(--pug-color-status-danger) 16%, transparent);
    color: var(--pug-color-status-danger);
  }

  .block-editor__drag-handle {
    display: flex;
    flex-direction: column;
    gap: 0;
  }

  .block-editor__content {
    flex: 1;
    min-width: 0;
  }

  .block-editor__paragraph-input,
  .block-editor__heading-input,
  .block-editor__code-input,
  .block-editor__quote-input,
  .block-editor__list-input,
  .block-editor__image-url {
    width: 100%;
    padding: 0.25rem 0.375rem;
    border: 0;
    background: transparent;
    color: var(--pug-color-text-primary);
    font-family: var(--pug-typography-body-family);
    font-size: 0.875rem;
    line-height: 1.6;
    outline: none;
    resize: vertical;
  }

  .block-editor__heading-input {
    font-size: 1.125rem;
    font-weight: 700;
  }

  .block-editor__code-input {
    font-family: var(--pug-typography-code-family);
    font-size: 0.8125rem;
    background: color-mix(in srgb, var(--pug-color-background-elevated) 72%, transparent);
    border-radius: var(--pug-radius-control);
    padding: 0.5rem;
  }

  .block-editor__quote-input {
    border-left: 0.1875rem solid var(--pug-color-border-default);
    padding-left: 0.625rem;
    color: var(--pug-color-text-secondary);
    font-style: italic;
  }

  .block-editor__list-input {
    padding-left: 1rem;
  }

  .block-editor__divider {
    border: 0;
    border-top: 0.0625rem solid var(--pug-color-border-subtle);
    margin: 0.5rem 0;
  }

  .block-editor__image-block {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
  }

  .block-editor__image-url {
    font-size: 0.75rem;
    color: var(--pug-color-text-secondary);
    font-family: var(--pug-typography-code-family);
  }

  .block-editor__image-preview img {
    max-width: 100%;
    max-height: 16rem;
    border-radius: var(--pug-radius-control);
    object-fit: contain;
  }

  .block-editor__actions {
    display: flex;
    align-items: flex-start;
    gap: 0.25rem;
    padding-top: 0.25rem;
    opacity: 0;
    transition: opacity var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard);
  }

  .block-editor__block:hover .block-editor__actions,
  .block-editor__block.active .block-editor__actions {
    opacity: 1;
  }

  .block-editor__type-select {
    padding: 0.125rem 0.25rem;
    border: 0.0625rem solid var(--pug-color-border-default);
    border-radius: var(--pug-radius-control);
    background: transparent;
    color: var(--pug-color-text-secondary);
    font: inherit;
    font-size: 0.625rem;
    cursor: pointer;
  }

  .block-editor__add-overlay {
    position: fixed;
    inset: 0;
    z-index: var(--pug-overlay-z-menu, 100);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .block-editor__add-menu {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(6rem, 1fr));
    gap: 0.25rem;
    padding: 0.5rem;
    border: 0.0625rem solid var(--pug-color-border-default);
    border-radius: var(--pug-radius-surface);
    background: var(--pug-color-background-elevated);
    box-shadow: var(--pug-elevation-overlay);
    min-width: 16rem;
  }

  .block-editor__add-menu-item {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.5rem;
    border: 0;
    border-radius: var(--pug-radius-control);
    background: transparent;
    color: var(--pug-color-text-primary);
    cursor: pointer;
    font: inherit;
    font-size: 0.8125rem;
    text-align: left;
    transition: background var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard);
  }

  .block-editor__add-menu-item:hover {
    background: color-mix(in srgb, var(--pug-color-accent-base) 12%, transparent);
  }

  .block-editor__add-menu-icon {
    font-size: 0.875rem;
    width: 1.25rem;
    text-align: center;
  }
</style>
