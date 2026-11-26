// Evidence for poodle#184: the shell, overlay and status stylesheets on the fill, surface
// and selection roles.
//
//   PREVIEW_URL=http://127.0.0.1:4391 bun docs/evidence/visual/rendering-style/shell-184/capture.ts capture OUTPUT_DIR style=default,quiet
//   bun docs/evidence/visual/rendering-style/shell-184/capture.ts compare BEFORE_DIR AFTER_DIR
//   bun docs/evidence/visual/rendering-style/shell-184/capture.ts compose SOURCE_DIR OUTPUT_DIR slug,slug
//
// `capture` renders every catalogue specimen in the Svelte preview at `style=<name>` for Eclipse
// (dark) and Clay (light), at density default and size md. `compare` byte-compares every default-style
// PNG; byte equality of the same render is zero pixel difference. `compose` places each default
// specimen beside its quiet specimen for review.
import { mkdirSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { chromium, type Page } from "playwright";
import { PNG } from "pngjs";
import { pinPage } from "../../../../../test/visual/capture";
import { sweepSlugs } from "../../../../../test/visual/config";

const themes = ["eclipse", "clay"] as const;
const SPECIMEN = ".poodle-component-page__section";

// Hides preview chrome and freezes motion, as test/visual/capture.ts does.
const FREEZE_CSS = `
  *, *::before, *::after {
    transition-duration: 0s !important;
    transition-delay: 0s !important;
    animation-duration: 0s !important;
    animation-delay: 0s !important;
    animation-iteration-count: 1 !important;
    caret-color: transparent !important;
  }
  .poodle-component-page__hero { display: none !important; }
  .poodle-component-page__section ~ .poodle-component-page__section { display: none !important; }
`;

async function capture(page: Page, root: string, slug: string, theme: string, style: string): Promise<Buffer | null> {
  const params = new URLSearchParams({ theme, density: "default", controlSize: "md", style });
  await page.goto("about:blank");
  await page.goto(`${root}/?${params}#components/${slug}`, { waitUntil: "load", timeout: 60_000 });
  const specimen = page.locator(SPECIMEN).first();
  await specimen.waitFor({ state: "visible", timeout: 60_000 });
  await page.addStyleTag({ content: FREEZE_CSS });
  await page.waitForFunction(
    (name) => document.querySelector(".poodle-app-shell")?.getAttribute("data-style") === name,
    style,
  );
  await page.evaluate(() => document.fonts.ready);
  await page.evaluate(
    () => new Promise((done) => requestAnimationFrame(() => requestAnimationFrame(done))),
  );
  let png = await specimen.screenshot({ animations: "disabled" });
  const again = await specimen.screenshot({ animations: "disabled" });
  if (!png.equals(again)) {
    await page.waitForTimeout(500);
    png = await specimen.screenshot({ animations: "disabled" });
    const settled = await specimen.screenshot({ animations: "disabled" });
    if (!png.equals(settled)) return null;
  }
  return png;
}

function selectedSlugs(): string[] {
  const only = process.env.SLUGS?.split(",").filter(Boolean);
  return only && only.length > 0 ? only : sweepSlugs();
}

async function runCapture(output: string, styles: string[]) {
  const root = process.env.PREVIEW_URL;
  if (!root) throw new Error("Set PREVIEW_URL to a Svelte preview started from this checkout.");
  const browser = await chromium.launch();
  const unsettled: string[] = [];
  let count = 0;
  try {
    for (const style of styles) {
      const dir = resolve(output, style);
      mkdirSync(dir, { recursive: true });
      for (const theme of themes) {
        for (const slug of selectedSlugs()) {
          // A fresh page per capture: one reused page stalls on navigation after about 29 captures.
          const page = await browser.newPage({ viewport: { width: 1280, height: 900 }, deviceScaleFactor: 1 });
          try {
            await pinPage(page);
            const png = await capture(page, root, slug, theme, style);
            if (png) writeFileSync(resolve(dir, `${theme}-${slug}.png`), png);
            else unsettled.push(`${style} ${theme} ${slug}`);
          } catch (error) {
            unsettled.push(`${style} ${theme} ${slug}: ${(error as Error).message.split("\n")[0]}`);
          } finally {
            await page.close();
          }
          count += 1;
        }
      }
    }
  } finally {
    await browser.close();
  }
  console.log(JSON.stringify({ captured: count, unsettled }, null, 2));
}

function compare(before: string, after: string) {
  const files = readdirSync(resolve(before, "default")).filter((name) => name.endsWith(".png")).sort();
  const missing = files.filter((name) => {
    try {
      readFileSync(resolve(after, "default", name));
      return false;
    } catch {
      return true;
    }
  });
  const differing = files.filter((name) => {
    if (missing.includes(name)) return false;
    const a = readFileSync(resolve(before, "default", name));
    const b = readFileSync(resolve(after, "default", name));
    return !a.equals(b);
  });
  const summary = { compared: files.length, identical: files.length - differing.length - missing.length, missing, differing };
  console.log(JSON.stringify(summary, null, 2));
  if (differing.length > 0 || missing.length > 0) process.exitCode = 1;
}

function compose(source: string, output: string, slugs: string[]) {
  mkdirSync(output, { recursive: true });
  const gap = 24;
  for (const slug of slugs) {
    const def = PNG.sync.read(readFileSync(resolve(source, "default", `eclipse-${slug}.png`)));
    const quiet = PNG.sync.read(readFileSync(resolve(source, "quiet", `eclipse-${slug}.png`)));
    const width = def.width + gap + quiet.width;
    const height = Math.max(def.height, quiet.height);
    const png = new PNG({ width, height });
    for (let i = 0; i < png.data.length; i += 4) png.data[i + 3] = 255;
    for (const [img, x0] of [[def, 0], [quiet, def.width + gap]] as const) {
      for (let y = 0; y < img.height; y++) {
        img.data.copy(png.data, (y * width + x0) * 4, y * img.width * 4, (y + 1) * img.width * 4);
      }
    }
    writeFileSync(resolve(output, `${slug}.png`), PNG.sync.write(png));
    console.log(slug, width, height);
  }
}

const [phase, first, second, third] = process.argv.slice(2);
if (phase === "capture" && first && second) {
  await runCapture(first, second.replace(/^style=/, "").split(","));
} else if (phase === "compare" && first && second) {
  compare(first, second);
} else if (phase === "compose" && first && second && third) {
  compose(first, second, third.split(","));
} else {
  throw new Error("Usage: capture OUTPUT style=a,b | compare BEFORE AFTER | compose SOURCE OUTPUT slug,slug");
}
