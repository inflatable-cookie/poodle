/** Web conformance execution for Tabs (g14.004). */

import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { act, createElement } from "react";
import { createRoot } from "react-dom/client";
import { mount, unmount } from "svelte";

import { serializeInterface, tabsCases, tabsInterface } from "../../../packages/core/src/conformance";
import { TabsSpecimen as ReactTabsSpecimen } from "../../../packages/react/preview/src/gallery/specimens/TabsSpecimen";
import SvelteTabsSpecimen from "../../../packages/svelte/preview/src/specimens/TabsSpecimen.svelte";
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
  it("projected specimens commit a changed selection", async () => {
    const assertSelectionChanges = async (container: HTMLElement) => {
      const trigger = container.querySelector<HTMLElement>('[role="tab"][data-value="billing"]');
      expect(trigger).not.toBeNull();
      trigger!.click();
      await Promise.resolve();
      expect(trigger!.getAttribute("aria-selected")).toBe("true");
    };

    const reactContainer = document.createElement("div");
    document.body.appendChild(reactContainer);
    const root = createRoot(reactContainer);
    await act(async () => root.render(createElement(ReactTabsSpecimen)));
    await act(async () => assertSelectionChanges(reactContainer));
    await act(async () => root.unmount());
    reactContainer.remove();

    const svelteContainer = document.createElement("div");
    document.body.appendChild(svelteContainer);
    const component = mount(SvelteTabsSpecimen, { target: svelteContainer });
    await assertSelectionChanges(svelteContainer);
    unmount(component);
    svelteContainer.remove();
  });

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
