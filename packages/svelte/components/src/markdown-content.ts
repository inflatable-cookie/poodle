import { sanitizeMarkdownHtml, type MarkdownHtmlPolicy } from "@inflatable-cookie/poodle-core";
import { marked } from "marked";

/**
 * Private Markdown content path shared by `MarkdownEditor` preview and the
 * public `MarkdownRenderer`. `marked` and this module stay inside the
 * `./markdown` package graph.
 *
 * Policy is applied to the *complete* parser result, after either the built-in
 * `marked` parse or a caller-supplied `renderHtml`. Supplying `renderHtml`
 * customizes parsing; it never implies trust.
 */
export type MarkdownHtmlRenderer = ((markdown: string) => string) | null;

export function renderMarkdownHtml(
  value: string,
  renderHtml: MarkdownHtmlRenderer,
  htmlPolicy: MarkdownHtmlPolicy,
): string {
  const parsed = renderHtml
    ? renderHtml(value)
    : (marked.parse(value, { async: false }) as string);

  // `trusted` is an explicit caller-owned bypass; anything else is sanitized.
  return htmlPolicy === "trusted" ? parsed : sanitizeMarkdownHtml(parsed);
}

export type { MarkdownHtmlPolicy };
