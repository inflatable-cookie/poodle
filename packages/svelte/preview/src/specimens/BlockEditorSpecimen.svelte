<script lang="ts">
  import { BlockEditor } from "@poodle/svelte-composites";
  import type { EditorBlock, BlockTypeDefinition } from "@poodle/svelte-composites";
  import { Eyebrow } from "@poodle/svelte-primitives";

  let blocks: EditorBlock[] = [
    { id: "1", type: "heading", content: "Welcome to the Block Editor" },
    { id: "2", type: "paragraph", content: "This is a basic block editor with support for multiple content types. Click the + button to add new blocks." },
    { id: "3", type: "quote", content: "The best way to predict the future is to invent it." },
    { id: "4", type: "code", content: 'console.log("Hello, world!");' },
    { id: "5", type: "list", content: "Paragraph blocks\nHeading blocks\nCode blocks\nQuotes and lists\nImage blocks\nDividers" },
    { id: "6", type: "divider", content: "" },
    { id: "7", type: "paragraph", content: "Use the arrows or drag handle to reorder blocks and the type selector to change a block's type." },
  ];

  function handleChange(event: CustomEvent<{ blocks: EditorBlock[] }>): void {
    blocks = event.detail.blocks;
  }

  // Custom block types example
  const customTypes: BlockTypeDefinition[] = [
    { type: "text", label: "Text", icon: "file-text" },
    { type: "callout", label: "Callout", icon: "alert-circle" },
    { type: "embed", label: "Embed", icon: "link" },
  ];

  let customBlocks: EditorBlock[] = [
    { id: "c1", type: "text", content: "Custom block types let you define your own editor." },
    { id: "c2", type: "callout", content: "This is a callout with custom rendering via the block slot." },
    { id: "c3", type: "embed", content: "https://example.com" },
  ];

  function handleCustomChange(event: CustomEvent<{ blocks: EditorBlock[] }>): void {
    customBlocks = event.detail.blocks;
  }
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default block types</Eyebrow>
    <BlockEditor {blocks} on:change={handleChange} />
    <p class="specimen__count">{blocks.length} blocks</p>
  </div>

  <div class="specimen__group">
    <Eyebrow>Custom block types with slot rendering</Eyebrow>
    <BlockEditor blocks={customBlocks} blockTypes={customTypes} on:change={handleCustomChange}>
      <svelte:fragment slot="block" let:block let:isDisabled let:update>
        {#if block.type === "callout"}
          <div class="custom-callout">
            <textarea
              class="custom-callout__input"
              placeholder="Callout message..."
              disabled={isDisabled}
              value={block.content}
              on:input={(e) => update({ content: (e.currentTarget).value })}
              rows="2"
            ></textarea>
          </div>
        {:else if block.type === "embed"}
          <input
            type="url"
            class="custom-embed__input"
            placeholder="Paste URL..."
            disabled={isDisabled}
            value={block.content}
            on:input={(e) => update({ content: (e.currentTarget).value })}
          />
        {:else}
          <textarea
            class="custom-text__input"
            placeholder="Type something..."
            disabled={isDisabled}
            value={block.content}
            on:input={(e) => update({ content: (e.currentTarget).value })}
            rows="2"
          ></textarea>
        {/if}
      </svelte:fragment>
    </BlockEditor>
    <p class="specimen__count">{customBlocks.length} blocks</p>
  </div>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .specimen__group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .specimen__count {
    margin: 0;
    font-size: 0.75rem;
    color: var(--poodle-color-text-tertiary);
  }

  .custom-callout {
    border-left: 0.1875rem solid var(--poodle-color-accent-base);
    padding-left: 0.625rem;
  }

  .custom-callout__input,
  .custom-text__input,
  .custom-embed__input {
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

  .custom-embed__input {
    font-family: var(--poodle-typography-code-family);
    font-size: 0.8125rem;
    color: var(--poodle-color-text-secondary);
  }
</style>
