import { useEffect, useRef, useState, type ChangeEvent } from "react";
import { editLabelTransition, type EditLabelEvent } from "@inflatable-cookie/poodle-headless";

import "@inflatable-cookie/poodle-styles/editable-label.css";

import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, EditableLabelActivationMode, SemanticControlSizeRole } from "./types";

export interface EditableLabelProps {
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  value?: string;
  ariaLabel?: string;
  disabled?: boolean;
  activationMode?: EditableLabelActivationMode;
  selectOnFocus?: boolean;
  variant?: "default" | "flush";
  emptyText?: string | null;
  placeholder?: string | null;
  maxLength?: number | null;
  showEditIcon?: boolean;
  onEditStart?: () => void;
  onCommit?: (detail: { value: string; previousValue: string }) => void;
  onCancel?: () => void;
}

export function EditableLabel({
  size = null,
  sizeRole = "control",
  density = null,
  value = "",
  ariaLabel = "Edit label",
  disabled = false,
  activationMode = "doubleClick",
  selectOnFocus = true,
  variant = "default",
  emptyText = null,
  placeholder = null,
  maxLength = null,
  showEditIcon = false,
  onEditStart,
  onCommit,
  onCancel,
}: EditableLabelProps) {
  const uiPresentation = useUiPresentation();

  const [isEditing, setIsEditing] = useState(false);
  const [draftValue, setDraftValue] = useState(value);
  const inputRef = useRef<HTMLInputElement | null>(null);
  const pendingFocus = useRef(false);

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const displayValue = value || emptyText || "";
  const isEmpty = !value && !!emptyText;

  useEffect(() => {
    if (!isEditing) setDraftValue(value);
  }, [isEditing, value]);

  useEffect(() => {
    if (isEditing && pendingFocus.current && inputRef.current) {
      pendingFocus.current = false;
      inputRef.current.focus();
      if (selectOnFocus) inputRef.current.select();
    }
  }, [isEditing, selectOnFocus]);

  function send(event: EditLabelEvent): void {
    const result = editLabelTransition(
      isEditing ? "editing" : "view",
      { value, draft: draftValue, disabled, canStartEdit: activationMode !== "programmatic" },
      event,
    );

    setIsEditing(result.state === "editing");
    setDraftValue(result.context.draft);

    for (const effect of result.effects) {
      switch (effect.type) {
        case "emitEditStart":
          onEditStart?.();
          break;
        case "focusInput":
          pendingFocus.current = true;
          break;
        case "emitCommit":
          onCommit?.({ value: effect.value, previousValue: effect.previousValue });
          break;
        case "emitCancel":
          onCancel?.();
          break;
      }
    }
  }

  return (
    <div
      className="poodle-editable-label"
      data-editing={isEditing}
      data-disabled={disabled}
      data-variant={variant}
      data-size={resolvedSize}
      data-density={resolvedDensity}
    >
      {isEditing ? (
        <input
          ref={inputRef}
          className="poodle-editable-label__input"
          type="text"
          value={draftValue}
          placeholder={placeholder ?? undefined}
          maxLength={maxLength ?? undefined}
          onChange={(event: ChangeEvent<HTMLInputElement>) => setDraftValue(event.currentTarget.value)}
          onBlur={() => send({ type: "COMMIT" })}
          onKeyDown={(event) => {
            if (event.key === "Enter") send({ type: "COMMIT" });
            if (event.key === "Escape") {
              event.preventDefault();
              send({ type: "CANCEL" });
            }
          }}
        />
      ) : (
        <button
          type="button"
          className={`poodle-editable-label__display${isEmpty ? " poodle-editable-label__display--empty" : ""}`}
          disabled={disabled}
          aria-label={ariaLabel}
          onDoubleClick={() => {
            if (activationMode === "doubleClick") send({ type: "START_EDIT" });
          }}
          onClick={() => {
            if (activationMode === "enterOrSpace") send({ type: "START_EDIT" });
          }}
          onKeyDown={(event) => {
            if (activationMode === "enterOrSpace" && (event.key === "Enter" || event.key === " ")) {
              event.preventDefault();
              send({ type: "START_EDIT" });
            }
          }}
        >
          <span className="poodle-editable-label__text">{displayValue}</span>
          {showEditIcon ? (
            <svg className="poodle-editable-label__icon" viewBox="0 0 16 16" fill="none" aria-hidden="true">
              <path
                d="M11.5 2.5l2 2-8 8H3.5v-2l8-8z"
                stroke="currentColor"
                strokeWidth="1.25"
                strokeLinecap="round"
                strokeLinejoin="round"
              />
            </svg>
          ) : null}
        </button>
      )}
    </div>
  );
}
