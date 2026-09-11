import type {
  CodeEditorDiagnostic,
  ProseMirrorDocumentJSON,
  ProseMirrorNodeJSON,
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

/**
 * Deterministic, self-contained raster fixtures (g18.014). Both are 96x48
 * striped PNGs carried inline as `data:image/png` URLs, so the specimens load
 * identically in every runtime with no DNS, no external service, and no
 * mutable remote content. The seeded and host-picked fixtures differ in hue
 * (indigo vs amber) and alt text, so an inserted image is visibly distinct
 * from the seeded one instead of looking like a no-op.
 *
 * Generated as: 96x48, 12px diagonal bands, `[79,70,229]`/`[224,231,255]` for
 * the seeded fixture and `[217,119,6]`/`[254,243,199]` for the picked one.
 */
export const RICH_TEXT_IMAGE_SRC =
  "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAGAAAAAwCAIAAABhdOiYAAAApElEQVR42u3ZIQ5CURAEwT4dlvs7DOEYcIN54iugktUjym26++15vMfrfbxf3YnO3onO3onO3onO3onO3onO3onO3onO3onO3onOXojOAYjOVaA//2ajs3eis3eis3eis3eis3eis3eis3eis3eic6gadK5mH12Mji6mi+liupgupovpYroYHV1MF9PFdDFdTBfTxXQxOrqYLqaL6WK6mC72/TsflWRCO0q1rJcAAAAASUVORK5CYII=";
export const RICH_TEXT_IMAGE_ALT = "Revenue chart";

/**
 * The paired fixture the Image Policy specimen's host-owned `requestImage`
 * stands-in for: a consumer media picker. Distinct source and alt text make a
 * successful insertion visible without opening DevTools.
 */
export const RICH_TEXT_PICKED_IMAGE_SRC =
  "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAGAAAAAwCAIAAABhdOiYAAAApUlEQVR42u3ZoRFCURAEwVGkTIjEQBx4igz2ia+Arjq9ot3V9Lzfjvd+PY73qzvR2TvR2TvR2TvR2TvR2TvR2TvR2TvR2TvR2TvR2QvROQDRuQr0599sdPZOdPZOdPZOdPZOdPZOdPZOdPZOdPZOdA5Vg87V7KOL0dHFdDFdTBfTxXQxXUwXo6OL6WK6mC6mi+liupguRkcX08V0MV1MF9PFvn/nA1LHgB3tlA3jAAAAAElFTkSuQmCC";
export const RICH_TEXT_PICKED_IMAGE_ALT = "Revenue chart (host pick)";

/**
 * Deterministic stand-in for a consumer media picker: the specimen's
 * `requestImage` waits this long (a real picker is never instantaneous), which
 * also gives the retained-selection proof a pending window to observe.
 */
export const RICH_TEXT_IMAGE_REQUEST_DELAY_MS = 300;

/** `type: "image"` nodes anywhere in a ProseMirror document JSON tree. */
export function countRichTextImages(document: ProseMirrorDocumentJSON): number {
  let count = 0;
  for (const child of document.content ?? []) count += countImageNodes(child);
  return count;
}

function countImageNodes(node: ProseMirrorNodeJSON): number {
  let count = node.type === "image" ? 1 : 0;
  for (const child of node.content ?? []) count += countImageNodes(child);
  return count;
}

/** Same feature family as the editor, with the optional image node admitted. */
export const RICH_TEXT_IMAGE_DOCUMENT: ProseMirrorDocumentJSON = {
  type: "doc",
  content: [
    {
      type: "paragraph",
      content: [{ type: "text", text: "Optional image feature, host-owned source:" }],
    },
    { type: "image", attrs: { src: RICH_TEXT_IMAGE_SRC, alt: RICH_TEXT_IMAGE_ALT } },
    {
      type: "paragraph",
      content: [
        { type: "text", text: "Images stay host-owned: the consumer picks the source." },
      ],
    },
  ],
};

export const RICH_TEXT_IMAGE_FEATURES: readonly RichTextFeature[] = [
  ...RICH_TEXT_STANDARD_FEATURES,
  "images",
];
