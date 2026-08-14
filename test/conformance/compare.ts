/**
 * Conformance orchestrator (spec 066, g14.001): reads the per-runtime reports
 * (`svelte.json`, `react.json` from the web vitest run; `gpui.json`,
 * `jetstream.json` from the native cargo runners) and applies the
 * cross-runtime rules:
 *
 *   - every case must have run in every runtime present in the run;
 *   - no assertion may fail in any runtime (failures name
 *     runtime / case / step / field);
 *   - no assertion may be vacuous in every runtime (an assertion no runtime
 *     can exercise is not evidence — coverage must be real somewhere).
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
  verdict: "pass" | "fail" | "vacuous";
  expected?: unknown;
  actual?: unknown;
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
const RUNTIMES = ["svelte", "react", "gpui", "jetstream"] as const;

function loadReport(runtime: string): RuntimeReport | null {
  const path = join(OUT_DIR, `${runtime}.json`);
  if (!existsSync(path)) return null;
  return JSON.parse(readFileSync(path, "utf8")) as RuntimeReport;
}

function main(): void {
  const available = RUNTIMES.filter((runtime) => loadReport(runtime) !== null);
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

  // Completeness: every case in every available runtime.
  for (const [runtime, report] of reports) {
    const present = new Set(report.results.map((r) => r.caseId));
    for (const caseId of expectedCases) {
      if (!present.has(caseId)) {
        problems.push(`${runtime} did not run case ${caseId}`);
      }
    }
  }

  // Failures name runtime/case/step/field.
  for (const [runtime, report] of reports) {
    for (const result of report.results) {
      for (const failure of result.failures) {
        problems.push(
          `${runtime} ${result.caseId} step ${failure.stepIndex} ${failure.field}: ` +
            `expected ${JSON.stringify(failure.expected)}, got ${JSON.stringify(failure.actual)}`,
        );
      }
    }
  }

  // Vacuity: each (case, step, field) assertion must be non-vacuous in at
  // least one runtime.
  const vacuousByKey = new Map<string, string[]>();
  const exercisedByKey = new Set<string>();
  for (const [runtime, report] of reports) {
    for (const result of report.results) {
      for (const assertion of result.assertions ?? []) {
        const key = `${result.caseId}|${assertion.stepIndex}|${assertion.field}`;
        if (assertion.verdict === "vacuous") {
          vacuousByKey.set(key, [...(vacuousByKey.get(key) ?? []), runtime]);
        } else {
          exercisedByKey.add(key);
        }
      }
    }
  }
  for (const [key, runtimes] of vacuousByKey) {
    if (!exercisedByKey.has(key)) {
      const [caseId, step, field] = key.split("|");
      problems.push(
        `${caseId} step ${step} ${field}: assertion not exercised by any runtime ` +
          `(vacuous in ${runtimes.join(", ")})`,
      );
    }
  }

  if (problems.length > 0) {
    console.error(`conformance: ${problems.length} problem(s)`);
    for (const problem of problems) console.error(`  - ${problem}`);
    process.exit(1);
  }

  const caseCount = expectedCases.size;
  console.log(
    `conformance: ${caseCount} cases × ${available.length} runtimes (${available.join(", ")}) — all passing, no vacuous-only assertions.`,
  );
}

main();
