/**
 * Headless Chromium + WebKit evidence for the drag-drop web substrate.
 *
 *   bun test/drag-drop/probe.ts --browser=chromium
 *   bun test/drag-drop/probe.ts --browser=webkit
 */

import { chromium, webkit, type BrowserType, type Page } from "playwright";
import { fileURLToPath } from "node:url";

const browserFlag = process.argv.find((arg) => arg.startsWith("--browser="))?.slice("--browser=".length);
const engines: Array<[string, BrowserType]> = ([["chromium", chromium], ["webkit", webkit]] as Array<[string, BrowserType]>)
  .filter(([name]) => !browserFlag || browserFlag === name);

if (engines.length === 0) {
  throw new Error(`Unknown --browser=${browserFlag}`);
}

const fixtureRoot = fileURLToPath(new URL(".", import.meta.url));
const repoRoot = fileURLToPath(new URL("../..", import.meta.url));
const viteBin = fileURLToPath(new URL("../../packages/svelte/preview/node_modules/vite/bin/vite.js", import.meta.url));
const port = 4179;
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
  throw new Error(`drag-drop fixture on :${port} did not start`);
}

const child = Bun.spawn(
  ["bun", viteBin, "--config", `${fixtureRoot}/vite.config.ts`, "--port", String(port), "--strictPort", "--host", "127.0.0.1"],
  {
    cwd: repoRoot,
    stdout: "inherit",
    stderr: "inherit",
  },
);
await waitForServer();

async function frames(page: Page, count = 2): Promise<void> {
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

async function box(page: Page, selector: string): Promise<{ x: number; y: number; width: number; height: number }> {
  const handle = await page.locator(selector).boundingBox();
  if (!handle) throw new Error(`no box for ${selector}`);
  return handle;
}

async function dispatchPointer(
  page: Page,
  type: "pointerdown" | "pointermove" | "pointerup",
  selector: string,
  clientX: number,
  clientY: number,
  pointerType = "mouse",
  pointerId = 1,
): Promise<void> {
  await page.evaluate(
    ({ type, selector, clientX, clientY, pointerType, pointerId }) => {
      const node = document.querySelector(selector);
      if (!(node instanceof HTMLElement)) throw new Error(`missing ${selector}`);
      node.dispatchEvent(
        new PointerEvent(type, {
          bubbles: true,
          cancelable: true,
          pointerId,
          pointerType,
          isPrimary: true,
          button: 0,
          buttons: type === "pointerup" ? 0 : 1,
          clientX,
          clientY,
        }),
      );
    },
    { type, selector, clientX, clientY, pointerType, pointerId },
  );
}

async function run(page: Page, name: string): Promise<void> {
  await page.goto(url, { waitUntil: "load" });
  await page.locator("#source").waitFor();

  const source = await box(page, "#source");
  const target = await box(page, "#target");
  const sx = source.x + source.width / 2;
  const sy = source.y + source.height / 2;
  const tx = target.x + target.width / 2;
  const ty = target.y + target.height / 2;

  await dispatchPointer(page, "pointerdown", "#source", sx, sy);
  await dispatchPointer(page, "pointermove", "#source", sx + 24, sy);
  await frames(page);
  const captured = await page.locator("#probe").getAttribute("data-captured");
  const phase = await page.locator("#probe").getAttribute("data-phase");
  check(
    `${name}: pointer capture after activation`,
    captured === "true" && phase === "dragging",
    `captured=${captured} phase=${phase}`,
  );

  await dispatchPointer(page, "pointermove", "#source", tx, ty);
  await frames(page);
  const preview = await page.locator(".poodle-drag-preview").count();
  check(`${name}: preview visible while dragging`, preview === 1, `count=${preview}`);

  await dispatchPointer(page, "pointerup", "#source", tx, ty);
  const afterPhase = await page.locator("#probe").getAttribute("data-phase");
  const afterPreview = await page.locator(".poodle-drag-preview").count();
  check(
    `${name}: preview and session clear after drop`,
    afterPhase === "idle" && afterPreview === 0,
    `phase=${afterPhase} preview=${afterPreview}`,
  );

  await page.reload({ waitUntil: "load" });
  await page.locator("#source").waitFor();
  await page.click("#shift");
  await frames(page);
  const shifted = await box(page, "#target");
  const origin = await box(page, "#source");
  const ox = origin.x + origin.width / 2;
  const oy = origin.y + origin.height / 2;
  await dispatchPointer(page, "pointerdown", "#source", ox, oy);
  await dispatchPointer(page, "pointermove", "#source", ox + 24, oy);
  await dispatchPointer(page, "pointermove", "#source", shifted.x + shifted.width / 2, shifted.y + shifted.height / 2);
  await frames(page);
  const posture = await page.locator("#probe").getAttribute("data-posture");
  await dispatchPointer(page, "pointerup", "#source", shifted.x + shifted.width / 2, shifted.y + shifted.height / 2);
  check(`${name}: geometry invalidation follows a moved target`, posture === "accepted", `posture=${posture}`);

  await page.reload({ waitUntil: "load" });
  await page.locator("#source").waitFor();
  const start = await box(page, "#source");
  await dispatchPointer(page, "pointerdown", "#source", start.x + start.width / 2, start.y + start.height / 2, "touch", 7);
  await dispatchPointer(page, "pointermove", "#source", start.x + start.width / 2, start.y + start.height / 2 + 40, "touch", 7);
  const touchPhase = await page.locator("#probe").getAttribute("data-phase");
  const touchCaptured = await page.locator("#probe").getAttribute("data-captured");
  check(
    `${name}: touch scroll wins before hold`,
    touchPhase === "idle" && touchCaptured === "false",
    `phase=${touchPhase} captured=${touchCaptured}`,
  );

  await page.reload({ waitUntil: "load" });
  await page.locator("#source").waitFor();
  await page.focus("#source");
  await page.keyboard.press("Space");
  await page.keyboard.press("ArrowDown");
  await page.keyboard.press("Enter");
  const focused = await page.evaluate(() => document.activeElement?.id ?? "");
  const idle = await page.locator("#probe").getAttribute("data-phase");
  check(`${name}: keyboard drop restores focus`, focused === "source" && idle === "idle", `focus=${focused} phase=${idle}`);
}

for (const [name, type] of engines) {
  console.log(`\n=== ${name} ===`);
  const browser = await type.launch();
  const page = await browser.newPage({ viewport: { width: 800, height: 600 } });
  try {
    await run(page, name);
  } catch (error) {
    failures += 1;
    console.log(`  FAIL  ${name}: ${error instanceof Error ? error.message : String(error)}`);
  }
  await browser.close();
}

child.kill();
await child.exited;

if (failures > 0) {
  console.error(`\n${failures} drag-drop browser check(s) failed`);
  process.exit(1);
}

console.log("\nall drag-drop browser checks passed");
