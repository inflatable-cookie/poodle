import { useState } from "react";
import { BlockEditor, MarkdownEditor, type BlockTypeDefinition, type EditorBlock } from "@poodle/react";
import { registerSpecimen, SpecimenSection } from "../harness";

const blockTypes: BlockTypeDefinition[] = [
  { type: "paragraph", label: "Paragraph", icon: "text" },
  { type: "heading", label: "Heading", icon: "heading" },
  { type: "quote", label: "Quote", icon: "quote" },
];

function EditorsDemo() {
  const [markdown, setMarkdown] = useState("# Hello\n\nSome **bold** text.");
  const [blocks, setBlocks] = useState<EditorBlock[]>([
    { id: "b1", type: "heading", content: "Release notes" },
    { id: "b2", type: "paragraph", content: "First paragraph." },
    { id: "b3", type: "quote", content: "Ship it." },
  ]);

  return (
    <>
      <SpecimenSection title="MarkdownEditor">
        <MarkdownEditor value={markdown} mode="split" onValueChange={setMarkdown} />
        <p data-testid="md-value">{markdown.length} chars</p>
      </SpecimenSection>

      <SpecimenSection title="BlockEditor">
        <BlockEditor blocks={blocks} blockTypes={blockTypes} onChange={setBlocks} />
        <p data-testid="block-order">{blocks.map((b) => b.type).join(",")}</p>
      </SpecimenSection>
    </>
  );
}

registerSpecimen({
  slug: "editors",
  title: "MarkdownEditor / BlockEditor",
  render: () => <EditorsDemo />,
});
