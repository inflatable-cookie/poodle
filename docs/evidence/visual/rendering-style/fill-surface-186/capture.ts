/* Evidence capture for the `fill` and `surface` roles on the controls and
   containers that only took `selection` (poodle#186).

   Every catalogue specimen section is captured, in Eclipse and in a light
   theme (Iceberg), under the default style. That covers every specimen page
   that shows the changed components, and every other page too, so an
   unintended change anywhere shows up in the same comparison.

   Usage, from the checkout root:

     effigy web:dist
     bun docs/evidence/visual/rendering-style/fill-surface-186/capture.ts before OUT
     ... edit, then:
     effigy web:dist
     bun docs/evidence/visual/rendering-style/fill-surface-186/capture.ts after OUT
     bun docs/evidence/visual/rendering-style/fill-surface-186/capture.ts compose OUT
     effigy web:dist
     bun docs/evidence/visual/rendering-style/fill-surface-186/capture.ts quiet OUT
     bun docs/evidence/visual/rendering-style/fill-surface-186/capture.ts compose-quiet OUT DEST

   `compose` diffs before and after byte-for-byte and writes
   OUT/report.json, listing every section that changed and the pixel count.
   Its exit status is non-zero when any default-style section changed.
   `quiet` captures only the sections listed in OUT/changed.json under
   `data-style="quiet"`, and `compose-quiet` puts each before, after and quiet
   section side by side under DEST.

   The preview reads the built `packages/core/dist/styles` copy, so rebuild it
   (`effigy web:dist`) before each phase. The capture refuses a stale copy. */

import { mkdirSync, readFileSync, readdirSync, writeFileSync, existsSync } from "node:fs";
import { resolve } from "node:path";
import { chromium, type Browser, type Page } from "playwright";
import { PNG } from "pngjs";
import { pinPage } from "../../../../../test/visual/capture";
import { SKIPPED } from "../../../../../test/visual/config";
import { allComponents } from "../../../../../packages/svelte/preview/src/component-registry";

const ROOT = resolve(import.meta.dir, "../../../../..");
const PREVIEW_PORT = Number(process.env.POODLE_FILL_SURFACE_CAPTURE_PORT ?? 4398);
const PREVIEW_ROOT = resolve(ROOT, "packages/svelte/preview");
const SOURCE_STYLES = resolve(ROOT, "packages/core/src/styles");
const DIST_STYLES = resolve(ROOT, "packages/core/dist/styles");

const themes = ["eclipse", "iceberg"] as const;

type Phase = "before" | "after" | "quiet";

/** Sections are captured by slug and index, in document order. */
type Section = { slug: string; index: number };

/**
 * Dist must match the source the phase claims to measure: the working tree for
 * `after` and `quiet`, and the base commit for `before`, which runs before any
 * stylesheet edit lands in dist.
 */
