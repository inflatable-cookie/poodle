/**
 * Conformance orchestrator (spec 066, g14.001): reads the active-runtime
 * reports (`svelte.json`, `react.json` from the web vitest run; `gpui.json`
 * from the real-window GPUI runner) and applies the cross-runtime rules:
 *
 *   - every case must have run in every active runtime;
 *   - no assertion may fail in any runtime (failures name
 *     runtime / case / step / field, with the reason when a runtime could
 *     not observe a required field);
 *   - Jetstream is program-deferred (working rules, active cohort): it is
 *     reported as deferred, never as passing.
 *
 * Exit 1 on any violation.
 */

import { readFileSync, existsSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

interface AssertionResult {
  stepIndex: number;
  part: string | null;
  field: string;
  verdict: "pass" | "fail";
  expected?: unknown;
  actual?: unknown;
  reason?: string;
}

interface CaseResult {
  caseId: string;
  pass: boolean;
  failures: AssertionResult[];
  assertions: AssertionResult[];
  observations: unknown[];
}

interface RuntimeReport {
  runtime: string;
  component: string;
  results: CaseResult[];
}

const OUT_DIR = join(dirname(fileURLToPath(import.meta.url)), "web", "out");
/** The active completion cohort (working rules). Jetstream is deferred. */
const ACTIVE_RUNTIMES = ["svelte", "react", "gpui"] as const;
const DEFERRED_RUNTIMES = ["jetstream"] as const;

function loadReport(runtime: string): RuntimeReport | null {
  const path = join(OUT_DIR, `${runtime}.json`);
  if (!existsSync(path)) return null;
  return JSON.parse(readFileSync(path, "utf8")) as RuntimeReport;
}

const IDENTITY_FIELDS = [
  "present",
  "role",
  "name",
  "text",
  "icon",
  "focusable",
  "focused",
  "focusVisible",
] as const;
const GEOMETRY_FIELDS = ["height", "minWidth", "paddingLeft", "paddingRight", "radius"] as const;
const GEOMETRY_TOLERANCE = 1.0;

interface ObservationShape {
  parts?: Record<string, Record<string, unknown>>;
  trace?: unknown[];
}

/** The observing set for a field across runtimes: runtimes whose value is
 * non-null. */
function observingSet(perRuntime: Map<string, unknown>, pick: (obs: unknown) => unknown): Set<string> {
  const set = new Set<string>();
  for (const [runtime, obs] of perRuntime) {
    const value = pick(obs);
    if (value !== null && value !== undefined) set.add(runtime);
  }
  return set;
}

function fieldOf(obs: unknown, partId: string, field: string): unknown {
  return (obs as ObservationShape).parts?.[partId]?.[field];
}

function compareFrame(
  problems: string[],
  caseId: string,
  index: number,
  perRuntime: Map<string, unknown>,
  runtimesOrder: string[],
): void {
  const anyObs = perRuntime.values().next().value as ObservationShape | undefined;
  const refObs = perRuntime.get("svelte") as ObservationShape | undefined;
  const partIds = new Set<string>();
  for (const obs of perRuntime.values()) {
    for (const partId of Object.keys((obs as ObservationShape).parts ?? {})) {
      partIds.add(partId);
    }
  }
  for (const partId of partIds) {
    const present = observingSet(perRuntime, (obs) => fieldOf(obs, partId, "present"));
    for (const runtime of runtimesOrder) {
      if (!present.has(runtime)) continue;
    }
    // Identity fields: shape (observing set) and value must agree.
    for (const field of IDENTITY_FIELDS) {
      const observers = observingSet(perRuntime, (obs) => fieldOf(obs, partId, field));
      const nonObservers = runtimesOrder.filter((r) => !observers.has(r));
      if (observers.size > 0 && nonObservers.length > 0) {
        problems.push(
          `${caseId} obs ${index} ${partId}.${field}: shape mismatch — observed by [${[...observers].join(", ")}] but not by [${nonObservers.join(", ")}]`,
        );
        continue;
      }
      const reference = fieldOf(refObs, partId, field);
      for (const runtime of observers) {
        const value = fieldOf(perRuntime.get(runtime), partId, field);
        if (reference !== value) {
          problems.push(
            `${runtime} ${caseId} obs ${index} ${partId}.${field}: expected ${JSON.stringify(reference)}, got ${JSON.stringify(value)}`,
          );
        }
      }
    }
    // Maps: states and token roles.
    for (const mapField of ["states", "tokenRoles"]) {
      const keySet = new Set<string>();
      for (const obs of perRuntime.values()) {
        for (const key of Object.keys((fieldOf(obs, partId, mapField) ?? {}) as Record<string, unknown>)) {
          keySet.add(key);
        }
      }
      for (const key of keySet) {
        const observers = observingSet(perRuntime, (obs) => (fieldOf(obs, partId, mapField) as Record<string, unknown>)?.[key]);
        const nonObservers = runtimesOrder.filter((r) => !observers.has(r));
        if (observers.size > 0 && nonObservers.length > 0) {
          problems.push(
            `${caseId} obs ${index} ${partId}.${mapField}.${key}: shape mismatch — observed by [${[...observers].join(", ")}] but not by [${nonObservers.join(", ")}]`,
          );
          continue;
        }
        const reference = (fieldOf(refObs, partId, mapField) as Record<string, unknown>)?.[key];
        for (const runtime of observers) {
          const value = (fieldOf(perRuntime.get(runtime), partId, mapField) as Record<string, unknown>)?.[key];
          if (reference !== value) {
            problems.push(
              `${runtime} ${caseId} obs ${index} ${partId}.${mapField}.${key}: expected ${JSON.stringify(reference)}, got ${JSON.stringify(value)}`,
            );
          }
        }
      }
    }
    // Geometry: values with a tolerance, where the reference observes them
    // (the one documented skip — calc() geometry the headless DOM cannot
    // resolve). A planted divergence where the reference observes fails.
    for (const field of GEOMETRY_FIELDS) {
      const reference = fieldOf(refObs, partId, "geometry") as Record<string, unknown> | undefined;
      const expected = reference?.[field];
      if (expected === null || expected === undefined || typeof expected !== "number") continue;
      for (const runtime of runtimesOrder) {
        if (runtime === "svelte") continue;
        const geometry = fieldOf(perRuntime.get(runtime), partId, "geometry") as Record<string, unknown> | undefined;
        const actual = geometry?.[field];
        if (typeof actual !== "number") {
          problems.push(
            `${runtime} ${caseId} obs ${index} ${partId}.geometry.${field}: expected ${JSON.stringify(expected)}, got ${JSON.stringify(actual)}`,
          );
          continue;
        }
        if (Math.abs(expected - actual) > GEOMETRY_TOLERANCE) {
          problems.push(
            `${runtime} ${caseId} obs ${index} ${partId}.geometry.${field}: expected ${JSON.stringify(expected)}, got ${JSON.stringify(actual)}`,
          );
        }
      }
    }
  }
  // The trace (event order + payloads) must agree exactly.
  const traces = [...perRuntime.entries()].map(([runtime, obs]) => [runtime, (obs as ObservationShape).trace ?? []] as const);
  const referenceTrace = JSON.stringify((refObs as ObservationShape | undefined)?.trace ?? []);
  for (const [runtime, trace] of traces) {
    if (JSON.stringify(trace) !== referenceTrace) {
      problems.push(
        `${runtime} ${caseId} obs ${index} trace: expected ${referenceTrace}, got ${JSON.stringify(trace)}`,
      );
    }
  }
}

function main(): void {
  const available = ACTIVE_RUNTIMES.filter((runtime) => loadReport(runtime) !== null);
  if (available.length === 0) {
    console.error("no runtime reports found — run conformance:test first");
    process.exit(1);
  }

  const reports = new Map(available.map((runtime) => [runtime, loadReport(runtime)!]));
  const expectedCases = new Set<string>();
  for (const report of reports.values()) {
    for (const result of report.results) expectedCases.add(result.caseId);
  }

  const problems: string[] = [];

  // Completeness: every case in every active runtime.
  for (const [runtime, report] of reports) {
    const present = new Set(report.results.map((r) => r.caseId));
    for (const caseId of expectedCases) {
      if (!present.has(caseId)) {
        problems.push(`${runtime} did not run case ${caseId}`);
      }
    }
  }

  // Missing active runtimes are incomplete.
  for (const runtime of ACTIVE_RUNTIMES) {
    if (!reports.has(runtime)) {
      problems.push(`runtime ${runtime} is missing from the active cohort run`);
    }
  }

  // Failures name runtime/case/step/field (and why, when unobservable).
  for (const [runtime, report] of reports) {
    for (const result of report.results) {
      for (const failure of result.failures) {
        const reason = failure.reason ? ` (${failure.reason})` : "";
        problems.push(
          `${runtime} ${result.caseId} step ${failure.stepIndex} ${failure.field}${reason}: ` +
            `expected ${JSON.stringify(failure.expected)}, got ${JSON.stringify(failure.actual)}`,
        );
      }
    }
  }

  // Normalized-observation comparison across the active cohort. Identity
  // fields (present/role/name/text/icon/focusable/focused/focusVisible,
  // states, token roles, and the trace) must agree in SHAPE — which
  // runtimes observe a value — and in VALUE. A field one runtime observes
  // and another cannot is a divergence, not a gap: the label part leaking
  // root's role fails here. Geometry compares values where the reference
  // observes them (a calc() the headless DOM cannot resolve is the one
  // documented skip, and it is geometry-only). Channels are recorded, not
  // compared — web cannot resolve color-mix recipes headlessly.
  const runtimesOrder = available;
  const resultsByCase = new Map<string, Map<string, CaseResult>>();
  for (const [runtime, report] of reports) {
    for (const result of report.results) {
      if (!resultsByCase.has(result.caseId)) resultsByCase.set(result.caseId, new Map());
      resultsByCase.get(result.caseId)!.set(runtime, result);
    }
  }
  for (const [caseId, byRuntime] of resultsByCase) {
    const lengths = new Map<string, number>();
    for (const [runtime, result] of byRuntime) {
      lengths.set(runtime, result.observations.length);
    }
    const expectedLength = Math.max(...lengths.values());
    for (const [runtime, length] of lengths) {
      if (length !== expectedLength) {
        problems.push(`${runtime} ${caseId}: observation count ${length} != ${expectedLength}`);
      }
    }
    const obsByIndex = new Map<number, Map<string, unknown>>();
    for (const [runtime, result] of byRuntime) {
      result.observations.forEach((obs, index) => {
        if (!obsByIndex.has(index)) obsByIndex.set(index, new Map());
        obsByIndex.get(index)!.set(runtime, obs);
      });
    }
    for (const [index, perRuntime] of obsByIndex) {
      compareFrame(problems, caseId, index, perRuntime, runtimesOrder);
    }
  }

  if (problems.length > 0) {
    console.error(`conformance: ${problems.length} problem(s)`);
    for (const problem of problems) console.error(`  - ${problem}`);
    process.exit(1);
  }

  const caseCount = expectedCases.size;
  const deferred = DEFERRED_RUNTIMES.map((runtime) => {
    const present = loadReport(runtime) !== null;
    return `  - ${runtime}: program-deferred${present ? " (stale report present — not counted as passing)" : ""}`;
  });
  console.log(
    `conformance: ${caseCount} cases × ${available.length} active runtimes ` +
      `(${available.join(", ")}) — all passing.\n${deferred.join("\n")}`,
  );
}

main();
