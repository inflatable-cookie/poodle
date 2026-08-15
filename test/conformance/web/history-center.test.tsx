/** Web conformance execution for HistoryCenter (g14.007). */

import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { describe, expect, it } from "vitest";

import {
  historyCenterCases,
  historyCenterInterface,
  serializeInterface,
} from "../../../packages/core/src/conformance";
import { runCase, summarize, type RuntimeAdapter } from "./runner";
import { ReactHistoryCenterAdapter } from "./react-history-center-adapter";
import { SvelteHistoryCenterAdapter } from "./svelte-history-center-adapter";

const OUT_DIR = `${import.meta.dirname}/out`;
const iface = serializeInterface(historyCenterInterface);
const css = ["tokens/generated/css/poodle-tokens.css", "styles/history-center.css"]
  .map((path) => readFileSync(`${import.meta.dirname}/../../../packages/core/src/${path}`, "utf8"))
  .join("\n");

async function collectResults(adapter: RuntimeAdapter): Promise<ReturnType<typeof summarize>> {
  const perCase: { caseId: string; results: never[]; observations: never[] }[] = [];
  for (const caseData of historyCenterCases.cases) {
    const { results, observations } = await runCase(
      adapter,
      iface,
      historyCenterCases.component,
      caseData,
    );
    perCase.push({ caseId: caseData.id, results, observations });
  }
  return summarize(adapter.runtime, historyCenterCases.component, perCase);
}

describe("history-center conformance (web)", () => {
  it("runs the shared corpus against Svelte and React", async () => {
    const style = document.createElement("style");
    style.textContent = css;
    document.head.appendChild(style);

    for (const adapter of [new SvelteHistoryCenterAdapter(), new ReactHistoryCenterAdapter()]) {
      const report = await collectResults(adapter);
      mkdirSync(OUT_DIR, { recursive: true });
      writeFileSync(
        `${OUT_DIR}/${report.runtime}-history-center.json`,
        JSON.stringify(report, null, 2),
      );
      const failed = report.results.filter((result) => !result.pass);
      expect(failed.length, `${report.runtime}: ${failed.map((f) => f.caseId).join(", ")}`).toBe(0);
    }
  });
});
