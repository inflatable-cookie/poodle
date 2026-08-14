/** Web conformance execution for Tabs (g14.004). */

import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { describe, expect, it } from "vitest";

import { serializeInterface, tabsCases, tabsInterface } from "../../../packages/core/src/conformance";
import { ReactTabsAdapter } from "./react-tabs-adapter";
import { runCase, summarize, type RuntimeAdapter } from "./runner";
import { SvelteTabsAdapter } from "./svelte-tabs-adapter";

const OUT_DIR = `${import.meta.dirname}/out`;
const iface = serializeInterface(tabsInterface);
const tokensCss = readFileSync(
  `${import.meta.dirname}/../../../packages/core/src/tokens/generated/css/poodle-tokens.css`,
  "utf8",
);
const tabsCss = readFileSync(
  `${import.meta.dirname}/../../../packages/core/src/styles/tabs.css`,
  "utf8",
);

async function collectResults(adapter: RuntimeAdapter): Promise<ReturnType<typeof summarize>> {
  const perCase: { caseId: string; results: never[]; observations: never[] }[] = [];
  for (const caseData of tabsCases.cases) {
    const { results, observations } = await runCase(adapter, iface, tabsCases.component, caseData);
    perCase.push({ caseId: caseData.id, results, observations });
  }
  return summarize(adapter.runtime, tabsCases.component, perCase);
}

describe("tabs conformance (web)", () => {
  it("runs the shared corpus against Svelte and React", async () => {
    const style = document.createElement("style");
    style.textContent = `${tokensCss}\n${tabsCss}`;
    document.head.appendChild(style);

    for (const adapter of [new SvelteTabsAdapter(), new ReactTabsAdapter()]) {
      const report = await collectResults(adapter);
      mkdirSync(OUT_DIR, { recursive: true });
      writeFileSync(`${OUT_DIR}/${report.runtime}-tabs.json`, JSON.stringify(report, null, 2));
      const failed = report.results.filter((result) => !result.pass);
      expect(failed, `${report.runtime}: ${JSON.stringify(failed, null, 2)}`).toEqual([]);
    }
  });
});
