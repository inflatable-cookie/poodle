/**
 * ModelCatalogueEditor fast off-handle release regression (poodle#131).
 *
 * A pointer drag the user releases away from the row's handle — a fast or
 * imprecise drop on the next row — must reorder the catalogue. The browser can
 * deliver `lostpointercapture` with the button already up (`buttons === 0`)
 * before `pointerup`; the controller must treat that as the release and commit
 * the drop, not report a lost transport.
 *
 * Twenty real fast releases assert every one commits. A final synthesized
 * `lostpointercapture`-then-`pointerup` ordering pins the exact browser
 * sequence deterministically, since real input only races occasionally.
 *
 *   bun test/drag-drop/fast-release.ts --browser=chromium
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
const port = 4181;
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
  { cwd: repoRoot, stdout: "inherit", stderr: "inherit" },
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

type Box = { x: number; y: number; width: number; height: number };

async function runFastReleases(name: string, browser: Browser): Promise<void> {
  const context = await browser.newContext({ viewport: { width: 1000, height: 900 } });
  const page = await context.newPage();
  await page.goto(`${url}components.html`, { waitUntil: "load" });
  await page.locator("#svelte-probe").waitFor({ state: "attached" });

  const order = async (): Promise<string[]> =>
    page.$$eval("#svelte-mce-a [data-model-catalogue-id]", (nodes) =>
      nodes.map((node) => node.getAttribute("data-model-catalogue-id") ?? ""),
    );
  const count = async (): Promise<number> =>
    Number((await page.locator("#svelte-probe").getAttribute("data-order-a-count")) ?? "0");

  const at = async (selector: string, fraction = 0.5): Promise<Box> => {
    const locator = page.locator(selector).first();
    const rect = await locator.boundingBox();
    if (!rect) throw new Error(`no box for ${selector}`);
    return { x: rect.x + rect.width / 2, y: rect.y + rect.height * fraction, width: rect.width, height: rect.height };
  };

  // ── Twenty real fast releases, each off the handle and onto another row ──
  let committed = 0;
  for (let iteration = 0; iteration < 20; iteration += 1) {
    const before = await order();
    const handle = await at(`#svelte-mce-a [data-model-catalogue-id="${before[0]}"] [data-reorder-handle]`);
    const destination = await at(`#svelte-mce-a [data-model-catalogue-id="${before[before.length - 1]}"]`);

    await page.mouse.move(handle.x, handle.y);
    await page.mouse.down();
    // A fast move: two steps, then release immediately away from the handle.
    await page.mouse.move(destination.x, destination.y, { steps: 2 });
    await page.mouse.up();
    await frames(page);

    const after = await order();
    const moved =
      after.length === before.length &&
      after[after.length - 1] === before[0] &&
      [...before].sort().join(",") === [...after].sort().join(",");
    if (moved) committed += 1;
  }
  check(
    `${name}: svelte ModelCatalogueEditor reorders on 20 fast off-handle releases`,
    committed === 20 && (await count()) === 20,
    `committed=${committed}/20 orderCount=${await count()} order=${(await order()).join(",")}`,
  );

  // ── The exact defect ordering, deterministically ──
  // Real input above only races occasionally; synthesize the browser sequence
  // the live recording showed: `lostpointercapture` (button up) first.
  await page.reload({ waitUntil: "load" });
  await page.locator("#svelte-probe").waitFor({ state: "attached" });
  await page.evaluate(() => {
    (window as unknown as Record<string, unknown>).__poodlePointerId = null;
    document.addEventListener(
      "pointerdown",
      (event) => {
        (window as unknown as Record<string, unknown>).__poodlePointerId = (event as PointerEvent).pointerId;
      },
      true,
    );
  });

  const handle = await at('#svelte-mce-a [data-model-catalogue-id="alpha"] [data-reorder-handle]');
  const destination = await at('#svelte-mce-a [data-model-catalogue-id="gamma"]');
  await page.mouse.move(handle.x, handle.y);
  await page.mouse.down();
  await page.mouse.move(destination.x, destination.y, { steps: 2 });
  await page.evaluate(
    ({ x, y }) => {
      const node = document.elementFromPoint(x, y) ?? document.body;
      const pointerId = (window as unknown as Record<string, unknown>).__poodlePointerId ?? 1;
      node.dispatchEvent(
        new PointerEvent("lostpointercapture", {
          bubbles: true,
          cancelable: true,
          composed: true,
          pointerId: pointerId as number,
          pointerType: "mouse",
          isPrimary: true,
          button: 0,
          buttons: 0,
          clientX: x,
          clientY: y,
          view: window,
        }),
      );
    },
    { x: destination.x, y: destination.y },
  );
  await page.mouse.up();
  await frames(page);
  check(
    `${name}: svelte ModelCatalogueEditor commits when lostpointercapture precedes pointerup`,
    (await count()) === 1 && (await order()).join(",") === "beta,gamma,alpha",
    `orderCount=${await count()} order=${(await order()).join(",")}`,
  );

  await context.close();
}

for (const [name, engine] of engines) {
  const browser = await engine.launch();
  try {
    await runFastReleases(name, browser);
  } catch (error) {
    failures += 1;
    console.error(`  FAIL  ${name}: probe threw — ${error instanceof Error ? error.message : String(error)}`);
  } finally {
    await browser.close();
  }
}

child.kill();
await child.exited;

if (failures > 0) {
  console.error(`\n${failures} drag-drop fast-release check(s) failed`);
  process.exit(1);
}

console.log("\nall drag-drop fast-release checks passed");
