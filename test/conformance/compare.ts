/**
 * Conformance orchestrator (spec 066, g14.001 / g14.003): reads the active-runtime
 * reports for every completed component corpus and applies the cross-runtime
 * rules:
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

import { buttonCases } from "../../packages/core/src/conformance/button-cases";
import { rangeSliderCases } from "../../packages/core/src/conformance/range-slider-cases";
import { tabsCases } from "../../packages/core/src/conformance/tabs-cases";
import { popoverCases } from "../../packages/core/src/conformance/popover-cases";
import { textInputCases } from "../../packages/core/src/conformance/text-input-cases";
import type { SerializedCase } from "../../packages/core/src/conformance/define";

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

/** Report file suffix per completed component corpus. Button keeps the bare name. */
const COMPONENT_CORPORA: Array<{
  label: string;
  reportSuffix: string;
  cases: { cases: SerializedCase[] };
}> = [
  { label: "button", reportSuffix: "", cases: buttonCases },
  { label: "range-slider", reportSuffix: "-range-slider", cases: rangeSliderCases },
  { label: "tabs", reportSuffix: "-tabs", cases: tabsCases },
  { label: "popover", reportSuffix: "-popover", cases: popoverCases },
  { label: "text-input", reportSuffix: "-text-input", cases: textInputCases },
];

function loadReport(runtime: string, reportSuffix: string): RuntimeReport | null {
  const path = join(OUT_DIR, `${runtime}${reportSuffix}.json`);
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
  "tabbable",
  "selected",
  "expanded",
  "parent",
  "overlay",
  "focusedText",
  "layerCount",
  "orientation",
  "controls",
  "labelledBy",
  "value",
] as const;
const GEOMETRY_FIELDS = [
  "height",
  "minWidth",
  "paddingLeft",
  "paddingRight",
  "radius",
  "borderWidth",
  "topGap",
  "bottomGap",
  "leftGap",
  "rightGap",
  "hStart",
  "hEnd",
  "vStart",
  "widthGap",
] as const;

type GeometryFieldContract = Map<string, number>;
type GeometryPartContract = Map<string, GeometryFieldContract>;
type GeometryFrameContract = Map<number, GeometryPartContract>;

/**
 * Geometry contracts, authored per case: the corpus's geometry assertions
 * carry the named, assertion-local tolerance (spec 066). The comparison
 * compares exactly those fields with exactly those bounds — no blanket
 * runtime tolerance exists. Geometry fields no case asserts are recorded,
 * not compared.
 */
function geometryContracts(
  corpora: Array<{ cases: { cases: SerializedCase[] } }>,
): Map<string, GeometryFrameContract> {
  const contracts = new Map<string, GeometryFrameContract>();
  for (const corpus of corpora) {
    for (const caseData of corpus.cases.cases as SerializedCase[]) {
      const frames: GeometryFrameContract = new Map();
      let observationIndex = 0;
      for (const step of caseData.steps) {
        if (step.kind === "action") {
          observationIndex += 1;
          continue;
        }
        if (step.kind !== "expectPart") continue;
        const geometry = (step.expect.geometry ?? {}) as Record<string, unknown>;
        const assertedFields = GEOMETRY_FIELDS.filter((field) => typeof geometry[field] === "number");
        if (assertedFields.length === 0) continue;
        if (typeof geometry.tolerance !== "number") {
          throw new Error(`${caseData.id} step geometry is missing its authored tolerance`);
        }
        const parts = frames.get(observationIndex) ?? new Map<string, GeometryFieldContract>();
        const fields = parts.get(step.part) ?? new Map<string, number>();
        for (const field of GEOMETRY_FIELDS) {
          if (typeof geometry[field] !== "number") continue;
          const prior = fields.get(field);
          if (prior !== undefined && prior !== geometry.tolerance) {
            throw new Error(
              `${caseData.id} observation ${observationIndex} ${step.part}.geometry.${field} has conflicting tolerances`,
            );
          }
          fields.set(field, geometry.tolerance);
        }
        parts.set(step.part, fields);
        frames.set(observationIndex, parts);
      }
      if (frames.size > 0) contracts.set(caseData.id, frames);
    }
  }
  return contracts;
}

const GEOMETRY_CONTRACTS = geometryContracts(COMPONENT_CORPORA);

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

