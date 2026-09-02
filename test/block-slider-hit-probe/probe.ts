/**
 * Headless Chromium + WebKit proof for g16.046 block hit targets.
 *
 *   bun test/block-slider-hit-probe/probe.ts --browser=chromium
 *   bun test/block-slider-hit-probe/probe.ts --browser=webkit
 */
import { chromium, webkit, type BrowserType, type Page } from "playwright";
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
const port = 4191;
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
  throw new Error(`block-slider-hit fixture on :${port} did not start`);
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
  { cwd: repoRoot, stdout: "inherit", stderr: "inherit" },
);

type Geometry = {
  rootHeight: number;
  hitHeight: number;
  capsuleHeight: number;
  pointerEvents: string;
  x: number;
  y: number;
  hitIsTarget: boolean;
};

async function geometry(page: Page, caseSel: string, hitSel: string): Promise<Geometry> {
  return page.evaluate(
    ({ caseSel, hitSel }) => {
      const root = document.querySelector<HTMLElement>(`${caseSel} .poodle-slider, ${caseSel} .poodle-range-slider`)!;
      const hit = document.querySelector<HTMLElement>(`${caseSel} ${hitSel}`)!;
      const capsule = document.querySelector<HTMLElement>(`${caseSel} .poodle-slider__capsule, ${caseSel} .poodle-range-slider__capsule`)!;
      const hitBox = hit.getBoundingClientRect();
      const capsuleBox = capsule.getBoundingClientRect();
      const x = hitBox.left + hitBox.width / 2;
      const y =
        hitBox.top + 2 < capsuleBox.top
          ? hitBox.top + 2
          : Math.min(hitBox.bottom - 2, capsuleBox.bottom + 4);
      const target = document.elementFromPoint(x, y);
      return {
        rootHeight: root.getBoundingClientRect().height,
        hitHeight: hitBox.height,
        capsuleHeight: capsuleBox.height,
        pointerEvents: getComputedStyle(hit).pointerEvents,
        x,
        y,
        hitIsTarget: Boolean(target?.closest(hitSel)),
      };
    },
    { caseSel, hitSel },
  );
}

async function probeCase(
  page: Page,
  engine: string,
  framework: string,
  caseName: string,
  hitSel: string,
): Promise<void> {
  const caseSel = `[data-framework="${framework}"] [data-case="${caseName}"]`;
  const prefix = `${engine} ${framework} ${caseName} ${hitSel}`;
  await page.locator(caseSel).scrollIntoViewIfNeeded();
  const metrics = await geometry(page, caseSel, hitSel);
  check(`${prefix} root is at least 44px tall`, metrics.rootHeight >= 44, `${metrics.rootHeight}`);
  check(`${prefix} hit is 44px tall`, metrics.hitHeight === 44, `${metrics.hitHeight}`);
  check(`${prefix} capsule stays visually smaller than 44px`, metrics.capsuleHeight < 44, `${metrics.capsuleHeight}`);
  check(`${prefix} hit accepts pointer events`, metrics.pointerEvents === "auto", metrics.pointerEvents);
  check(`${prefix} overflow point is inside the hit`, metrics.hitIsTarget, `y=${metrics.y}`);
  const traceSel = `${caseSel} [data-testid="trace"]`;
  const hitsBefore = Number(await page.locator(traceSel).getAttribute("data-hits"));
  await page.mouse.click(metrics.x, metrics.y);
  const hitsAfter = Number(await page.locator(traceSel).getAttribute("data-hits"));
  const trace = await page.locator(traceSel).innerText();
  check(
    `${prefix} pointer outside capsule dispatches`,
    hitsAfter === hitsBefore + 1,
    `hits ${hitsBefore}->${hitsAfter} ${trace}`,
  );
}

async function probe(page: Page, engine: string): Promise<void> {
  await page.goto(url, { waitUntil: "domcontentloaded", timeout: 60_000 });
  await page.locator('[data-framework="svelte"] [data-case="slider-xs"]').waitFor();
  for (const framework of ["svelte", "react"]) {
    await probeCase(page, engine, framework, "slider-xs", ".poodle-slider__hit");
    await probeCase(page, engine, framework, "slider-xs-compact", ".poodle-slider__hit");
    await probeCase(page, engine, framework, "range-xs", ".poodle-range-slider__hit--lower");
    await probeCase(page, engine, framework, "range-xs", ".poodle-range-slider__hit--upper");
  }
}

try {
  await waitForServer();
  for (const [name, engine] of engines) {
    console.log(`\n[block-slider-hit] ${name}`);
    const browser = await engine.launch({ headless: true });
    const page = await browser.newPage({ viewport: { width: 1280, height: 900 } });
    await probe(page, name);
    await browser.close();
  }
} finally {
  child.kill();
}

process.exit(failures === 0 ? 0 : 1);
