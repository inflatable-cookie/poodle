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
