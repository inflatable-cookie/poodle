/** Web conformance execution for TextInput (g14.006). */

import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { describe, expect, it } from "vitest";

import {
  serializeInterface,
  textInputCases,
  textInputInterface,
} from "../../../packages/core/src/conformance";
import { ReactTextInputAdapter } from "./react-text-input-adapter";
import { runCase, summarize, type RuntimeAdapter } from "./runner";
import { SvelteTextInputAdapter } from "./svelte-text-input-adapter";

const OUT_DIR = `${import.meta.dirname}/out`;
const iface = serializeInterface(textInputInterface);
const tokensCss = readFileSync(
  `${import.meta.dirname}/../../../packages/core/src/tokens/generated/css/poodle-tokens.css`,
  "utf8",
);
const textInputCss = readFileSync(
  `${import.meta.dirname}/../../../packages/core/src/styles/text-input.css`,
  "utf8",
);

async function collectResults(adapter: RuntimeAdapter): Promise<ReturnType<typeof summarize>> {
  const perCase: { caseId: string; results: never[]; observations: never[] }[] = [];
  for (const caseData of textInputCases.cases) {
    const { results, observations } = await runCase(
      adapter,
      iface,
      textInputCases.component,
      caseData,
    );
    perCase.push({ caseId: caseData.id, results, observations });
  }
  return summarize(adapter.runtime, textInputCases.component, perCase);
}

describe("text-input conformance (web)", () => {
  it("runs the shared corpus against Svelte and React", async () => {
    const style = document.createElement("style");
    style.textContent = [tokensCss, textInputCss].join("\n");
    document.head.appendChild(style);

    for (const adapter of [new SvelteTextInputAdapter(), new ReactTextInputAdapter()]) {
      const report = await collectResults(adapter);
      mkdirSync(OUT_DIR, { recursive: true });
      writeFileSync(`${OUT_DIR}/${report.runtime}-text-input.json`, JSON.stringify(report, null, 2));
      const failed = report.results.filter((result) => !result.pass);
      expect(failed, `${report.runtime}: ${JSON.stringify(failed, null, 2)}`).toEqual([]);
    }
  });
});
