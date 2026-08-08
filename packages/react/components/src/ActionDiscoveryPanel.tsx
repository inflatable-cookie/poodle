import "@inflatable-cookie/poodle-styles/action-discovery-panel.css";

import { forwardRef, useImperativeHandle, useRef, useState } from "react";

import { EmptyState } from "./EmptyState";
import { Eyebrow } from "./Eyebrow";
import { ListCard } from "./ListCard";
import { Skeleton } from "./Skeleton";
import { UiPresentationProvider, resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type {
  CommandActionItem,
  ControlDensity,
  ControlSize,
  DiscoveryState,
  SemanticControlSizeRole,
} from "./types";

export interface ActionDiscoveryPanelProps {
  items?: CommandActionItem[];
  state?: DiscoveryState;
  activeId?: string | null;
  defaultActiveId?: string | null;
  ariaLabel?: string;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  onItemSelect?: ((id: string) => void) | null;
  onActiveChange?: ((id: string | null) => void) | null;
}

export interface ActionDiscoveryPanelHandle {
  moveActive: (step: 1 | -1) => void;
  moveToBoundary: (direction: "start" | "end") => void;
  selectActive: () => void;
  getEnabledItems: () => CommandActionItem[];
}

export const ActionDiscoveryPanel = forwardRef<ActionDiscoveryPanelHandle, ActionDiscoveryPanelProps>(
  function ActionDiscoveryPanel(
    {
      items = [],
      state: discoveryState = "ready",
      activeId: controlledActiveId,
      defaultActiveId = null,
      ariaLabel = "Actions",
      size = null,
      sizeRole = "control",
      density = null,
      onItemSelect = null,
      onActiveChange = null,
    },
    ref,
  ) {
    const uiPresentation = useUiPresentation();
    const [uncontrolledActiveId, setUncontrolledActiveId] = useState<string | null>(defaultActiveId);
    const activeId = controlledActiveId !== undefined ? controlledActiveId : uncontrolledActiveId;
    const itemRefs = useRef<Array<HTMLElement | null>>([]);

    const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
    const resolvedDensity = density ?? uiPresentation.density;
    const enabledItems = items.filter((item) => !item.disabled);
    const groupedItems = items.reduce<Record<string, CommandActionItem[]>>((acc, item) => {
      const group = item.group ?? "Commands";
      acc[group] ??= [];
      acc[group].push(item);
      return acc;
    }, {});
    const groupEntries = Object.entries(groupedItems);

    function setActive(id: string | null): void {
      if (controlledActiveId === undefined) {
        setUncontrolledActiveId(id);
      }
      onActiveChange?.(id);

      if (id) {
        const idx = enabledItems.findIndex((item) => item.id === id);
        if (idx >= 0) {
          queueMicrotask(() => {
            itemRefs.current[idx]?.scrollIntoView({ block: "nearest" });
          });
        }
      }
    }

    useImperativeHandle(ref, () => ({
      moveActive(step: 1 | -1): void {
        if (enabledItems.length === 0) return;
        const idx = enabledItems.findIndex((item) => item.id === activeId);
        const next = idx === -1 ? 0 : (idx + step + enabledItems.length) % enabledItems.length;
        setActive(enabledItems[next]?.id ?? null);
      },
      moveToBoundary(direction: "start" | "end"): void {
        if (enabledItems.length === 0) return;
        setActive(
          direction === "start" ? (enabledItems[0]?.id ?? null) : (enabledItems[enabledItems.length - 1]?.id ?? null),
        );
      },
      selectActive(): void {
        if (activeId) onItemSelect?.(activeId);
      },
      getEnabledItems(): CommandActionItem[] {
        return enabledItems;
      },
    }));

    return (
      <UiPresentationProvider sizeScale={resolvedSize} density={resolvedDensity}>
        <div
          className="poodle-action-discovery-panel"
          role="listbox"
          aria-label={ariaLabel}
          data-size={resolvedSize}
          data-density={resolvedDensity}
        >
          {discoveryState === "loading" ? (
            <div className="poodle-action-discovery-panel__state">
              <div className="poodle-action-discovery-panel__skeletons" aria-hidden="true">
                {Array.from({ length: 5 }).map((_, index) => (
                  <div key={index} className="poodle-action-discovery-panel__skeleton-row">
                    <Skeleton width="48%" />
                    <Skeleton width="20%" />
                  </div>
                ))}
              </div>
            </div>
          ) : discoveryState === "error" ? (
            <EmptyState title="Could not load actions" message="Actions could not be loaded. Try again." />
          ) : discoveryState === "empty" ? (
            <EmptyState title="No actions available" message="No actions are available in this context." />
          ) : discoveryState === "no-results" ? (
            <EmptyState title="No matching actions" message="No actions match the current search." variant="search" />
          ) : (
            groupEntries.map(([group, groupItems]) => (
              <div key={group} className="poodle-action-discovery-panel__group">
                <Eyebrow>{group}</Eyebrow>
                <ul className="poodle-action-discovery-panel__list">
                  {groupItems.map((item) => (
                    <li
                      key={item.id}
                      ref={(el) => {
                        const idx = enabledItems.findIndex((enabledItem) => enabledItem.id === item.id);
                        if (idx >= 0) {
                          itemRefs.current[idx] = el;
                        }
                      }}
                      role="option"
                      aria-selected={activeId === item.id}
                      onMouseEnter={() => setActive(item.id)}
                      onFocus={() => setActive(item.id)}
                    >
                      <ListCard
                        title={item.title}
                        subtitle={item.description}
                        interactive={!item.disabled}
                        disabled={item.disabled ?? false}
                        ariaLabel={item.title}
                        onClick={() => onItemSelect?.(item.id)}
                        trailing={
                          <span className="poodle-action-discovery-panel__trailing">
                            {item.badge ? (
                              <span className="poodle-action-discovery-panel__badge">{item.badge}</span>
                            ) : null}
                            {item.shortcut ? (
                              <kbd className="poodle-action-discovery-panel__kbd">{item.shortcut}</kbd>
                            ) : null}
                          </span>
                        }
                      />
                    </li>
                  ))}
                </ul>
              </div>
            ))
          )}
        </div>
      </UiPresentationProvider>
    );
  },
);
