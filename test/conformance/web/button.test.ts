/**
 * Web conformance execution (spec 066): runs the Button corpus against the
 * real Svelte and React implementations in happy-dom, asserts every case,
 * and writes the per-runtime observation reports consumed by
 * `conformance:compare`. The assertions here are the Svelte<->React drift
 * gate; native execution happens in the cargo runners and is compared by the
 * orchestrator.
 */

import { describe, expect, it } from "vitest";
import { mkdirSync, writeFileSync } from "node:fs";


import tokensCss from "../../../packages/core/src/tokens/generated/css/poodle-tokens.css?raw";
import buttonCss from "../../../packages/core/src/styles/button.css?raw";
import { buttonCases } from "../../../packages/core/src/conformance";

import { runCase, summarize, type RuntimeAdapter } from "./runner";
import { SvelteButtonAdapter } from "./svelte-adapter";
import { ReactButtonAdapter } from "./react-adapter";

const OUT_DIR = `${import.meta.dirname}/out`;

function injectRealCss(): void {
  const style = document.createElement("style");
  style.id = "conformance-web-css";
  style.textContent = `${tokensCss}\n${buttonCss}`;
  document.head.appendChild(style);
}

async function collectResults(adapter: RuntimeAdapter): Promise<ReturnType<typeof summarize>> {
  const perCase: { caseId: string; results: never[]; observations: never[] }[] = [];
  for (const caseData of buttonCases.cases) {
    const { results, observations } = await runCase(adapter, caseData);
    perCase.push({ caseId: caseData.id, results, observations });
  }
  return summarize(adapter.runtime, buttonCases.component, perCase);
}

function writeReport(report: ReturnType<typeof summarize>): void {
  const dir = OUT_DIR;
  mkdirSync(dir, { recursive: true });
  writeFileSync(`${OUT_DIR}/${report.runtime}.json`, JSON.stringify(report, null, 2));
}

describe("button conformance (web)", () => {
  it("runs the corpus against Svelte and React", async () => {
    injectRealCss();

    const adapters: RuntimeAdapter[] = [new SvelteButtonAdapter(), new ReactButtonAdapter()];
    const reports = [];
    for (const adapter of adapters) {
      reports.push(await collectResults(adapter));
    }

    for (const report of reports) {
      writeReport(report);
      const failed = report.results.filter((result) => !result.pass);
      const detail = failed.map((f) => ({ caseId: f.caseId, failures: f.failures }));
      expect(
        failed,
        `${report.runtime}: ${failed.length} failing case(s) — ${JSON.stringify(detail, null, 2)}`,
      ).toEqual([]);
    }
  });
});
