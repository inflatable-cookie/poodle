import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { chromium, type Browser, type Page } from "playwright";
import { PNG } from "pngjs";
import { pinPage } from "../../../../test/visual/capture";
import { startPreviews } from "../../../../test/visual/server";

type Matrix = readonly (readonly [style: string, themes: readonly string[], slugs: readonly string[]])[];

const cohort = ["button", "text-input", "select", "card", "dialog"] as const;
const darkThemes = ["eclipse", "graphite"];
const iceberg = ["iceberg"];
const icebergSpecimens = ["card", "media-preview", "metric-tile"];
const matrices = {
  baseline: [["before", darkThemes, cohort], ["before", iceberg, icebergSpecimens]],
  compare: [
    ["absent", darkThemes, cohort],
    ["default", darkThemes, cohort],
    ["default", iceberg, icebergSpecimens],
  ],
  variants: [
    ["filled", ["eclipse"], cohort.slice(0, 3)],
    ["ghost", ["eclipse"], cohort.slice(0, 3)],
    ["elevation", ["eclipse"], cohort.slice(3)],
    ["tonal", ["eclipse"], cohort.slice(3)],
  ],
} satisfies Record<string, Matrix>;

async function capture(page: Page, root: string, slug: string, theme: string, style: string) {
  const params = new URLSearchParams({ theme, density: "default", controlSize: "md" });
  const hasStyle = style !== "before" && style !== "absent";
  if (hasStyle) params.set("style", style);

  await page.goto("about:blank");
  await page.goto(`${root}/?${params}#components/${slug}`);
  const specimen = page.locator(".poodle-app-shell .poodle-component-page__section").first();
  await specimen.waitFor();

  await page.addStyleTag({ content: `
    *, *::before, *::after {
      transition: none !important;
      animation: none !important;
      caret-color: transparent !important;
    }
    .poodle-app-top-bar, .poodle-display-controls, .poodle-component-page__hero,
    .poodle-component-page__section ~ .poodle-component-page__section { display: none !important; }
    .poodle-app-main { flex: auto !important; overflow: visible !important; }
  ` });

  if (hasStyle) {
    await page.waitForFunction((name) =>
      document.querySelector(".poodle-app-shell")?.getAttribute("data-style") === name, style);
  } else if (style === "absent") {
    await page.evaluate(() => {
      document.querySelectorAll("[data-style]").forEach((node) => node.removeAttribute("data-style"));
    });
  }
  if (slug === "dialog") {
    await page.getByRole("button", { name: "View details", exact: true }).click();
    await page.locator(".poodle-dialog__surface").waitFor();
    await page.mouse.move(0, 0);
  }
  await page.evaluate(() => document.fonts.ready);
  // TextInput's async slug validation must settle before comparing its paint.
  if (slug === "text-input") await page.locator("#slug-field").locator("..").waitFor();
  if (slug === "text-input") await page.waitForTimeout(850);
  await page.evaluate(() => new Promise((done) =>
    requestAnimationFrame(() => requestAnimationFrame(done))));

  return slug === "dialog"
    ? await page.screenshot({ clip: { x: 256, y: 185, width: 768, height: 540 } })
    : await specimen.screenshot();
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

const [phase, output] = process.argv.slice(2);
if ((phase !== "baseline" && phase !== "compare" && phase !== "variants") || !output) {
  throw new Error("Usage: bun capture.ts baseline|compare|variants OUTPUT_DIRECTORY");
}

const previews = await startPreviews();
let browser: Browser | undefined;
const differences: Record<string, number> = {};

try {
  browser = await chromium.launch();
  const page = await browser.newPage({ viewport: { width: 1280, height: 900 }, deviceScaleFactor: 1 });
  await pinPage(page);
  for (const [style, themes, slugs] of matrices[phase]) {
    mkdirSync(resolve(output, style), { recursive: true });
    for (const theme of themes) {
      for (const slug of slugs) {
        const png = await capture(page, previews.urls.svelte, slug, theme, style);
        const file = `${theme}-${slug}.png`;
        writeFileSync(resolve(output, style, file), png);
        console.log(style, theme, slug);
        if (phase === "compare") {
          const before = PNG.sync.read(readFileSync(resolve(output, "before", file)));
          differences[`${style}/${file}`] = countPixelChanges(before, PNG.sync.read(png));
        }
      }
    }
  }
} finally {
  await browser?.close();
  await previews.stop();
}

if (phase === "compare") {
  console.log(JSON.stringify(differences, null, 2));
  const hasUnexpectedDiff = Object.entries(differences).some(([key, count]) =>
    key.includes("/iceberg-") ? count === 0 : count !== 0);
  if (hasUnexpectedDiff) process.exitCode = 1;
}
