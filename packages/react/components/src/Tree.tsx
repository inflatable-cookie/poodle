import {
  useEffect,
  useRef,
  useState,
  type DragEvent,
  type KeyboardEvent,
  type MouseEvent,
  type UIEvent,
} from "react";
import {
  findTreeNode,
  flattenVisibleTreeRows,
  isTreeBranch,
  treeCheckState,
  treeKeydownIntent,
  treeRangeSelection,
  treeSiblingReorderTarget,
  treeToggleCheck,
  treeVirtualWindow,
} from "@inflatable-cookie/poodle-headless";

import "@inflatable-cookie/poodle-styles/tree.css";

import { Checkbox } from "./Checkbox";
import { Icon } from "./Icon";
import { Spinner } from "./Spinner";
import type {
  ControlDensity,
  ControlSize,
  SemanticControlSizeRole,
  TreeDropPosition,
  TreeNode,
} from "./types";

export interface TreeProps {
  nodes?: TreeNode[];
  selectedValues?: string[];
  expandedValues?: string[] | null;
  defaultExpandedValues?: string[];
  checkedValues?: string[];
  loadingValues?: string[];
  editingValue?: string | null;
  ariaLabel?: string | null;
  showGuides?: boolean;
  /**
   * Reclaim the twisty gutter when nothing in the tree can expand.
   *
   * Leaves render a twisty-sized spacer so their labels align with branch
   * labels; in a genuinely flat tree that aligns them with nothing. Opt in and
   * it collapses, returning the moment any node becomes a branch. tree.md §7.
   */
  collapseTwistyWhenFlat?: boolean;
  showIcons?: boolean;
  showCheckboxes?: boolean;
  reorderable?: boolean;
  virtualized?: boolean;
  virtualHeight?: number;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  onSelectionChange?: (values: string[]) => void;
  onExpandedChange?: (values: string[]) => void;
  onCheckedChange?: (values: string[]) => void;
  onLoadChildren?: (value: string) => void;
  onRenameCommit?: (value: string, text: string) => void;
  onRenameCancel?: () => void;
  onEditingChange?: (value: string | null) => void;
  onContextMenu?: (value: string, x: number, y: number) => void;
  onReorder?: (from: string, to: string, position: TreeDropPosition) => void;
  onActivate?: (value: string) => void;
}

type Row = { node: TreeNode; depth: number; parent: string | null };

const ROW_PX: Record<string, number> = { xs: 22, sm: 24, md: 28, lg: 32, xl: 36 };

