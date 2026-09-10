import {
  forwardRef,
  useEffect,
  useImperativeHandle,
  useRef,
  useState,
  type KeyboardEvent as ReactKeyboardEvent,
} from "react";
import {
  RICH_TEXT_COMMAND_LABELS,
  RICH_TEXT_STANDARD_FEATURES,
  RICH_TEXT_TOGGLE_COMMANDS,
  installInputModality,
} from "@inflatable-cookie/poodle-core";
import type {
  ProseMirrorDocumentJSON,
  RichTextCommand,
  RichTextFeature,
} from "@inflatable-cookie/poodle-core";
import type { ControlDensity } from "./types";

import "@inflatable-cookie/poodle-core/styles/rich-text.css";

import {
  assertAdmittedFeatures,
  assertAdmittedToolbar,
  assertValidRichTextDocument,
  createRichTextEngine,
  createRichTextSchema,
} from "./rich-text-engine";
import type {
  RichTextEngine,
  RichTextImageInput,
  RichTextToolbarSnapshot,
} from "./rich-text-engine";
import { useUiPresentation } from "./presentation";

export interface RichTextEditorProps {
  value: ProseMirrorDocumentJSON;
  features?: readonly RichTextFeature[];
  toolbar?: "auto" | readonly RichTextCommand[];
  readOnly?: boolean;
  disabled?: boolean;
  placeholder?: string;
  ariaLabel?: string;
  requestImage?: (() => Promise<RichTextImageInput | null>) | null;
  density?: ControlDensity | null;
  onChange?: ((document: ProseMirrorDocumentJSON) => void) | null;
}

export interface RichTextEditorHandle {
  focus: () => void;
}

const isToggle = (command: RichTextCommand): boolean =>
  (RICH_TEXT_TOGGLE_COMMANDS as readonly string[]).includes(command);

/**
 * Web-admitted controlled rich-text editing surface over TipTap 3 and
 * ProseMirror. ProseMirror document JSON and schema semantics are the
 * authority; the engine stays private and the component is reached only
 * through `./rich-text`.
 */
