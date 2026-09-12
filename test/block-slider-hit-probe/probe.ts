/**
 * Headless Chromium + WebKit proof for g18.024 block layout repair.
 *
 *   bun test/block-slider-hit-probe/probe.ts --browser=chromium
 *   bun test/block-slider-hit-probe/probe.ts --browser=webkit
 *
 * Law under test:
 * - block capsules consume the shared control-size ladder (24/28/36/44/52px);
 * - a Slider aligns edge for edge with same-size Button/TextInput controls;
 * - the root's layout box equals the visible capsule at every size/density;
 * - density never inflates the block axis;
 * - the ≥44×44 hit rectangle stays measurable, interactive, and out of layout.
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

// The shared control-size ladder, in px.
const LADDER: Record<string, number> = { xs: 24, sm: 28, md: 36, lg: 44, xl: 52 };

type Geometry = {
  root: DOMRect;
  capsule: DOMRect;
  button: DOMRect;
  input: DOMRect;
  hit: DOMRect;
  pointerEvents: string;
  hitIsTarget: boolean;
  probe: { x: number; y: number };
};

async function measureRow(page: Page, caseSel: string, hitSel: string): Promise<Geometry> {
  return page.evaluate(
    ({ caseSel, hitSel }) => {
      const root = document.querySelector<HTMLElement>(`${caseSel} .poodle-slider`)!;
      const capsule = root.querySelector<HTMLElement>(".poodle-slider__capsule")!;
      const hit = root.querySelector<HTMLElement>(hitSel)!;
      const button = document.querySelector<HTMLElement>(`${caseSel} .poodle-button`)!;
      const input = document.querySelector<HTMLElement>(`${caseSel} .poodle-text-input`)!;
      const hitBox = hit.getBoundingClientRect();
      const capsuleBox = capsule.getBoundingClientRect();
      // Probe a point inside the hit but outside the capsule footprint, so a
      // passing click proves the hit is interactive without adding layout.
      const x = hitBox.left + hitBox.width / 2;
      const y = hitBox.top < capsuleBox.top
        ? hitBox.top + 2
        : Math.min(hitBox.bottom - 2, capsuleBox.bottom + 4);
      const target = document.elementFromPoint(x, y);
      return {
        root: root.getBoundingClientRect(),
        capsule: capsuleBox,
        button: button.getBoundingClientRect(),
        input: input.getBoundingClientRect(),
        hit: hitBox,
        pointerEvents: getComputedStyle(hit).pointerEvents,
        hitIsTarget: Boolean(target?.closest(hitSel)),
        probe: { x, y },
      };
    },
    { caseSel, hitSel },
  );
}

function near(a: number, b: number, tolerance = 0.75): boolean {
  return Math.abs(a - b) <= tolerance;
}

async function probeRow(
  page: Page,
  engine: string,
  framework: string,
  size: string,
): Promise<void> {
  const caseSel = `[data-framework="${framework}"] [data-case="slider-${size}-row"]`;
  const prefix = `${engine} ${framework} ${size} row`;
  await page.locator(caseSel).scrollIntoViewIfNeeded();
  const m = await measureRow(page, caseSel, ".poodle-slider__hit");
  const expected = LADDER[size]!;
  check(`${prefix} root height is the shared ${expected}px ladder size`, near(m.root.height, expected), `${m.root.height}`);
  check(`${prefix} capsule height equals the root layout box`, near(m.capsule.height, m.root.height), `${m.capsule.height} vs ${m.root.height}`);
  check(`${prefix} slider top aligns with same-size Button`, near(m.root.top, m.button.top) && near(m.root.bottom, m.button.bottom), `slider ${m.root.top}..${m.root.bottom} button ${m.button.top}..${m.button.bottom}`);
  // TextInput draws its own 1px border outside the shared ladder (a
  // TextInput-owned trait), so its edges sit 1px outside the Slider's.
  check(
    `${prefix} slider aligns with same-size TextInput within its border`,
    near(m.root.top, m.input.top, 2) && near(m.root.bottom, m.input.bottom, 2),
    `slider ${m.root.top}..${m.root.bottom} input ${m.input.top}..${m.input.bottom}`,
  );
  check(`${prefix} hit is a 44×44 rectangle`, near(m.hit.width, 44) && near(m.hit.height, 44), `${m.hit.width}x${m.hit.height}`);
  check(`${prefix} hit accepts pointer events`, m.pointerEvents === "auto", m.pointerEvents);
  if (expected < 44) {
    check(`${prefix} hit overflows the capsule without adding layout`, m.hit.height > m.root.height + 4, `hit ${m.hit.height} root ${m.root.height}`);
  }
  check(`${prefix} overflow point resolves into the hit`, m.hitIsTarget, `y=${m.probe.y}`);
  // The row sliders have no trace node; prove dispatch through the value
  // moving when the click lands off the thumb's exact center.
  const before = await page.locator(caseSel).locator(".poodle-slider").getAttribute("aria-valuenow");
  await page.mouse.click(m.hit.right - 2, m.hit.top + m.hit.height / 2);
  const after = await page.locator(caseSel).locator(".poodle-slider").getAttribute("aria-valuenow");
  check(`${prefix} pointer outside the capsule dispatches`, after !== null && after !== before, `aria-valuenow ${before}->${after}`);
}

async function probeSimpleCase(
  page: Page,
  engine: string,
  framework: string,
  caseName: string,
  hitSel: string,
  rootSel: string,
  capsuleSel: string,
): Promise<void> {
  const caseSel = `[data-framework="${framework}"] [data-case="${caseName}"]`;
  const prefix = `${engine} ${framework} ${caseName} ${hitSel}`;
  await page.locator(caseSel).scrollIntoViewIfNeeded();
  const metrics = await page.evaluate(
    ({ caseSel, hitSel, rootSel, capsuleSel }) => {
      const root = document.querySelector<HTMLElement>(`${caseSel} ${rootSel}`)!;
      const hit = document.querySelector<HTMLElement>(`${caseSel} ${hitSel}`)!;
      const capsule = document.querySelector<HTMLElement>(`${caseSel} ${capsuleSel}`)!;
      const hitBox = hit.getBoundingClientRect();
      const capsuleBox = capsule.getBoundingClientRect();
      const x = hitBox.left + hitBox.width / 2;
      const y = hitBox.top < capsuleBox.top
        ? hitBox.top + 2
        : Math.min(hitBox.bottom - 2, capsuleBox.bottom + 4);
      const target = document.elementFromPoint(x, y);
      return {
        rootHeight: root.getBoundingClientRect().height,
        capsuleHeight: capsuleBox.height,
        hitHeight: hitBox.height,
        hitWidth: hitBox.width,
        pointerEvents: getComputedStyle(hit).pointerEvents,
        x,
        y,
        hitIsTarget: Boolean(target?.closest(hitSel)),
      };
    },
    { caseSel, hitSel, rootSel, capsuleSel },
  );
  check(`${prefix} root layout box stays at the 24px xs capsule`, near(metrics.rootHeight, 24), `${metrics.rootHeight}`);
  check(`${prefix} capsule equals the root layout box`, near(metrics.capsuleHeight, metrics.rootHeight), `${metrics.capsuleHeight}`);
  check(`${prefix} hit is a 44×44 rectangle`, near(metrics.hitWidth, 44) && near(metrics.hitHeight, 44), `${metrics.hitWidth}x${metrics.hitHeight}`);
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
    const base = `[data-framework="${framework}"]`;
    // Density never inflates the block axis: compact matches default.
    const plain = await page.locator(`${base} [data-case="slider-xs"] .poodle-slider`).boundingBox();
    const compact = await page.locator(`${base} [data-case="slider-xs-compact"] .poodle-slider`).boundingBox();
    check(
      `${engine} ${framework} compact density keeps the 24px block height`,
      near(plain!.height, compact!.height) && near(plain!.height, 24),
      `default ${plain!.height} compact ${compact!.height}`,
    );
    await probeSimpleCase(page, engine, framework, "slider-xs", ".poodle-slider__hit", ".poodle-slider", ".poodle-slider__capsule");
    await probeSimpleCase(page, engine, framework, "range-xs", ".poodle-range-slider__hit--lower", ".poodle-range-slider", ".poodle-range-slider__capsule");
    await probeSimpleCase(page, engine, framework, "range-xs", ".poodle-range-slider__hit--upper", ".poodle-range-slider", ".poodle-range-slider__capsule");
    for (const size of ["xs", "sm", "md", "lg", "xl"] as const) {
      await probeRow(page, engine, framework, size);
    }
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
