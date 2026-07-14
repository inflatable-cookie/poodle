import { useState, type ChangeEvent, type CSSProperties } from "react";
import { BlockEditor, type BlockTypeDefinition, type BlockTypeGroup, type EditorBlock } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";

const blockTypes: BlockTypeDefinition[] = [
  { type: "paragraph", label: "Paragraph", icon: "file-text" },
  { type: "heading", label: "Heading", icon: "hash" },
  { type: "code", label: "Code", icon: "code" },
  { type: "quote", label: "Quote", icon: "bookmark" },
  { type: "divider", label: "Divider", icon: "minus" },
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

const countStyle: CSSProperties = { margin: 0, fontSize: "0.75rem", color: "var(--poodle-color-text-tertiary)" };

const baseInputStyle: CSSProperties = {
  width: "100%",
  padding: "0.25rem 0.375rem",
  border: 0,
  background: "transparent",
  color: "var(--poodle-color-text-primary)",
  fontFamily: "var(--poodle-typography-body-family)",
  fontSize: "0.875rem",
  lineHeight: 1.6,
  outline: "none",
};

const textareaStyle: CSSProperties = { ...baseInputStyle, resize: "vertical" };

const headingStyle: CSSProperties = { ...baseInputStyle, fontSize: "1.125rem", fontWeight: 700 };

const codeStyle: CSSProperties = {
  ...textareaStyle,
  fontFamily: "var(--poodle-typography-code-family)",
  fontSize: "0.8125rem",
  color: "var(--poodle-color-text-secondary)",
};

const quoteStyle: CSSProperties = {
  ...textareaStyle,
  borderLeft: "0.1875rem solid var(--poodle-color-border-default)",
  paddingLeft: "0.625rem",
  color: "var(--poodle-color-text-secondary)",
  fontStyle: "italic",
};

const dividerStyle: CSSProperties = {
  border: 0,
  borderTop: "0.0625rem solid var(--poodle-color-border-subtle)",
  margin: "0.5rem 0",
};

function blockText(block: EditorBlock): string {
  return block.content ?? (block.data as { text?: string } | undefined)?.text ?? "";
}

export function BlockEditorSpecimen() {
  const [blocks, setBlocks] = useState<EditorBlock[]>([
    { id: "1", type: "heading", version: 3, hash: "a1", data: { text: "Block Editor Shell" }, content: "Block Editor Shell" },
    {
      id: "2",
      type: "paragraph",
      version: 3,
      hash: "a2",
      data: {
        text: "Block types and rendering are provided by the consumer. The shell handles ordering, type selection, add/remove, and drag-drop.",
      },
      content:
        "Block types and rendering are provided by the consumer. The shell handles ordering, type selection, add/remove, and drag-drop.",
    },
    {
      id: "3",
      type: "quote",
      version: 3,
      hash: "a3",
      data: { text: "The best way to predict the future is to invent it." },
      content: "The best way to predict the future is to invent it.",
    },
    {
      id: "4",
      type: "code",
      version: 3,
      hash: "a4",
      data: { text: 'console.log("Hello, world!");' },
      content: 'console.log("Hello, world!");',
    },
    { id: "5", type: "divider", version: 3, hash: "a5", data: {}, content: "" },
    {
      id: "6",
      type: "paragraph",
      version: 3,
      hash: "a6",
      data: { text: "Use the type dropdown to change a block, or the + dropdown to add a new one after." },
      content: "Use the type dropdown to change a block, or the + dropdown to add a new one after.",
    },
  ]);

  const [singleBlocks, setSingleBlocks] = useState<EditorBlock[]>([
    {
      id: "hero-1",
      type: "heading",
      version: "initial",
      hash: "hero-hash",
      data: { text: "Single-block Nightfire posture" },
      content: "Single-block Nightfire posture",
    },
  ]);

  return (
    <div className="poodle-specimen">
      <SpecimenGroup label="Consumer-driven block types">
        <BlockEditor
          blocks={blocks}
          blockTypes={blockTypes}
          onChange={setBlocks}
          block={({ block, disabled, update }) => {
            const value = blockText(block);
            const onInput = (e: ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) =>
              update({
                content: e.currentTarget.value,
                data: { ...((block.data as Record<string, unknown>) ?? {}), text: e.currentTarget.value },
              });
            if (block.type === "divider") {
              return <hr style={dividerStyle} />;
            }
            if (block.type === "heading") {
              return (
                <input type="text" style={headingStyle} placeholder="Heading..." disabled={disabled} value={value} onChange={onInput} />
              );
            }
            if (block.type === "code") {
              return (
                <textarea style={codeStyle} placeholder="Code..." disabled={disabled} value={value} onChange={onInput} rows={3} />
              );
            }
            if (block.type === "quote") {
              return (
                <textarea style={quoteStyle} placeholder="Quote..." disabled={disabled} value={value} onChange={onInput} rows={2} />
              );
            }
            return (
              <textarea style={textareaStyle} placeholder="Type something..." disabled={disabled} value={value} onChange={onInput} rows={2} />
            );
          }}
        />
        <p style={countStyle}>{blocks.length} blocks</p>
      </SpecimenGroup>

      <SpecimenGroup label="Single posture with custom grouped type picker">
        <BlockEditor
          blocks={singleBlocks}
          blockTypes={blockTypes}
          blockTypeItems={groupedTypeOptions}
          mode="single"
          onChange={setSingleBlocks}
          block={({ block, disabled, update }) => {
            const value = blockText(block);
            const onInput = (e: ChangeEvent<HTMLInputElement>) =>
              update({
                content: e.currentTarget.value,
                data: { ...((block.data as Record<string, unknown>) ?? {}), text: e.currentTarget.value },
              });
            return (
              <input type="text" style={headingStyle} placeholder="Heading..." disabled={disabled} value={value} onChange={onInput} />
            );
          }}
        />
        <p style={countStyle}>
          Single posture hides reorder, add, and remove controls while the built-in picker accepts grouped Nightfire-style options.
        </p>
      </SpecimenGroup>
    </div>
  );
}