export const RichTextEditor = forwardRef<RichTextEditorHandle, RichTextEditorProps>(
  function RichTextEditor(
    {
      value,
      features = RICH_TEXT_STANDARD_FEATURES,
      toolbar = "auto",
      readOnly = false,
      disabled = false,
      placeholder = "",
      ariaLabel = "Rich text editor",
      requestImage = null,
      density = null,
      onChange = null,
    }: RichTextEditorProps,
    ref,
  ) {
    const uiPresentation = useUiPresentation();
    const resolvedDensity = density ?? uiPresentation.density;
    const hostRef = useRef<HTMLDivElement | null>(null);
    const engineRef = useRef<RichTextEngine | null>(null);
    // Pre-mount validation runs exactly once, before the engine exists: an
    // invalid document, feature, or toolbar refuses with zero callbacks and
    // zero partial output. Later controlled updates are refused by the
    // engine, which keeps the prior editor intact and reports a development
    // error instead of breaking the host tree.
    const mountValidatedRef = useRef(false);
    if (!mountValidatedRef.current) {
      assertAdmittedFeatures(features);
      assertAdmittedToolbar(toolbar, features, requestImage);
      assertValidRichTextDocument(createRichTextSchema(features), value);
      mountValidatedRef.current = true;
    }
    const onChangeRef = useRef(onChange);
    onChangeRef.current = onChange;
    // The last host value object pushed to the engine: only a genuinely new
    // value object is a controlled push; re-renders with the previous value
    // never count as a host revert.
    const sentValueRef = useRef(value);
    const [snapshot, setSnapshot] = useState<RichTextToolbarSnapshot | null>(null);
    const [linkEditorOpen, setLinkEditorOpen] = useState(false);
    const [linkValue, setLinkValue] = useState("");
    const [surfaceId] = useState(
      () => `poodle-rich-text-editor-${Math.random().toString(36).slice(2)}`,
    );
    // Live prop snapshot for the sync effect below.
    const latestRef = useRef({
      value,
      features,
      toolbar,
      readOnly,
      disabled,
      placeholder,
      ariaLabel,
      requestImage,
    });
    latestRef.current = {
      value,
      features,
      toolbar,
      readOnly,
      disabled,
      placeholder,
      ariaLabel,
      requestImage,
    };

    useImperativeHandle(ref, () => ({
      focus: () => {
        (hostRef.current?.querySelector(".ProseMirror") as HTMLElement | null)?.focus();
      },
    }));

    useEffect(() => {
      installInputModality();
      const host = hostRef.current;
      if (!host) return;
      engineRef.current = createRichTextEngine(host, latestRef.current, {
        onChange: (document) => onChangeRef.current?.(document),
        onToolbar: setSnapshot,
      });
      return () => {
        engineRef.current?.destroy();
        engineRef.current = null;
      };
    }, []);

    useEffect(() => {
      const engine = engineRef.current;
      if (!engine) return;
      const next = latestRef.current;
      // Push the value only when the host actually sent a new value object.
      // Snapshot-driven re-renders carry the previous host value, which must
      // never be treated as a host revert.
      if (next.value !== sentValueRef.current) {
        engine.update(next);
        sentValueRef.current = next.value;
      } else {
        const { value: _sentValue, ...rest } = next;
        engine.update(rest);
      }
    });

    const runToolbarCommand = (command: RichTextCommand): void => {
      if (command === "link") {
        setLinkValue(engineRef.current?.linkHref() ?? "");
        setLinkEditorOpen(true);
        return;
      }
      engineRef.current?.runCommand(command);
    };

    const submitLink = (): void => {
      const engine = engineRef.current;
      if (!engine) return;
      engine.applyLink(linkValue);
      setLinkEditorOpen(false);
      focusCurrent();
    };

    const removeLink = (): void => {
      engineRef.current?.removeLink();
      setLinkEditorOpen(false);
      focusCurrent();
    };

    const closeLinkEditor = (): void => {
      setLinkEditorOpen(false);
      focusCurrent();
    };

    const focusCurrent = (): void => {
      (hostRef.current?.querySelector(".ProseMirror") as HTMLElement | null)?.focus();
    };

    const handleLinkInputKeydown = (event: ReactKeyboardEvent): void => {
      if (event.key === "Enter") {
        event.preventDefault();
        submitLink();
        return;
      }
      if (event.key === "Escape") {
        event.stopPropagation();
        closeLinkEditor();
      }
    };

    const handleToolbarKeydown = (event: ReactKeyboardEvent): void => {
      // Rove between visible toolbar controls with the arrow keys.
      if (event.key !== "ArrowRight" && event.key !== "ArrowLeft") return;
      const root = event.currentTarget as HTMLElement;
      const buttons = [...root.querySelectorAll<HTMLButtonElement>("button:not(:disabled)")];
      if (buttons.length === 0) return;
      event.preventDefault();
      const index = buttons.indexOf(document.activeElement as HTMLButtonElement);
      const offset = event.key === "ArrowRight" ? 1 : -1;
      const next = buttons[(index + offset + buttons.length) % buttons.length];
      if (next) next.focus();
    };

    return (
      <div
        className="poodle-rich-text-editor"
        data-density={resolvedDensity}
        data-disabled={disabled || undefined}
        data-readonly={readOnly || undefined}
        inert={disabled || undefined}
      >
        {snapshot && snapshot.commands.length > 0 ? (
          <div
            className="poodle-rich-text-editor__toolbar"
            role="toolbar"
            aria-label={`${ariaLabel} toolbar`}
            aria-controls={surfaceId}
            onKeyDown={handleToolbarKeydown}
          >
            {snapshot.commands.map((command) => (
              <button
                key={command}
                type="button"
                className="poodle-rich-text-editor__toolbar-button"
                data-command={command}
                data-pressed={snapshot.states[command].active || undefined}
                aria-pressed={
                  isToggle(command)
                    ? snapshot.states[command].active
                      ? "true"
                      : "false"
                    : undefined
                }
                disabled={!snapshot.states[command].available || disabled}
                onClick={() => runToolbarCommand(command)}
              >
                {RICH_TEXT_COMMAND_LABELS[command]}
              </button>
            ))}
          </div>
        ) : null}
        {linkEditorOpen ? (
          <div className="poodle-rich-text-editor__link-editor" role="group" aria-label="Edit link">
            <input
              className="poodle-rich-text-editor__link-input"
              type="url"
              aria-label="Link URL"
              value={linkValue}
              onChange={(event) => setLinkValue(event.target.value)}
              onKeyDown={handleLinkInputKeydown}
            />
            <button
              type="button"
              className="poodle-rich-text-editor__toolbar-button"
              data-command="apply-link"
              onClick={submitLink}
            >
              Apply
            </button>
            {snapshot?.states.link.active ? (
              <button
                type="button"
                className="poodle-rich-text-editor__toolbar-button"
                data-command="remove-link"
                onClick={removeLink}
              >
                Remove link
              </button>
            ) : null}
          </div>
        ) : null}
        <div ref={hostRef} id={surfaceId} className="poodle-rich-text-editor__viewport" />
      </div>
    );
  },
);
