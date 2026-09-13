<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/markdown-editor.css";

  import {
    renderMarkdownHtml,
    type MarkdownHtmlPolicy,
    type MarkdownHtmlRenderer,
  } from "./markdown-content";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  /**
   * Standalone, read-only Markdown document rendering over the same private
   * parse/sanitize path as `MarkdownEditor` preview. No toolbar, textarea,
   * form behavior, editing state, or contenteditable surface. Reached only
   * through `./markdown`.
   */
  interface Props {
    value: string;
    renderHtml?: MarkdownHtmlRenderer;
    htmlPolicy?: MarkdownHtmlPolicy;
    ariaLabel?: string | null;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
  }

  let {
    value,
    renderHtml = null,
    htmlPolicy = "safe",
    ariaLabel = null,
    size = null,
    sizeRole = "control",
    density = null,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const html = $derived(renderMarkdownHtml(value, renderHtml, htmlPolicy));
</script>

<div
  class="poodle-md-renderer"
  data-size={resolvedSize}
  data-density={resolvedDensity}
  role={ariaLabel ? "region" : undefined}
  aria-label={ariaLabel ?? undefined}
>
  <div class="poodle-md-renderer__content poodle-md-prose" data-poodle-markdown-rendered="">
    {@html html}
  </div>
</div>
