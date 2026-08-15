import { chromium, webkit, type Browser, type BrowserType, type Page } from "playwright";

import { specimenUrl } from "./capture";
import { SERVERS, type Framework } from "./config";
import { startPreviews } from "./server";

/**
 * Real-browser behavior matrix for the batched meter surface (g14.024):
 *
 *   effigy test:meter-surface-browser
 *
 * Headless Chromium and WebKit both drive the Svelte and React previews and
 * assert what happy-dom cannot see: canvas/placeholder geometry, scroll
 * alignment after repeated scroll, resize and DPR backing-store sizing, live
 * theme palette change, add/remove registration, clip latch/reset, culled
 * ballistics continuing, and destroy/remount. Paint truth is read back from
 * the overlay canvas itself via getImageData.
 */

const AXIS = { theme: "iceberg", density: "default", controlSize: "md" } as const;

let failures = 0;

function check(label: string, ok: boolean, detail = ""): void {
  if (ok) {
    console.log(`  ok   ${label}`);
  } else {
    failures += 1;
    console.error(`  FAIL ${label}${detail ? ` — ${detail}` : ""}`);
  }
}

async function frames(page: Page, count = 3): Promise<void> {
  await Promise.race([
    page.evaluate(async (n) => {
      for (let i = 0; i < n; i += 1) {
        await new Promise((resolve) => requestAnimationFrame(() => resolve(null)));
      }
    }, count),
    new Promise((_, reject) => setTimeout(() => reject(new Error("frames(): rAF did not tick within 15s — page suspended?")), 15_000)),
  ]);
}

interface Pixel { r: number; g: number; b: number; a: number }

/**
 * Sample one canvas pixel at an offset within a meter root's viewport rect.
 * Negative `fx` measures from the meter's right edge — the clip lamp sits at
 * the top-right of a vertical meter and its off color is the theme-dependent
 * surface fill, which makes it the reference pixel for theme and clip checks.
 */
async function samplePixel(page: Page, meterIndex: number, fx: number, fy: number): Promise<Pixel | null> {
  return page.evaluate(([index, offsetX, offsetY]) => {
    const canvas = document.querySelector<HTMLCanvasElement>(".poodle-meter-surface__canvas");
    const meters = document.querySelectorAll<HTMLElement>(".poodle-audio-meter[data-surface]");
    const meter = meters[index as number];
    if (canvas === null || meter === undefined) return null;
    const context = canvas.getContext("2d");
    if (context === null) return null;
    const canvasRect = canvas.getBoundingClientRect();
    const meterRect = meter.getBoundingClientRect();
    const dpr = devicePixelRatio;
    const baseX = (offsetX as number) < 0 ? meterRect.right - canvasRect.left : meterRect.left - canvasRect.left;
    const x = Math.round((baseX + (offsetX as number)) * dpr);
    const y = Math.round((meterRect.top - canvasRect.top + (offsetY as number)) * dpr);
    if (x < 0 || y < 0 || x >= canvas.width || y >= canvas.height) return null;
    const data = context.getImageData(x, y, 1, 1).data;
    return { r: data[0]!, g: data[1]!, b: data[2]!, a: data[3]! };
  }, [meterIndex, fx, fy] as const);
}

function pixelsDiffer(left: Pixel | null, right: Pixel | null): boolean {
  if (left === null || right === null) return false;
  return Math.abs(left.r - right.r) + Math.abs(left.g - right.g) + Math.abs(left.b - right.b) + Math.abs(left.a - right.a) > 12;
}

