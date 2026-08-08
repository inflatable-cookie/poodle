import { useEffect, useRef, useState } from "react";

import "@inflatable-cookie/poodle-core/styles/code.css";

import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

export interface CodeProps {
  source?: string;
  language?: string | null;
  showLineNumbers?: boolean;
  highlightLines?: number[];
  showCopyButton?: boolean;
  maxHeight?: string | null;
  inline?: boolean;
  ariaLabel?: string | null;
  inlineVariant?: "default" | "plain";
  typography?: "body" | "inline";
  sizeRole?: SemanticControlSizeRole;
  size?: ControlSize | null;
  density?: ControlDensity | null;
}

function CopyGlyph({ copied }: { copied: boolean }) {
  return copied ? (
    <svg viewBox="0 0 16 16" fill="none" aria-hidden="true">
      <path d="M3 8.5l3 3 7-7" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round" />
    </svg>
  ) : (
    <svg viewBox="0 0 16 16" fill="none" aria-hidden="true">
      <rect x="5" y="5" width="8" height="8" rx="1" stroke="currentColor" strokeWidth="1.25" />
      <path d="M3 11V3a1 1 0 011-1h8" stroke="currentColor" strokeWidth="1.25" strokeLinecap="round" />
    </svg>
  );
}

export function Code({
  source = "",
  language = null,
  showLineNumbers = false,
  highlightLines = [],
  showCopyButton = true,
  maxHeight = null,
  inline = false,
  ariaLabel = null,
  inlineVariant = "default",
  typography = "body",
  sizeRole = "chrome",
  size = null,
  density = null,
}: CodeProps) {
  const uiPresentation = useUiPresentation();
  const [copied, setCopied] = useState(false);
  const resetTimer = useRef<ReturnType<typeof setTimeout> | null>(null);

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const lines = source.split("\n");
  const highlightSet = new Set(highlightLines);

  useEffect(
    () => () => {
      if (resetTimer.current !== null) clearTimeout(resetTimer.current);
    },
    [],
  );

  async function copyToClipboard(): Promise<void> {
    try {
      await navigator.clipboard.writeText(source);
      if (resetTimer.current !== null) clearTimeout(resetTimer.current);
      setCopied(true);
      resetTimer.current = setTimeout(() => {
        setCopied(false);
        resetTimer.current = null;
      }, 2000);
    } catch {
      // Fallback for browsers without clipboard API
    }
  }

  if (inline) {
    return (
      <span
        className="poodle-code poodle-code--inline-wrap"
        data-size={resolvedSize}
        data-density={resolvedDensity}
        data-inline-variant={inlineVariant}
        data-typography={typography}
      >
        <code
          className="poodle-code poodle-code--inline"
          aria-label={ariaLabel ?? undefined}
          data-language={language}
          data-size={resolvedSize}
          data-density={resolvedDensity}
          data-inline-variant={inlineVariant}
          data-typography={typography}
        >
          {source}
        </code>
        {showCopyButton ? (
          <button
            type="button"
            className="poodle-code__copy poodle-code__copy--inline"
            aria-label={copied ? "Copied" : "Copy to clipboard"}
            onClick={copyToClipboard}
          >
            <CopyGlyph copied={copied} />
          </button>
        ) : null}
      </span>
    );
  }

  return (
    <div
      className="poodle-code poodle-code--block"
      aria-label={ariaLabel ?? `Code block${language ? ` (${language})` : ""}`}
      data-language={language}
      data-size={resolvedSize}
      data-density={resolvedDensity}
      style={maxHeight ? { maxHeight } : undefined}
    >
      {language || showCopyButton ? (
        <div className="poodle-code__toolbar">
          {language ? <span className="poodle-code__language">{language}</span> : null}
          <div className="poodle-code__toolbar-actions">
            {showCopyButton ? (
              <button
                type="button"
                className="poodle-code__copy"
                aria-label={copied ? "Copied" : "Copy to clipboard"}
                onClick={copyToClipboard}
              >
                <CopyGlyph copied={copied} />
              </button>
            ) : null}
          </div>
        </div>
      ) : null}

      <div className="poodle-code__scroll">
        <pre className="poodle-code__pre">
          <code className="poodle-code__source">
            {lines.map((line, i) => (
              <span
                key={i}
                className={`poodle-code__line${highlightSet.has(i + 1) ? " poodle-code__line--highlighted" : ""}`}
              >
                {showLineNumbers ? (
                  <span className="poodle-code__line-number" aria-hidden="true">
                    {i + 1}
                  </span>
                ) : null}
                <span className="poodle-code__line-content">{line}</span>
              </span>
            ))}
          </code>
        </pre>
      </div>
    </div>
  );
}