export function Tree({
  nodes = [],
  selectedValues: selectedValuesProp,
  expandedValues = null,
  defaultExpandedValues = [],
  checkedValues: checkedValuesProp,
  loadingValues = [],
  editingValue: editingValueProp,
  ariaLabel = null,
  showGuides = true,
  collapseTwistyWhenFlat = false,
  showIcons = true,
  showCheckboxes = false,
  reorderable = false,
  virtualized = false,
  virtualHeight = 320,
  size = null,
  sizeRole = "chrome",
  density = null,
  onSelectionChange,
  onExpandedChange,
  onCheckedChange,
  onLoadChildren,
  onRenameCommit,
  onRenameCancel,
  onEditingChange,
  onContextMenu,
  onReorder,
  onActivate,
}: TreeProps) {
  const rootRef = useRef<HTMLDivElement | null>(null);

  // Bindable pairs: controlled when prop provided, internal otherwise.
  const [internalSelected, setInternalSelected] = useState<string[]>([]);
  const selectedValues = selectedValuesProp ?? internalSelected;
  const [internalChecked, setInternalChecked] = useState<string[]>([]);
  const checkedValues = checkedValuesProp ?? internalChecked;
  const [internalEditing, setInternalEditing] = useState<string | null>(null);
  const editingValue = editingValueProp !== undefined ? editingValueProp : internalEditing;
  const [internalExpanded, setInternalExpanded] = useState<string[]>(() => [...defaultExpandedValues]);
  const isControlledExpansion = expandedValues !== null;
  const expanded = isControlledExpansion ? (expandedValues ?? []) : internalExpanded;

  const [focusedValue, setFocusedValue] = useState<string | null>(null);
  const [renameDraft, setRenameDraft] = useState("");
  const [dragValue, setDragValue] = useState<string | null>(null);
  const [dropTarget, setDropTarget] = useState<string | null>(null);
  const [dropPosition, setDropPosition] = useState<TreeDropPosition>("after");
  const [scrollTop, setScrollTop] = useState(0);
  const renameInputRef = useRef<HTMLInputElement | null>(null);
  const anchorValue = useRef<string | null>(null);
  const lastEditing = useRef<string | null>(null);

  const rowHeightPx = ROW_PX[size ?? "md"] ?? 28;
  const isExpanded = (value: string) => expanded.includes(value);
  const isSelected = (value: string) => selectedValues.includes(value);
  const isChecked = (value: string) => checkedValues.includes(value);
  const isLoading = (value: string) => loadingValues.includes(value);
  const isEditing = (value: string) => editingValue === value;
  const findNode = (value: string) => findTreeNode(nodes, value);

  const visibleRows = flattenVisibleTreeRows(nodes, expanded) as Row[];
  const effectiveFocus =
    focusedValue && visibleRows.some((r) => r.node.value === focusedValue)
      ? focusedValue
      : (selectedValues.find((v) => visibleRows.some((r) => r.node.value === v)) ?? visibleRows[0]?.node.value ?? null);

  const virtualWindow = treeVirtualWindow(visibleRows.length, rowHeightPx, scrollTop, virtualHeight);

  /** Whether any node in the whole tree can expand. */
  const hasAnyBranch = (list: TreeNode[]): boolean =>
    list.some((node) => isTreeBranch(node) || hasAnyBranch(node.children ?? []));
  const isFlat = collapseTwistyWhenFlat && !hasAnyBranch(nodes);

  const windowRows = visibleRows.slice(virtualWindow.startIndex, virtualWindow.endIndex);

  // Seed rename draft + focus input whenever rename starts.
  useEffect(() => {
    if (editingValue !== lastEditing.current) {
      lastEditing.current = editingValue;
      if (editingValue) setRenameDraft(findNode(editingValue)?.label ?? "");
    }
    if (editingValue && renameInputRef.current) {
      renameInputRef.current.focus();
      renameInputRef.current.select();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [editingValue, nodes]);

  function setEditing(next: string | null): void {
    if (editingValueProp === undefined) setInternalEditing(next);
    onEditingChange?.(next);
  }

  function setSelection(next: string[]): void {
    if (selectedValuesProp === undefined) setInternalSelected(next);
    onSelectionChange?.(next);
  }

  function setChecked(next: string[]): void {
    if (checkedValuesProp === undefined) setInternalChecked(next);
    onCheckedChange?.(next);
  }

  function setExpanded(next: string[]): void {
    if (!isControlledExpansion) setInternalExpanded(next);
    onExpandedChange?.(next);
  }

  function expand(value: string): void {
    if (isExpanded(value)) return;
    setExpanded([...expanded, value]);
    const node = findNode(value);
    if (node && isTreeBranch(node) && !node.children?.length && !isLoading(value)) {
      onLoadChildren?.(value);
    }
  }

  function collapse(value: string): void {
    if (isExpanded(value)) setExpanded(expanded.filter((v) => v !== value));
  }

  function selectOnly(value: string): void {
    anchorValue.current = value;
    setSelection([value]);
  }

  function toggleSelection(value: string): void {
    anchorValue.current = value;
    setSelection(isSelected(value) ? selectedValues.filter((v) => v !== value) : [...selectedValues, value]);
  }

  function extendSelection(toValue: string): void {
    if (anchorValue.current === null) anchorValue.current = effectiveFocus;
    const range = treeRangeSelection(visibleRows, anchorValue.current, toValue);
    if (range === null) {
      selectOnly(toValue);
      return;
    }
    setSelection(range);
  }

  function siblingListOf(value: string, list: TreeNode[] = nodes): TreeNode[] | null {
    if (list.some((n) => n.value === value)) return list;
    for (const n of list) {
      if (n.children) {
        const found = siblingListOf(value, n.children);
        if (found) return found;
      }
    }
    return null;
  }

  function moveSibling(node: TreeNode, dir: 1 | -1): void {
    const sibs = siblingListOf(node.value);
    if (!sibs) return;
    const move = treeSiblingReorderTarget(sibs, node.value, dir);
    if (!move) return;
    onReorder?.(node.value, move.target, move.position);
  }

  function startRename(node: TreeNode): void {
    if (node.isDisabled) return;
    setEditing(node.value);
  }

  function commitRename(node: TreeNode): void {
    if (editingValue !== node.value) return;
    const text = renameDraft;
    setEditing(null);
    onRenameCommit?.(node.value, text);
  }

  function cancelRename(): void {
    if (editingValue === null) return;
    setEditing(null);
    onRenameCancel?.();
  }

  function focusRow(value: string | null): void {
    if (!value) return;
    setFocusedValue(value);
    rootRef.current?.querySelector<HTMLElement>(`[data-value="${CSS.escape(value)}"]`)?.focus();
  }

  function handleKeydown(row: Row, event: KeyboardEvent): void {
    event.stopPropagation();
    const intent = treeKeydownIntent(
      visibleRows,
      row.node.value,
      event.key,
      { altKey: event.altKey, shiftKey: event.shiftKey },
      { reorderable, expandedValues: expanded },
    );
    if (!intent) return;
    event.preventDefault();

    switch (intent.type) {
      case "focus":
        if (intent.extendSelection && intent.value) extendSelection(intent.value);
        focusRow(intent.value);
        break;
      case "expand":
        expand(intent.value);
        break;
      case "collapse":
        collapse(intent.value);
        break;
      case "focusParent":
        focusRow(intent.parent);
        break;
      case "moveSibling":
        moveSibling(row.node, intent.direction);
        break;
      case "activate":
        selectOnly(row.node.value);
        onActivate?.(row.node.value);
        break;
      case "toggleSelection":
        toggleSelection(row.node.value);
        break;
      case "startRename":
        startRename(row.node);
        break;
    }
  }

  function handleRowClick(node: TreeNode, event: MouseEvent): void {
    event.stopPropagation();
    if ((event.target as HTMLElement | null)?.closest(".poodle-tree__checkbox")) return;
    if (node.isDisabled) return;
    setFocusedValue(node.value);
    if (event.shiftKey) extendSelection(node.value);
    else if (event.metaKey || event.ctrlKey) toggleSelection(node.value);
    else selectOnly(node.value);
  }

  function rowMarkup(node: TreeNode, depth: number, branch: boolean, open: boolean) {
    const cs = showCheckboxes ? treeCheckState(node, checkedValues) : null;
    return (
      <div className="poodle-tree__row">
        {Array.from({ length: depth }, (_, i) => (
          <span key={i} className="poodle-tree__indent" data-guide={showGuides ? "true" : undefined} aria-hidden="true" />
        ))}
        <span
          className="poodle-tree__twisty"
          data-expanded={open ? "true" : undefined}
          onClick={(event) => {
            event.stopPropagation();
            if (node.isDisabled) return;
            if (isExpanded(node.value)) collapse(node.value);
            else expand(node.value);
          }}
          aria-hidden="true"
        >
          {branch ? <Icon name="chevron-right" /> : null}
        </span>
        {showCheckboxes && cs ? (
          <span className="poodle-tree__checkbox">
            <Checkbox
              checked={cs === "checked"}
              mixed={cs === "mixed"}
              disabled={node.isDisabled}
              label={null}
              size={size ?? "xs"}
              onCheckedChange={() => setChecked(treeToggleCheck(node, checkedValues))}
            />
          </span>
        ) : null}
        {showIcons ? (
          <span className="poodle-tree__icon" aria-hidden="true">
            {node.icon ? <Icon name={node.icon} /> : null}
          </span>
        ) : null}
        {isEditing(node.value) ? (
          <input
            ref={renameInputRef}
            className="poodle-tree__rename"
            value={renameDraft}
            onChange={(event) => setRenameDraft(event.currentTarget.value)}
            onKeyDown={(event) => {
              event.stopPropagation();
              if (event.key === "Enter") {
                event.preventDefault();
                commitRename(node);
              } else if (event.key === "Escape") {
                event.preventDefault();
                cancelRename();
              }
            }}
            onBlur={() => commitRename(node)}
            onClick={(event) => event.stopPropagation()}
            aria-label={`Rename ${node.label}`}
          />
        ) : (
          <>
            <span className="poodle-tree__label">{node.label}</span>
            {node.endLabel ? <span className="poodle-tree__end-label">{node.endLabel}</span> : null}
          </>
        )}
      </div>
    );
  }

  function itemProps(node: TreeNode, depth: number, parent: string | null, branch: boolean, open: boolean) {
    return {
      className: "poodle-tree__item",
      role: "treeitem" as const,
      "data-value": node.value,
      "data-branch": branch ? "true" : undefined,
      "data-selected": isSelected(node.value) ? "true" : undefined,
      "data-muted": node.isMuted ? "true" : undefined,
      "data-drop": dropTarget === node.value ? dropPosition : undefined,
      draggable: reorderable && !node.isDisabled && !isEditing(node.value),
      tabIndex: effectiveFocus === node.value ? 0 : -1,
      "aria-level": depth + 1,
      "aria-selected": isSelected(node.value),
      "aria-expanded": branch ? open : undefined,
      "aria-disabled": node.isDisabled ? true : undefined,
      onClick: (event: MouseEvent) => handleRowClick(node, event),
      onDoubleClick: (event: MouseEvent) => {
        event.stopPropagation();
        if (node.isDisabled) return;
        onActivate?.(node.value);
      },
      onContextMenu: (event: MouseEvent) => {
        if (!onContextMenu) return;
        event.preventDefault();
        event.stopPropagation();
        setFocusedValue(node.value);
        onContextMenu(node.value, event.clientX, event.clientY);
      },
      onDragStart: (event: DragEvent) => {
        if (!reorderable || node.isDisabled) return;
        setDragValue(node.value);
        if (event.dataTransfer) {
          event.dataTransfer.effectAllowed = "move";
          event.dataTransfer.setData("text/plain", node.value);
        }
      },
      onDragOver: (event: DragEvent) => {
        if (!reorderable || !dragValue || dragValue === node.value) return;
        event.preventDefault();
        const rect = (event.currentTarget as HTMLElement).getBoundingClientRect();
        const ratio = rect.height > 0 ? (event.clientY - rect.top) / rect.height : 0.5;
        const pos: TreeDropPosition = isTreeBranch(node)
          ? ratio < 0.25
            ? "before"
            : ratio > 0.75
              ? "after"
              : "inside"
          : ratio < 0.5
            ? "before"
            : "after";
        setDropTarget(node.value);
        setDropPosition(pos);
      },
      onDrop: (event: DragEvent) => {
        if (!reorderable || !dragValue) return;
        event.preventDefault();
        const from = dragValue;
        const pos = dropPosition;
        setDragValue(null);
        setDropTarget(null);
        if (from !== node.value) onReorder?.(from, node.value, pos);
      },
      onDragEnd: () => {
        setDragValue(null);
        setDropTarget(null);
      },
      onKeyDown: (event: KeyboardEvent) => handleKeydown({ node, depth, parent }, event),
    };
  }

  function loadingRow(depth: number) {
    return (
      <div className="poodle-tree__item" role="treeitem" aria-level={depth + 1} aria-selected={false} aria-disabled={true}>
        <div className="poodle-tree__row poodle-tree__row--loading">
          {Array.from({ length: depth }, (_, i) => (
            <span key={i} className="poodle-tree__indent" aria-hidden="true" />
          ))}
          <span className="poodle-tree__twisty" aria-hidden="true" />
          <span className="poodle-tree__spinner">
            <Spinner size="xs" ariaLabel="Loading" />
          </span>
          <span className="poodle-tree__label poodle-tree__label--muted">Loading…</span>
        </div>
      </div>
    );
  }

  function renderNode(node: TreeNode, depth: number, parent: string | null) {
    const branch = isTreeBranch(node);
    const open = branch && isExpanded(node.value);
    return (
      <div key={node.value} {...itemProps(node, depth, parent, branch, open)}>
        {rowMarkup(node, depth, branch, open)}
        {branch && open ? (
          node.children?.length ? (
            <div className="poodle-tree__group" role="group">
              {node.children.map((child) => renderNode(child, depth + 1, node.value))}
            </div>
          ) : isLoading(node.value) ? (
            <div className="poodle-tree__group" role="group">
              {loadingRow(depth + 1)}
            </div>
          ) : null
        ) : null}
      </div>
    );
  }

  return (
    <div
      ref={rootRef}
      className="poodle-tree"
      role="tree"
      aria-multiselectable="true"
      aria-label={ariaLabel ?? undefined}
      data-size={size ?? undefined}
      data-density={density ?? undefined}
      data-size-role={sizeRole}
      data-flat={isFlat ? "true" : undefined}
      data-virtualized={virtualized ? "true" : undefined}
      style={virtualized ? { height: `${virtualHeight}px`, overflowY: "auto" } : undefined}
      onScroll={virtualized ? (event: UIEvent<HTMLDivElement>) => setScrollTop(event.currentTarget.scrollTop) : undefined}
    >
      {virtualized ? (
        <div className="poodle-tree__viewport" style={{ height: `${virtualWindow.totalHeight}px` }}>
          <div className="poodle-tree__window" style={{ transform: `translateY(${virtualWindow.offsetY}px)` }}>
            {windowRows.map((row) => {
              const branch = isTreeBranch(row.node);
              const open = branch && isExpanded(row.node.value);
              return (
                <div key={row.node.value} {...itemProps(row.node, row.depth, row.parent, branch, open)}>
                  {rowMarkup(row.node, row.depth, branch, open)}
                </div>
              );
            })}
          </div>
        </div>
      ) : (
        nodes.map((node) => renderNode(node, 0, null))
      )}
    </div>
  );
}
