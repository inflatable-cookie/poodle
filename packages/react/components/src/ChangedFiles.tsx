import { useMemo, useState } from "react";

import "@inflatable-cookie/poodle-core/styles/changed-files.css";

import {
  buildChangedFileTree,
  changedFileScopes,
  changedFilesTotals,
  type ChangedFileNode,
} from "@inflatable-cookie/poodle-core";

import { Button } from "./Button";
import { Icon } from "./Icon";
import { Tree } from "./Tree";
import { resolveSemanticControlSize, resolveSupportingVisualSize, useUiPresentation } from "./presentation";
import type {
  ChangedFile,
  ControlDensity,
  ControlSize,
  SemanticControlSizeRole,
} from "./types";

export interface ChangedFilesProps {
  id: string;
  files?: ChangedFile[];
  expanded?: boolean;
  expandedPaths?: string[];
  chipLimit?: number;
  showOpenDiff?: boolean;
  openDiffLabel?: string;
  showFilesLabel?: string;
  hideFilesLabel?: string;
  countLabel?: (count: number) => string;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  onToggle?: (id: string) => void;
  onOpenDiff?: (id: string) => void;
  onFileSelect?: (path: string) => void;
  onExpandedPathsChange?: (paths: string[]) => void;
}

function toTreeNodes(nodes: ChangedFileNode[]): any[] {
  return nodes.map((node) => ({
    value: node.path,
    label: node.label,
    isBranch: node.isDirectory,
    children: node.children.length > 0 ? toTreeNodes(node.children) : undefined,
    meta: { additions: node.additions, deletions: node.deletions },
  }));
}

export function ChangedFiles({
  id,
  files = [],
  expanded,
  expandedPaths,
  chipLimit = 3,
  showOpenDiff = true,
  openDiffLabel = "Open diff",
  showFilesLabel = "Show files",
  hideFilesLabel = "Hide files",
  countLabel = (count: number) => `${count} changed files`,
  size = null,
  sizeRole = "control",
  density = null,
  onToggle,
  onOpenDiff,
  onFileSelect,
  onExpandedPathsChange,
}: ChangedFilesProps) {
  const presentation = useUiPresentation();
  const resolvedSize = size ?? resolveSemanticControlSize(presentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? presentation.density;
  const glyphSize = resolveSupportingVisualSize(resolvedSize);

  const [uncontrolledExpanded, setUncontrolledExpanded] = useState(false);
  const [uncontrolledPaths, setUncontrolledPaths] = useState<string[]>([]);
  const isExpanded = expanded ?? uncontrolledExpanded;
  const openPaths = expandedPaths ?? uncontrolledPaths;

  const totals = useMemo(() => changedFilesTotals(files), [files]);
  const scopes = useMemo(() => changedFileScopes(files), [files]);
  const treeNodes = useMemo(() => toTreeNodes(buildChangedFileTree(files)), [files]);
  const visibleChips = files.slice(0, chipLimit);

  /** Counts are colour-coded, and colour alone is not a signal. */
  const headerName = `${countLabel(totals.fileCount)}, ${totals.additions} added, ${totals.deletions} removed`;

  const toggle = () => {
    if (expanded === undefined) setUncontrolledExpanded((value) => !value);
    onToggle?.(id);
  };

  // An empty card renders nothing rather than an empty state. A turn that
  // changed no files should not have a box saying so — the absence is the
  // message.
  if (files.length === 0) return null;

  return (
    <div
      className="poodle-changed-files"
      data-expanded={String(isExpanded)}
      data-file-count={totals.fileCount}
      data-size={resolvedSize}
      data-density={resolvedDensity}
    >
      <div className="poodle-changed-files__header">
        <button
          type="button"
          className="poodle-changed-files__toggle"
          aria-expanded={isExpanded}
          aria-controls={`${id}-files`}
          aria-label={headerName}
          onClick={toggle}
        >
          <span className="poodle-changed-files__toggle-icon">
            <Icon name="chevron-right" size={glyphSize} />
          </span>
          <span className="poodle-changed-files__count">{countLabel(totals.fileCount)}</span>
          <span className="poodle-changed-files__additions">+{totals.additions}</span>
          <span className="poodle-changed-files__deletions">−{totals.deletions}</span>
        </button>

        {/* The same action as the chevron, duplicated visually. Only one carries
            the accessible name: two controls announcing the same thing is a
            worse outcome than one. */}
        <button
          type="button"
          className="poodle-changed-files__files-toggle"
          aria-hidden="true"
          tabIndex={-1}
          onClick={toggle}
        >
          {isExpanded ? hideFilesLabel : showFilesLabel}
        </button>

        {showOpenDiff ? (
          <div className="poodle-changed-files__actions">
            <Button variant="secondary" size={resolvedSize} icon="diff" onClick={() => onOpenDiff?.(id)}>
              {openDiffLabel}
            </Button>
          </div>
        ) : null}
      </div>

      {isExpanded ? (
        <div className="poodle-changed-files__tree" id={`${id}-files`}>
          <Tree
            nodes={treeNodes}
            expandedValues={openPaths}
            collapseTwistyWhenFlat
            size={resolvedSize}
            density={resolvedDensity}
            onExpandedChange={(paths: string[]) => {
              if (expandedPaths === undefined) setUncontrolledPaths(paths);
              onExpandedPathsChange?.(paths);
            }}
            onActivate={(value: string) => onFileSelect?.(value)}
          />
        </div>
      ) : (
        <div className="poodle-changed-files__summary" id={`${id}-files`}>
          <div className="poodle-changed-files__scopes">
            {scopes.map((scope) => (
              <span key={scope.name}>
                <span className="poodle-changed-files__scope-name">{scope.name}</span> {scope.fileCount}{" "}
                {scope.fileCount === 1 ? "file" : "files"}
              </span>
            ))}
          </div>

          {visibleChips.map((file) => (
            <button
              key={file.path}
              type="button"
              className="poodle-changed-files__chip"
              onClick={() => onFileSelect?.(file.path)}
            >
              <Icon name="file" size={glyphSize} />
              <span className="poodle-changed-files__chip-label">{file.path.split("/").pop()}</span>
            </button>
          ))}

          {files.length > visibleChips.length ? (
            <button type="button" className="poodle-changed-files__more" onClick={toggle}>
              Show all {totals.fileCount} files
            </button>
          ) : null}
        </div>
      )}
    </div>
  );
}
