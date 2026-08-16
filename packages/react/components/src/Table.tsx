import "@inflatable-cookie/poodle-core/styles/table.css";

import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, SemanticControlSizeRole, TableColumn, TableRow } from "./types";

export interface TableProps {
  columns?: TableColumn[];
  rows?: TableRow[];
  caption?: string | null;
  emptyMessage?: string;
  ariaLabel?: string | null;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
}

export function Table({
  columns = [],
  rows = [],
  caption = null,
  emptyMessage = "No rows available.",
  ariaLabel = null,
  size = null,
  sizeRole = "control",
  density = null,
}: TableProps) {
  const uiPresentation = useUiPresentation();
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const rowHeaderColumnId = columns.find((column) => column.isRowHeader)?.id ?? columns[0]?.id ?? null;

  return (
    <div className="poodle-table-shell" data-size={resolvedSize} data-density={resolvedDensity}>
      <table className="poodle-table" aria-label={ariaLabel ?? undefined}>
        {caption ? <caption className="poodle-table__caption">{caption}</caption> : null}

        <thead>
          <tr>
            {columns.map((column) => (
              <th key={column.id} className="poodle-table__header" data-align={column.align ?? "start"} scope="col">
                {column.label}
              </th>
            ))}
          </tr>
        </thead>

        <tbody>
          {rows.length === 0 ? (
            <tr>
              <td className="poodle-table__empty" colSpan={Math.max(columns.length, 1)}>
                {emptyMessage}
              </td>
            </tr>
          ) : (
            rows.map((row) => (
              <tr key={row.id} className="poodle-table__row">
                {columns.map((column) =>
                  column.id === rowHeaderColumnId ? (
                    <th
                      key={column.id}
                      className="poodle-table__cell poodle-table__cell--row-header"
                      data-align={column.align ?? "start"}
                      scope="row"
                    >
                      {row.cells[column.id] ?? ""}
                    </th>
                  ) : (
                    <td key={column.id} className="poodle-table__cell" data-align={column.align ?? "start"}>
                      {row.cells[column.id] ?? ""}
                    </td>
                  ),
                )}
              </tr>
            ))
          )}
        </tbody>
      </table>
    </div>
  );
}
