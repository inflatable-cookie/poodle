<script lang="ts">
  import { BlockEditor } from "@poodle/svelte-composites";
  import type { EditorBlock, BlockTypeDefinition } from "@poodle/svelte-composites";
  import { Eyebrow } from "@poodle/svelte-primitives";

  const blockTypes: BlockTypeDefinition[] = [
    { type: "paragraph", label: "Paragraph", icon: "file-text" },
    { type: "heading", label: "Heading", icon: "hash" },
    { type: "code", label: "Code", icon: "code" },
    { type: "quote", label: "Quote", icon: "bookmark" },
    { type: "divider", label: "Divider", icon: "minus" },
  ];

  let blocks: EditorBlock[] = [
    { id: "1", type: "heading", content: "Block Editor Shell" },
    { id: "2", type: "paragraph", content: "Block types and rendering are provided by the consumer. The shell handles ordering, type selection, add/remove, and drag-drop." },
    { id: "3", type: "quote", content: "The best way to predict the future is to invent it." },
    { id: "4", type: "code", content: 'console.log("Hello, world!");' },
    { id: "5", type: "divider", content: "" },
    { id: "6", type: "paragraph", content: "Use the type dropdown to change a block, or the + dropdown to add a new one after." },
  ];

  function handleChange(event: CustomEvent<{ blocks: EditorBlock[] }>): void {
    blocks = event.detail.blocks;
  }
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Consumer-driven block types</Eyebrow>
    <BlockEditor {blocks} {blockTypes} on:change={handleChange}>
      <svelte:fragment slot="block" let:block let:disabled let:update>
        {#if block.type === "divider"}
          <hr class="block-divider" />
        {:else if block.type === "heading"}
          <input
            type="text"
            class="block-input block-input--heading"
            placeholder="Heading..."
            disabled={disabled}
            value={block.content}
            on:input={(e) => update({ content: (e.currentTarget).value })}
          />
        {:else if block.type === "code"}
          <textarea
            class="block-input block-input--code"
            placeholder="Code..."
            disabled={disabled}
            value={block.content}
            on:input={(e) => update({ content: (e.currentTarget).value })}
            rows="3"
          ></textarea>
        {:else if block.type === "quote"}
          <textarea
            class="block-input block-input--quote"
            placeholder="Quote..."
            disabled={disabled}
            value={block.content}
            on:input={(e) => update({ content: (e.currentTarget).value })}
            rows="2"
          ></textarea>
        {:else}
          <textarea
            class="block-input"
            placeholder="Type something..."
            disabled={disabled}
            value={block.content}
            on:input={(e) => update({ content: (e.currentTarget).value })}
            rows="2"
          ></textarea>
        {/if}
      </svelte:fragment>
    </BlockEditor>
    <p class="specimen__count">{blocks.length} blocks</p>
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

  /* Block rendering styles — owned by the consumer, not the shell */
  .block-input {
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

  .block-input--heading {
    font-size: 1.125rem;
    font-weight: 700;
  }

  .block-input--code {
    font-family: var(--poodle-typography-code-family);
    font-size: 0.8125rem;
    color: var(--poodle-color-text-secondary);
  }

  .block-input--quote {
    border-left: 0.1875rem solid var(--poodle-color-border-default);
    padding-left: 0.625rem;
    color: var(--poodle-color-text-secondary);
    font-style: italic;
  }

  .block-divider {
    border: 0;
    border-top: 0.0625rem solid var(--poodle-color-border-subtle);
    margin: 0.5rem 0;
  }
</style>
