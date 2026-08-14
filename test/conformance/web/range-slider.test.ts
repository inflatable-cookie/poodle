/**
 * Web conformance execution for RangeSlider (g14.003).
 */

import { describe, expect, it } from "vitest";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";

import {
  rangeSliderCases,
  rangeSliderInterface,
  serializeInterface,
} from "../../../packages/core/src/conformance";

import { runCase, summarize, type RuntimeAdapter } from "./runner";
import { SvelteRangeSliderAdapter } from "./svelte-range-slider-adapter";
import { ReactRangeSliderAdapter } from "./react-range-slider-adapter";

const OUT_DIR = `${import.meta.dirname}/out`;
const iface = serializeInterface(rangeSliderInterface);

const tokensCss = readFileSync(
  `${import.meta.dirname}/../../../packages/core/src/tokens/generated/css/poodle-tokens.css`,
  "utf8",
);
const rangeSliderCss = readFileSync(
  `${import.meta.dirname}/../../../packages/core/src/styles/range-slider.css`,
  "utf8",
);

function injectRealCss(): void {
  const style = document.createElement("style");
  style.id = "conformance-web-css-range-slider";
  style.textContent = `${tokensCss}\n${rangeSliderCss}`;
  document.head.appendChild(style);
}

async function collectResults(adapter: RuntimeAdapter): Promise<ReturnType<typeof summarize>> {
  const perCase: { caseId: string; results: never[]; observations: never[] }[] = [];
  for (const caseData of rangeSliderCases.cases) {
    const { results, observations } = await runCase(
      adapter,
      iface,
      rangeSliderCases.component,
      caseData,
    );
    perCase.push({ caseId: caseData.id, results, observations });
  }
  return summarize(adapter.runtime, rangeSliderCases.component, perCase);
}

function writeReport(report: ReturnType<typeof summarize>): void {
  mkdirSync(OUT_DIR, { recursive: true });
  writeFileSync(`${OUT_DIR}/${report.runtime}-range-slider.json`, JSON.stringify(report, null, 2));
}

describe("range-slider conformance (web)", () => {
  it("runs the corpus against Svelte and React", async () => {
    injectRealCss();

    const adapters: RuntimeAdapter[] = [
      new SvelteRangeSliderAdapter(),
      new ReactRangeSliderAdapter(),
    ];
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
