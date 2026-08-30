/**
 * Headless Chromium + WebKit evidence for the drag-drop web substrate.
 *
 * Mouse uses Playwright's real input. Chromium touch uses CDP
 * Input.dispatchTouchEvent. WebKit has no hold/move touch protocol, so touch
 * there is dispatched at document.elementFromPoint with real hold timing — not
 * replayed onto #source.
 *
 *   bun test/drag-drop/probe.ts --browser=chromium
 *   bun test/drag-drop/probe.ts --browser=webkit
 */

import { chromium, webkit, type BrowserType, type CDPSession, type Page } from "playwright";
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

function center(rect: { x: number; y: number; width: number; height: number }): { x: number; y: number } {
  return { x: rect.x + rect.width / 2, y: rect.y + rect.height / 2 };
}

async function probeAttr(page: Page, name: string): Promise<string> {
  return (await page.locator("#probe").getAttribute(`data-${name}`)) ?? "";
}

async function waitProbe(page: Page, name: string, expected: string, timeout = 2_000): Promise<string> {
  try {
    await page.waitForFunction(
      ({ name, expected }) => document.querySelector("#probe")?.getAttribute(`data-${name}`) === expected,
      { name, expected },
      { timeout },
    );
  } catch {
    // Fall through and report the actual value.
  }
  return probeAttr(page, name);
}

async function dispatchAtPoint(
  page: Page,
  type: "pointerdown" | "pointermove" | "pointerup",
  x: number,
  y: number,
  pointerType: string,
  pointerId: number,
): Promise<void> {
  await page.evaluate(
    ({ type, x, y, pointerType, pointerId }) => {
      const node = document.elementFromPoint(x, y) ?? document.body;
      node.dispatchEvent(
        new PointerEvent(type, {
          bubbles: true,
          cancelable: true,
          composed: true,
          pointerId,
          pointerType,
          isPrimary: true,
          button: 0,
          buttons: type === "pointerup" ? 0 : 1,
          clientX: x,
          clientY: y,
          view: window,
        }),
      );
    },
    { type, x, y, pointerType, pointerId },
  );
}

async function touchAt(
  page: Page,
  cdp: CDPSession | null,
  type: "touchStart" | "touchMove" | "touchEnd",
  x: number,
  y: number,
): Promise<void> {
  if (cdp) {
    await cdp.send("Input.dispatchTouchEvent", {
      type,
      touchPoints: type === "touchEnd" ? [] : [{ x, y }],
    });
    return;
  }
  const pointerType = "touch";
  const pointerId = 7;
  if (type === "touchStart") await dispatchAtPoint(page, "pointerdown", x, y, pointerType, pointerId);
  else if (type === "touchMove") await dispatchAtPoint(page, "pointermove", x, y, pointerType, pointerId);
  else await dispatchAtPoint(page, "pointerup", x, y, pointerType, pointerId);
}

