<script lang="ts">
  import { BlockEditor } from "@pug/svelte-composites";
  import type { EditorBlock } from "@pug/svelte-composites";
  import { Eyebrow } from "@pug/svelte-primitives";

  let blocks: EditorBlock[] = [
    { id: "1", type: "heading", content: "Welcome to the Block Editor" },
    { id: "2", type: "paragraph", content: "This is a basic block editor with support for multiple content types. Click the + button to add new blocks." },
    { id: "3", type: "quote", content: "The best way to predict the future is to invent it." },
    { id: "4", type: "code", content: 'console.log("Hello, world!");' },
    { id: "5", type: "list", content: "Paragraph blocks\nHeading blocks\nCode blocks\nQuotes and lists\nImage blocks\nDividers" },
    { id: "6", type: "divider", content: "" },
    { id: "7", type: "paragraph", content: "Use the arrows to reorder blocks and the type selector to change a block's type." },
  ];

  function handleChange(event: CustomEvent<{ blocks: EditorBlock[] }>): void {
    blocks = event.detail.blocks;
  }
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Block editor</Eyebrow>
    <BlockEditor {blocks} on:change={handleChange} />
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
    color: var(--pug-color-text-tertiary);
  }
</style>
