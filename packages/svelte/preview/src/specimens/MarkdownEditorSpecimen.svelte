<script lang="ts">
  import { MarkdownEditor } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let content = $state(`# Hello World

This is a **markdown editor** with *formatting* support.

## Features

- Bold, italic, and headings
- Links and inline \`code\`
- Lists and blockquotes

> This is a blockquote

Check out [Poodle](https://example.com) for more.`);

  let emptyContent = $state("");
  let compactContent = "## Compact workspace\n\nToolbar and pane spacing should tighten with density while controls still scale semantically.";
</script>

<SpecimenLayout bareVariants>
  {#snippet children()}
    <SpecimenGroup label="Split view">
      <MarkdownEditor bind:value={content} mode="split" />
    </SpecimenGroup>

    <SpecimenGroup label="Edit mode">
      <MarkdownEditor bind:value={emptyContent} mode="edit" placeholder="Start writing..." />
    </SpecimenGroup>

    <SpecimenGroup label="Disabled">
      <MarkdownEditor value="Read-only content" disabled />
    </SpecimenGroup>
  {/snippet}

  {#snippet sizes(size)}
    <SpecimenGroup label={size}>
      <MarkdownEditor value={compactContent} mode="split" {size} minHeight="10rem" />
    </SpecimenGroup>
  {/snippet}

  {#snippet densities(density)}
    <SpecimenGroup label={density}>
      <MarkdownEditor value={compactContent} mode="split" {density} minHeight="10rem" />
    </SpecimenGroup>
  {/snippet}
</SpecimenLayout>
