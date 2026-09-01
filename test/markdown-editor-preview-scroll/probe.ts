/**
 * Headless Chromium + WebKit geometry/scroll proof for g16.035.
 *
 * A definite 16rem host must keep a long MarkdownEditor preview inside the host
 * while the preview scrolls; a short unconstrained preview stays natural.
 *
 *   bun test/markdown-editor-preview-scroll/probe.ts --browser=chromium
 *   bun test/markdown-editor-preview-scroll/probe.ts --browser=webkit
 */

import { chromium, webkit, type Browser, type BrowserType, type Page } from "playwright";
import { fileURLToPath } from "node:url";

const browserFlag = process.argv.find((arg) => arg.startsWith("--browser="))?.slice("--browser=".length);
const engines: Array<[string, BrowserType]> = (
  [
    ["chromium", chromium],
    ["webkit", webkit],
  ] as Array<[string, BrowserType]>
).filter(([name]) => !browserFlag || browserFlag === name);

if (engines.length === 0) {
  throw new Error(`Unknown --browser=${browserFlag}`);
}

const fixtureRoot = fileURLToPath(new URL(".", import.meta.url));
const repoRoot = fileURLToPath(new URL("../..", import.meta.url));
const viteBin = fileURLToPath(
  new URL("../../packages/svelte/preview/node_modules/vite/bin/vite.js", import.meta.url),
);
const port = 4187;
const url = `http://127.0.0.1:${port}/`;

let failures = 0;

function check(label: string, ok: boolean, detail = ""): void {
  if (!ok) failures += 1;
  console.log(`${ok ? "  ok  " : "  FAIL"}  ${label}${detail ? `  — ${detail}` : ""}`);
}

async function waitForServer(timeoutMs = 30_000): Promise<void> {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    try {
      const res = await fetch(url, { signal: AbortSignal.timeout(1000) });
      if (res.ok) return;
    } catch {
      await new Promise((resolve) => setTimeout(resolve, 150));
    }
  }
  throw new Error(`markdown-editor preview-scroll fixture on :${port} did not start`);
}

const child = Bun.spawn(
  [
    "bun",
    viteBin,
    "--config",
    `${fixtureRoot}/vite.config.ts`,
    "--port",
    String(port),
    "--strictPort",
    "--host",
    "127.0.0.1",
  ],
  {
    cwd: repoRoot,
    stdout: "inherit",
    stderr: "inherit",
  },
);

await waitForServer();

async function frames(page: Page, count = 3): Promise<void> {
  await page.evaluate((n) => {
    return new Promise<void>((resolve) => {
      const tick = (left: number) => {
        if (left <= 0) {
          resolve();
          return;
        }
        requestAnimationFrame(() => tick(left - 1));
      };
      tick(n);
    });
  }, count);
}

type CaseSnapshot = {
  hostHeight: number;
  editorHeight: number;
  previewClientHeight: number;
  previewScrollHeight: number;
  previewScrollTop: number;
  siblingTop: number;
  editorTop: number;
  textareaClientHeight: number | null;
  bodyClientHeight: number;
  maxHeight: string;
  rootHeight: string;
};

async function snapshot(page: Page, framework: string, caseName: string): Promise<CaseSnapshot> {
  return page.evaluate(
    ({ framework, caseName }) => {
      const section = document.querySelector(`[data-framework="${framework}"]`) as HTMLElement;
      const host = section.querySelector(`[data-case="${caseName}"]`) as HTMLElement;
      const editor = host.querySelector(".poodle-md-editor") as HTMLElement;
      const preview = editor.querySelector(".poodle-md-editor__preview") as HTMLElement;
      const body = editor.querySelector(".poodle-md-editor__body") as HTMLElement;
      const textarea = editor.querySelector("textarea.poodle-md-editor__textarea") as HTMLTextAreaElement | null;
      const sibling = section.querySelector(`[data-sibling="${caseName}"]`) as HTMLElement | null;
      const style = getComputedStyle(editor);
      return {
        hostHeight: host.getBoundingClientRect().height,
        editorHeight: editor.getBoundingClientRect().height,
        previewClientHeight: preview.clientHeight,
        previewScrollHeight: preview.scrollHeight,
        previewScrollTop: preview.scrollTop,
        siblingTop: sibling?.getBoundingClientRect().top ?? -1,
        editorTop: editor.getBoundingClientRect().top,
        textareaClientHeight: textarea?.clientHeight ?? null,
        bodyClientHeight: body.clientHeight,
        maxHeight: style.maxHeight,
        rootHeight: style.height,
      };
    },
    { framework, caseName },
  );
}

