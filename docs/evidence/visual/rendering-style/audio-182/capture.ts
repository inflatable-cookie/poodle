// Evidence for poodle#182: the audio control stylesheets on the fill, surface and selection roles.
//
//   bun docs/evidence/visual/rendering-style/audio-182/capture.ts capture OUTPUT_DIR style=default,quiet
//   bun docs/evidence/visual/rendering-style/audio-182/capture.ts compare BEFORE_DIR AFTER_DIR
//   bun docs/evidence/visual/rendering-style/audio-182/capture.ts compose SOURCE_DIR OUTPUT_DIR
//
// `capture` renders each specimen in the Svelte preview at `style=<name>` for Eclipse
// (dark) and Clay (light). `compare` byte-compares every default-style PNG and prints the
// count; byte equality of the same render is zero pixel difference. `compose` places each
// default specimen beside the quiet specimen for review.
import { mkdirSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { chromium, type Page } from "playwright";
import { PNG } from "pngjs";
import { pinPage } from "../../../../../test/visual/capture";

export const slugs = [
  "audio-meter",
  "audio-switch",
  "fader",
  "gain-reduction-meter",
  "keyboard",
  "knob",
  "meter",
  "meter-surface",
  "range-slider",
  "slider",
  "tri-state-switch",
  "xy-pad",
  "value-readout",
  "drag-number-field",
  "duration-input",
  "envelope-editor",
  "mod-matrix-grid",
] as const;
export const themes = ["eclipse", "clay"] as const;

// SLUGS=xy-pad,fader re-captures only those specimens (the same procedure, so
// a before/after pair for one specimen stays comparable).
function selectedSlugs(): readonly string[] {
  const only = process.env.SLUGS?.split(",").filter(Boolean);
  return only && only.length > 0 ? only : slugs;
}

async function capture(page: Page, root: string, slug: string, theme: string, style: string) {
  const params = new URLSearchParams({ theme, density: "compact", controlSize: "sm", style });
  await page.goto("about:blank");
  await page.goto(`${root}/?${params}#components/${slug}`);
  const specimen = page.locator(".poodle-app-shell .poodle-component-page__section").first();
  await specimen.waitFor({ timeout: 60_000 });
  await page.addStyleTag({
    content: `
    *, *::before, *::after { transition: none !important; animation: none !important; caret-color: transparent !important; }
    .poodle-app-top-bar, .poodle-display-controls, .poodle-component-page__hero,
    .poodle-component-page__section ~ .poodle-component-page__section { display: none !important; }
    .poodle-app-main { flex: auto !important; overflow: visible !important; }
  `,
  });
  await page.waitForFunction(
    (name) => document.querySelector(".poodle-app-shell")?.getAttribute("data-style") === name,
    style,
  );
  const target = specimen;
  // Exercise the real focus/selection path when capturing the selected point evidence.
  if (slug === "envelope-editor" && process.env.SELECT_ENVELOPE === "1") {
    const editor = specimen.locator(".poodle-envelope-editor").nth(2);
    await editor.locator(".poodle-envelope-editor__point-control").nth(2).focus();
    await editor.locator('.poodle-envelope-editor-visual__point[data-selected="true"]').waitFor();
  }
  await page.evaluate(() => document.fonts.ready);
  await page.evaluate(() => new Promise((done) => requestAnimationFrame(() => requestAnimationFrame(done))));
  return await target.screenshot();
}

// The preview must be the checkout under test. The harness's fixed ports are reused when
// something already listens there, so this takes an explicit URL instead of starting one:
//   PREVIEW_URL=http://127.0.0.1:4374 bun .../capture.ts capture OUTPUT style=default,quiet
async function runCapture(output: string, styles: string[]) {
  const root = process.env.PREVIEW_URL;
  if (!root) throw new Error("Set PREVIEW_URL to a Svelte preview started from this checkout.");
  const browser = await chromium.launch();
  try {
    // A fresh page per capture: one reused page stalls on navigation after about 29 captures.
    for (const style of styles) {
      const dir = resolve(output, style);
      mkdirSync(dir, { recursive: true });
      for (const theme of themes) {
        for (const slug of selectedSlugs()) {
          const page = await browser.newPage({ viewport: { width: 1280, height: 900 }, deviceScaleFactor: 1 });
          try {
            await pinPage(page);
            writeFileSync(resolve(dir, `${theme}-${slug}.png`), await capture(page, root, slug, theme, style));
          } finally {
            await page.close();
          }
          console.log(style, theme, slug);
        }
      }
    }
  } finally {
    await browser.close();
  }
}

function compare(before: string, after: string) {
  const files = readdirSync(resolve(before, "default")).filter((name) => name.endsWith(".png")).sort();
  const differing = files.filter((name) => {
    const a = readFileSync(resolve(before, "default", name));
    const b = readFileSync(resolve(after, "default", name));
    return !a.equals(b);
  });
  const summary = { compared: files.length, identical: files.length - differing.length, differing };
  console.log(JSON.stringify(summary, null, 2));
  if (differing.length > 0) process.exitCode = 1;
}

function compose(source: string, output: string) {
  mkdirSync(output, { recursive: true });
  const gap = 24;
  for (const slug of selectedSlugs()) {
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

const [phase, first, second] = process.argv.slice(2);
if (phase === "capture" && first && second) {
  await runCapture(first, second.replace(/^style=/, "").split(","));
} else if (phase === "compare" && first && second) {
  compare(first, second);
} else if (phase === "compose" && first && second) {
  compose(first, second);
} else {
  throw new Error("Usage: capture OUTPUT style=a,b | compare BEFORE AFTER | compose SOURCE OUTPUT");
}
