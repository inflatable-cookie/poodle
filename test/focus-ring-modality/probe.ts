/**
 * Headless Chromium + WebKit proof that TokenInput's composite focus
 * treatment follows input modality. jsdom cannot prove computed styles.
 *
 *   bun test/focus-ring-modality/probe.ts --browser=chromium
 *   bun test/focus-ring-modality/probe.ts --browser=webkit
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
const port = 4192;
const url = `http://127.0.0.1:${port}/`;

let failures = 0;

type Chrome = {
  boxShadow: string;
  borderColor: string;
  backgroundColor: string;
  modality: string;
};

function check(label: string, ok: boolean, detail = ""): void {
  if (!ok) failures += 1;
  console.log(`${ok ? "  ok  " : "  FAIL"}  ${label}${detail ? `  — ${detail}` : ""}`);
}

function formatChrome(chrome: Chrome): string {
  return `modality=${chrome.modality} box-shadow=${JSON.stringify(chrome.boxShadow)} border-color=${JSON.stringify(chrome.borderColor)} background=${JSON.stringify(chrome.backgroundColor)}`;
}

function chromeEqual(a: Chrome, b: Chrome): boolean {
  return a.boxShadow === b.boxShadow && a.borderColor === b.borderColor && a.backgroundColor === b.backgroundColor;
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
  throw new Error(`focus-ring modality fixture on :${port} did not start`);
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

async function settle(page: Page): Promise<void> {
  await page.evaluate(
    () =>
      new Promise<void>((resolve) => {
        requestAnimationFrame(() => {
          window.setTimeout(resolve, 200);
        });
      }),
  );
}

async function readChrome(page: Page, framework: "svelte" | "react"): Promise<Chrome> {
  return page.evaluate((fw) => {
    const root = document.querySelector(`[data-framework="${fw}"] .poodle-token-input`);
    if (!(root instanceof HTMLElement)) {
      throw new Error(`missing ${fw} TokenInput root`);
    }
    const style = getComputedStyle(root);
    return {
      boxShadow: style.boxShadow,
      borderColor: style.borderColor,
      backgroundColor: style.backgroundColor,
      modality: document.documentElement.getAttribute("data-poodle-input-modality") ?? "",
    };
  }, framework);
}

async function runFramework(page: Page, framework: "svelte" | "react", browserName: string): Promise<void> {
  const section = `[data-framework="${framework}"]`;
  const input = page.locator(`${section} .poodle-token-input__control`);
  const before = page.locator(`${section} [data-before]`);
  await input.waitFor();

  await before.click();
  await settle(page);
  const resting = await readChrome(page, framework);
  console.log(`  evidence  ${browserName} ${framework} resting  ${formatChrome(resting)}`);

  await input.click();
  await settle(page);
  const pointer = await readChrome(page, framework);
  console.log(`  evidence  ${browserName} ${framework} pointer  ${formatChrome(pointer)}`);
  check(
    `${browserName} ${framework} pointer focus paints no composite treatment`,
    chromeEqual(pointer, resting) && pointer.modality === "pointer",
    formatChrome(pointer),
  );

  await before.click();
  await settle(page);
  await page.keyboard.press("Tab");
  await settle(page);
  const keyboard = await readChrome(page, framework);
  console.log(`  evidence  ${browserName} ${framework} keyboard  ${formatChrome(keyboard)}`);
  const focused = await page.evaluate((fw) => {
    const control = document.querySelector(`[data-framework="${fw}"] .poodle-token-input__control`);
    return document.activeElement === control;
  }, framework);
  check(`${browserName} ${framework} Tab lands on the live input`, focused);
  check(
    `${browserName} ${framework} keyboard focus paints the composite treatment`,
    keyboard.modality === "keyboard" &&
      !chromeEqual(keyboard, resting) &&
      keyboard.boxShadow !== resting.boxShadow &&
      keyboard.borderColor !== resting.borderColor,
    formatChrome(keyboard),
  );
}

try {
  for (const [browserName, browserType] of engines) {
    let browser: Browser | undefined;
    try {
      browser = await browserType.launch({ headless: true });
      const page = await browser.newPage({ viewport: { width: 960, height: 720 } });
      await page.goto(url, { waitUntil: "networkidle" });
      await page.waitForSelector('[data-framework="svelte"] .poodle-token-input');
      await page.waitForSelector('[data-framework="react"] .poodle-token-input');
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

console.log("\nall focus-ring input-modality checks passed");
