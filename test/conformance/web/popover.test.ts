/** Web conformance execution for Popover (g14.005). */

import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { describe, expect, it } from "vitest";

import {
  popoverCases,
  popoverInterface,
  serializeInterface,
} from "../../../packages/core/src/conformance";
import { installLayoutStub } from "./layout-stub";
import { ReactPopoverAdapter } from "./react-popover-adapter";
import { runCase, summarize, type RuntimeAdapter } from "./runner";
import { SveltePopoverAdapter } from "./svelte-popover-adapter";

const OUT_DIR = `${import.meta.dirname}/out`;
const iface = serializeInterface(popoverInterface);
const tokensCss = readFileSync(
  `${import.meta.dirname}/../../../packages/core/src/tokens/generated/css/poodle-tokens.css`,
  "utf8",
);
const popoverCss = readFileSync(
  `${import.meta.dirname}/../../../packages/core/src/styles/popover.css`,
  "utf8",
);
const anchoredCss = readFileSync(
  `${import.meta.dirname}/../../../packages/core/src/styles/anchored-surface.css`,
  "utf8",
);

async function collectResults(adapter: RuntimeAdapter): Promise<ReturnType<typeof summarize>> {
  const perCase: { caseId: string; results: never[]; observations: never[] }[] = [];
  for (const caseData of popoverCases.cases) {
    const { results, observations } = await runCase(adapter, iface, popoverCases.component, caseData);
    perCase.push({ caseId: caseData.id, results, observations });
  }
  return summarize(adapter.runtime, popoverCases.component, perCase);
}

describe("popover conformance (web)", () => {
  it("runs the shared corpus against Svelte and React", async () => {
    installLayoutStub();
    const style = document.createElement("style");
    style.textContent = [
      tokensCss,
      popoverCss,
      anchoredCss,
      // happy-dom implements no layout, so the harness supplies the anchor
      // box a real browser would give the component root and its trigger
      // (see layout-stub); the interior origin keeps every placement family
      // collision-free, and the trigger's box is the surface's positioning
      // reference (the interface's relativeTo anchor).
      ".poodle-popover { position: absolute !important; top: 40px; left: 24px; width: 96px; height: 32px; }",
      ".poodle-popover__trigger { position: absolute !important; top: 40px; left: 24px; width: 96px; height: 32px; }",
    ].join("\n");
    document.head.appendChild(style);

    for (const adapter of [new SveltePopoverAdapter(), new ReactPopoverAdapter()]) {
      const report = await collectResults(adapter);
      mkdirSync(OUT_DIR, { recursive: true });
      writeFileSync(`${OUT_DIR}/${report.runtime}-popover.json`, JSON.stringify(report, null, 2));
      const failed = report.results.filter((result) => !result.pass);
      expect(failed, `${report.runtime}: ${JSON.stringify(failed, null, 2)}`).toEqual([]);
    }
  });
});
