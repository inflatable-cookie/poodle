import {
  forwardRef,
  useEffect,
  useImperativeHandle,
  useRef,
  useState,
  type KeyboardEvent as ReactKeyboardEvent,
} from "react";
import {
  RICH_TEXT_COMMAND_GROUP_LABELS,
  RICH_TEXT_COMMAND_PRESENTATION,
  RICH_TEXT_HEADING_MIXED_VALUE,
  RICH_TEXT_HEADING_NORMAL_LABEL,
  RICH_TEXT_HEADING_NORMAL_VALUE,
  RICH_TEXT_HEADING_SELECT_LABEL,
  RICH_TEXT_STANDARD_FEATURES,
  installInputModality,
  projectRichTextToolbar,
  richTextHeadingCommand,
  richTextHeadingCommandLevel,
  richTextHeadingModeLabel,
  richTextHeadingOptions,
} from "@inflatable-cookie/poodle-core";
import type {
  ProseMirrorDocumentJSON,
  RichTextCommand,
  RichTextCommandGroup,
  RichTextFeature,
  RichTextHeadingMode,
  RichTextToolbarItem,
  RichTextToolbarHeadingSelectItem,
} from "@inflatable-cookie/poodle-core";
import type { ControlDensity, SelectOptionRenderState, SelectTriggerRenderState } from "./types";

import "@inflatable-cookie/poodle-core/styles/rich-text.css";

import { Button } from "./Button";
import { IconButton } from "./IconButton";
import { Select } from "./Select";
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

interface RichTextToolbarCluster {
  group: RichTextCommandGroup;
  items: RichTextToolbarItem[];
}

/** Resolved toolbar items: every admitted heading command is projected as one
 *  selector at the first heading position. Feature-derived order is preserved;
 *  consecutive same-group controls form one intact cluster that wraps as a
 *  unit at constrained widths. */
function toolbarClusters(items: readonly RichTextToolbarItem[]): RichTextToolbarCluster[] {
  const clusters: RichTextToolbarCluster[] = [];
  for (const item of items) {
    const group =
      item.kind === "heading-select"
        ? "headings"
        : RICH_TEXT_COMMAND_PRESENTATION[item.command].group;
    const last = clusters[clusters.length - 1];
    if (last && last.group === group) last.items.push(item);
    else clusters.push({ group, items: [item] });
  }
  return clusters;
}

