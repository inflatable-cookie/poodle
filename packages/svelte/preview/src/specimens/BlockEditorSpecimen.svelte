<script lang="ts">
  import { BlockEditor } from "@poodle/svelte";
  import type { BlockTypeDefinition, BlockTypeGroup, EditorBlock } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const blockTypes: BlockTypeDefinition[] = [
    { type: "paragraph", label: "Paragraph", icon: "file-text" },
    { type: "heading", label: "Heading", icon: "hash" },
    { type: "code", label: "Code", icon: "code" },
    { type: "quote", label: "Quote", icon: "bookmark" },
    { type: "divider", label: "Divider", icon: "minus" },
  ];

  let blocks: EditorBlock[] = [
    { id: "1", type: "heading", version: 3, hash: "a1", data: { text: "Block Editor Shell" }, content: "Block Editor Shell" },
    { id: "2", type: "paragraph", version: 3, hash: "a2", data: { text: "Block types and rendering are provided by the consumer. The shell handles ordering, type selection, add/remove, and drag-drop." }, content: "Block types and rendering are provided by the consumer. The shell handles ordering, type selection, add/remove, and drag-drop." },
    { id: "3", type: "quote", version: 3, hash: "a3", data: { text: "The best way to predict the future is to invent it." }, content: "The best way to predict the future is to invent it." },
    { id: "4", type: "code", version: 3, hash: "a4", data: { text: 'console.log("Hello, world!");' }, content: 'console.log("Hello, world!");' },
    { id: "5", type: "divider", version: 3, hash: "a5", data: {}, content: "" },
    { id: "6", type: "paragraph", version: 3, hash: "a6", data: { text: "Use the type dropdown to change a block, or the + dropdown to add a new one after." }, content: "Use the type dropdown to change a block, or the + dropdown to add a new one after." },
  ];

  let singleBlocks: EditorBlock[] = [
    {
      id: "hero-1",
      type: "heading",
      version: "initial",
      hash: "hero-hash",
      data: { text: "Single-block Nightfire posture" },
      content: "Single-block Nightfire posture",
    },
  ];

  const groupedTypeOptions: BlockTypeGroup[] = [
    {
      label: "Text",
      options: [
        { type: "paragraph", label: "Paragraph", icon: "file-text" },
        { type: "heading", label: "Heading", icon: "hash" },
        { type: "quote", label: "Quote", icon: "bookmark" },
      ],
    },
    {
      label: "Structure",
      options: [
        { type: "code", label: "Code", icon: "code" },
        { type: "divider", label: "Divider", icon: "minus" },
      ],
    },
  ];

  function handleChange(event: CustomEvent<{ blocks: EditorBlock[] }>): void {
    blocks = event.detail.blocks;
  }

  function handleSingleChange(event: CustomEvent<{ blocks: EditorBlock[] }>): void {
    singleBlocks = event.detail.blocks;
  }
</script>

<div class="poodle-specimen">
  <SpecimenGroup label="Consumer-driven block types">
    <BlockEditor {blocks} {blockTypes} on:change={handleChange}>
      <svelte:fragment slot="block" let:block let:disabled let:update>
        {#if block.type === "divider"}
          <hr class="poodle-block-divider" />
        {:else if block.type === "heading"}
          <input
            type="text"
            class="poodle-block-input poodle-block-input--heading"
            placeholder="Heading..."
            disabled={disabled}
            value={block.content ?? block.data?.text ?? ""}
            on:input={(e) => update({ content: (e.currentTarget).value, data: { ...(block.data ?? {}), text: (e.currentTarget).value } })}
          />
        {:else if block.type === "code"}
          <textarea
            class="poodle-block-input poodle-block-input--code"
            placeholder="Code..."
            disabled={disabled}
            value={block.content ?? block.data?.text ?? ""}
            on:input={(e) => update({ content: (e.currentTarget).value, data: { ...(block.data ?? {}), text: (e.currentTarget).value } })}
            rows="3"
          ></textarea>
        {:else if block.type === "quote"}
          <textarea
            class="poodle-block-input poodle-block-input--quote"
            placeholder="Quote..."
            disabled={disabled}
            value={block.content ?? block.data?.text ?? ""}
            on:input={(e) => update({ content: (e.currentTarget).value, data: { ...(block.data ?? {}), text: (e.currentTarget).value } })}
            rows="2"
          ></textarea>
        {:else}
          <textarea
            class="poodle-block-input"
            placeholder="Type something..."
            disabled={disabled}
            value={block.content ?? block.data?.text ?? ""}
            on:input={(e) => update({ content: (e.currentTarget).value, data: { ...(block.data ?? {}), text: (e.currentTarget).value } })}
            rows="2"
          ></textarea>
        {/if}
      </svelte:fragment>
    </BlockEditor>
    <p class="poodle-specimen__count">{blocks.length} blocks</p>
  </SpecimenGroup>

  <SpecimenGroup label="Single posture with custom grouped type picker">
    <BlockEditor
      blocks={singleBlocks}
      {blockTypes}
      blockTypeItems={groupedTypeOptions}
      mode="single"
      on:change={handleSingleChange}
    >
      <svelte:fragment slot="block" let:block let:disabled let:update>
        <input
          type="text"
          class="poodle-block-input poodle-block-input--heading"
          placeholder="Heading..."
          disabled={disabled}
          value={block.content ?? block.data?.text ?? ""}
          on:input={(e) => update({ content: (e.currentTarget).value, data: { ...(block.data ?? {}), text: (e.currentTarget).value } })}
        />
      </svelte:fragment>
    </BlockEditor>
    <p class="poodle-specimen__count">Single posture hides reorder, add, and remove controls while the built-in picker accepts grouped Nightfire-style options.</p>
  </SpecimenGroup>
</div>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-specimen__count {
    margin: 0;
    font-size: 0.75rem;
    color: var(--poodle-color-text-tertiary);
  }

  /* Block rendering styles — owned by the consumer, not the shell */
  .poodle-block-input {
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

  .poodle-block-input--heading {
    font-size: 1.125rem;
    font-weight: 700;
  }

  .poodle-block-input--code {
    font-family: var(--poodle-typography-code-family);
    font-size: 0.8125rem;
    color: var(--poodle-color-text-secondary);
  }

  .poodle-block-input--quote {
    border-left: 0.1875rem solid var(--poodle-color-border-default);
    padding-left: 0.625rem;
    color: var(--poodle-color-text-secondary);
    font-style: italic;
  }

  .poodle-block-divider {
    border: 0;
    border-top: 0.0625rem solid var(--poodle-color-border-subtle);
    margin: 0.5rem 0;
  }
</style>
