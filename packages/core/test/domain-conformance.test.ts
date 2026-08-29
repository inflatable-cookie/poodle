/**
 * Cross-runtime domain-math conformance: runs the shared vectors in
 * packages/contracts/headless/vectors/domain.json against the TypeScript core.
 * The Rust mirror runs the same vectors (tests/domain_conformance.rs).
 *
 * The vectors were generated FROM this core (expectations exact by
 * construction), so this runner is the pin: any edit to either side that
 * diverges from the shared file fails here or on the Rust side.
 */

import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";

import {
  buildCalendarWeeks,
  compareIsoDate,
  dayDeltaForWeekBoundary,
  daysBetween,
  formatIsoDate,
  isIsoDateWithinRange,
  monthAnchorIso,
  normalizeDateRange,
  parseIsoDate,
  startOfWeek,
  addDays,
  addMonths,
  type CalendarWeekStart,
} from "../src/date.ts";
import {
  hexToHsv,
  hexToRgb,
  hslToRgb,
  hsvToHex,
  hsvToRgb,
  isValidHex,
  normalizeHex,
  rgbToHex,
  rgbToHsl,
  rgbToHsv,
} from "../src/color.ts";
import { buildVisiblePages, canRequestPage } from "../src/pagination.ts";
import {
  flattenVisibleTreeRows,
  findTreeNode,
  treeCheckState,
  treeKeydownIntent,
  treeRangeSelection,
  treeSiblingReorderTarget,
  treeToggleCheck,
  treeVirtualWindow,
  type TreeNodeLike,
} from "../src/tree.ts";
import {
  adjustDurationSegment,
  durationTotalSeconds,
  padDurationSegment,
  setDurationSegment,
  type DurationSegment,
  type DurationValue,
} from "../src/duration.ts";
import { findNextEnabledIndex, firstEnabledIndex } from "../src/nav.ts";
import {
  formatTime,
  parseTime,
  secondsToTime,
  stepTimeSeconds,
  timeInBounds,
  timeInputInvalid,
  timeInputTransition,
  timeSecondsVisible,
  timeStepAligned,
  timeToSeconds,
  type TimeInputContext,
  type TimeInputEvent,
  type TimeParts,
} from "../src/time-input.ts";

interface DomainVectors {
  date: DateCase[];
  color: ColorCase[];
  pagination: PaginationCase[];
  tree: TreeCase[];
  treeNodes: JsonTreeNode[];
  duration: DurationCase[];
  nav: NavCase[];
  timeInput: TimeInputCase[];
}

interface DateCase {
  op: string;
  iso?: string | null;
  amount?: number;
  left?: string | null;
  right?: string | null;
  start?: string | null;
  end?: string | null;
  edge?: "start" | "end";
  weekStartsOn?: string;
  visibleMonth?: string;
  expect: unknown;
}

interface ColorCase {
  op: string;
  hex?: string;
  r?: number;
  g?: number;
  b?: number;
  h?: number;
  s?: number;
  v?: number;
  l?: number;
  a?: number;
  expect: unknown;
}

interface PaginationCase {
  op: string;
  page?: number;
  count?: number;
  siblings?: number;
  next?: number;
  current?: number;
  total?: number;
  expect: unknown;
}

interface JsonTreeNode {
  value: string;
  children?: JsonTreeNode[];
  isBranch?: boolean;
  isDisabled?: boolean;
}

interface TreeCase {
  op: string;
  expanded?: string[];
  value?: string;
  checked?: string[];
  anchor?: string;
  to?: string;
  siblings?: string[];
  up?: boolean;
  key?: string;
  shift?: boolean;
  rowCount?: number;
  rowHeight?: number;
  scrollTop?: number;
  viewport?: number;
  overscan?: number;
  expect: unknown;
}

interface DurationCase {
  op: string;
  name?: string;
  value?: number | { hours?: number; minutes?: number; seconds?: number };
  segment?: string;
  delta?: number;
  raw?: number;
  maxHours?: number;
  expect: unknown;
}

interface NavCase {
  op: string;
  name?: string;
  disabled?: boolean[];
  startIndex?: number;
  direction?: number;
  expect: unknown;
}

interface TimeInputCase {
  op: string;
  name?: string;
  value?: string | null;
  parts?: TimeParts;
  seconds?: boolean;
  step?: number;
  committed?: string | null;
  defaultValue?: string | null;
  min?: string | null;
  max?: string | null;
  current?: string | null;
  direction?: 1 | -1;
  context?: TimeInputContext;
  event?: TimeInputEvent;
  expect: unknown;
}