/** Heading level previewed by an option row, or `normal` for Normal text. */
function headingOptionLevel(value: string): string {
  if (value === RICH_TEXT_HEADING_NORMAL_VALUE) return "normal";
  const level = richTextHeadingCommandLevel(value as RichTextCommand);
  return level === null ? "normal" : String(level);
}

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
    const [headingSelectOpen, setHeadingSelectOpen] = useState(false);
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

    /**
     * The text-mode selector is one control: Normal text maps to the intrinsic
     * off state, every other value to one exact heading level. Selecting the
     * active level again re-runs the same set command, which changes nothing.
     */
    const selectHeadingMode = (value: string): void => {
      if (value === RICH_TEXT_HEADING_NORMAL_VALUE) {
        engineRef.current?.setHeadingMode(null);
        return;
      }
      const level = richTextHeadingCommandLevel(value as RichTextCommand);
      if (level !== null) engineRef.current?.setHeadingMode(level);
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
      // An open heading listbox owns its keyboard events: the toolbar consumes
      // the arrow keys instead of roving behind it, and closing it restores the
      // ordinary roving journey. This also stops a browser's own arrow-key
      // focus navigation from pulling focus out of the open composition.
      if (headingSelectOpen) {
        event.preventDefault();
        return;
      }
      const root = event.currentTarget as HTMLElement;
      // One roving stop per control: the text-mode Select trigger counts once,
      // and its decorative indicator (tabindex="-1") never becomes a stop.
      const buttons = [
        ...root.querySelectorAll<HTMLButtonElement>('button:not(:disabled):not([tabindex="-1"])'),
      ];
      if (buttons.length === 0) return;
      event.preventDefault();
      const index = buttons.indexOf(document.activeElement as HTMLButtonElement);
      const offset = event.key === "ArrowRight" ? 1 : -1;
      const next = buttons[(index + offset + buttons.length) % buttons.length];
      if (next) next.focus();
    };

    const toolbarItems = projectRichTextToolbar(snapshot?.commands ?? []);
    const headingItem = toolbarItems.find(
      (item): item is RichTextToolbarHeadingSelectItem => item.kind === "heading-select",
    );
    const headingMode: RichTextHeadingMode | null = snapshot?.headingMode ?? null;
    const headingValue =
      headingMode === null
        ? ""
        : headingMode.kind === "mixed"
          ? RICH_TEXT_HEADING_MIXED_VALUE
          : headingMode.kind === "normal"
            ? RICH_TEXT_HEADING_NORMAL_VALUE
            : richTextHeadingCommand(headingMode.level) ?? RICH_TEXT_HEADING_NORMAL_VALUE;
    const headingLabel = headingMode
      ? richTextHeadingModeLabel(headingMode)
      : RICH_TEXT_HEADING_NORMAL_LABEL;
    const headingOptions = richTextHeadingOptions(headingItem?.commands ?? []);

    const renderHeadingTrigger = (_state: SelectTriggerRenderState) => (
      <span
        className="poodle-rich-text-editor__heading-value"
        data-heading-mode={headingMode?.kind ?? "normal"}
      >
        {headingLabel}
      </span>
    );

    const renderHeadingOption = (state: SelectOptionRenderState) => (
      <span
        className="poodle-rich-text-editor__heading-option"
        data-heading-level={headingOptionLevel(state.option.value)}
      >
        {state.option.value !== RICH_TEXT_HEADING_NORMAL_VALUE ? (
          <span className="poodle-rich-text-editor__glyph" aria-hidden="true">
            {RICH_TEXT_COMMAND_PRESENTATION[state.option.value as RichTextCommand].glyph}
          </span>
        ) : null}
        <span className="poodle-rich-text-editor__heading-option-label">{state.option.label}</span>
      </span>
    );

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
            {toolbarClusters(toolbarItems).map((cluster) => (
              <div
                key={cluster.items[0].kind === "command" ? cluster.items[0].command : "heading-select"}
                className="poodle-rich-text-editor__group"
                role="group"
                aria-label={RICH_TEXT_COMMAND_GROUP_LABELS[cluster.group]}
              >
                {cluster.items.map((item) => {
                  if (item.kind === "heading-select") {
                    return (
                      <span
                        key="heading-select"
                        className="poodle-rich-text-editor__command poodle-rich-text-editor__heading-select"
                        data-command="heading-select"
                        data-heading-commands={item.commands.join(" ")}
                      >
                        <Select
                          value={headingValue}
                          options={headingOptions}
                          sizeRole="chrome"
                          density={resolvedDensity}
                          variant="ghost"
                          disabled={disabled || readOnly}
                          menuMinWidth="12rem"
                          ariaLabel={`${RICH_TEXT_HEADING_SELECT_LABEL}, ${headingLabel}`}
                          onOpenChange={setHeadingSelectOpen}
                          onValueChange={selectHeadingMode}
                          trigger={renderHeadingTrigger}
                          option={renderHeadingOption}
                        />
                      </span>
                    );
                  }
                  const command = item.command;
                  const presentation = RICH_TEXT_COMMAND_PRESENTATION[command];
                  const state = snapshot.states[command];
                  return (
                    <span key={command} className="poodle-rich-text-editor__command" data-command={command}>
                      <IconButton
                        variant="ghost"
                        tone={presentation.destructive ? "danger" : "default"}
                        sizeRole="chrome"
                        density={resolvedDensity}
                        icon={presentation.icon}
                        ariaLabel={presentation.label}
                        tooltip={presentation.label}
                        pressed={presentation.toggle ? state.active : null}
                        disabled={!state.available || disabled}
                        onClick={() => runToolbarCommand(command)}
                      />
                    </span>
                  );
                })}
              </div>
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
            <Button variant="secondary" sizeRole="chrome" density={resolvedDensity} onClick={submitLink}>
              Apply
            </Button>
            {snapshot?.states.link.active ? (
              <Button
                variant="ghost"
                tone="danger"
                sizeRole="chrome"
                density={resolvedDensity}
                onClick={removeLink}
              >
                Remove link
              </Button>
            ) : null}
          </div>
        ) : null}
        <div ref={hostRef} id={surfaceId} className="poodle-rich-text-editor__viewport" />
      </div>
    );
  },
);
