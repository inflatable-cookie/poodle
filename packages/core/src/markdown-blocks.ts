/**
 * The markdown block model shared by every Poodle target.
 * Contract: docs/contracts/components/agent-message.md, "Markdown Subset".
 *
 * Agent output is markdown, and Poodle renders it on four targets that cannot
 * share a parser: the web has `marked`, the natives have `pulldown-cmark`. Left
 * alone, the two would disagree — on tight vs loose lists, on what counts as a
 * paragraph break, on nested emphasis — and the disagreement would surface as
 * "the desktop build renders this answer differently", months later, with no
 * obvious cause.
 *
 * So neither parser's output is the model. Both normalise into the block model
 * below, and `test/fixtures/markdown-blocks.json` pins what each fixture must
 * produce, in both languages. The contract states the supported subset; anything
 * outside it degrades to text rather than being silently dropped.
 *
 * This module stays dependency-free — `@inflatable-cookie/poodle-core` has no runtime deps and
 * should keep none. `MarkedToken` is structural, so the component packages own
 * the `marked` dependency and pass `marked.lexer(src)` output in.
 */

export type MdInline =
  | { type: "text"; value: string }
  | { type: "code"; value: string }
  | { type: "strong"; children: MdInline[] }
  | { type: "em"; children: MdInline[] }
  | { type: "del"; children: MdInline[] }
  | { type: "link"; href: string; children: MdInline[] }
  | { type: "break" };

export type MdBlock =
  | { type: "paragraph"; children: MdInline[] }
  | { type: "heading"; level: 1 | 2 | 3 | 4 | 5 | 6; children: MdInline[] }
  | { type: "code"; lang: string | null; value: string }
  | { type: "list"; ordered: boolean; start: number; items: MdBlock[][] }
  | { type: "blockquote"; children: MdBlock[] }
  | { type: "rule" };

/**
 * The shape of a `marked` token, structurally.
 *
 * Declared rather than imported so this module keeps no dependency. It is
 * deliberately loose: `marked` adds token fields between versions, and a parser
 * upgrade should not be a type error here.
 */
export interface MarkedToken {
  type: string;
  raw?: string;
  text?: string;
  depth?: number;
  lang?: string;
  href?: string;
  ordered?: boolean;
  start?: number | "";
  loose?: boolean;
  items?: MarkedToken[];
  tokens?: MarkedToken[];
}

const clampHeadingLevel = (depth: number | undefined): 1 | 2 | 3 | 4 | 5 | 6 => {
  const level = Math.min(6, Math.max(1, Math.round(depth ?? 1)));
  return level as 1 | 2 | 3 | 4 | 5 | 6;
};

/**
 * Normalise `marked` inline tokens into the shared inline model.
 *
 * Unknown token types fall through to their text rather than vanishing. An
 * agent explaining HTML should not have the explanation disappear because the
 * parser classified part of it as raw HTML, and dropping content silently is
 * the worst available failure for a transcript.
 */
export function inlineFromMarked(tokens: readonly MarkedToken[] | undefined): MdInline[] {
  if (!tokens?.length) return [];

  const out: MdInline[] = [];

  for (const token of tokens) {
    switch (token.type) {
      case "text":
        // A `text` token can itself carry children when it contains inline
        // markup; the flat `text` field would lose that markup.
        if (token.tokens?.length) {
          out.push(...inlineFromMarked(token.tokens));
        } else {
          out.push({ type: "text", value: token.text ?? "" });
        }
        break;
      case "codespan":
        out.push({ type: "code", value: token.text ?? "" });
        break;
      case "strong":
        out.push({ type: "strong", children: inlineFromMarked(token.tokens) });
        break;
      case "em":
        out.push({ type: "em", children: inlineFromMarked(token.tokens) });
        break;
      case "del":
        out.push({ type: "del", children: inlineFromMarked(token.tokens) });
        break;
      case "link":
        out.push({
          type: "link",
          href: token.href ?? "",
          children: inlineFromMarked(token.tokens),
        });
        break;
      case "br":
        out.push({ type: "break" });
        break;
      case "escape":
        out.push({ type: "text", value: token.text ?? "" });
        break;
      default:
        if (token.tokens?.length) {
          out.push(...inlineFromMarked(token.tokens));
        } else if (token.text) {
          out.push({ type: "text", value: token.text });
        }
        break;
    }
  }

  return out;
}

/** Normalise `marked.lexer()` output into the shared block model. */
export function blocksFromMarked(tokens: readonly MarkedToken[] | undefined): MdBlock[] {
  if (!tokens?.length) return [];

  const out: MdBlock[] = [];

  for (const token of tokens) {
    switch (token.type) {
      case "space":
        break;
      case "paragraph":
        out.push({ type: "paragraph", children: inlineFromMarked(token.tokens) });
        break;
      case "heading":
        out.push({
          type: "heading",
          level: clampHeadingLevel(token.depth),
          children: inlineFromMarked(token.tokens),
        });
        break;
      case "code":
        out.push({
          type: "code",
          // An unannotated fence is `null`, not `""`, so the renderer can tell
          // "no language given" from "a language that happens to be empty" and
          // the Rust mirror has an `Option<String>` to match.
          lang: token.lang ? token.lang.trim().split(/\s+/)[0] : null,
          value: token.text ?? "",
        });
        break;
      case "list":
        out.push({
          type: "list",
          ordered: Boolean(token.ordered),
          start: typeof token.start === "number" ? token.start : 1,
          items: (token.items ?? []).map((item) => blocksFromMarked(item.tokens)),
        });
        break;
      case "blockquote":
        out.push({ type: "blockquote", children: blocksFromMarked(token.tokens) });
        break;
      case "hr":
        out.push({ type: "rule" });
        break;
      case "text":
        // Loose list items and stray text arrive as bare `text` at block level.
        // Promoting them to paragraphs keeps the model free of a second way to
        // say "prose", which the natives would otherwise each handle differently.
        out.push({
          type: "paragraph",
          children: token.tokens?.length
            ? inlineFromMarked(token.tokens)
            : [{ type: "text", value: token.text ?? "" }],
        });
        break;
      default:
        if (token.text) {
          out.push({ type: "paragraph", children: [{ type: "text", value: token.text }] });
        }
        break;
    }
  }

  return out;
}

/**
 * Plain text of a block tree — for `aria-label`, copy, and truncation measurement.
 *
 * Code blocks contribute their source. A screen reader announcing a message
 * needs to know code was there; skipping it would make an answer that is mostly
 * a snippet read as almost empty.
 */
export function markdownPlainText(blocks: readonly MdBlock[]): string {
  const inlineText = (nodes: readonly MdInline[]): string =>
    nodes
      .map((node) => {
        switch (node.type) {
          case "text":
          case "code":
            return node.value;
          case "break":
            return " ";
          default:
            return inlineText(node.children);
        }
      })
      .join("");

  const blockText = (list: readonly MdBlock[]): string[] =>
    list.flatMap((block) => {
      switch (block.type) {
        case "paragraph":
        case "heading":
          return [inlineText(block.children)];
        case "code":
          return [block.value];
        case "blockquote":
          return blockText(block.children);
        case "list":
          return block.items.flatMap((item) => blockText(item));
        case "rule":
          return [];
      }
    });

  return blockText(blocks).filter(Boolean).join("\n");
}