const vectors = JSON.parse(
  readFileSync(
    join(import.meta.dir, "..", "..", "contracts", "headless", "vectors", "domain.json"),
    "utf8",
  ),
) as DomainVectors;

describe("domain conformance: date", () => {
  for (const case_ of vectors.date) {
    test(`${case_.op}: ${JSON.stringify(case_.iso ?? case_.left ?? case_.start)}`, () => {
      switch (case_.op) {
        case "addDays": {
          const date = parseIsoDate(case_.iso ?? null);
          expect(formatIsoDate(date ? addDays(date, case_.amount ?? 0) : null)).toBe(case_.expect);
          return;
        }
        case "addMonths": {
          const date = parseIsoDate(case_.iso ?? null);
          expect(formatIsoDate(date ? addMonths(date, case_.amount ?? 0) : null)).toBe(case_.expect);
          return;
        }
        case "parse": {
          expect(formatIsoDate(parseIsoDate(case_.iso ?? null))).toBe(case_.expect);
          return;
        }
        case "compare": {
          expect(compareIsoDate(case_.left ?? null, case_.right ?? null)).toBe(case_.expect);
          return;
        }
        case "monthAnchor": {
          expect(monthAnchorIso(case_.iso ?? null)).toBe(case_.expect);
          return;
        }
        case "normalizeRange": {
          const result = normalizeDateRange({ start: case_.start ?? null, end: case_.end ?? null });
          const expected = case_.expect as { start: string | null; end: string | null };
          expect(result.start).toBe(expected.start);
          expect(result.end).toBe(expected.end);
          return;
        }
        case "withinRange": {
          expect(
            isIsoDateWithinRange(case_.iso ?? "", { start: case_.start ?? null, end: case_.end ?? null }),
          ).toBe(case_.expect);
          return;
        }
        case "startOfWeek": {
          const date = parseIsoDate(case_.iso ?? null);
          expect(
            formatIsoDate(date ? startOfWeek(date, case_.weekStartsOn as CalendarWeekStart) : null),
          ).toBe(case_.expect);
          return;
        }
        case "weekBoundaryDelta": {
          expect(
            dayDeltaForWeekBoundary(case_.iso ?? "", case_.weekStartsOn as CalendarWeekStart, case_.edge ?? "start"),
          ).toBe(case_.expect);
          return;
        }
        case "daysBetween": {
          expect(daysBetween(case_.start ?? "", case_.end ?? "")).toBe(case_.expect);
          return;
        }
        case "calendarWeeks": {
          // today far outside every vector month so isToday never fires
          const weeks = buildCalendarWeeks(
            case_.visibleMonth ?? "",
            case_.weekStartsOn as CalendarWeekStart,
            "1900-01-01",
          );
          expect(
            weeks.map((week) =>
              week.map((day) => ({ iso: day.iso, label: day.label, inMonth: day.inMonth })),
            ),
          ).toEqual(case_.expect);
          return;
        }
        default:
          throw new Error(`unknown date op: ${case_.op}`);
      }
    });
  }
});

describe("domain conformance: color", () => {
  for (const case_ of vectors.color) {
    test(`${case_.op}: ${case_.hex ?? [case_.r, case_.g, case_.b].join(",")}`, () => {
      switch (case_.op) {
        case "normalizeHex": {
          expect(normalizeHex(case_.hex ?? "")).toBe(case_.expect);
          return;
        }
        case "isValidHex": {
          expect(isValidHex(case_.hex ?? "")).toBe(case_.expect);
          return;
        }
        case "hexToRgb": {
          const rgb = hexToRgb(case_.hex ?? "");
          const expected = case_.expect as { r: number; g: number; b: number; a?: number };

          if (expected.a !== undefined) {
            expect(Math.abs((rgb.a ?? 0) - expected.a)).toBeLessThan(1e-9);
            return;
          }

          expect({ r: rgb.r, g: rgb.g, b: rgb.b }).toEqual(expected);
          return;
        }
        case "rgbToHex": {
          expect(rgbToHex(case_.r ?? 0, case_.g ?? 0, case_.b ?? 0)).toBe(case_.expect);
          return;
        }
        case "rgbToHexAlpha": {
          expect(rgbToHex(case_.r ?? 0, case_.g ?? 0, case_.b ?? 0, case_.a)).toBe(case_.expect);
          return;
        }
        case "rgbToHsv": {
          expect(rgbToHsv(case_.r ?? 0, case_.g ?? 0, case_.b ?? 0)).toEqual(case_.expect);
          return;
        }
        case "rgbToHsl": {
          expect(rgbToHsl(case_.r ?? 0, case_.g ?? 0, case_.b ?? 0)).toEqual(case_.expect);
          return;
        }
        case "hsvToRgb": {
          expect(hsvToRgb(case_.h ?? 0, case_.s ?? 0, case_.v ?? 0)).toEqual(case_.expect);
          return;
        }
        case "hsvToHex": {
          expect(hsvToHex(case_.h ?? 0, case_.s ?? 0, case_.v ?? 0)).toBe(case_.expect);
          return;
        }
        case "hslToRgb": {
          expect(hslToRgb(case_.h ?? 0, case_.s ?? 0, case_.l ?? 0)).toEqual(case_.expect);
          return;
        }
        case "hexToHsv": {
          expect(hexToHsv(case_.hex ?? "")).toEqual(case_.expect);
          return;
        }
        default:
          throw new Error(`unknown color op: ${case_.op}`);
      }
    });
  }
});

