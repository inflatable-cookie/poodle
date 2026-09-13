import "@inflatable-cookie/poodle-core/styles/markdown-editor.css";

import {
  renderMarkdownHtml,
  type MarkdownHtmlPolicy,
  type MarkdownHtmlRenderer,
} from "./markdown-content";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

export interface MarkdownRendererProps {
  value: string;
  /** Same parser customization seam as the editor preview; output remains governed by `htmlPolicy`. */
  renderHtml?: MarkdownHtmlRenderer;
  /** Safe sanitizes every parser result. Trusted bypasses sanitization and requires fully trusted caller-owned content. */
  htmlPolicy?: MarkdownHtmlPolicy;
  /** When supplied, exposes the renderer as a labelled region; null keeps ordinary document semantics. */
  ariaLabel?: string | null;
  /** Explicit typography-size override for rendered prose. */
  size?: ControlSize | null;
  /** Semantic role used to resolve the inherited presentation size. */
  sizeRole?: SemanticControlSizeRole;
  /** Explicit spacing-density override for rendered prose. */
  density?: ControlDensity | null;
}

/**
 * Standalone, read-only Markdown document rendering over the same private
 * parse/sanitize path as `MarkdownEditor` preview. No toolbar, textarea, form
 * behavior, editing state, or contenteditable surface. Reached only through
 * `./markdown`.
 */
export function MarkdownRenderer({
  value,
  renderHtml = null,
  htmlPolicy = "safe",
  ariaLabel = null,
  size = null,
  sizeRole = "control",
  density = null,
}: MarkdownRendererProps) {
  const uiPresentation = useUiPresentation();
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const html = renderMarkdownHtml(value, renderHtml, htmlPolicy);

  return (
    <div
      className="poodle-md-renderer"
      data-size={resolvedSize}
      data-density={resolvedDensity}
      role={ariaLabel ? "region" : undefined}
      aria-label={ariaLabel ?? undefined}
    >
      <div
        className="poodle-md-renderer__content poodle-md-prose"
        data-poodle-markdown-rendered=""
        dangerouslySetInnerHTML={{ __html: html }}
      />
    </div>
  );
}
