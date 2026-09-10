import { useEffect, useRef } from "react";
import { RICH_TEXT_STANDARD_FEATURES } from "@inflatable-cookie/poodle-core";
import type {
  ProseMirrorDocumentJSON,
  RichTextFeature,
} from "@inflatable-cookie/poodle-core";

import "@inflatable-cookie/poodle-core/styles/rich-text.css";

import {
  assertAdmittedFeatures,
  assertValidRichTextDocument,
  createRichTextSchema,
  renderRichTextDocument,
} from "./rich-text-engine";

export interface RichTextRendererProps {
  value: ProseMirrorDocumentJSON;
  features?: readonly RichTextFeature[];
  ariaLabel?: string | null;
}

/**
 * Read-only rich-text rendering over the same ProseMirror document, feature
 * configuration, schema, and tokens as the editor. No contenteditable node,
 * history, selection state, or editor instance. Reached only through
 * `./rich-text`.
 */
export function RichTextRenderer({
  value,
  features = RICH_TEXT_STANDARD_FEATURES,
  ariaLabel = null,
}: RichTextRendererProps) {
  // Fail closed before any output, including server render. Validation is
  // pure: it never creates an editor and never touches browser globals.
  assertAdmittedFeatures(features);
  assertValidRichTextDocument(createRichTextSchema(features), value);

  const contentRef = useRef<HTMLDivElement | null>(null);
  const latestRef = useRef({ value, features });
  latestRef.current = { value, features };

  useEffect(() => {
    // Document rendering happens after client mount only.
    const target = contentRef.current;
    if (!target) return;
    renderRichTextDocument(target, latestRef.current.value, latestRef.current.features);
  });

  return (
    <div
      className="poodle-rich-text-renderer"
      role={ariaLabel ? "region" : undefined}
      aria-label={ariaLabel ?? undefined}
    >
      <div
        ref={contentRef}
        className="poodle-rich-text-renderer__content"
        data-poodle-rich-text-rendered=""
      />
    </div>
  );
}
