/* Evidence capture for the `surface` treatment on the container cohort
   (poodle#174). One capture per container, in Eclipse and in a light theme
   (Iceberg), under the default style and under `data-style="quiet"`.

   Usage, from the checkout root:

     effigy web:dist
     bun docs/evidence/visual/rendering-style/capture-containers.ts before OUT
     bun docs/evidence/visual/rendering-style/capture-containers.ts after OUT
     effigy web:dist
     bun docs/evidence/visual/rendering-style/capture-containers.ts quiet OUT
     bun docs/evidence/visual/rendering-style/capture-containers.ts compose OUT DEST

   `before` and `after` both capture the default style, at two commits; the
   `compose` step diffs them and fails on any changed pixel, then writes one
   before | after | quiet sheet per theme. The preview reads the built
   `packages/core/dist/styles` copy, so rebuild it (`effigy web:dist`) for each
   state before capturing. */

import { mkdirSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { chromium, type Browser, type Page } from "playwright";
import { PNG } from "pngjs";
import { pinPage } from "../../../../test/visual/capture";

/**
 * The visual gate owns ports 4174/4180 and reuses whatever answers there, which
 * is another worktree's checkout when a sibling run is mid-flight. This harness
 * boots and stops its own Svelte preview on a private port instead.
 */
const PREVIEW_PORT = Number(process.env.POODLE_CONTAINER_CAPTURE_PORT ?? 4399);
const PREVIEW_ROOT = "packages/svelte/preview";

/**
 * The preview resolves `@inflatable-cookie/poodle-core/styles/*` through the
 * package export map, which points at the built `dist/styles` copy. Capturing
 * against a stale copy silently compares the build with itself, so refuse to
 * run until `effigy web:dist` has refreshed it from the current sources.
 */
function assertDistMatchesSource(): void {
  const sourceDir = resolve(import.meta.dir, "../../../../packages/core/src/styles");
  const distDir = resolve(import.meta.dir, "../../../../packages/core/dist/styles");
  const stale = readdirSync(sourceDir)
    .filter((file) => file.endsWith(".css"))
    .filter((file) => {
      try {
        return readFileSync(resolve(sourceDir, file), "utf8") !== readFileSync(resolve(distDir, file), "utf8");
      } catch {
        return true;
      }
    });
  if (stale.length > 0) {
    throw new Error(
      `packages/core/dist/styles is stale (${stale.join(", ")}); run \`effigy web:dist\` and capture again`,
    );
  }
}

async function isUp(port: number): Promise<boolean> {
  try {
    const response = await fetch(`http://127.0.0.1:${port}/`, { signal: AbortSignal.timeout(2000) });
    return response.ok;
  } catch {
    return false;
  }
}

async function startPreview(): Promise<{ url: string; stop: () => Promise<void> }> {
  if (await isUp(PREVIEW_PORT)) {
    throw new Error(`port ${PREVIEW_PORT} already answers; stop that process or set POODLE_CONTAINER_CAPTURE_PORT`);
  }
  const proc = Bun.spawn(
    ["bun", "run", "dev", "--port", String(PREVIEW_PORT), "--strictPort", "--host", "127.0.0.1"],
    { cwd: resolve(import.meta.dir, "../../../..", PREVIEW_ROOT), stdout: "ignore", stderr: "ignore" },
  );
  const deadline = Date.now() + 120_000;
  while (Date.now() < deadline) {
    if (await isUp(PREVIEW_PORT)) {
      return {
        url: `http://127.0.0.1:${PREVIEW_PORT}`,
        stop: async () => {
          proc.kill();
          await proc.exited;
        },
      };
    }
    await new Promise((done) => setTimeout(done, 250));
  }
  proc.kill();
  throw new Error(`preview on port ${PREVIEW_PORT} did not come up within 120s`);
}

type Phase = "before" | "after" | "quiet";

type Target = {
  slug: string;
  /** Container that draws the separation; the clip is padded around its box. */
  selector: string;
  /** Overlays are closed on load; this reveals the one being captured. */
  reveal?: (page: Page) => Promise<void>;
  /** Page-level specimens are captured whole, like the visual gate does. */
  specimen?: boolean;
};

const targets: Target[] = [
  { slug: "surface", selector: ".poodle-component-page__section", specimen: true },
  { slug: "picker-shell", selector: ".poodle-component-page__section", specimen: true },
  { slug: "toast-stack", selector: ".poodle-component-page__section", specimen: true },
  { slug: "callout", selector: ".poodle-component-page__section", specimen: true },
  { slug: "empty-state", selector: ".poodle-component-page__section", specimen: true },
  {
    slug: "popover",
    selector: ".poodle-popover__surface",
    reveal: async (page) => {
      await page.getByRole("button", { name: "Open popover", exact: true }).click();
      await page.locator(".poodle-popover__surface").waitFor();
    },
  },
  {
    slug: "tooltip",
    selector: ".poodle-tooltip__bubble",
    reveal: async (page) => {
      await page.getByRole("button", { name: "Hover me", exact: true }).hover();
      await page.locator(".poodle-tooltip__bubble").waitFor();
    },
  },
  {
    slug: "hover-card",
    selector: ".poodle-hover-card__surface",
    reveal: async (page) => {
      await page.getByText("@clay", { exact: true }).hover();
      await page.locator(".poodle-hover-card__surface").waitFor();
    },
  },
  {
    slug: "drawer",
    selector: ".poodle-drawer__surface",
    reveal: async (page) => {
      await page.getByRole("button", { name: "Open right drawer", exact: true }).click();
      await page.locator(".poodle-drawer__surface").waitFor();
    },
  },
  {
    slug: "command-palette",
    selector: ".poodle-command-palette",
    reveal: async (page) => {
      await page.getByRole("button", { name: "Open Command Palette", exact: true }).click();
      await page.locator(".poodle-command-palette").waitFor();
    },
  },
  {
    slug: "alert-dialog",
    selector: ".poodle-dialog__surface",
    reveal: async (page) => {
      await page.getByRole("button", { name: "Delete item", exact: true }).click();
      await page.locator(".poodle-dialog__surface").waitFor();
    },
  },
];

const themes = ["eclipse", "iceberg"] as const;

/** Room around the container so its separation from the page is visible. */
const PADDING = 48;

const FREEZE_CSS = `
  *, *::before, *::after {
    transition: none !important;
    animation: none !important;
    caret-color: transparent !important;
  }
  .poodle-app-top-bar, .poodle-display-controls, .poodle-component-page__hero,
  .poodle-component-page__section ~ .poodle-component-page__section {
    display: none !important;
  }
  .poodle-app-main { flex: auto !important; overflow: visible !important; }
`;

async function settle(page: Page): Promise<void> {
  await page.evaluate(() => document.fonts.ready);
  await page.evaluate(() => new Promise((done) =>
    requestAnimationFrame(() => requestAnimationFrame(done))));
}

/**
 * Overlays open through JS transitions that set inline styles, so freezing CSS
 * transitions is not enough: poll until the box stops moving before clipping.
 */
async function stableBox(page: Page, selector: string) {
  let last = "";
  let stable = 0;
  let box: Awaited<ReturnType<ReturnType<Page["locator"]>["boundingBox"]>> = null;
  for (let attempt = 0; attempt < 40 && stable < 3; attempt += 1) {
    box = await page.locator(`${selector}:visible`).first().boundingBox();
    const key = box
      ? `${Math.round(box.x)},${Math.round(box.y)},${Math.round(box.width)},${Math.round(box.height)}`
      : "none";
    stable = key === last && key !== "none" ? stable + 1 : 0;
    last = key;
    if (stable < 3) await page.waitForTimeout(80);
  }
  return box;
}

async function capture(page: Page, root: string, target: Target, theme: string, phase: Phase): Promise<Buffer> {
  const params = new URLSearchParams({ theme, density: "default", controlSize: "md" });
  if (phase === "quiet") params.set("style", "quiet");

  await page.goto("about:blank");
  await page.goto(`${root}/?${params}#components/${target.slug}`);
  const specimen = page.locator(".poodle-component-page__section").first();
  await specimen.waitFor();
  await page.addStyleTag({ content: FREEZE_CSS });
  if (phase === "quiet") {
    await page.waitForFunction(() =>
      document.querySelector(".poodle-app-shell")?.getAttribute("data-style") === "quiet");
  }

  await target.reveal?.(page);
  await settle(page);

  if (target.specimen) return await specimen.screenshot();

  const box = await stableBox(page, target.selector);
  if (!box) throw new Error(`${target.slug}: ${target.selector} has no box`);
  const x = Math.max(0, Math.round(box.x - PADDING));
  const y = Math.max(0, Math.round(box.y - PADDING));
  const width = Math.min(1280 - x, Math.round(box.width + PADDING * 2));
  const height = Math.min(900 - y, Math.round(box.height + PADDING * 2));
  return await page.screenshot({ clip: { x, y, width, height } });
}

function countPixelChanges(before: PNG, after: PNG): number {
  if (before.width !== after.width || before.height !== after.height) {
    throw new Error(`Size changed from ${before.width}×${before.height} to ${after.width}×${after.height}`);
  }
  let changed = 0;
  for (let index = 0; index < before.data.length; index += 4) {
    if (before.data.readUInt32LE(index) !== after.data.readUInt32LE(index)) changed++;
  }
  return changed;
}

function dataUrl(path: string): string {
  return `data:image/png;base64,${readFileSync(path).toString("base64")}`;
}

const [command, output, destination] = process.argv.slice(2);

if (command === "compose") {
  if (!output || !destination) throw new Error("Usage: capture-containers.ts compose INPUT_DIRECTORY DEST_DIRECTORY");
  const report: Record<string, Record<string, number>> = {};
  let failed = false;

  for (const theme of themes) {
    report[theme] = {};
    for (const target of targets) {
      const file = `${theme}-${target.slug}.png`;
      const before = PNG.sync.read(readFileSync(resolve(output, "before", file)));
      const after = PNG.sync.read(readFileSync(resolve(output, "after", file)));
      const changed = countPixelChanges(before, after);
      report[theme][target.slug] = changed;
      if (changed !== 0) failed = true;
    }
  }

  mkdirSync(destination, { recursive: true });
  const browser = await chromium.launch();
  try {
    const page = await browser.newPage({ viewport: { width: 1240, height: 900 }, deviceScaleFactor: 1 });
    for (const theme of themes) {
      const rows = targets.map((target) => {
        const file = `${theme}-${target.slug}.png`;
        const delta = report[theme][target.slug];
        return `<tr>
          <th><span class="slug">${target.slug}</span><span class="delta">${delta} px changed</span></th>
          <td><img src="${dataUrl(resolve(output, "before", file))}"></td>
          <td><img src="${dataUrl(resolve(output, "after", file))}"></td>
          <td><img src="${dataUrl(resolve(output, "quiet", file))}"></td>
        </tr>`;
      }).join("\n");
      await page.setContent(`<!doctype html>
<html><head><meta charset="utf-8"><style>
  body { margin: 0; padding: 20px; background: #ffffff; font: 12px/1.4 ui-monospace, monospace; color: #16181d; }
  h1 { font-size: 15px; margin: 0 0 4px; }
  p.note { margin: 0 0 16px; color: #5a6270; }
  table { border-collapse: collapse; }
  th, td { padding: 8px; border-top: 1px solid #d6dae1; vertical-align: top; text-align: left; }
  thead th { border-top: 0; font-weight: 600; }
  th { width: 96px; }
  .slug { display: block; font-weight: 600; }
  .delta { display: block; color: #5a6270; }
  td img { display: block; max-width: 400px; max-height: 300px; width: auto; height: auto; border: 1px solid #d6dae1; }
</style></head><body>
<h1>Surface treatment — container cohort — ${theme}</h1>
<p class="note">Default style before and after poodle#174, then the same container under data-style="quiet". Pixel deltas come from an exact byte comparison of the two default captures.</p>
<table>
  <thead><tr><th></th><th>default, before</th><th>default, after</th><th>quiet</th></tr></thead>
  <tbody>
${rows}
  </tbody>
</table>
</body></html>`, { waitUntil: "load" });
      await page.evaluate(() => document.fonts.ready);
      const shot = await page.screenshot({ fullPage: true });
      writeFileSync(resolve(destination, `surface-containers-${theme}.png`), shot);
    }
  } finally {
    await browser.close();
  }

  const summaryPath = resolve(destination, "surface-containers.json");
  writeFileSync(summaryPath, `${JSON.stringify({ capturedAt: new Date().toISOString().slice(0, 10), themes, targets: targets.map((t) => t.slug), defaultStylePixelChanges: report }, null, 2)}\n`);
  console.log(JSON.stringify(report, null, 2));
  if (failed) {
    process.exitCode = 1;
    throw new Error("the default style is not pixel-identical to the capture before the change");
  }
} else {
  const phase = command as Phase;
  if ((phase !== "before" && phase !== "after" && phase !== "quiet") || !output) {
    throw new Error("Usage: capture-containers.ts before|after|quiet OUTPUT_DIRECTORY");
  }
  mkdirSync(resolve(output, phase), { recursive: true });
  assertDistMatchesSource();
  const previews = await startPreview();
  let browser: Browser | undefined;
  try {
    browser = await chromium.launch();
    const page = await browser.newPage({ viewport: { width: 1280, height: 900 }, deviceScaleFactor: 1 });
    await pinPage(page);
    for (const theme of themes) {
      for (const target of targets) {
        const png = await capture(page, previews.url, target, theme, phase);
        writeFileSync(resolve(output, phase, `${theme}-${target.slug}.png`), png);
        console.log(phase, theme, target.slug);
      }
    }
  } finally {
    await browser?.close();
    await previews.stop();
  }
}
