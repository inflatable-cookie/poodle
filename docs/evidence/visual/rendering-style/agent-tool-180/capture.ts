// Evidence for poodle#180: bring the agent and tool components into the rendering style.
//
//   bun docs/evidence/visual/rendering-style/agent-tool-180/capture.ts capture OUTPUT_DIR style=default,quiet
//   bun docs/evidence/visual/rendering-style/agent-tool-180/capture.ts compare BEFORE_DIR AFTER_DIR
//   bun docs/evidence/visual/rendering-style/agent-tool-180/capture.ts compose SOURCE_DIR OUTPUT_DIR
import { mkdirSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { chromium, type Page } from "playwright";
import { PNG } from "pngjs";
import { pinPage } from "../../../../../test/visual/capture";

export const slugs = [
  "agent-chat-input",
  "agent-message",
  "agent-plan",
  "agent-plan-record",
  "agent-question-record",
  "agent-subagent",
  "agent-transcript",
  "tool-call",
  "tool-call-group",
  "changed-files",
  "model-connection-picker",
  "model-connection-setup",
  "model-connection-card",
  "model-catalogue-editor",
  "agent-question",
] as const;

export const themes = ["eclipse", "clay"] as const;

const VIEWPORT = { width: 1280, height: 900 };
const SPECIMEN_SELECTOR = ".poodle-app-shell .poodle-component-page__section";

async function startSveltePreview(repoRoot: string, port: number) {
  const proc = Bun.spawn(
    ["bun", "run", "--cwd", "packages/svelte/preview", "dev", "--port", String(port), "--strictPort", "--host", "127.0.0.1"],
    { cwd: repoRoot, stdout: "pipe", stderr: "pipe" },
  );
  const url = `http://127.0.0.1:${port}`;
  const deadline = Date.now() + 60_000;
  while (Date.now() < deadline) {
    try {
      const res = await fetch(`${url}/`, { signal: AbortSignal.timeout(2000) });
      if (res.ok) {
        return {
          url,
          stop: async () => {
            proc.kill();
            await proc.exited;
          },
        };
      }
    } catch {
      // keep polling
    }
    await new Promise((r) => setTimeout(r, 250));
  }
  proc.kill();
  throw new Error(`Svelte preview failed to start on port ${port}`);
}

async function captureOne(page: Page, root: string, slug: string, theme: string, style: string) {
  const params = new URLSearchParams({ theme, density: "default", controlSize: "md", style });
  await page.setViewportSize(VIEWPORT);
  await page.goto("about:blank");
  await page.goto(`${root}/?${params}#components/${slug}`, { waitUntil: "load", timeout: 60_000 });
  const specimen = page.locator(SPECIMEN_SELECTOR).first();
  await specimen.waitFor({ state: "visible", timeout: 60_000 });

  await page.addStyleTag({
    content: `
      *, *::before, *::after {
        transition-duration: 0s !important;
        transition-delay: 0s !important;
        animation-duration: 0s !important;
        animation-delay: 0s !important;
        animation-iteration-count: 1 !important;
        caret-color: transparent !important;
      }
      html { scroll-behavior: auto !important; }
      .poodle-app-top-bar, .poodle-display-controls, .poodle-component-page__hero,
      .poodle-component-page__section ~ .poodle-component-page__section { display: none !important; }
    `,
  });

  await page.waitForFunction(
    (name) => document.querySelector(".poodle-app-shell")?.getAttribute("data-style") === name,
    style,
  );

  const specimenHeight = await page.evaluate((selector) => {
    const el = document.querySelector<HTMLElement>(selector);
    if (!el) return 0;
    for (let node = el.parentElement; node; node = node.parentElement) {
      const styles = getComputedStyle(node);
      if (styles.overflow !== "visible" || styles.overflowY !== "visible") {
        node.style.setProperty("overflow", "visible", "important");
        node.style.setProperty("height", "auto", "important");
        node.style.setProperty("max-height", "none", "important");
      }
    }
    document.documentElement.style.setProperty("overflow", "visible", "important");
    document.body.style.setProperty("overflow", "visible", "important");
    return Math.ceil(el.getBoundingClientRect().height);
  }, SPECIMEN_SELECTOR);

  await page.setViewportSize({
    width: VIEWPORT.width,
    height: Math.min(Math.max(specimenHeight + 200, VIEWPORT.height), 14_000),
  });

  await page.evaluate((selector) => {
    window.scrollTo(0, 0);
    const el = document.querySelector<HTMLElement>(selector);
    if (!el) return;
    const fraction = el.getBoundingClientRect().top % 1;
    if (fraction !== 0) el.style.marginTop = `${-fraction}px`;
  }, SPECIMEN_SELECTOR);

  await page.evaluate(() => document.fonts.ready);
  await page.waitForFunction(() => document.fonts.status === "loaded");
  await page.evaluate(() => new Promise((done) => requestAnimationFrame(() => requestAnimationFrame(done))));

  let stable = 0;
  let last = -1;
  for (let i = 0; i < 20 && stable < 3; i++) {
    const h = await page.evaluate(
      (s) => Math.round(document.querySelector(s)?.getBoundingClientRect().height ?? -1),
      SPECIMEN_SELECTOR,
    );
    stable = h === last ? stable + 1 : 0;
    last = h;
    if (stable < 3) await page.waitForTimeout(80);
  }

  return await specimen.screenshot({ animations: "disabled" });
}

async function runCapture(output: string, styles: string[]) {
  const repoRoot = process.cwd();
  const port = Number(process.env.PREVIEW_PORT ?? 4389);
  const server = await startSveltePreview(repoRoot, port);
  const browser = await chromium.launch();
  try {
    for (const style of styles) {
      const dir = resolve(output, style);
      mkdirSync(dir, { recursive: true });
      for (const theme of themes) {
        for (const slug of slugs) {
          const page = await browser.newPage({ viewport: VIEWPORT, deviceScaleFactor: 1 });
          try {
            await pinPage(page);
            writeFileSync(resolve(dir, `${theme}-${slug}.png`), await captureOne(page, server.url, slug, theme, style));
          } finally {
            await page.close();
          }
          console.log(style, theme, slug);
        }
      }
    }
  } finally {
    await browser.close();
    await server.stop();
  }
}

function compare(before: string, after: string) {
  const files = readdirSync(resolve(before, "default")).filter((name) => name.endsWith(".png")).sort();
  const results: Record<string, number> = {};
  for (const name of files) {
    const a = PNG.sync.read(readFileSync(resolve(before, "default", name)));
    const b = PNG.sync.read(readFileSync(resolve(after, "default", name)));
    if (a.width !== b.width || a.height !== b.height) {
      throw new Error(`Size mismatch for ${name}: ${a.width}x${a.height} vs ${b.width}x${b.height}`);
    }
    let changed = 0;
    for (let i = 0; i < a.data.length; i += 4) {
      if (a.data.readUInt32LE(i) !== b.data.readUInt32LE(i)) changed++;
    }
    results[name] = changed;
  }
  const differing = Object.entries(results).filter(([, count]) => count > 0);
  console.log(JSON.stringify({ compared: files.length, identical: files.length - differing.length, differing }, null, 2));
  if (differing.length > 0) process.exitCode = 1;
}

function compose(source: string, output: string) {
  mkdirSync(output, { recursive: true });
  const gap = 24;
  for (const slug of slugs) {
    if (slug === "agent-question") continue;
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