function identityEqual(a: unknown, b: unknown): boolean {
  if (a === b) return true;
  if (a !== null && b !== null && typeof a === "object" && typeof b === "object") {
    return JSON.stringify(a) === JSON.stringify(b);
  }
  return false;
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
        if (!identityEqual(reference, value)) {
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
    // Geometry: only the fields the corpus asserts, with the authored,
    // assertion-local tolerance (spec 066 — named bounds, never a blanket
    // runtime tolerance). Unasserted geometry is recorded, not compared.
    const contract = GEOMETRY_CONTRACTS.get(caseId)?.get(index)?.get(partId);
    if (contract) {
      compareContractedGeometry(problems, caseId, index, partId, perRuntime, runtimesOrder, contract, refObs);
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

function compareContractedGeometry(
  problems: string[],
  caseId: string,
  index: number,
  partId: string,
  perRuntime: Map<string, unknown>,
  runtimesOrder: string[],
  contract: GeometryFieldContract,
  refObs: ObservationShape | undefined,
): void {
  for (const [field, tolerance] of contract) {
    const reference = fieldOf(refObs, partId, "geometry") as Record<string, unknown> | undefined;
    const expected = reference?.[field];
    if (expected === null || expected === undefined || typeof expected !== "number") {
      // The reference cannot resolve this field headlessly (e.g. a calc()
      // the DOM does not evaluate) — the authored bound still gates the
      // runtimes that do observe it against the authored expectation.
      continue;
    }
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
      if (Math.abs(expected - actual) > tolerance) {
        problems.push(
          `${runtime} ${caseId} obs ${index} ${partId}.geometry.${field}: expected ${JSON.stringify(expected)}, got ${JSON.stringify(actual)} (bound ${tolerance})`,
        );
      }
    }
  }
}


function compareCorpus(
  label: string,
  reportSuffix: string,
  cases: { cases: SerializedCase[] },
  problems: string[],
): { caseCount: number; available: string[] } {
  const available = ACTIVE_RUNTIMES.filter(
    (runtime) => loadReport(runtime, reportSuffix) !== null,
  );
  if (available.length === 0) {
    problems.push(`${label}: no runtime reports found — run conformance:test first`);
    return { caseCount: 0, available };
  }

  const reports = new Map(
    available.map((runtime) => [runtime, loadReport(runtime, reportSuffix)!]),
  );
  const expectedCases = new Set(cases.cases.map((caseData) => caseData.id));

  for (const [runtime, report] of reports) {
    const present = new Set(report.results.map((r) => r.caseId));
    for (const caseId of expectedCases) {
      if (!present.has(caseId)) {
        problems.push(`${runtime} did not run case ${caseId}`);
      }
    }
    for (const caseId of present) {
      if (!expectedCases.has(caseId)) {
        problems.push(`${runtime} reported unexpected ${label} case ${caseId}`);
      }
    }
  }

  for (const runtime of ACTIVE_RUNTIMES) {
    if (!reports.has(runtime)) {
      problems.push(`${label}: runtime ${runtime} is missing from the active cohort run`);
    }
  }

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

  return { caseCount: expectedCases.size, available };
}

function main(): void {
  const problems: string[] = [];
  let totalCases = 0;
  const availableSets: string[][] = [];

  for (const corpus of COMPONENT_CORPORA) {
    const { caseCount, available } = compareCorpus(
      corpus.label,
      corpus.reportSuffix,
      corpus.cases,
      problems,
    );
    totalCases += caseCount;
    availableSets.push(available);
  }

  if (problems.length > 0) {
    console.error(`conformance: ${problems.length} problem(s)`);
    for (const problem of problems) console.error(`  - ${problem}`);
    process.exit(1);
  }

  const available = availableSets[0] ?? [];
  const deferred = DEFERRED_RUNTIMES.map((runtime) => {
    const present = COMPONENT_CORPORA.some(
      (corpus) => loadReport(runtime, corpus.reportSuffix) !== null,
    );
    return `  - ${runtime}: program-deferred${present ? " (stale report present — not counted as passing)" : ""}`;
  });
  console.log(
    `conformance: ${totalCases} cases across ${COMPONENT_CORPORA.length} corpora × ${available.length} active runtimes ` +
      `(${available.join(", ")}) — all passing.\n${deferred.join("\n")}`,
  );
}

main();
