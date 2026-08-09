import {
  useLayoutEffect,
  useRef,
  useState,
  type ChangeEvent,
  type KeyboardEvent,
  type ReactNode,
} from "react";

import "@inflatable-cookie/poodle-core/styles/agent-chat-input.css";

import { Icon } from "./Icon";
import { IconButton } from "./IconButton";
import { Meter } from "./Meter";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import {
  actionIcon,
  actionState,
  canSubmit as canSubmitGate,
  contextPercentage,
  resolveSubmitIntent,
} from "./agent-chat-input-model";
import type {
  AgentChatAttachment,
  AgentChatStatus,
  ControlDensity,
  ControlSize,
  SemanticControlSizeRole,
} from "./types";

export interface AgentChatInputProps {
  value?: string;
  placeholder?: string;
  /** Editor placeholder while a question is live; the editor is the override. */
  questionPlaceholder?: string;
  /** The question region. Host composes an `AgentQuestion` into it. */
  question?: ReactNode;
  /** Whether the live question could be answered from its own state alone. */
  questionCanSubmit?: boolean;
  /** Editor placeholder while a plan awaits a decision; the editor is the revise channel. */
  planPlaceholder?: string;
  /** The plan region. Host composes its plan review surface into it. */
  plan?: ReactNode;
  status?: AgentChatStatus;
  disabled?: boolean;
  readOnly?: boolean;
  ariaLabel?: string;
  submitLabel?: string;
  stopLabel?: string;
  submitOnEnter?: boolean;
  minRows?: number;
  maxRows?: number;
  maxLength?: number | null;
  allowEmptySubmit?: boolean;
  attachments?: AgentChatAttachment[];
  contextUsed?: number | null;
  contextLimit?: number | null;
  contextWarnAt?: number;
  contextLabel?: string;
  toolbarDividers?: boolean;
  sizeRole?: SemanticControlSizeRole;
  size?: ControlSize | null;
  density?: ControlDensity | null;
  onSubmit?: ((value: string) => void) | null;
  onStop?: (() => void) | null;
  onValueChange?: ((value: string) => void) | null;
  onRemoveAttachment?: ((id: string) => void) | null;
  toolbar?: ReactNode;
  footer?: ReactNode;
}

