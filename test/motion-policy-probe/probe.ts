/**
 * Headless Chromium + WebKit proof for g16.034 shared motion policy.
 *
 *   bun test/motion-policy-probe/probe.ts --browser=chromium
 *   bun test/motion-policy-probe/probe.ts --browser=webkit
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
const port = 4188;
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
  throw new Error(`motion-policy fixture on :${port} did not start`);
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

async function runFramework(page: Page, framework: string, browserName: string): Promise<void> {
  console.log(`\n${browserName} / ${framework}`);
  const section = `[data-framework="${framework}"]`;

  await page.waitForSelector(`${section} [data-case="disclosure"] [data-motion-ready="true"]`);
  const beforeClose = await page.evaluate((sel) => {
    const host = document.querySelector(sel) as HTMLElement;
    const clip = host.querySelector(".poodle-collapsible__content-clip") as HTMLElement;
    const content = host.querySelector(".poodle-collapsible__content") as HTMLElement;
    return {
      clipHeight: clip.getBoundingClientRect().height,
      hidden: content.hasAttribute("hidden"),
    };
  }, `${section} [data-case="disclosure"]`);
  check(
    `${framework} open disclosure occupies layout`,
    beforeClose.clipHeight > 8 && !beforeClose.hidden,
    `h=${beforeClose.clipHeight.toFixed(1)} hidden=${beforeClose.hidden}`,
  );

  await page.click(`${section} [data-case="disclosure"] .poodle-collapsible__trigger`);
  const closing = await page.evaluate((sel) => {
    const host = document.querySelector(sel) as HTMLElement;
    const clip = host.querySelector(".poodle-collapsible__content-clip") as HTMLElement;
    const content = host.querySelector(".poodle-collapsible__content") as HTMLElement;
    return {
      clipHeight: clip.getBoundingClientRect().height,
      hidden: content.hasAttribute("hidden"),
      inert: content.hasAttribute("inert"),
    };
  }, `${section} [data-case="disclosure"]`);
  check(
    `${framework} closing disclosure stays in layout and is inert`,
    closing.clipHeight > 0 && !closing.hidden && closing.inert,
    `h=${closing.clipHeight.toFixed(1)} hidden=${closing.hidden} inert=${closing.inert}`,
  );

  const indicatorBefore = await page.evaluate((sel) => {
    const host = document.querySelector(sel) as HTMLElement;
    const indicator = host.querySelector(".poodle-tabs__indicator") as HTMLElement;
    const box = indicator.getBoundingClientRect();
    return { width: box.width, left: box.left };
  }, `${section} [data-case="tabs"]`);
  check(
    `${framework} underline indicator paints`,
    indicatorBefore.width > 8,
    `w=${indicatorBefore.width.toFixed(1)}`,
  );
  await page.evaluate((sel) => {
    const host = document.querySelector(sel) as HTMLElement;
    host.style.width = "8rem";
  }, `${section} [data-case="tabs"]`);
  await frames(page, 4);
  const indicatorAfter = await page.evaluate((sel) => {
    const host = document.querySelector(sel) as HTMLElement;
    const indicator = host.querySelector(".poodle-tabs__indicator") as HTMLElement;
    return indicator.getBoundingClientRect().width;
  }, `${section} [data-case="tabs"]`);
  check(
    `${framework} tabs indicator snaps on resize`,
    Math.abs(indicatorAfter - indicatorBefore.width) > 1,
    `before=${indicatorBefore.width.toFixed(1)} after=${indicatorAfter.toFixed(1)}`,
  );

  const liveBefore = await page.evaluate((sel) => {
    const toasts = [...document.querySelectorAll(`${sel} .poodle-toast`)].map((node) => {
      const toast = node as HTMLElement;
      return {
        id: toast.querySelector("strong")?.textContent,
        phase: toast.dataset.motion,
        live: toast.getAttribute("aria-live"),
      };
    });
    return toasts;
  }, `${section} [data-case="toast"]`);
  check(
    `${framework} preloaded toasts do not enter`,
    liveBefore.length === 2 && liveBefore.every((toast) => toast.phase === "settled"),
    JSON.stringify(liveBefore),
  );
  check(
    `${framework} danger toast is assertive once`,
    liveBefore.some((toast) => toast.id === "Danger" && toast.live === "assertive"),
    JSON.stringify(liveBefore),
  );

  await page.focus(`${section} [data-case="toast"] [aria-label="Dismiss Danger"]`);
  await page.click(`${section} [data-case="toast"] [aria-label="Dismiss Danger"]`);
  await frames(page, 2);
  const afterDismiss = await page.evaluate((sel) => {
    const toasts = [...document.querySelectorAll(`${sel} .poodle-toast`)].map((node) => {
      const toast = node as HTMLElement;
      return {
        id: toast.querySelector("strong")?.textContent,
        phase: toast.dataset.motion,
        live: toast.getAttribute("aria-live"),
        hidden: toast.getAttribute("aria-hidden"),
      };
    });
    return {
      toasts,
      active: (document.activeElement as HTMLElement | null)?.getAttribute("aria-label") ?? null,
    };
  }, `${section} [data-case="toast"]`);
  const exiting = afterDismiss.toasts.find((toast) => toast.phase === "exit");
  check(
    `${framework} exit remnant is inert to live region`,
    Boolean(exiting && exiting.hidden === "true" && exiting.live === null),
    JSON.stringify(afterDismiss.toasts),
  );
  check(
    `${framework} focused dismissal moves to remaining toast`,
    afterDismiss.active === "Dismiss Kept",
    `active=${afterDismiss.active}`,
  );

  const reduced = await page.evaluate((sel) => {
    const button = document.querySelector(`${sel} .poodle-icon-button`) as HTMLElement;
    return getComputedStyle(button).transitionProperty;
  }, `${section} [data-case="icon-reduced"]`);
  check(
    `${framework} reduced IconButton transitions opacity only`,
    reduced === "opacity",
    `transition-property=${reduced}`,
  );
}

try {
  for (const [browserName, browserType] of engines) {
    let browser: Browser | undefined;
    try {
      browser = await browserType.launch({ headless: true });
      const page = await browser.newPage({ viewport: { width: 960, height: 720 } });
      await page.goto(url, { waitUntil: "networkidle" });
      await page.waitForSelector('[data-framework="svelte"] .poodle-collapsible');
      await page.waitForSelector('[data-framework="react"] .poodle-collapsible');
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

console.log("\nall motion-policy probe checks passed");