async function runMatrix(page: Page, base: string, framework: Framework, browserName: string): Promise<void> {
  console.log(`\n${browserName} / ${framework}`);
  await page.goto(specimenUrl(base, "meter-surface", AXIS as never), { waitUntil: "domcontentloaded" });
  await page.waitForSelector(".poodle-meter-surface__canvas");
  await frames(page, 4);

  // Canvas and placeholder geometry.
  const geometry = await page.evaluate(() => {
    const root = document.querySelector(".poodle-meter-surface")!;
    const canvas = document.querySelector<HTMLCanvasElement>(".poodle-meter-surface__canvas")!;
    const viewport = document.querySelector<HTMLElement>(".poodle-meter-surface__viewport")!;
    const rootRect = root.getBoundingClientRect();
    const canvasRect = canvas.getBoundingClientRect();
    return {
      overlayAligned: Math.abs(rootRect.left - canvasRect.left) < 1 && Math.abs(rootRect.top - canvasRect.top) < 1,
      backingWidth: canvas.width,
      expectedWidth: Math.max(Math.round(viewport.clientWidth * devicePixelRatio), 1),
      ariaHidden: canvas.getAttribute("aria-hidden") === "true",
      visuals: document.querySelectorAll(".poodle-audio-meter-visual").length,
      canvases: document.querySelectorAll(".poodle-meter-surface canvas").length,
    };
  });
  check("overlay canvas aligned to surface root", geometry.overlayAligned);
  check("backing store sized for DPR", geometry.backingWidth === geometry.expectedWidth, `${geometry.backingWidth} != ${geometry.expectedWidth}`);
  check("canvas accessibility-hidden", geometry.ariaHidden);
  check("no visual DOM under surface placeholders", geometry.visuals === 0);
  check("exactly one canvas", geometry.canvases === 1);

  // 128-meter structure: lightweight roots, no segment DOM.
  await page.click('button[data-count="128"]');
  await frames(page, 6);
  const structure = await page.evaluate(() => ({
    roots: document.querySelectorAll(".poodle-audio-meter[data-surface]").length,
    visuals: document.querySelectorAll(".poodle-audio-meter-visual").length,
    segments: document.querySelectorAll(".poodle-audio-meter-visual__segment").length,
    canvases: document.querySelectorAll(".poodle-meter-surface canvas").length,
  }));
  check("128-meter scene renders 132 lightweight roots", structure.roots === 132, String(structure.roots));
  check("no segment DOM at 128 meters", structure.visuals === 0 && structure.segments === 0);
  check("still one canvas at 128 meters", structure.canvases === 1);

  // Scroll alignment after repeated scroll: a pixel inside the first visible
  // meter's track paints, a pixel in the inter-meter gap stays transparent.
  let aligned = true;
  let gapClear = true;
  for (let step = 0; step < 6; step += 1) {
    await page.evaluate((offset) => {
      const viewport = document.querySelector<HTMLElement>(".poodle-meter-surface__viewport")!;
      viewport.scrollLeft = offset;
    }, step * 137);
    await frames(page, 3);
    const probe = await page.evaluate(() => {
      const canvas = document.querySelector<HTMLCanvasElement>(".poodle-meter-surface__canvas")!;
      const context = canvas.getContext("2d")!;
      const canvasRect = canvas.getBoundingClientRect();
      const meters = [...document.querySelectorAll<HTMLElement>(".poodle-audio-meter[data-surface]")];
      const visible = meters.find((meter) => {
        const rect = meter.getBoundingClientRect();
        return rect.left >= canvasRect.left + 2 && rect.right <= canvasRect.right - 2 && rect.height > 0;
      });
      if (visible === undefined) return null;
      const rect = visible.getBoundingClientRect();
      const dpr = devicePixelRatio;
      const sample = (clientX: number, clientY: number) => {
        const x = Math.round((clientX - canvasRect.left) * dpr);
        const y = Math.round((clientY - canvasRect.top) * dpr);
        return context.getImageData(x, y, 1, 1).data[3]!;
      };
      return {
        track: sample(rect.left + 4, rect.top + rect.height * 0.55),
        gap: sample(rect.left - 5, rect.top + rect.height * 0.55),
      };
    });
    if (probe === null) continue;
    aligned = aligned && probe.track > 0;
    gapClear = gapClear && probe.gap === 0;
  }
  check("track pixels painted at placeholder positions across scrolls", aligned);
  check("gap pixels stay unpainted across scrolls", gapClear);

  // Culled ballistics + aria continue: run the workload, scroll the first
  // meter offscreen, and require its aria value to keep moving.
  await page.click('button[data-part="workload-toggle"]');
  await page.evaluate(() => {
    const viewport = document.querySelector<HTMLElement>(".poodle-meter-surface__viewport")!;
    viewport.scrollLeft = viewport.scrollWidth;
  });
  await frames(page, 3);
  const ariaBefore = await page.evaluate(() => document.querySelector(".poodle-audio-meter[data-surface]")!.getAttribute("aria-valuenow"));
  await page.waitForTimeout(700);
  const ariaAfter = await page.evaluate(() => document.querySelector(".poodle-audio-meter[data-surface]")!.getAttribute("aria-valuenow"));
  check("culled meter aria keeps sampling from live ballistics", ariaBefore !== ariaAfter, `${ariaBefore} -> ${ariaAfter}`);
  await page.click('button[data-part="workload-toggle"]');
  await page.evaluate(() => {
    const viewport = document.querySelector<HTMLElement>(".poodle-meter-surface__viewport")!;
    viewport.scrollLeft = 0;
  });
  await frames(page, 4);

  // Live theme palette change repaints without remount (reference pixel: the
  // clip lamp's off color, i.e. the theme surface fill).
  const beforeTheme = await samplePixel(page, 0, -6, 1);
  await page.click('button[data-part="theme-toggle"]');
  await frames(page, 6);
  const afterTheme = await samplePixel(page, 0, -6, 1);
  const remountedCanvas = await page.evaluate(() => document.querySelectorAll(".poodle-meter-surface canvas").length);
  check("theme change replaces painted colors", pixelsDiffer(beforeTheme, afterTheme), JSON.stringify({ beforeTheme, afterTheme }));
  check("theme change keeps the same single canvas", remountedCanvas === 1);
  await page.click('button[data-part="theme-toggle"]');
  await frames(page, 6);

  // Clip latch and host-owned reset, painted on the lamp.
  const lampBefore = await samplePixel(page, 0, -6, 1);
  await page.click('button[data-part="clip-trigger"]');
  await frames(page, 3);
  const lampClipped = await samplePixel(page, 0, -6, 1);
  await page.click('button[data-part="clip-reset"]');
  await frames(page, 3);
  const lampReset = await samplePixel(page, 0, -6, 1);
  check("clip latch changes the painted lamp", pixelsDiffer(lampBefore, lampClipped), JSON.stringify({ lampBefore, lampClipped }));
  check("clip reset restores the lamp", !pixelsDiffer(lampBefore, lampReset), JSON.stringify({ lampBefore, lampReset }));

  // Registration add/remove while live.
  const countsBefore = await page.evaluate(() => document.querySelectorAll(".poodle-audio-meter[data-surface]").length);
  await page.click('button[data-part="meter-remove"]');
  await frames(page, 2);
  const countsRemoved = await page.evaluate(() => document.querySelectorAll(".poodle-audio-meter[data-surface]").length);
  await page.click('button[data-part="meter-add"]');
  await frames(page, 2);
  const countsAdded = await page.evaluate(() => document.querySelectorAll(".poodle-audio-meter[data-surface]").length);
  check("meter unregistration removes one placeholder", countsRemoved === countsBefore - 1, `${countsBefore} -> ${countsRemoved}`);
  check("meter registration adds one placeholder", countsAdded === countsRemoved + 1, `${countsRemoved} -> ${countsAdded}`);

  // Destroy and remount the surface.
  await page.click('button[data-part="remount-toggle"]');
  await frames(page, 2);
  const destroyed = await page.evaluate(() => document.querySelectorAll(".poodle-meter-surface").length);
  await page.click('button[data-part="remount-toggle"]');
  await frames(page, 4);
  const remounted = await page.evaluate(() => document.querySelectorAll(".poodle-meter-surface canvas").length);
  const paintedAfterRemount = await samplePixel(page, 0, 4, 60);
  check("destroy removes the surface", destroyed === 0);
  check("remount restores one painted canvas", remounted === 1 && paintedAfterRemount !== null && paintedAfterRemount.a > 0);

  // Window resize keeps the backing store in step.
  await page.setViewportSize({ width: 560, height: 860 });
  await frames(page, 4);
  const resized = await page.evaluate(() => {
    const canvas = document.querySelector<HTMLCanvasElement>(".poodle-meter-surface__canvas")!;
    const viewport = document.querySelector<HTMLElement>(".poodle-meter-surface__viewport")!;
    return { width: canvas.width, expected: Math.max(Math.round(viewport.clientWidth * devicePixelRatio), 1) };
  });
  check("resize retunes the backing store", resized.width === resized.expected, `${resized.width} != ${resized.expected}`);
  await page.setViewportSize({ width: 1280, height: 900 });
}

