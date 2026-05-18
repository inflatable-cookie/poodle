<script lang="ts">
  import { MarkdownEditor, Eyebrow } from "@poodle/svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let content = `# Hello World

This is a **markdown editor** with *formatting* support.

## Features

- Bold, italic, and headings
- Links and inline \`code\`
- Lists and blockquotes

> This is a blockquote

Check out [Poodle](https://example.com) for more.`;

  let emptyContent = "";
  let compactContent = "## Compact workspace\n\nToolbar and pane spacing should tighten with density while controls still scale semantically.";
</script>

<SpecimenLayout bareVariants>
  {#snippet children()}
    <div class="poodle-specimen">
      <div class="poodle-specimen__item">
        <Eyebrow>Split view</Eyebrow>
        <MarkdownEditor bind:value={content} mode="split" />
      </div>

      <div class="poodle-specimen__item">
        <Eyebrow>Edit mode</Eyebrow>
        <MarkdownEditor bind:value={emptyContent} mode="edit" placeholder="Start writing..." />
      </div>

      <div class="poodle-specimen__item">
        <Eyebrow>Disabled</Eyebrow>
        <MarkdownEditor value="Read-only content" disabled />
      </div>
    </div>
  {/snippet}

  {#snippet sizes(size)}
    <div class="poodle-md-editor-specimen__variant-block">
      <Eyebrow>{size}</Eyebrow>
      <MarkdownEditor
        value={compactContent}
        mode="split"
        size={size}
        minHeight="10rem"
      />
    </div>
  {/snippet}

  {#snippet densities(density)}
    <div class="poodle-md-editor-specimen__variant-block">
      <Eyebrow>{density}</Eyebrow>
      <MarkdownEditor
        value={compactContent}
        mode="split"
        density={density}
        minHeight="10rem"
      />
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-specimen__item,
  .poodle-md-editor-specimen__variant-block {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
</style>
