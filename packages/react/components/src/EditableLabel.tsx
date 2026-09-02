import {
  forwardRef,
  useEffect,
  useImperativeHandle,
  useRef,
  useState,
  type ChangeEvent,
  type KeyboardEvent,
} from "react";
import { editLabelTransition, type EditLabelEvent } from "@inflatable-cookie/poodle-core";

import "@inflatable-cookie/poodle-core/styles/editable-label.css";

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

export interface EditableLabelHandle {
  focus(): void;
  startEditing(): void;
  cancelEditing(): void;
}

export const EditableLabel = forwardRef<EditableLabelHandle, EditableLabelProps>(function EditableLabel(
  {
    size = null,
    sizeRole = "control",
    density = null,
    value = "",
    ariaLabel,
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
  },
  ref,
) {
  const uiPresentation = useUiPresentation();

  const [isEditing, setIsEditing] = useState(false);
  const [draftValue, setDraftValue] = useState(value);
  const inputRef = useRef<HTMLInputElement | null>(null);
  const displayRef = useRef<HTMLButtonElement | null>(null);
  const pendingFocus = useRef(false);
  const pendingRestore = useRef(false);
  const tearingDown = useRef(false);
  const selectOnFocusRef = useRef(selectOnFocus);
  selectOnFocusRef.current = selectOnFocus;
  const session = useRef({
    isEditing: false,
    draftValue: value,
    machineValue: value,
    disabled,
    maxLength,
  });

  session.current.isEditing = isEditing;
  session.current.draftValue = draftValue;
  session.current.disabled = disabled;
  session.current.maxLength = maxLength;

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const displayValue = value || emptyText || "";
  const isEmpty = !value && !!emptyText;
  const accessibleName = ariaLabel || value || emptyText || "Edit label";

  function send(event: EditLabelEvent): void {
    const current = session.current;
    const result = editLabelTransition(
      current.isEditing ? "editing" : "view",
      {
        value: current.machineValue,
        draft: current.draftValue,
        disabled: current.disabled,
        maxLength: current.maxLength,
      },
      event,
    );

    const nextEditing = result.state === "editing";
    const nextDraft = result.context.draft;
    if (current.isEditing !== nextEditing) setIsEditing(nextEditing);
    if (current.draftValue !== nextDraft) setDraftValue(nextDraft);
    session.current.isEditing = nextEditing;
    session.current.draftValue = nextDraft;
    session.current.machineValue = result.context.value;

    if (event.type === "SET_DRAFT" && inputRef.current && inputRef.current.value !== nextDraft) {
      inputRef.current.value = nextDraft;
    }

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
          if (effect.restoreFocus) pendingRestore.current = true;
          break;
        case "emitCancel":
          onCancel?.();
          if (effect.restoreFocus) pendingRestore.current = true;
          break;
      }
    }
  }

  useEffect(() => {
    send({ type: "REPLACE_VALUE", value });
  }, [value]);

  useEffect(() => {
    send({ type: "SET_DISABLED", disabled });
  }, [disabled]);

  useEffect(() => {
    const onWindowBlur = (): void => {
      queueMicrotask(() => {
        if (!tearingDown.current) send({ type: "COMMIT_BLUR" });
      });
    };

    window.addEventListener("blur", onWindowBlur);
    return () => window.removeEventListener("blur", onWindowBlur);
  }, []);

  useEffect(() => {
    tearingDown.current = false;
    return () => {
      tearingDown.current = true;
      send({ type: "TEARDOWN" });
    };
  }, []);

  useImperativeHandle(ref, () => ({
    focus() {
      if (session.current.isEditing) inputRef.current?.focus();
      else displayRef.current?.focus();
    },
    startEditing() {
      send({ type: "START_EDIT" });
    },
    cancelEditing() {
      send({ type: "CANCEL" });
    },
  }));

  function activateFromKey(event: KeyboardEvent<HTMLButtonElement>): void {
    if (activationMode === "programmatic") return;
    if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      send({ type: "START_EDIT" });
    }
  }

  function assignInput(node: HTMLInputElement | null): void {
    inputRef.current = node;
    if (!node || !pendingFocus.current) return;
    pendingFocus.current = false;
    node.focus();
    if (selectOnFocusRef.current) node.select();
    else {
      const end = node.value.length;
      node.setSelectionRange(end, end);
    }
  }

  function assignDisplay(node: HTMLButtonElement | null): void {
    displayRef.current = node;
    if (!node || !pendingRestore.current) return;
    pendingRestore.current = false;
    node.focus();
  }

  function handleBlur(): void {
    queueMicrotask(() => {
      if (!tearingDown.current) send({ type: "COMMIT_BLUR" });
    });
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
          ref={assignInput}
          className="poodle-editable-label__input"
          type="text"
          aria-label={accessibleName}
          value={draftValue}
          placeholder={placeholder ?? undefined}
          onChange={(event: ChangeEvent<HTMLInputElement>) =>
            send({ type: "SET_DRAFT", draft: event.currentTarget.value })
          }
          onBlur={handleBlur}
          onKeyDown={(event) => {
            if (event.key === "Enter") {
              event.preventDefault();
              send({ type: "COMMIT" });
            }
            if (event.key === "Escape") {
              event.preventDefault();
              send({ type: "CANCEL" });
            }
          }}
        />
      ) : (
        <button
          ref={assignDisplay}
          type="button"
          className={`poodle-editable-label__display${isEmpty ? " poodle-editable-label__display--empty" : ""}`}
          disabled={disabled}
          aria-label={accessibleName}
          onDoubleClick={() => {
            if (activationMode === "doubleClick") send({ type: "START_EDIT" });
          }}
          onClick={() => {
            if (activationMode === "enterOrSpace") send({ type: "START_EDIT" });
          }}
          onKeyDown={activateFromKey}
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
});
