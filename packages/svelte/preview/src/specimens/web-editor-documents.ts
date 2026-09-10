import type {
  CodeEditorDiagnostic,
  ProseMirrorDocumentJSON,
  RichTextFeature,
} from "@inflatable-cookie/poodle-core";
import { RICH_TEXT_STANDARD_FEATURES } from "@inflatable-cookie/poodle-core";

/** Representative TypeScript source for the live CodeEditor specimen. */
export const CODE_TYPESCRIPT_SOURCE = `type Point = { x: number; y: number };

export function distance(a: Point, b: Point): number {
  const dx = a.x - b.x;
  const dy = a.y - b.y;
  return Math.hypot(dx, dy);
}
`;

/** Compact JSON source used by the language-configuration control. */
export const CODE_JSON_SOURCE = `{
  "ok": true,
  "editor": "code"
}
`;

/** Short source with a known diagnostic range on `42`. */
export const CODE_DIAGNOSTIC_SOURCE = "const greeting: string = 42;";

export const CODE_DIAGNOSTICS: CodeEditorDiagnostic[] = [
  {
    id: "type-mismatch",
    severity: "error",
    message: "Type 'number' is not assignable to type 'string'.",
    range: { from: 25, to: 27 },
    source: "preview",
  },
];

/** Ordinary formatted document with a table. Images stay absent. */
export const RICH_TEXT_STANDARD_DOCUMENT: ProseMirrorDocumentJSON = {
  type: "doc",
  content: [
    { type: "heading", attrs: { level: 1 }, content: [{ type: "text", text: "Release notes" }] },
    {
      type: "paragraph",
      content: [
        { type: "text", text: "A " },
        { type: "text", text: "formatted", marks: [{ type: "bold" }] },
        { type: "text", text: " document with " },
        { type: "text", text: "italic", marks: [{ type: "italic" }] },
        { type: "text", text: " text and a " },
        {
          type: "text",
          text: "link",
          marks: [{ type: "link", attrs: { href: "https://example.com" } }],
        },
        { type: "text", text: "." },
      ],
    },
    {
      type: "bulletList",
      content: [
        {
          type: "listItem",
          content: [{ type: "paragraph", content: [{ type: "text", text: "Lists stay in the document" }] }],
        },
        {
          type: "listItem",
          content: [{ type: "paragraph", content: [{ type: "text", text: "Tables are standard" }] }],
        },
      ],
    },
    {
      type: "blockquote",
      content: [{ type: "paragraph", content: [{ type: "text", text: "Quotes are ordinary content." }] }],
    },
    { type: "codeBlock", content: [{ type: "text", text: "const shipped = true;" }] },
    { type: "horizontalRule" },
    {
      type: "table",
      content: [
        {
          type: "tableRow",
          content: [
            { type: "tableHeader", content: [{ type: "paragraph", content: [{ type: "text", text: "Surface" }] }] },
            { type: "tableHeader", content: [{ type: "paragraph", content: [{ type: "text", text: "Status" }] }] },
          ],
        },
        {
          type: "tableRow",
          content: [
            { type: "tableCell", content: [{ type: "paragraph", content: [{ type: "text", text: "Web editor" }] }] },
            { type: "tableCell", content: [{ type: "paragraph", content: [{ type: "text", text: "Admitted" }] }] },
          ],
        },
      ],
    },
  ],
};

export const RICH_TEXT_IMAGE_SRC = "https://x.test/chart.png";
export const RICH_TEXT_IMAGE_ALT = "Revenue chart";

/** Same feature family as the editor, with the optional image node admitted. */
export const RICH_TEXT_IMAGE_DOCUMENT: ProseMirrorDocumentJSON = {
  type: "doc",
  content: [
    {
      type: "paragraph",
      content: [{ type: "text", text: "Optional image feature, host-owned source:" }],
    },
    { type: "image", attrs: { src: RICH_TEXT_IMAGE_SRC, alt: RICH_TEXT_IMAGE_ALT } },
  ],
};

export const RICH_TEXT_IMAGE_FEATURES: readonly RichTextFeature[] = [
  ...RICH_TEXT_STANDARD_FEATURES,
  "images",
];
