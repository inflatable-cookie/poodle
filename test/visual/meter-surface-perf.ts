import { chromium, webkit, type BrowserType } from "playwright";

import { specimenUrl } from "./capture";
import { SERVERS, type Framework } from "./config";
import { startPreviews } from "./server";

/**
 * Reference performance run for g14.024:
 *
 *   effigy test:meter-surface-perf
 *
 * Drives the 128-meter specimen workload (12 segments, 15 Hz data, 60 Hz
 * paint, culling on) headlessly in Chromium and WebKit for both previews and
 * prints each specimen's own readout: warm-up, sample count, mean, p50, p95,
 * max. The numbers are review evidence for the implementation log, not a CI
 * threshold — the specimen measures bus advance + draw-pass assembly + paint
 * inside its wrapped frame scheduler.
 */

const AXIS = { theme: "iceberg", density: "default", controlSize: "md" } as const;
const MEASURE_MS = 20_000;

async function measure(browserType: BrowserType, browserName: string, framework: Framework, base: string): Promise<void> {
  const browser = await browserType.launch();
  try {
    const context = await browser.newContext({ viewport: { width: 1280, height: 900 } });
    const page = await context.newPage();
    await page.goto(specimenUrl(base, "meter-surface", AXIS as never), { waitUntil: "domcontentloaded" });
    await page.waitForSelector(".poodle-meter-surface__canvas");
    await page.click('button[data-count="128"]');
    await page.waitForTimeout(400);
    await page.click('button[data-part="workload-toggle"]');
    await page.waitForTimeout(MEASURE_MS);
    await page.click('button[data-part="workload-toggle"]');
    await page.waitForTimeout(200);
    const readout = await page.evaluate(() => document.querySelector('[data-part="perf-readout"]')?.textContent ?? "missing readout");
    console.log(`${browserName} / ${framework}: ${readout}`);
  } finally {
    await browser.close();
  }
}

async function main(): Promise<void> {
  await startPreviews();
  const bases: Record<Framework, string> = {
    svelte: `http://127.0.0.1:${SERVERS.svelte.port}`,
    react: `http://127.0.0.1:${SERVERS.react.port}`,
  };
  for (const [browserName, browserType] of [["chromium", chromium], ["webkit", webkit]] as Array<[string, BrowserType]>) {
    for (const framework of ["svelte", "react"] as Framework[]) {
      await measure(browserType, browserName, framework, bases[framework]);
    }
  }
  process.exit(0);
}

await main();
