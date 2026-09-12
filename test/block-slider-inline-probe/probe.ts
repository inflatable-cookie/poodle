/**
 * Headless Chromium + WebKit proof for g18.017 block Slider fixed inline
 * presentation (rounded-square family corners, stable glyph coordinates under
 * a moving fill boundary, split-colour crossover, whole-track collision
 * priority, RTL mirroring) extended with the g18.024 repair proof: short
 * step-aware default decimals and complete upright vertical rails for both
 * families — over paired Svelte and React fixtures.
 *
 *   bun test/block-slider-inline-probe/probe.ts --browser=chromium
 *   bun test/block-slider-inline-probe/probe.ts --browser=webkit
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
const port = 4192;
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
  throw new Error(`block-slider-inline fixture on :${port} did not start`);
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

type Rect = { left: number; right: number; top: number; bottom: number; width: number; height: number };

type Case = {
  capsule: Rect;
  fill: Rect;
  selectedLabel: Rect;
  selectedValue: Rect;
  remainderLabel: Rect;
  remainderValue: Rect;
  selectedLabelColor: string;
  remainderLabelColor: string;
  selectedValueColor: string;
  remainderValueColor: string;
  selectedLabelText: string;
  remainderLabelText: string;
  selectedValueText: string;
  thumb: Rect;
  thumbRadius: string;
  capsuleRadius: string;
  dir: string | null;
};

async function measure(page: Page, caseSel: string): Promise<Case> {
  return page.evaluate(
    (caseSel) => {
      const root = document.querySelector<HTMLElement>(`${caseSel} .poodle-slider`)!;
      const capsule = document.querySelector<HTMLElement>(`${caseSel} .poodle-slider__capsule`)!;
      const fill = document.querySelector<HTMLElement>(`${caseSel} .poodle-slider__fill`)!;
      const thumb = document.querySelector<HTMLElement>(`${caseSel} .poodle-slider__thumb`)!;
      const selected = document.querySelector<HTMLElement>(`${caseSel} .poodle-slider__inline--selected`)!;
      const remainder = document.querySelector<HTMLElement>(`${caseSel} .poodle-slider__inline--remainder`)!;
      const rect = (el: Element) => {
        const box = el.getBoundingClientRect();
        return {
          left: box.left,
          right: box.right,
          top: box.top,
          bottom: box.bottom,
          width: box.width,
          height: box.height,
        };
      };
      const selectedLabel = selected.querySelector<HTMLElement>(".poodle-slider__inline-label")!;
      const selectedValue = selected.querySelector<HTMLElement>(".poodle-slider__inline-value")!;
      const remainderLabel = remainder.querySelector<HTMLElement>(".poodle-slider__inline-label")!;
      const remainderValue = remainder.querySelector<HTMLElement>(".poodle-slider__inline-value")!;
      return {
        capsule: rect(capsule),
        fill: rect(fill),
        selectedLabel: rect(selectedLabel),
        selectedValue: rect(selectedValue),
        remainderLabel: rect(remainderLabel),
        remainderValue: rect(remainderValue),
        selectedLabelColor: getComputedStyle(selectedLabel).color,
        remainderLabelColor: getComputedStyle(remainderLabel).color,
        selectedValueColor: getComputedStyle(selectedValue).color,
        remainderValueColor: getComputedStyle(remainderValue).color,
        selectedLabelText: selectedLabel.textContent ?? "",
        remainderLabelText: remainderLabel.textContent ?? "",
        selectedValueText: selectedValue.textContent ?? "",
        thumb: rect(thumb),
        thumbRadius: getComputedStyle(thumb).borderRadius,
        capsuleRadius: getComputedStyle(capsule).borderRadius,
        dir: root.getAttribute("dir"),
      };
    },
    caseSel,
  );
}

function sameRect(a: Rect, b: Rect, tolerance = 0.5): boolean {
  return (
    Math.abs(a.left - b.left) <= tolerance &&
    Math.abs(a.right - b.right) <= tolerance &&
    Math.abs(a.top - b.top) <= tolerance &&
    Math.abs(a.width - b.width) <= tolerance
  );
}

/** Journey cases sit at different page heights, so stability is x-axis only. */
function sameX(a: Rect, b: Rect, tolerance = 0.5): boolean {
  return Math.abs(a.left - b.left) <= tolerance && Math.abs(a.width - b.width) <= tolerance;
}