async function run(page: Page, name: string, cdp: CDPSession | null): Promise<void> {
  await page.goto(url, { waitUntil: "load" });
  await page.locator("#source").waitFor();

  const source = center(await box(page, "#source"));
  const target = center(await box(page, "#target"));

  await page.mouse.move(source.x, source.y);
  await page.mouse.down();
  await page.mouse.move(source.x + 28, source.y, { steps: 8 });
  const phase = await waitProbe(page, "phase", "dragging");
  const captured = await waitProbe(page, "captured", "true");
  check(
    `${name}: real mouse capture after activation`,
    phase === "dragging" && captured === "true",
    `captured=${captured} phase=${phase}`,
  );

  await page.mouse.move(target.x, target.y, { steps: 10 });
  await frames(page);
  const preview = await page.locator(".poodle-drag-preview").count();
  const posture = await probeAttr(page, "posture");
  const hoverTarget = await probeAttr(page, "target");
  check(
    `${name}: captured drag routes over the target, not the source`,
    preview === 1 && posture === "accepted" && hoverTarget === "list",
    `preview=${preview} posture=${posture} target=${hoverTarget}`,
  );

  await page.mouse.up();
  const afterPhase = await waitProbe(page, "phase", "idle");
  const afterPreview = await page.locator(".poodle-drag-preview").count();
  check(
    `${name}: preview and session clear after drop`,
    afterPhase === "idle" && afterPreview === 0,
    `phase=${afterPhase} preview=${afterPreview}`,
  );

  await page.reload({ waitUntil: "load" });
  await page.locator("#source").waitFor();
  const origin = center(await box(page, "#source"));
  const targetBox = await box(page, "#target");
  const miss = { x: targetBox.x + targetBox.width + 48, y: targetBox.y + Math.min(12, targetBox.height / 2) };
  await page.mouse.move(origin.x, origin.y);
  await page.mouse.down();
  await page.mouse.move(origin.x + 28, origin.y, { steps: 6 });
  await waitProbe(page, "phase", "dragging");
  await page.mouse.move(miss.x, miss.y, { steps: 8 });
  await frames(page);
  const missed = await probeAttr(page, "posture");
  await page.evaluate((width) => {
    const node = document.getElementById("target");
    if (node) node.style.width = `${width}px`;
  }, miss.x - targetBox.x + 40);
  await frames(page, 6);
  const afterResize = await probeAttr(page, "posture");
  const targetCenter = center(await box(page, "#target"));
  await page.mouse.move(targetCenter.x, targetCenter.y, { steps: 6 });
  await frames(page);
  await waitProbe(page, "posture", "accepted");
  await page.evaluate(() => {
    const scroller = document.getElementById("scroller");
    if (!scroller) return;
    scroller.scrollTop += 220;
  });
  await frames(page, 6);
  const afterScroll = await probeAttr(page, "posture");
  await page.mouse.up();
  check(
    `${name}: resize/scroll re-hit-test without invalidateLayout`,
    missed !== "accepted" && afterResize === "accepted" && afterScroll !== "accepted",
    `miss=${missed} resized=${afterResize} scrolled=${afterScroll}`,
  );

  await page.reload({ waitUntil: "load" });
  await page.locator("#source").waitFor();
  const start = center(await box(page, "#source"));
  await touchAt(page, cdp, "touchStart", start.x, start.y);
  await touchAt(page, cdp, "touchMove", start.x, start.y + 40);
  await page.waitForTimeout(350);
  const touchPhase = await probeAttr(page, "phase");
  const touchCaptured = await probeAttr(page, "captured");
  await touchAt(page, cdp, "touchEnd", start.x, start.y + 40);
  check(
    `${name}: touch scroll wins before hold`,
    touchPhase === "idle" && touchCaptured !== "true",
    `phase=${touchPhase} captured=${touchCaptured}`,
  );

  await page.reload({ waitUntil: "load" });
  await page.locator("#source").waitFor();
  const hold = center(await box(page, "#source"));
  const drop = center(await box(page, "#target"));
  await touchAt(page, cdp, "touchStart", hold.x, hold.y);
  await page.waitForTimeout(350);
  const heldPhase = await waitProbe(page, "phase", "dragging", 1_000);
  const steps = 8;
  for (let i = 1; i <= steps; i++) {
    await touchAt(
      page,
      cdp,
      "touchMove",
      hold.x + ((drop.x - hold.x) * i) / steps,
      hold.y + ((drop.y - hold.y) * i) / steps,
    );
  }
  await frames(page);
  const heldPosture = await probeAttr(page, "posture");
  const heldTarget = await probeAttr(page, "target");
  await touchAt(page, cdp, "touchEnd", drop.x, drop.y);
  check(
    `${name}: touch hold activates, then routes over the target`,
    heldPhase === "dragging" && heldPosture === "accepted" && heldTarget === "list",
    `phase=${heldPhase} posture=${heldPosture} target=${heldTarget}`,
  );

  await page.reload({ waitUntil: "load" });
  await page.locator("#source").waitFor();
  await page.focus("#source");
  await page.keyboard.press("Space");
  await page.keyboard.press("ArrowDown");
  await page.keyboard.press("Enter");
  const focused = await page.evaluate(() => document.activeElement?.id ?? "");
  const idle = await probeAttr(page, "phase");
  check(`${name}: keyboard drop restores focus`, focused === "source" && idle === "idle", `focus=${focused} phase=${idle}`);
}

for (const [name, type] of engines) {
  console.log(`\n=== ${name} ===`);
  const browser = await type.launch();
  const context = await browser.newContext({
    viewport: { width: 800, height: 600 },
    hasTouch: true,
  });
  const page = await context.newPage();
  let cdp: CDPSession | null = null;
  try {
    if (name === "chromium") cdp = await context.newCDPSession(page);
    await run(page, name, cdp);
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