async function scrollPreview(page: Page, framework: string, caseName: string, top: number): Promise<number> {
  return page.evaluate(
    ({ framework, caseName, top }) => {
      const preview = document
        .querySelector(`[data-framework="${framework}"] [data-case="${caseName}"] .poodle-md-editor__preview`) as HTMLElement;
      preview.scrollTop = top;
      return preview.scrollTop;
    },
    { framework, caseName, top },
  );
}

async function runFramework(page: Page, framework: string, browserName: string): Promise<void> {
  console.log(`\n${browserName} / ${framework}`);

  const beforePreview = await snapshot(page, framework, "constrained-preview");
  check(
    `${framework} constrained preview stays within 16rem host`,
    beforePreview.editorHeight <= beforePreview.hostHeight + 1,
    `editor=${beforePreview.editorHeight.toFixed(1)} host=${beforePreview.hostHeight.toFixed(1)}`,
  );
  check(
    `${framework} constrained preview overflows internally`,
    beforePreview.previewScrollHeight > beforePreview.previewClientHeight + 8,
    `scroll=${beforePreview.previewScrollHeight} client=${beforePreview.previewClientHeight}`,
  );

  const scrolled = await scrollPreview(page, framework, "constrained-preview", 120);
  await frames(page, 2);
  const afterPreview = await snapshot(page, framework, "constrained-preview");
  check(
    `${framework} preview scrollTop moves`,
    scrolled >= 100 && afterPreview.previewScrollTop >= 100,
    `scrolled=${scrolled} after=${afterPreview.previewScrollTop}`,
  );
  check(
    `${framework} editor root stationary while preview scrolls`,
    Math.abs(afterPreview.editorTop - beforePreview.editorTop) < 1 &&
      Math.abs(afterPreview.editorHeight - beforePreview.editorHeight) < 1,
    `top ${beforePreview.editorTop}→${afterPreview.editorTop}, h ${beforePreview.editorHeight}→${afterPreview.editorHeight}`,
  );
  check(
    `${framework} sibling stationary while preview scrolls`,
    Math.abs(afterPreview.siblingTop - beforePreview.siblingTop) < 1,
    `sibling ${beforePreview.siblingTop}→${afterPreview.siblingTop}`,
  );

  const split = await snapshot(page, framework, "constrained-split");
  check(
    `${framework} split stays within 16rem host`,
    split.editorHeight <= split.hostHeight + 1,
    `editor=${split.editorHeight.toFixed(1)} host=${split.hostHeight.toFixed(1)}`,
  );
  check(
    `${framework} split preview overflows internally`,
    split.previewScrollHeight > split.previewClientHeight + 8,
    `scroll=${split.previewScrollHeight} client=${split.previewClientHeight}`,
  );
  if (split.textareaClientHeight !== null) {
    check(
      `${framework} split panes share one body height`,
      split.textareaClientHeight <= split.bodyClientHeight + 1 &&
        split.previewClientHeight <= split.bodyClientHeight + 1,
      `textarea=${split.textareaClientHeight} preview=${split.previewClientHeight} body=${split.bodyClientHeight}`,
    );
  }

  const natural = await snapshot(page, framework, "natural-short");
  const viewportHeight = await page.evaluate(() => window.innerHeight);
  check(
    `${framework} short unconstrained preview stays natural`,
    natural.editorHeight < viewportHeight * 0.5 &&
      natural.rootHeight !== `${viewportHeight}px` &&
      !natural.maxHeight.includes("100vh"),
    `editor=${natural.editorHeight.toFixed(1)} maxHeight=${natural.maxHeight} height=${natural.rootHeight}`,
  );
  check(
    `${framework} short preview does not invent a scroll pane`,
    natural.previewScrollHeight <= natural.previewClientHeight + 1,
    `scroll=${natural.previewScrollHeight} client=${natural.previewClientHeight}`,
  );
}

try {
  for (const [browserName, browserType] of engines) {
    let browser: Browser | undefined;
    try {
      browser = await browserType.launch({ headless: true });
      const page = await browser.newPage({ viewport: { width: 960, height: 720 } });
      await page.goto(url, { waitUntil: "networkidle" });
      await page.waitForSelector('[data-framework="svelte"] .poodle-md-editor');
      await page.waitForSelector('[data-framework="react"] .poodle-md-editor');
      await frames(page, 4);
      await runFramework(page, "svelte", browserName);
      await runFramework(page, "react", browserName);
    } finally {
      await browser?.close();
    }
  }
} finally {
  child.kill();
  await child.exited;
}

if (failures > 0) {
  console.error(`\n${failures} check(s) failed`);
  process.exit(1);
}

console.log("\nall markdown-editor preview-scroll checks passed");