async function probeFramework(page: Page, engine: string, framework: string): Promise<void> {
  const prefix = `${engine} ${framework}`;
  const base = `[data-framework="${framework}"]`;

  // Family corners: rounded-square capsule, circular thumb. The journey
  // values match the harness cases (10 / 50 / 90).
  for (const [caseName, value] of [["slider-low", 10], ["slider-mid", 50], ["slider-high", 90]] as const) {
    const caseSel = `${base} [data-case="${caseName}"]`;
    const metrics = await measure(page, caseSel);
    check(
      `${prefix} ${caseName} capsule resolves the rounded-square control radius`,
      metrics.capsuleRadius === "6px",
      metrics.capsuleRadius,
    );
    check(
      `${prefix} ${caseName} visible thumb stays circular`,
      metrics.thumbRadius === "999px" && Math.abs(metrics.thumb.width - metrics.thumb.height) <= 0.5,
      `${metrics.thumbRadius} ${metrics.thumb.width}x${metrics.thumb.height}`,
    );
    check(
      `${prefix} ${caseName} paints no external fallback`,
      (await page.locator(`${caseSel} .poodle-slider__fallback`).count()) === 0,
    );
    check(
      `${prefix} ${caseName} both layers carry the identical strings`,
      metrics.selectedLabelText === "Gain" &&
        metrics.selectedValueText === "67" &&
        sameRect(metrics.selectedLabel, metrics.remainderLabel) &&
        sameRect(metrics.selectedValue, metrics.remainderValue),
      `label="${metrics.selectedLabelText}" value="${metrics.selectedValueText}"`,
    );
    const boundaryX = metrics.capsule.left + (metrics.capsule.width * value) / 100;
    check(
      `${prefix} ${caseName} fill edge tracks the value`,
      Math.abs(metrics.fill.left + metrics.fill.width - boundaryX) <= 1.5,
      `fill edge ${metrics.fill.left + metrics.fill.width} vs ${boundaryX}`,
    );
    check(
      `${prefix} ${caseName} layers stay stacked above the fill paint`,
      metrics.selectedValue.top >= metrics.capsule.top && metrics.selectedValue.bottom <= metrics.capsule.bottom,
    );
    if (caseName === "slider-low") {
      const inside =
        metrics.selectedLabel.left < boundaryX - 1 && metrics.selectedLabel.right > boundaryX + 1;
      check(`${prefix} low value crosses the label glyphs`, inside, `boundary ${boundaryX}`);
      check(
        `${prefix} crossover changes only the foreground`,
        metrics.selectedLabelColor !== metrics.remainderLabelColor,
        `${metrics.selectedLabelColor} vs ${metrics.remainderLabelColor}`,
      );
    }
    if (caseName === "slider-high") {
      const inside =
        metrics.selectedValue.left < boundaryX - 1 && metrics.selectedValue.right > boundaryX + 1;
      check(`${prefix} high value crosses the value glyphs`, inside, `boundary ${boundaryX}`);
      check(
        `${prefix} value crossover changes only the foreground`,
        metrics.selectedValueColor !== metrics.remainderValueColor,
        `${metrics.selectedValueColor} vs ${metrics.remainderValueColor}`,
      );
    }
  }

  // Stable glyph coordinates: identical x boxes across the whole journey.
  const low = await measure(page, `${base} [data-case="slider-low"]`);
  const mid = await measure(page, `${base} [data-case="slider-mid"]`);
  const high = await measure(page, `${base} [data-case="slider-high"]`);
  check(`${prefix} label glyphs never move`, sameX(low.selectedLabel, mid.selectedLabel) && sameX(mid.selectedLabel, high.selectedLabel), `${low.selectedLabel.left}, ${mid.selectedLabel.left}, ${high.selectedLabel.left}`);
  check(`${prefix} value glyphs never move`, sameX(low.selectedValue, mid.selectedValue) && sameX(mid.selectedValue, high.selectedValue), `${low.selectedValue.left},${mid.selectedValue.left},${high.selectedValue.left}`);
  check(`${prefix} boundary moves across the journey`, low.fill.width < mid.fill.width && mid.fill.width < high.fill.width);

  // Whole-track collision: label suppressed, exact value stays in-track.
  const collision = await measure(page, `${base} [data-case="slider-collision"]`);
  check(
    `${prefix} collision suppresses the optional label`,
    collision.selectedLabelText === "" && collision.remainderLabelText === "",
    collision.selectedLabelText,
  );
  check(
    `${prefix} collision keeps the exact numeric value in-track`,
    collision.selectedValueText === "12",
    collision.selectedValueText,
  );
  check(
    `${prefix} collision never creates a fallback line`,
    (await page.locator(`${base} [data-case="slider-collision"] .poodle-slider__fallback`).count()) === 0,
  );

  // RTL: logical anchors stay stable, physical order mirrors.
  const rtl = await measure(page, `${base} [data-case="slider-rtl"]`);
  check(`${prefix} rtl marks the direction on the root`, rtl.dir === "rtl", `${rtl.dir}`);
  check(
    `${prefix} rtl keeps the label at the logical start`,
    rtl.selectedLabel.left > rtl.selectedValue.left &&
      rtl.capsule.right - rtl.selectedLabel.right < 24,
    `label ${rtl.selectedLabel.left}..${rtl.selectedLabel.right}`,
  );
  check(
    `${prefix} rtl keeps the value at the logical end`,
    rtl.selectedValue.left - rtl.capsule.left < 24,
    `value ${rtl.selectedValue.left}..${rtl.selectedValue.right}`,
  );
  check(
    `${prefix} rtl splits the foreground across the boundary`,
    rtl.selectedLabelColor !== rtl.remainderLabelColor,
  );

  // g18.022 vertical block: the value paints at the capsule's physical top
  // and the optional label at its center, anchored to the whole capsule.
  // g18.024: the anchored text also stays fully inside the rail with a
  // fixed, value-independent inset.
  const vertical = await measure(page, `${base} [data-case="slider-vertical"]`);
  const verticalTopInset = vertical.selectedValue.top - vertical.capsule.top;
  check(
    `${prefix} vertical value paints at the capsule top`,
    vertical.selectedValueText === "50" && verticalTopInset < 14,
    `value top inset ${verticalTopInset.toFixed(1)}`,
  );
  const verticalLabelCenter = (vertical.selectedLabel.top + vertical.selectedLabel.bottom) / 2;
  const verticalCapsuleCenter = (vertical.capsule.top + vertical.capsule.bottom) / 2;
  check(
    `${prefix} vertical label stays centered`,
    Math.abs(verticalLabelCenter - verticalCapsuleCenter) <= 4,
    `label center ${verticalLabelCenter.toFixed(1)} vs capsule center ${verticalCapsuleCenter.toFixed(1)}`,
  );
  check(
    `${prefix} vertical row fills the capsule`,
    vertical.selectedValue.top < vertical.selectedLabel.top &&
      vertical.selectedLabel.bottom <= vertical.capsule.bottom &&
      vertical.capsule.height > 100,
    `capsule ${vertical.capsule.height}px value ${vertical.selectedValue.top} label ${vertical.selectedLabel.top}..${vertical.selectedLabel.bottom}`,
  );
  check(
    `${prefix} vertical text stays inside the capsule with a visible inset`,
    vertical.selectedValue.top >= vertical.capsule.top &&
      vertical.selectedValue.top - vertical.capsule.top >= 2 &&
      vertical.selectedLabel.bottom <= vertical.capsule.bottom,
    `value top inset ${verticalTopInset.toFixed(1)}`,
  );
  check(
    `${prefix} vertical paints no fallback`,
    (await page.locator(`${base} [data-case="slider-vertical"] .poodle-slider__fallback`).count()) === 0,
  );

  // g18.024: fractional steps render short decimals — the default serializer
  // rounds snapped binary debris to the precision implied by min and step.
  const fraction = await measure(page, `${base} [data-case="slider-fraction"]`);
  check(
    `${prefix} a snapped 0.85 renders without a binary tail`,
    fraction.selectedValueText === "0.85",
    fraction.selectedValueText,
  );
  const rangeFractionSel = `${base} [data-case="range-fraction"]`;
  const rangeFractionTexts = await page.evaluate(
    (sel) =>
      Array.from(
        document.querySelectorAll<HTMLElement>(
          `${sel} .poodle-range-slider__inline--selected .poodle-range-slider__inline-value`,
        ),
      ).map((slot) => slot.textContent),
    rangeFractionSel,
  );
  check(
    `${prefix} both range endpoints render short decimals`,
    rangeFractionTexts.join("/") === "0.3/0.85",
    rangeFractionTexts.join("/"),
  );

  // g18.024 vertical range repair: upper value at the physical top, label
  // centered on the exact middle, lower value at the physical bottom; every
  // glyph inside the capsule, anchors value-independent.
  const rangeVerticalSel = `${base} [data-case="range-vertical"]`;
  const rv = await page.evaluate(
    (sel) => {
      const capsule = document.querySelector<HTMLElement>(`${sel} .poodle-range-slider__capsule`)!;
      const row = document.querySelector<HTMLElement>(
        `${sel} .poodle-range-slider__inline--selected .poodle-range-slider__inline-row--vertical`,
      )!;
      const rect = (el: Element) => {
        const box = el.getBoundingClientRect();
        return { top: box.top, bottom: box.bottom, left: box.left, right: box.right, height: box.height };
      };
      const upper = row.querySelector<HTMLElement>(".poodle-range-slider__inline-value--upper")!;
      const lower = row.querySelector<HTMLElement>(".poodle-range-slider__inline-value--lower")!;
      const label = row.querySelector<HTMLElement>(".poodle-range-slider__inline-label")!;
      return {
        capsule: rect(capsule),
        upper: { box: rect(upper), text: upper.textContent ?? "" },
        lower: { box: rect(lower), text: lower.textContent ?? "" },
        label: { box: rect(label), text: label.textContent ?? "" },
      };
    },
    rangeVerticalSel,
  );
  const rvCapsuleCenter = (rv.capsule.top + rv.capsule.bottom) / 2;
  const rvLabelCenter = (rv.label.box.top + rv.label.box.bottom) / 2;
  check(
    `${prefix} vertical range upper value paints at the top`,
    rv.upper.text === "80" && rv.upper.box.top - rv.capsule.top >= 2 && rv.upper.box.top - rv.capsule.top < 14,
    `top inset ${(rv.upper.box.top - rv.capsule.top).toFixed(1)} text ${rv.upper.text}`,
  );
  check(
    `${prefix} vertical range lower value paints at the bottom`,
    rv.lower.text === "20" && rv.capsule.bottom - rv.lower.box.bottom >= 2 && rv.capsule.bottom - rv.lower.box.bottom < 14,
    `bottom inset ${(rv.capsule.bottom - rv.lower.box.bottom).toFixed(1)} text ${rv.lower.text}`,
  );
  check(
    `${prefix} vertical range label centers on the exact middle`,
    rv.label.text === "Price" && Math.abs(rvLabelCenter - rvCapsuleCenter) <= 4,
    `label ${rvLabelCenter.toFixed(1)} vs ${rvCapsuleCenter.toFixed(1)}`,
  );
  check(
    `${prefix} vertical range keeps every glyph inside the capsule`,
    rv.upper.box.top >= rv.capsule.top &&
      rv.lower.box.bottom <= rv.capsule.bottom &&
      rv.label.box.bottom <= rv.capsule.bottom,
    `capsule ${rv.capsule.top}..${rv.capsule.bottom}`,
  );
  check(
    `${prefix} vertical range rail is the shared 36px capsule width`,
    Math.abs(rv.capsule.right - rv.capsule.left - 36) <= 0.75,
    `${rv.capsule.right - rv.capsule.left}`,
  );

  // RangeSlider: radius-only family change; inline placement untouched.
  const rangeSel = `${base} [data-case="range-block"]`;
  const rangeCapsule = page.locator(`${rangeSel} .poodle-range-slider__capsule`);
  check(
    `${prefix} range capsule resolves the rounded-square control radius`,
    (await rangeCapsule.evaluate((el) => getComputedStyle(el).borderRadius)) === "6px",
  );
  check(
    `${prefix} range keeps its selected-window inline label`,
    (await page.locator(`${rangeSel} .poodle-range-slider__inline--selected`).innerText()).includes("Price"),
  );
  check(
    `${prefix} range paints no fallback at the specimen width`,
    (await page.locator(`${rangeSel} .poodle-range-slider__fallback`).count()) === 0,
  );

  // Overlays must not steal the pointer: a click at the thumb dispatches.
  const midSel = `${base} [data-case="slider-mid"]`;
  const traceSel = `${midSel} [data-testid="trace"]`;
  await page.locator(midSel).scrollIntoViewIfNeeded();
  const before = Number(await page.locator(traceSel).getAttribute("data-hits"));
  const capsuleBox = await page.locator(`${midSel} .poodle-slider__capsule`).boundingBox();
  if (!capsuleBox) throw new Error("missing capsule box");
  await page.mouse.click(capsuleBox.x + capsuleBox.width / 2, capsuleBox.y + capsuleBox.height / 2);
  await page.waitForTimeout(120);
  const after = Number(await page.locator(traceSel).getAttribute("data-hits"));
  check(`${prefix} pointer still dispatches with text layers mounted`, after === before + 1, `hits ${before}->${after}`);
}

try {
  await waitForServer();
  for (const [name, engine] of engines) {
    console.log(`\n[block-slider-inline] ${name}`);
    const browser = await engine.launch({ headless: true });
    const page = await browser.newPage({ viewport: { width: 1280, height: 900 } });
    await page.goto(url, { waitUntil: "domcontentloaded", timeout: 60_000 });
    await page.locator('[data-framework="svelte"] [data-case="slider-low"]').waitFor();
    for (const framework of ["svelte", "react"]) {
      await probeFramework(page, name, framework);
    }
    await browser.close();
  }
} finally {
  child.kill();
}

process.exit(failures === 0 ? 0 : 1);