describe("domain conformance: pagination", () => {
  for (const case_ of vectors.pagination) {
    test(`${case_.op}`, () => {
      switch (case_.op) {
        case "visiblePages": {
          expect(buildVisiblePages(case_.page ?? 0, case_.count ?? 0, case_.siblings ?? 0)).toEqual(
            case_.expect,
          );
          return;
        }
        case "canRequestPage": {
          expect(canRequestPage(case_.next ?? 0, case_.current ?? 0, case_.total ?? 0)).toBe(
            case_.expect,
          );
          return;
        }
        default:
          throw new Error(`unknown pagination op: ${case_.op}`);
      }
    });
  }
});

function jsonNodeToTreeNode(node: JsonTreeNode): TreeNodeLike {
  return {
    value: node.value,
    children: node.children?.map(jsonNodeToTreeNode),
    isBranch: node.isBranch,
    isDisabled: node.isDisabled,
  };
}

const treeNodes: TreeNodeLike[] = vectors.treeNodes.map(jsonNodeToTreeNode);

describe("domain conformance: tree", () => {
  for (const case_ of vectors.tree) {
    test(`${case_.op}`, () => {
      const expanded: string[] = (case_.expanded ?? []).map(String);

      switch (case_.op) {
        case "flatten": {
          const rows = flattenVisibleTreeRows(treeNodes, expanded);
          expect(
            rows.map((row) => ({
              value: row.node.value,
              depth: row.depth,
              parent: row.parent,
              disabled: row.node.isDisabled === true,
            })),
          ).toEqual(case_.expect);
          return;
        }
        case "checkState": {
          const node = findTreeNode(treeNodes, case_.value ?? "");
          expect(node ? treeCheckState(node, (case_.checked ?? []).map(String)) : null).toBe(
            case_.expect,
          );
          return;
        }
        case "toggleCheck": {
          const node = findTreeNode(treeNodes, case_.value ?? "");
          const next = node ? treeToggleCheck(node, (case_.checked ?? []).map(String)) : [];
          expect([...next].sort()).toEqual([...(case_.expect as string[])].sort());
          return;
        }
        case "range": {
          const rows = flattenVisibleTreeRows(treeNodes, expanded);
          expect(treeRangeSelection(rows, case_.anchor ?? case_.to ?? null, case_.to ?? "")).toEqual(
            case_.expect,
          );
          return;
        }
        case "siblingTarget": {
          const siblings = (case_.siblings ?? []).map((value: string) => ({ value }));
          expect(treeSiblingReorderTarget(siblings, case_.value ?? "", case_.up ? -1 : 1)).toEqual(
            case_.expect,
          );
          return;
        }
        case "keydown": {
          const rows = flattenVisibleTreeRows(treeNodes, expanded);
          const intent = treeKeydownIntent(
            rows,
            case_.value ?? "",
            case_.key ?? "",
            { altKey: false, shiftKey: case_.shift === true },
            { reorderable: false, expandedValues: expanded },
          );
          expect(intent).toEqual(case_.expect);
          return;
        }
        case "virtualWindow": {
          expect(
            treeVirtualWindow(
              case_.rowCount ?? 0,
              case_.rowHeight ?? 0,
              case_.scrollTop ?? 0,
              case_.viewport ?? 0,
              case_.overscan ?? 0,
            ),
          ).toEqual(case_.expect);
          return;
        }
        default:
          throw new Error(`unknown tree op: ${case_.op}`);
      }
    });
  }
});

