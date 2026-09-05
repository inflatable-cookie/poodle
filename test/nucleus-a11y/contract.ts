// g16.111 — the shared A1 accessibility snapshot contract.
//
// One scenario file per Nucleus cohort row drives both extractors: the GPUI
// walker in `packages/gpui/preview/src/headless_driver.rs` and the Svelte DOM
// extractor in `./extract.ts`. Both emit the `SnapshotNode` shape below; the
// comparison law here is the same positional, field-by-field diff the Rust
// side applies in `packages/gpui/preview/src/nucleus_receipts.rs`.

import { createHash } from "node:crypto";
import { existsSync, readdirSync, readFileSync } from "node:fs";
import path from "node:path";

export const A1_SCENARIO_SCHEMA = "poodle.g16-nucleus-a11y-scenario.v1";
export const A1_SNAPSHOT_SCHEMA = "poodle.g16-nucleus-a11y-snapshot.v1";
export const A1_SCENARIO_DIR = "test/nucleus-a11y/scenarios";
export const A1_SNAPSHOT_DIR = "test/nucleus-a11y/snapshots";
export const A1_GPUI_RUNTIME = "gpui-headless";
export const A1_SVELTE_RUNTIME = "svelte-happy-dom";
export const A1_SVELTE_COMMAND = "effigy test:nucleus-a11y";

export const GPUI_RUN_RECORD = {
  command: "effigy regressions:native",
  mount: "HeadlessDriver",
  render_path: "poodle_render -> poodle_gpui_node_backend::to_gpui",
  input_dispatch: "gpui-test-platform-dispatch",
} as const;

export const SVELTE_RUN_RECORD = {
  command: A1_SVELTE_COMMAND,
  mount: "@testing-library/svelte render",
  render_path: "svelte mount -> happy-dom document",
  input_dispatch: "dom-events",
} as const;

/// States the contract vocabulary allows a scenario to declare.
export const A1_STATE_NAMES = ["checked", "expanded", "selected", "disabled", "invalid", "busy"] as const;
export type A1StateName = (typeof A1_STATE_NAMES)[number];

export type A1Target = { role?: string; name?: string };
export type A1Action =
  | { type: "pointer_activate"; target: A1Target }
  | { type: "key"; target: A1Target; key: string };

export type A1Exclusion = { attribute: string; reason: string };

export type A1Scenario = {
  schema: typeof A1_SCENARIO_SCHEMA;
  component: string;
  scenario_id: string;
  props: Record<string, unknown>;
  fixtures?: Record<string, unknown>;
  actions: A1Action[];
  declared_states: A1StateName[];
  web_only_exclusions: A1Exclusion[];
};

export type SnapshotNode = {
  role: string;
  name: string | null;
  value: number | null;
  value_text: string | null;
  states: Record<string, boolean | "mixed" | null>;
  relationships: { controls: number[]; labelled_by: number[]; described_by: number[] };
  level: number | null;
  orientation: string | null;
  focus_order: number | null;
  focused: boolean | null;
};

export type SnapshotFile = {
  schema: typeof A1_SNAPSHOT_SCHEMA;
  component: string;
  scenario_id: string;
  scenario_path: string;
  scenario_sha256: string;
  runtime: typeof A1_GPUI_RUNTIME | typeof A1_SVELTE_RUNTIME;
  run: typeof GPUI_RUN_RECORD | typeof SVELTE_RUN_RECORD;
  nodes: SnapshotNode[];
};

export type A1DiffEntry = { index: number; field: string; gpui: unknown; svelte: unknown };

export type LoadedScenario = { row: string; path: string; sha256: string; scenario: A1Scenario };

export function sha256Hex(bytes: Uint8Array | string): string {
  return createHash("sha256").update(bytes).digest("hex");
}

export function listScenarioRows(root: string): string[] {
  const directory = path.join(root, A1_SCENARIO_DIR);
  if (!existsSync(directory)) return [];
  return readdirSync(directory)
    .filter((file) => file.endsWith(".json"))
    .map((file) => file.slice(0, -".json".length))
    .sort();
}

export function readScenario(root: string, row: string): LoadedScenario {
  const relative = `${A1_SCENARIO_DIR}/${row}.json`;
  const bytes = readFileSync(path.join(root, relative));
  const scenario = JSON.parse(bytes.toString("utf8")) as A1Scenario;
  if (scenario.schema !== A1_SCENARIO_SCHEMA) throw new Error(`${relative} schema is not ${A1_SCENARIO_SCHEMA}`);
  for (const state of scenario.declared_states) {
    if (!A1_STATE_NAMES.includes(state)) throw new Error(`${relative} declares an unknown state ${state}`);
  }
  return { row, path: relative, sha256: sha256Hex(bytes), scenario };
}

export function snapshotPath(row: string, runtime: "gpui" | "svelte"): string {
  return `${A1_SNAPSHOT_DIR}/${row}.${runtime}.json`;
}

export function serializeSnapshot(file: SnapshotFile): string {
  return `${JSON.stringify(file, null, 2)}\n`;
}

function canonical(value: unknown): string {
  return JSON.stringify(value, (_key, inner) =>
    inner !== null && typeof inner === "object" && !Array.isArray(inner)
      ? Object.fromEntries(Object.entries(inner as Record<string, unknown>).sort(([a], [b]) => a.localeCompare(b)))
      : inner,
  );
}

/// Positional, field-by-field comparison. An extra node on either side is
/// reported against `role` with `null` on the side that lacks it.
export function diffSnapshotNodes(gpui: SnapshotNode[], svelte: SnapshotNode[]): A1DiffEntry[] {
  const diff: A1DiffEntry[] = [];
  const length = Math.max(gpui.length, svelte.length);
  for (let index = 0; index < length; index += 1) {
    const left = gpui[index];
    const right = svelte[index];
    if (left === undefined || right === undefined) {
      diff.push({ index, field: "role", gpui: left?.role ?? null, svelte: right?.role ?? null });
      continue;
    }
    const keys = [...new Set([...Object.keys(left), ...Object.keys(right)])].sort();
    for (const key of keys) {
      const leftValue = (left as Record<string, unknown>)[key] ?? null;
      const rightValue = (right as Record<string, unknown>)[key] ?? null;
      if (canonical(leftValue) !== canonical(rightValue)) diff.push({ index, field: key, gpui: leftValue, svelte: rightValue });
    }
  }
  return diff;
}
