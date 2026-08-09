import "@inflatable-cookie/poodle-core/styles/command-palette.css";

import { useEffect, useRef, useState } from "react";

import { trapFocusKeydown } from "@inflatable-cookie/poodle-core";

import { ActionDiscoveryPanel, type ActionDiscoveryPanelHandle } from "./ActionDiscoveryPanel";
import { Icon } from "./Icon";
import { TextInput } from "./TextInput";
import { UiPresentationProvider, resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type {
  CommandActionItem,
  ControlDensity,
  ControlSize,
  DiscoveryState,
  SemanticControlSizeRole,
} from "./types";

export interface CommandPaletteProps {
  open?: boolean;
  title?: string;
  description?: string | null;
  query?: string;
  items?: CommandActionItem[];
  state?: DiscoveryState;
  ariaLabel?: string | null;
  invocationHint?: string | null;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  onQueryChange?: ((value: string) => void) | undefined;
  onCommandSelect?: ((id: string) => void) | undefined;
  onOpenChange?: ((open: boolean) => void) | undefined;
  onActiveChange?: ((id: string | null) => void) | undefined;
}

const queryInputId = "command-palette-query";
const statusId = "command-palette-status";

export function CommandPalette({
  open = false,
  title = "Command palette",
  description = null,
  query = "",
  items = [],
  state: discoveryState = "ready",
  ariaLabel = null,
  invocationHint = null,
  size = null,
  sizeRole = "control",
  density = null,
  onQueryChange = undefined,
  onCommandSelect = undefined,
  onOpenChange = undefined,
  onActiveChange = undefined,
}: CommandPaletteProps) {
  const uiPresentation = useUiPresentation();

  const [activeId, setActiveId] = useState<string | null>(null);
  const panelRef = useRef<ActionDiscoveryPanelHandle | null>(null);
  const previousFocusedElement = useRef<HTMLElement | null>(null);
  const previousHtmlOverflow = useRef("");
  const previousBodyOverflow = useRef("");

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const enabledItems = items.filter((item) => !item.disabled);
  const activeItem = enabledItems.find((item) => item.id === activeId) ?? null;
  const paletteStatus =
    discoveryState === "loading"
      ? "Loading commands."
      : discoveryState === "error"
        ? "Command palette unavailable."
        : discoveryState === "empty"
          ? "No commands are available in this workspace."
          : discoveryState === "no-results"
            ? `No commands match "${query}".`
            : `${enabledItems.length} command${enabledItems.length === 1 ? "" : "s"} available.${activeItem ? ` Active command: ${activeItem.title}.` : ""}`;

  const activeIdRef = useRef(activeId);
  activeIdRef.current = activeId;
  const setActive = (id: string | null) => {
    setActiveId(id);
    onActiveChange?.(id);
  };
  const setActiveRef = useRef(setActive);
  setActiveRef.current = setActive;
  const enabledItemsRef = useRef(enabledItems);
  enabledItemsRef.current = enabledItems;

  // Open/close side effects: focus capture + scroll lock, restore on close.
  useEffect(() => {
    if (!open) return;

    previousFocusedElement.current =
      document.activeElement instanceof HTMLElement ? document.activeElement : null;
    previousHtmlOverflow.current = document.documentElement.style.overflow;
    previousBodyOverflow.current = document.body.style.overflow;
    document.documentElement.style.overflow = "hidden";
    document.body.style.overflow = "hidden";

    const input = document.getElementById(queryInputId) as HTMLInputElement | null;
    input?.focus();
    if (enabledItemsRef.current.length > 0) {
      setActiveRef.current(enabledItemsRef.current[0]?.id ?? null);
    }

    return () => {
      setActiveId(null);
      document.documentElement.style.overflow = previousHtmlOverflow.current;
      document.body.style.overflow = previousBodyOverflow.current;
      // Defer past the closing event's default action: React flushes this
      // cleanup synchronously inside the keydown dispatch, and restoring
      // focus immediately would let the browser's default Enter action
      // "click" the newly focused trigger and reopen the palette.
      const previous = previousFocusedElement.current;
      previousFocusedElement.current = null;
      window.setTimeout(() => previous?.focus(), 0);
    };
  }, [open]);

  // Keep the active row valid as the item set changes.
  useEffect(() => {
    if (!open) return;

    if (enabledItems.length > 0 && (!activeId || !enabledItems.some((item) => item.id === activeId))) {
      setActive(enabledItems[0]?.id ?? null);
    }

    if (enabledItems.length === 0 && activeId !== null) {
      setActive(null);
    }
  }, [open, items, activeId]);

  function close(): void {
    onOpenChange?.(false);
  }

  useEffect(() => {
    if (!open) return;

    function handleKeydown(event: KeyboardEvent): void {
      trapFocusKeydown(document.querySelector<HTMLElement>(".poodle-command-palette"), event);

      if (event.key === "Escape") {
        event.preventDefault();
        onOpenChange?.(false);
        return;
      }
      if (event.key === "ArrowDown") {
        event.preventDefault();
        panelRef.current?.moveActive(1);
        return;
      }
      if (event.key === "ArrowUp") {
        event.preventDefault();
        panelRef.current?.moveActive(-1);
        return;
      }
      if (event.key === "Home") {
        event.preventDefault();
        panelRef.current?.moveToBoundary("start");
        return;
      }
      if (event.key === "End") {
        event.preventDefault();
        panelRef.current?.moveToBoundary("end");
        return;
      }
      if (event.key === "Enter" && activeIdRef.current) {
        event.preventDefault();
        onCommandSelect?.(activeIdRef.current);
      }
    }

    window.addEventListener("keydown", handleKeydown);
    return () => window.removeEventListener("keydown", handleKeydown);
  }, [open]);

  if (!open) {
    return null;
  }

  return (
    <>
      <div className="poodle-command-palette__overlay" aria-hidden="true" onClick={close} />
      <UiPresentationProvider sizeScale={resolvedSize} density={resolvedDensity}>
        <div
          className="poodle-command-palette"
          role="dialog"
          aria-modal="true"
          aria-label={ariaLabel ?? title}
          aria-describedby={description ? "command-palette-description" : undefined}
          data-size={resolvedSize}
          data-density={resolvedDensity}
        >
          <div className="poodle-command-palette__header">
            <div>
              <h3>{title}</h3>
              {description ? <p id="command-palette-description">{description}</p> : null}
            </div>
            <div className="poodle-command-palette__meta">
              {invocationHint ? <span className="poodle-command-palette__hint">{invocationHint}</span> : null}
              <button
                type="button"
                className="poodle-command-palette__close"
                aria-label="Close command palette"
                onClick={close}
              >
                <Icon name="x" />
              </button>
            </div>
          </div>

          <div className="poodle-command-palette__query">
            <TextInput
              id={queryInputId}
              type="search"
              value={query}
              ariaLabel="Search commands"
              describedBy={statusId}
              placeholder="Search commands, panels, and actions"
              onValueChange={(nextValue) => onQueryChange?.(nextValue)}
              onClear={() => onQueryChange?.("")}
              onCancel={close}
              onSubmit={() => {
                if (activeIdRef.current) {
                  onCommandSelect?.(activeIdRef.current);
                }
              }}
            />
          </div>

          <p id={statusId} className="poodle-command-palette__status" role="status" aria-live="polite" aria-atomic="true">
            {paletteStatus}
          </p>

          <ActionDiscoveryPanel
            ref={panelRef}
            items={items}
            state={discoveryState}
            activeId={activeId}
            ariaLabel="Command results"
            size={resolvedSize}
            density={resolvedDensity}
            onItemSelect={(id) => onCommandSelect?.(id)}
            onActiveChange={(id) => setActive(id)}
          />
        </div>
      </UiPresentationProvider>
    </>
  );
}