function durationValueFrom(case_: DurationCase): DurationValue {
  const value = case_.value;
  return {
    hours: typeof value === "number" ? 0 : (value?.hours ?? 0),
    minutes: typeof value === "number" ? 0 : (value?.minutes ?? 0),
    seconds: typeof value === "number" ? 0 : (value?.seconds ?? 0),
  };
}

describe("domain conformance: duration", () => {
  for (const case_ of vectors.duration) {
    test(`${case_.op}: ${case_.name ?? ""}`, () => {
      switch (case_.op) {
        case "totalSeconds": {
          expect(durationTotalSeconds(durationValueFrom(case_))).toBe(case_.expect);
          return;
        }
        case "adjust": {
          expect(
            adjustDurationSegment(
              durationValueFrom(case_),
              case_.segment as DurationSegment,
              case_.delta ?? 0,
              case_.maxHours ?? 0,
            ),
          ).toEqual(case_.expect);
          return;
        }
        case "set": {
          expect(
            setDurationSegment(
              durationValueFrom(case_),
              case_.segment as DurationSegment,
              case_.raw ?? 0,
              case_.maxHours ?? 0,
            ),
          ).toEqual(case_.expect);
          return;
        }
        case "pad": {
          expect(padDurationSegment(Number(case_.value))).toBe(case_.expect);
          return;
        }
        default:
          throw new Error(`unknown duration op: ${case_.op}`);
      }
    });
  }
});

describe("domain conformance: nav", () => {
  for (const case_ of vectors.nav) {
    test(`${case_.op}: ${case_.name ?? ""}`, () => {
      const items = (case_.disabled ?? []).map((disabled: boolean) => ({ disabled }));

      switch (case_.op) {
        case "findNext": {
          const result = findNextEnabledIndex(items, case_.startIndex ?? 0, case_.direction ?? 1);
          expect(result < 0 ? null : result).toBe(case_.expect);
          return;
        }
        case "firstEnabled": {
          const result = firstEnabledIndex(items);
          expect(result < 0 ? null : result).toBe(case_.expect);
          return;
        }
        default:
          throw new Error(`unknown nav op: ${case_.op}`);
      }
    });
  }
});

describe("domain conformance: timeInput", () => {
  for (const case_ of vectors.timeInput) {
    test(`${case_.op}: ${case_.name ?? ""}`, () => {
      switch (case_.op) {
        case "parse": {
          expect(parseTime(case_.value ?? null)).toEqual(case_.expect);
          return;
        }
        case "format": {
          expect(formatTime(case_.parts ?? { hour: 0, minute: 0, second: 0 }, case_.seconds === true)).toBe(
            case_.expect,
          );
          return;
        }
        case "secondsVisible": {
          expect(
            timeSecondsVisible({
              step: case_.step ?? 60,
              committed: case_.committed ?? null,
              defaultValue: case_.defaultValue ?? null,
              min: case_.min ?? null,
              max: case_.max ?? null,
            }),
          ).toBe(case_.expect);
          return;
        }
        case "inBounds": {
          expect(timeInBounds(case_.parts ?? { hour: 0, minute: 0, second: 0 }, case_.min ?? null, case_.max ?? null)).toBe(
            case_.expect,
          );
          return;
        }
        case "stepAligned": {
          expect(timeStepAligned(case_.parts ?? { hour: 0, minute: 0, second: 0 }, case_.min ?? null, case_.step ?? 60)).toBe(
            case_.expect,
          );
          return;
        }
        case "step": {
          const currentParts = case_.current === null || case_.current === undefined ? null : parseTime(case_.current);
          const next = stepTimeSeconds(
            currentParts === null ? null : timeToSeconds(currentParts),
            case_.direction ?? 1,
            case_.min ?? null,
            case_.max ?? null,
            case_.step ?? 60,
          );
          const formatted =
            next === null
              ? null
              : formatTime(
                  secondsToTime(next),
                  timeSecondsVisible({
                    committed: case_.current ?? null,
                    min: case_.min ?? null,
                    max: case_.max ?? null,
                    step: case_.step ?? 60,
                  }),
                );
          expect(formatted).toBe(case_.expect);
          return;
        }
        case "transition": {
          const result = timeInputTransition(case_.context as TimeInputContext, case_.event as TimeInputEvent);
          expect({
            context: result.context,
            effects: result.effects,
            invalid: timeInputInvalid(result.context),
          }).toEqual(case_.expect);
          return;
        }
        default:
          throw new Error(`unknown timeInput op: ${case_.op}`);
      }
    });
  }
});