function assertDistMatchesSource(phase: Phase): void {
  const stale = readdirSync(SOURCE_STYLES)
    .filter((file) => file.endsWith(".css"))
    .filter((file) => {
      const expected =
        phase === "before"
          ? Bun.spawnSync(["git", "show", `HEAD:packages/core/src/styles/${file}`], { cwd: ROOT }).stdout.toString()
          : readFileSync(resolve(SOURCE_STYLES, file), "utf8");
      try {
        return expected !== readFileSync(resolve(DIST_STYLES, file), "utf8");
      } catch {
        return true;
      }
    });
  if (stale.length > 0) {
    throw new Error(`packages/core/dist/styles does not match ${phase === "before" ? "HEAD" : "the working tree"} (${stale.join(", ")}); run \`effigy web:dist\``);
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
    throw new Error(`port ${PREVIEW_PORT} already answers; set POODLE_FILL_SURFACE_CAPTURE_PORT`);
  }
  const proc = Bun.spawn(
    ["bun", "run", "dev", "--port", String(PREVIEW_PORT), "--strictPort", "--host", "127.0.0.1"],
    { cwd: PREVIEW_ROOT, stdout: "ignore", stderr: "ignore" },
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

const FREEZE_CSS = `
  *, *::before, *::after {
    transition: none !important;
    animation: none !important;
    caret-color: transparent !important;
  }
`;

async function settle(page: Page): Promise<void> {
  await page.evaluate(() => document.fonts.ready);
  await page.evaluate(() => new Promise((done) =>
    requestAnimationFrame(() => requestAnimationFrame(done))));
}

function catalogueSlugs(): string[] {
  return allComponents
    .filter((component) => component.hasSpecimen && !(component.slug in SKIPPED))
    .map((component) => component.slug)
    .sort();
}

/** Opens one catalogue page and returns its visible specimen sections. */
async function openPage(page: Page, root: string, slug: string, theme: string, phase: Phase): Promise<number> {
  const params = new URLSearchParams({ theme, density: "default", controlSize: "md" });
  if (phase === "quiet") params.set("style", "quiet");
  await page.goto("about:blank");
  await page.goto(`${root}/?${params}#components/${slug}`, { waitUntil: "domcontentloaded", timeout: 120_000 });
  await page.locator(".poodle-component-page__section").first().waitFor({ timeout: 90_000 });
  await page.addStyleTag({ content: FREEZE_CSS });
  if (phase === "quiet") {
    await page.waitForFunction(() =>
      document.querySelector(".poodle-app-shell")?.getAttribute("data-style") === "quiet");
  }
  await settle(page);
  return await page.locator(".poodle-component-page__section").count();
}

async function captureSections(
  page: Page,
  root: string,
  slug: string,
  theme: string,
  phase: Phase,
  only?: number[],
  classes?: string[],
): Promise<Array<{ index: number; png: Buffer }>> {
  const count = await openPage(page, root, slug, theme, phase);
  if (classes) {
    const renders = await page.evaluate((names: string[]) => {
      const wanted = new Set(names);
      return Array.from(document.querySelectorAll(".poodle-component-page__section *")).some((element) =>
        Array.from(element.classList).some((name) => wanted.has(name)));
    }, classes);
    if (!renders) return [];
  }
  const out: Array<{ index: number; png: Buffer }> = [];
  for (let index = 0; index < count; index += 1) {
    if (only && !only.includes(index)) continue;
    const section = page.locator(".poodle-component-page__section").nth(index);
    if (!(await section.isVisible())) continue;
    out.push({ index, png: await section.screenshot() });
  }
  return out;
}

function countPixelChanges(before: PNG, after: PNG): number {
  if (before.width !== after.width || before.height !== after.height) {
    return Number.POSITIVE_INFINITY;
  }
  let changed = 0;
  for (let index = 0; index < before.data.length; index += 4) {
    if (before.data.readUInt32LE(index) !== after.data.readUInt32LE(index)) changed++;
  }
  return changed;
}

function fileFor(dir: string, theme: string, slug: string, index: number): string {
  return resolve(dir, theme, slug, `${index}.png`);
}

function listCaptured(dir: string, theme: string): Section[] {
  const base = resolve(dir, theme);
  if (!existsSync(base)) return [];
  const sections: Section[] = [];
  for (const slug of readdirSync(base).sort()) {
    for (const file of readdirSync(resolve(base, slug)).sort()) {
      sections.push({ slug, index: Number(file.replace(/\.png$/, "")) });
    }
  }
  return sections;
}

const [command, output, destination] = process.argv.slice(2);

if (command === "compose") {
  if (!output) throw new Error("Usage: capture.ts compose OUT");
  const changed: Array<Section & { theme: string; pixels: number }> = [];
  let compared = 0;
  for (const theme of themes) {
    for (const section of listCaptured(resolve(output, "before"), theme)) {
      const beforePath = fileFor(resolve(output, "before"), theme, section.slug, section.index);
      const afterPath = fileFor(resolve(output, "after"), theme, section.slug, section.index);
      if (!existsSync(afterPath)) {
        changed.push({ ...section, theme, pixels: Number.POSITIVE_INFINITY });
        continue;
      }
      compared += 1;
      const pixels = countPixelChanges(
        PNG.sync.read(readFileSync(beforePath)),
        PNG.sync.read(readFileSync(afterPath)),
      );
      if (pixels !== 0) changed.push({ ...section, theme, pixels });
    }
  }
  const report = {
    compared,
    changedSections: changed,
    changedSlugs: [...new Set(changed.map((entry) => entry.slug))].sort(),
  };
  writeFileSync(resolve(output, "report.json"), `${JSON.stringify(report, null, 2)}\n`);
  writeFileSync(resolve(output, "changed.json"), `${JSON.stringify(report.changedSlugs, null, 2)}\n`);
  console.log(`compared ${compared} sections; ${changed.length} changed in ${report.changedSlugs.length} pages`);
  if (changed.length > 0) process.exitCode = 1;
} else if (command === "compose-quiet") {
  if (!output || !destination) throw new Error("Usage: capture.ts compose-quiet OUT DEST");
  const browser = await chromium.launch();
  try {
    const page = await browser.newPage({ viewport: { width: 1600, height: 900 }, deviceScaleFactor: 1 });
    for (const theme of themes) {
      const quietSlugs = [...new Set(listCaptured(resolve(output, "quiet"), theme).map((entry) => entry.slug))];
      for (const slug of quietSlugs.sort()) {
        const sections = listCaptured(resolve(output, "quiet"), theme).filter((entry) => entry.slug === slug);
        const rows = sections.map((section) => {
          const before = fileFor(resolve(output, "before"), theme, slug, section.index);
          const after = fileFor(resolve(output, "after"), theme, slug, section.index);
          const quiet = fileFor(resolve(output, "quiet"), theme, slug, section.index);
          return `<tr><td><img src="file://${before}"></td><td><img src="file://${after}"></td><td><img src="file://${quiet}"></td></tr>`;
        }).join("\n");
        const sheetHtml = `<!doctype html><html><head><meta charset="utf-8"><style>
          body { margin: 0; padding: 16px; font: 12px/1.4 ui-monospace, monospace; color: #16181d; background: #fff; }
          h1 { font-size: 14px; margin: 0 0 8px; }
          table { border-collapse: collapse; }
          th, td { padding: 6px; vertical-align: top; text-align: left; border-top: 1px solid #d6dae1; }
          th { font-weight: 600; }
          img { display: block; max-width: 500px; height: auto; border: 1px solid #d6dae1; }
        </style></head><body>
          <h1>${slug} — ${theme}</h1>
          <table><thead><tr><th>default, before</th><th>default, after</th><th>quiet</th></tr></thead>
          <tbody>${rows}</tbody></table>
        </body></html>`;
        mkdirSync(resolve(destination, theme), { recursive: true });
        const sheetFile = resolve(destination, theme, `${slug}.html`);
        writeFileSync(sheetFile, sheetHtml);
        await page.goto(`file://${sheetFile}`, { waitUntil: "load" });
        await page.evaluate(() => document.fonts.ready);
        mkdirSync(resolve(destination, theme), { recursive: true });
        await page.screenshot({ path: resolve(destination, theme, `${slug}.png`), fullPage: true });
        console.log("sheet", theme, slug, sections.length);
      }
    }
  } finally {
    await browser.close();
  }
} else {
  const phase = command as Phase;
  if (phase !== "before" && phase !== "after" && phase !== "quiet") {
    throw new Error("Usage: capture.ts before|after|quiet OUT");
  }
  if (!output) throw new Error("Usage: capture.ts before|after|quiet OUT");
  assertDistMatchesSource(phase);

  let slugs = catalogueSlugs();
  /* The quiet pass covers every page that renders a class from a changed stylesheet. */
  const classesFile = process.env.CAPTURE_CLASSES;
  const classes: string[] | undefined = classesFile ? JSON.parse(readFileSync(classesFile, "utf8")) : undefined;
  /* A rerun of named pages only, for pages a loaded run timed out on. */
  const only = process.env.CAPTURE_ONLY?.split(",").filter(Boolean);
  if (only && only.length > 0) slugs = slugs.filter((slug) => only.includes(slug));

  const previews = await startPreview();
  let browser: Browser | undefined;
  try {
    browser = await chromium.launch();
    const page = await browser.newPage({ viewport: { width: 1280, height: 900 }, deviceScaleFactor: 1 });
    await pinPage(page);
    for (const theme of themes) {
      for (const slug of slugs) {
        let sections: Array<{ index: number; png: Buffer }> = [];
        try {
          sections = await captureSections(page, previews.url, slug, theme, phase, undefined, classes);
        } catch (error) {
          /* A page that never shows a specimen is recorded, not fatal: the run
             covers every catalogue page and reports the ones it could not open. */
          console.log(phase, theme, slug, "FAILED", String(error).split("\n")[0]);
          continue;
        }
        for (const section of sections) {
          const file = fileFor(resolve(output, phase), theme, slug, section.index);
          mkdirSync(resolve(file, ".."), { recursive: true });
          writeFileSync(file, section.png);
        }
        console.log(phase, theme, slug, sections.length);
      }
    }
  } finally {
    await browser?.close();
    await previews.stop();
  }
}
