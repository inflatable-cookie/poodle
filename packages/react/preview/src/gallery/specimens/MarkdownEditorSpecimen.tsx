import { useState } from "react";
import { Eyebrow, MarkdownEditor } from "@inflatable-cookie/poodle-react";
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
        <div className="poodle-specimen__stack">
          <Eyebrow>{size}</Eyebrow>
          <MarkdownEditor value={compactContent} mode="split" size={size} minHeight="10rem" />
        </div>
      )}
      densities={(density) => (
        <div className="poodle-specimen__stack">
          <Eyebrow>{density}</Eyebrow>
          <MarkdownEditor value={compactContent} mode="split" density={density} minHeight="10rem" />
        </div>
      )}
    >
      <div className="poodle-specimen">
        <div className="poodle-specimen__stack">
          <Eyebrow>Split view</Eyebrow>
          <MarkdownEditor value={content} onValueChange={setContent} mode="split" />
        </div>

        <div className="poodle-specimen__stack">
          <Eyebrow>Edit mode</Eyebrow>
          <MarkdownEditor
            value={emptyContent}
            onValueChange={setEmptyContent}
            mode="edit"
            placeholder="Start writing..."
          />
        </div>

        <div className="poodle-specimen__stack">
          <Eyebrow>Disabled</Eyebrow>
          <MarkdownEditor value="Read-only content" disabled />
        </div>
      </div>
    </SpecimenLayout>
  );
}
