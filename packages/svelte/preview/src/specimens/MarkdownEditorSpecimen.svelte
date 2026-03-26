<script lang="ts">
  import { MarkdownEditor } from "@poodle/svelte-composites";
  import { Eyebrow } from "@poodle/svelte-primitives";

  let content = `# Hello World

This is a **markdown editor** with *formatting* support.

## Features

- Bold, italic, and headings
- Links and inline \`code\`
- Lists and blockquotes

> This is a blockquote

Check out [Poodle](https://example.com) for more.`;

  let emptyContent = "";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Split view</Eyebrow>
    <MarkdownEditor bind:value={content} mode="split" />
  </div>

  <div class="specimen__group">
    <Eyebrow>Edit mode</Eyebrow>
    <MarkdownEditor bind:value={emptyContent} mode="edit" placeholder="Start writing..." />
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <MarkdownEditor value="Read-only content" disabled />
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
</style>
