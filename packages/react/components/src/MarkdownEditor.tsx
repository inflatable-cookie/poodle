import "@inflatable-cookie/poodle-styles/markdown-editor.css";

import { useEffect, useRef, useState, type ChangeEvent } from "react";

import { marked } from "marked";

import { Icon } from "./Icon";
import { IconButton } from "./IconButton";
import { UiPresentationProvider, resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

export interface MarkdownEditorProps {
  value?: string | undefined;
  name?: string | null;
  placeholder?: string;
  disabled?: boolean;
  required?: boolean;
  ariaLabel?: string;
  minHeight?: string;
  mode?: "edit" | "preview" | "split";
  /** Custom markdown-to-HTML renderer. When provided, replaces the built-in
   *  fallback. Use this to plug in a real parser (marked, remark, etc.). */
  renderHtml?: ((markdown: string) => string) | null;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  onValueChange?: ((value: string) => void) | null;
}

const toolbarActions: Array<{ label: string; icon: string; kind: "wrap" | "line"; before: string; after?: string }> = [
  { label: "Bold", icon: "bold", kind: "wrap", before: "**", after: "**" },
  { label: "Italic", icon: "italic", kind: "wrap", before: "*", after: "*" },
  { label: "Heading", icon: "heading", kind: "line", before: "## " },
  { label: "Link", icon: "link", kind: "wrap", before: "[", after: "](url)" },
  { label: "Code", icon: "code", kind: "wrap", before: "`", after: "`" },
  { label: "Quote", icon: "quote", kind: "line", before: "> " },
  { label: "List", icon: "list", kind: "line", before: "- " },
];

export function MarkdownEditor({
  value,
  name = null,
  placeholder = "Write markdown...",
  disabled = false,
  required = false,
  ariaLabel = "Markdown editor",
  minHeight = "12rem",
  mode = "edit",
  renderHtml = null,
  size = null,
  sizeRole = "control",
  density = null,
  onValueChange = null,
}: MarkdownEditorProps) {
  const uiPresentation = useUiPresentation();

  const textareaRef = useRef<HTMLTextAreaElement | null>(null);
  const pendingSelection = useRef<{ start: number; end: number } | null>(null);
  const [uncontrolledValue, setUncontrolledValue] = useState(value ?? "");
  const [currentMode, setCurrentMode] = useState<"edit" | "preview" | "split">(mode);

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const hasControlledValue = value !== undefined;
  const currentValue = hasControlledValue ? (value ?? "") : uncontrolledValue;

  useEffect(() => {
    setCurrentMode(mode);
  }, [mode]);

  // Restore selection after a toolbar edit re-renders the textarea.
  useEffect(() => {
    if (!pendingSelection.current) return;
    const textarea = textareaRef.current;
    if (textarea) {
      textarea.focus();
      textarea.setSelectionRange(pendingSelection.current.start, pendingSelection.current.end);
    }
    pendingSelection.current = null;
  });

  function setValue(nextValue: string): void {
    if (!hasControlledValue) {
      setUncontrolledValue(nextValue);
    }

    onValueChange?.(nextValue);
  }

  function applyNativeTextEdit(start: number, end: number, replacement: string): void {
    const textarea = textareaRef.current;
    if (!textarea) return;

    textarea.focus();
    textarea.setSelectionRange(start, end);

    const usedNativeCommand =
      typeof document !== "undefined" &&
      typeof document.execCommand === "function" &&
      document.execCommand("insertText", false, replacement);

    if (!usedNativeCommand) {
      textarea.setRangeText(replacement, start, end, "end");
    }

    setValue(textarea.value);
  }

  function insertMarkdown(before: string, after = ""): void {
    const textarea = textareaRef.current;
    if (!textarea || disabled) return;

    const start = textarea.selectionStart;
    const end = textarea.selectionEnd;
    const selected = currentValue.slice(start, end);
    const replacement = `${before}${selected || "text"}${after}`;
    applyNativeTextEdit(start, end, replacement);

    const cursorPos = start + before.length;
    pendingSelection.current = { start: cursorPos, end: cursorPos + (selected.length || 4) };
  }

  function insertLine(prefix: string): void {
    const textarea = textareaRef.current;
    if (!textarea || disabled) return;

    const start = textarea.selectionStart;
    const lineStart = currentValue.lastIndexOf("\n", start - 1) + 1;
    const lineEnd = currentValue.indexOf("\n", start);
    const end = lineEnd === -1 ? currentValue.length : lineEnd;
    const line = currentValue.slice(lineStart, end);
    applyNativeTextEdit(lineStart, end, `${prefix}${line}`);

    const nextCursor = start + prefix.length;
    pendingSelection.current = { start: nextCursor, end: nextCursor };
  }

  function handleInput(event: ChangeEvent<HTMLTextAreaElement>): void {
    setValue(event.currentTarget.value);
  }

  const previewHtml = renderHtml ? renderHtml(currentValue) : (marked.parse(currentValue, { async: false }) as string);

  return (
    <UiPresentationProvider sizeScale={resolvedSize} density={resolvedDensity}>
      <div
        className={disabled ? "poodle-md-editor poodle-md-editor--disabled" : "poodle-md-editor"}
        data-size={resolvedSize}
        data-density={resolvedDensity}
      >
        <div className="poodle-md-editor__toolbar">
          <div className="poodle-md-editor__tools">
            {toolbarActions.map((tool) => (
              <button
                key={tool.label}
                type="button"
                className="poodle-md-editor__tool-btn"
                title={tool.label}
                aria-label={tool.label}
                disabled={disabled || currentMode === "preview"}
                onClick={() => (tool.kind === "wrap" ? insertMarkdown(tool.before, tool.after) : insertLine(tool.before))}
              >
                <Icon icon={tool.icon} />
              </button>
            ))}
          </div>

          <div className="poodle-md-editor__modes">
            <IconButton
              icon="pencil"
              ariaLabel="Edit"
              tooltip="Edit"
              variant={currentMode === "edit" ? "secondary" : "ghost"}
              sizeRole="chrome"
              onClick={() => setCurrentMode("edit")}
            />
            <IconButton
              icon="columns-2"
              ariaLabel="Split"
              tooltip="Split"
              variant={currentMode === "split" ? "secondary" : "ghost"}
              sizeRole="chrome"
              onClick={() => setCurrentMode("split")}
            />
            <IconButton
              icon="eye"
              ariaLabel="Preview"
              tooltip="Preview"
              variant={currentMode === "preview" ? "secondary" : "ghost"}
              sizeRole="chrome"
              onClick={() => setCurrentMode("preview")}
            />
          </div>
        </div>

        <div className="poodle-md-editor__body" data-mode={currentMode}>
          {currentMode !== "preview" ? (
            <textarea
              ref={textareaRef}
              className="poodle-md-editor__textarea"
              name={name ?? undefined}
              placeholder={placeholder}
              disabled={disabled}
              required={required}
              aria-label={ariaLabel}
              style={{ minHeight }}
              onChange={handleInput}
              value={currentValue}
            />
          ) : null}

          {currentMode !== "edit" ? (
            currentValue.trim() ? (
              <div
                className="poodle-md-editor__preview"
                aria-label="Preview"
                dangerouslySetInnerHTML={{ __html: previewHtml }}
              />
            ) : (
              <div className="poodle-md-editor__preview" aria-label="Preview">
                <p className="poodle-md-editor__preview-empty">Nothing to preview</p>
              </div>
            )
          ) : null}
        </div>
      </div>
    </UiPresentationProvider>
  );
}