async function runDpr(browserType: BrowserType, base: string, framework: Framework, browserName: string): Promise<void> {
  const browser = await browserType.launch();
  try {
    const context = await browser.newContext({ deviceScaleFactor: 2, viewport: { width: 1280, height: 900 } });
    const page = await context.newPage();
    await page.goto(specimenUrl(base, "meter-surface", AXIS as never), { waitUntil: "domcontentloaded" });
    await page.waitForSelector(".poodle-meter-surface__canvas");
    await frames(page, 4);
    const result = await page.evaluate(() => {
      const canvas = document.querySelector<HTMLCanvasElement>(".poodle-meter-surface__canvas")!;
      const viewport = document.querySelector<HTMLElement>(".poodle-meter-surface__viewport")!;
      return { dpr: devicePixelRatio, width: canvas.width, expected: Math.max(Math.round(viewport.clientWidth * devicePixelRatio), 1) };
    });
    check(`${browserName} / ${framework} DPR=2 backing store`, result.dpr === 2 && result.width === result.expected, JSON.stringify(result));
  } finally {
    await browser.close();
  }
}


/**
 * One fresh browser per framework run: headless WebKit suspends rAF in a
 * second context of a shared instance, and long driver sessions can wedge a
 * single un-timed protocol call. A section watchdog plus one fresh-browser
 * retry keeps a transient wedge from hanging the whole matrix.
 */
