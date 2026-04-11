<script lang="ts">
  import { MarkdownEditor } from "@poodle/svelte";
  import { UiPresentationProvider } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

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

<div class="specimen">
  <SpecimenGroup label="Split view">
    <MarkdownEditor bind:value={content} mode="split" />
  </SpecimenGroup>

  <SpecimenGroup label="Edit mode">
    <MarkdownEditor bind:value={emptyContent} mode="edit" placeholder="Start writing..." />
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <MarkdownEditor value="Read-only content" disabled />
  </SpecimenGroup>

  <SpecimenGroup label="Semantic presentation">
    <UiPresentationProvider density="compact" sizeScale="sm">
      <div class="specimen__stack">
        <MarkdownEditor bind:value={compactContent} mode="split" />
        <MarkdownEditor value={compactContent} mode="preview" sizeRole="prominent" />
      </div>
    </UiPresentationProvider>
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .specimen__stack {
    display: grid;
    gap: 0.75rem;
  }
</style>