export function AgentChatInput({
  value,
  placeholder = "Send a message",
  questionPlaceholder = "Type your own answer, or leave this blank to use the selected option",
  question,
  questionCanSubmit = false,
  planPlaceholder = "Describe what to change, or decide the plan above",
  plan,
  status = "idle",
  disabled = false,
  readOnly = false,
  ariaLabel = "Message",
  submitLabel = "Send",
  stopLabel = "Stop",
  submitOnEnter = true,
  minRows = 2,
  maxRows = 12,
  maxLength = null,
  allowEmptySubmit = false,
  attachments = [],
  contextUsed = null,
  contextLimit = null,
  contextWarnAt = 0.8,
  contextLabel = "Context used",
  toolbarDividers = true,
  sizeRole = "control",
  size = null,
  density = null,
  onSubmit = null,
  onStop = null,
  onValueChange = null,
  onRemoveAttachment = null,
  toolbar,
  footer,
}: AgentChatInputProps) {
  const uiPresentation = useUiPresentation();
  const editorRef = useRef<HTMLTextAreaElement | null>(null);
  const composingRef = useRef(false);
  const [uncontrolledValue, setUncontrolledValue] = useState("");

  const hasValueProp = value !== undefined;
  const effectiveValue = hasValueProp ? value ?? "" : uncontrolledValue;

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const isBusy = status === "busy";
  const isQuestioning = status === "questioning";
  const isReviewingPlan = status === "reviewing-plan";
  const actionEnabled = isQuestioning
    ? !disabled && (effectiveValue.trim().length > 0 || questionCanSubmit)
    : canSubmitGate({
        disabled,
        isBusy,
        value: effectiveValue,
        allowEmptySubmit,
      });
  const resolvedPlaceholder = isQuestioning
    ? questionPlaceholder
    : isReviewingPlan
      ? planPlaceholder
      : placeholder;
  const contextPercent = contextPercentage(contextUsed, contextLimit);
  const showContext = contextLimit !== null && contextLimit > 0;
  const contextHigh = contextLimit === null ? null : contextLimit * contextWarnAt;
  const contextAriaLabel =
    contextPercent === null ? contextLabel : `${contextLabel}, ${Math.round(contextPercent)}%`;

  // Auto-grow: reset to the row floor, then take the content height up to the
  // `maxRows` ceiling.
  useLayoutEffect(() => {
    const editor = editorRef.current;
    if (!editor) return;

    const autosize = () => {
      editor.style.height = "auto";
      const lineHeight = Number.parseFloat(getComputedStyle(editor).lineHeight);
      const ceiling = Number.isFinite(lineHeight) ? lineHeight * maxRows : Number.POSITIVE_INFINITY;
      editor.style.height = `${Math.min(editor.scrollHeight, ceiling)}px`;
    };

    autosize();

    // The first pass can measure before the web font is applied, which reports
    // fallback-font metrics and leaves the editor short of `minRows` — and
    // nothing re-measures it, because the value never changed. Re-run once the
    // fonts settle.
    if (typeof document === "undefined" || !document.fonts) return;
    let cancelled = false;
    void document.fonts.ready.then(() => {
      if (!cancelled) autosize();
    });
    return () => {
      cancelled = true;
    };
  }, [effectiveValue, maxRows]);

  function setValue(next: string): void {
    if (!hasValueProp) setUncontrolledValue(next);
    onValueChange?.(next);
  }

  function submit(): void {
    // Busy never submits — stopping is deliberate, not an accidental Enter.
    if (isBusy || !actionEnabled) return;
    onSubmit?.(effectiveValue);
  }

  function handleAction(): void {
    if (!actionEnabled) return;
    if (isBusy) {
      onStop?.();
      return;
    }
    submit();
  }

  function handleChange(event: ChangeEvent<HTMLTextAreaElement>): void {
    setValue(event.currentTarget.value);
  }

  function handleKeyDown(event: KeyboardEvent<HTMLTextAreaElement>): void {
    const intent = resolveSubmitIntent(
      {
        key: event.key,
        shiftKey: event.shiftKey,
        metaKey: event.metaKey,
        ctrlKey: event.ctrlKey,
        isComposing: composingRef.current || event.nativeEvent.isComposing,
      },
      { submitOnEnter, isBusy },
    );
    if (intent === "submit") {
      event.preventDefault();
      submit();
      return;
    }
    if (intent === "stop") {
      event.preventDefault();
      onStop?.();
    }
  }

  function removeAttachment(id: string): void {
    if (disabled) return;
    onRemoveAttachment?.(id);
  }

  return (
    <div
      className="poodle-agent-chat-input"
      data-size={resolvedSize}
      data-density={resolvedDensity}
      data-status={status}
      data-disabled={disabled}
    >
      <div className="poodle-agent-chat-input__field">
        {isQuestioning && question ? (
          <div className="poodle-agent-chat-input__question">{question}</div>
        ) : null}
        {isReviewingPlan && plan ? (
          <div className="poodle-agent-chat-input__plan">{plan}</div>
        ) : null}

        {attachments.length > 0 ? (
          <ul className="poodle-agent-chat-input__attachments" aria-label="Attachments">
            {attachments.map((attachment) => (
              <li
                key={attachment.id}
                className="poodle-agent-chat-input__attachment"
                data-kind={attachment.kind}
                data-variant={attachment.thumbnailUrl ? "thumbnail" : "chip"}
                title={attachment.thumbnailUrl ? attachment.label : undefined}
              >
                {attachment.thumbnailUrl ? (
                  <img
                    className="poodle-agent-chat-input__attachment-thumb"
                    src={attachment.thumbnailUrl}
                    alt={attachment.label}
                  />
                ) : (
                  <>
                    {attachment.icon ? (
                      <span className="poodle-agent-chat-input__attachment-icon">
                        <Icon name={attachment.icon} size="xs" />
                      </span>
                    ) : null}
                    <span className="poodle-agent-chat-input__attachment-label">
                      {attachment.label}
                    </span>
                  </>
                )}
                <IconButton
                  icon="x"
                  ariaLabel={`Remove ${attachment.label}`}
                  variant="ghost"
                  size="xs"
                  disabled={disabled || attachment.disabled}
                  onClick={() => removeAttachment(attachment.id)}
                />
              </li>
            ))}
          </ul>
        ) : null}

        <textarea
          ref={editorRef}
          className="poodle-agent-chat-input__editor"
          aria-label={ariaLabel}
          placeholder={resolvedPlaceholder}
          rows={minRows}
          maxLength={maxLength ?? undefined}
          disabled={disabled}
          readOnly={readOnly}
          value={effectiveValue}
          onChange={handleChange}
          onKeyDown={handleKeyDown}
          onCompositionStart={() => {
            composingRef.current = true;
          }}
          onCompositionEnd={() => {
            composingRef.current = false;
          }}
        />

        <div className="poodle-agent-chat-input__toolbar">
          <div className="poodle-agent-chat-input__leading" data-dividers={toolbarDividers}>
            {toolbar}
          </div>
          <div className="poodle-agent-chat-input__trailing">
            {showContext ? (
              <span className="poodle-agent-chat-input__context">
                <Meter
                  shape="ring"
                  value={contextUsed ?? 0}
                  max={contextLimit ?? 100}
                  high={contextHigh}
                  ariaLabel={contextAriaLabel}
                  size={resolvedSize}
                />
              </span>
            ) : null}
            <button
              type="button"
              className="poodle-agent-chat-input__action"
              data-state={actionState(isBusy)}
              aria-label={isBusy ? stopLabel : submitLabel}
              disabled={!actionEnabled}
              onClick={handleAction}
            >
              <Icon name={actionIcon(isBusy)} size="sm" />
            </button>
          </div>
        </div>
      </div>

      {footer ? <div className="poodle-agent-chat-input__footer">{footer}</div> : null}
    </div>
  );
}
