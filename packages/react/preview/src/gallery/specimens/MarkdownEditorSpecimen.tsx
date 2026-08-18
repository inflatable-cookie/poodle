import { useState } from "react";
import { MarkdownEditor } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const compactContent =
  "## Compact workspace\n\nToolbar and pane spacing should tighten with density while controls still scale semantically.";

const initialContent = `# Hello World

This is a **markdown editor** with *formatting* support.

## Features

- Bold, italic, and headings
- Links and inline \`code\`
- Lists and blockquotes

> This is a blockquote

Check out [Poodle](https://example.com) for more.`;

export function MarkdownEditorSpecimen() {
  const [content, setContent] = useState(initialContent);
  const [emptyContent, setEmptyContent] = useState("");

  return (
    <SpecimenLayout
      bareVariants
      sizes={(size) => (
        <SpecimenGroup label={size}>
          <MarkdownEditor value={compactContent} mode="split" size={size} minHeight="10rem" />
        </SpecimenGroup>
      )}
      densities={(density) => (
        <SpecimenGroup label={density}>
          <MarkdownEditor value={compactContent} mode="split" density={density} minHeight="10rem" />
        </SpecimenGroup>
      )}
    >
      <SpecimenGroup label="Split view">
        <MarkdownEditor value={content} onValueChange={setContent} mode="split" />
      </SpecimenGroup>

      <SpecimenGroup label="Edit mode">
        <MarkdownEditor
          value={emptyContent}
          onValueChange={setEmptyContent}
          mode="edit"
          placeholder="Start writing..."
        />
      </SpecimenGroup>

      <SpecimenGroup label="Disabled">
        <MarkdownEditor value="Read-only content" disabled />
      </SpecimenGroup>
    </SpecimenLayout>
  );
}