async function runSection(browserType: BrowserType, browserName: string, framework: Framework, base: string): Promise<void> {
  const attempts = 2;
  for (let attempt = 1; attempt <= attempts; attempt += 1) {
    const failuresBefore = failures;
    let browser: Browser | null = null;
    try {
      browser = await browserType.launch();
      const context = await browser.newContext({ viewport: { width: 1280, height: 900 } });
      const page = await context.newPage();
      await Promise.race([
        runMatrix(page, base, framework, browserName),
        new Promise((_, reject) => setTimeout(() => reject(new Error("section watchdog: no completion within 240s")), 240_000)),
      ]);
      await browser.close();
      return;
    } catch (error) {
      await browser?.close().catch(() => {});
      if (attempt === attempts) throw error;
      failures = failuresBefore;
      console.log(`  retrying ${browserName}/${framework} on a fresh browser after: ${String(error).slice(0, 140)}`);
    }
  }
}

async function main(): Promise<void> {
  await startPreviews();
  const bases: Record<Framework, string> = {
    svelte: `http://127.0.0.1:${SERVERS.svelte.port}`,
    react: `http://127.0.0.1:${SERVERS.react.port}`,
  };
  // `--browser=chromium|webkit` runs one engine per invocation. Long
  // multi-engine sessions proved fragile locally (a wedged driver call can
  // stall the whole matrix), and per-engine runs keep each one bounded.
  const only = process.argv.find((arg) => arg.startsWith("--browser="))?.slice("--browser=".length);
  const engines = ([["chromium", chromium], ["webkit", webkit]] as Array<[string, BrowserType]>)
    .filter(([name]) => only === undefined || name === only);
  for (const [browserName, browserType] of engines) {
    for (const framework of ["svelte", "react"] as Framework[]) {
      await runSection(browserType, browserName, framework, bases[framework]);
      try {
        await runDpr(browserType, bases[framework], framework, browserName);
      } catch (error) {
        console.log(`  retrying DPR ${browserName}/${framework} after: ${String(error).slice(0, 120)}`);
        await runDpr(browserType, bases[framework], framework, browserName);
      }
    }
  }
  if (failures > 0) {
    console.error(`\nmeter-surface probe: ${failures} failing check(s)`);
    process.exit(1);
  }
  console.log("\nmeter-surface probe: all checks passed");
  process.exit(0);
}

await main();
