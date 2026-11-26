// Compare production preview builds without starting a server.
// Build base 037ddb7bad540f25ddcdf4329b9da98d494e2578 and the candidate with
// `effigy web:dist && effigy svelte:build`,
// copying packages/svelte/preview/dist after each build. Then run:
// bun docs/evidence/visual/rendering-style/editors-183/capture.ts BEFORE_DIST AFTER_DIST OUTPUT_DIR
// The output includes both sets of PNGs and a SHA-256 receipt for each pair.
// Mismatches are recaptured up to twice to account for transient rendering;
// the receipt records the attempt count, and persistent differences fail.
import { createHash } from "node:crypto";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { chromium } from "playwright";
import { pinPage } from "../../../../../test/visual/capture";

const slugs = [
  "block-editor", "code-editor", "markdown-editor", "rich-text-editor", "code",
  "color-picker", "file-upload", "editable-list", "ref-select", "filter-builder",
  "filter-toolbar", "order-by", "split-button", "collapse-toggle", "card-radio-group",
  "card-toggle-group", "embed-input", "listbox", "agent-chat-input", "agent-message",
  "list-container", "meta-bar", "meta-item", "dock-region", "token-input",
];
const [beforeDist, afterDist, output] = process.argv.slice(2);
if (!beforeDist || !afterDist || !output) {
  throw new Error("Usage: capture.ts BEFORE_DIST AFTER_DIST OUTPUT_DIR");
}
const themes = ["eclipse", "clay"];
const captures: Array<{ theme: string; slug: string; before: string; after: string; identical: boolean; attempts: Array<{ before: string; after: string }> }> = [];
const browser = await chromium.launch();
try {
  for (const slug of slugs) {
    for (const theme of themes) {
      let hashes: string[] = [];
      const attempts: Array<{ before: string; after: string }> = [];
      do {
        hashes = [];
        for (const [name, dist] of [["before", beforeDist], ["after", afterDist]]) {
          const page = await browser.newPage({ viewport: { width: 1280, height: 900 }, deviceScaleFactor: 1 });
          try {
            await pinPage(page);
            await page.route("https://poodle.test/**", (route) => {
              const path = new URL(route.request().url()).pathname.slice(1);
              return route.fulfill({ path: resolve(dist, path || "index.html") });
            });
            const params = new URLSearchParams({ theme, style: "default", density: "compact", controlSize: "sm", capture: "specimen" });
            await page.goto(`https://poodle.test/?${params}#components/${slug}`);
            const target = page.locator(`[data-specimen-capture="${slug}"]`);
            await target.waitFor({ timeout: 60_000 });
            await page.locator(`[data-capture-ready="${slug}"]`).waitFor({ timeout: 60_000 });
            if (slug === "code-editor") {
              await page.waitForFunction(() => {
                const contents = document.querySelectorAll(".poodle-code-editor__viewport .cm-content");
                return contents.length >= 4 && Array.from(contents).every((content) => content.textContent?.trim());
              }, undefined, { timeout: 60_000 });
            }
            await page.addStyleTag({ content: `
              *, *::before, *::after { animation: none !important; transition: none !important; caret-color: transparent !important; }
              .poodle-specimen-capture-frame { height: auto !important; min-height: 100vh; }
            ` });
            await page.evaluate(async () => {
              await document.fonts.ready;
              await new Promise((done) => requestAnimationFrame(() => requestAnimationFrame(done)));
            });
            await page.waitForTimeout(slug === "color-picker" ? 1500 : 500);
            const dir = resolve(output, name);
            mkdirSync(dir, { recursive: true });
            const path = resolve(dir, `${theme}-${slug}-attempt-${attempts.length + 1}.png`);
            await target.screenshot({ path });
            hashes.push(createHash("sha256").update(readFileSync(path)).digest("hex"));
          } finally {
            await page.close();
          }
        }
        const [before, after] = hashes;
        if (!before || !after) throw new Error("Missing capture hash");
        attempts.push({ before, after });
      } while (hashes[0] !== hashes[1] && attempts.length < 3);
      const [before, after] = hashes;
      if (!before || !after) throw new Error("Missing capture hash");
      captures.push({ theme, slug, before, after, identical: before === after, attempts });
      console.log(theme, slug, before === after ? "identical" : "different");
    }
  }
} finally {
  await browser.close();
}
const differing = captures.filter((capture) => !capture.identical);
writeFileSync(resolve(output, "default-identity.json"), JSON.stringify({ compared: captures.length, identical: captures.length - differing.length, captures }, null, 2) + "\n");
if (differing.length) process.exitCode = 1;
