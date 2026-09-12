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

/** RangeSlider block geometry: capsule, window fill and both handles. */
async function measureRange(page: Page, caseSel: string): Promise<{
  capsule: Rect;
  window: Rect;
  lowerThumb: Rect;
  upperThumb: Rect;
  lowerCenter: number;
  upperCenter: number;
  inlineZ: number;
  hitZ: number;
}> {
  return page.evaluate(
    (caseSel) => {
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
      const capsule = document.querySelector<HTMLElement>(`${caseSel} .poodle-range-slider__capsule`)!;
      const positive = document.querySelector<HTMLElement>(`${caseSel} .poodle-range-slider__fill--positive`)!;
      const negative = document.querySelector<HTMLElement>(`${caseSel} .poodle-range-slider__fill--negative`)!;
      const windowEl = positive.getBoundingClientRect().width > 0 ? positive : negative;
      const lowerThumb = document.querySelector<HTMLElement>(
        `${caseSel} .poodle-range-slider__hit--lower .poodle-range-slider__thumb`,
      )!;
      const upperThumb = document.querySelector<HTMLElement>(
        `${caseSel} .poodle-range-slider__hit--upper .poodle-range-slider__thumb`,
      )!;
      const lowerBox = lowerThumb.getBoundingClientRect();
      const upperBox = upperThumb.getBoundingClientRect();
      const inline = document.querySelector<HTMLElement>(
        `${caseSel} .poodle-range-slider__inline--selected`,
      )!;
      const lowerHit = document.querySelector<HTMLElement>(`${caseSel} .poodle-range-slider__hit--lower`)!;
      return {
        capsule: rect(capsule),
        window: rect(windowEl),
        lowerThumb: rect(lowerThumb),
        upperThumb: rect(upperThumb),
        lowerCenter: lowerBox.left + lowerBox.width / 2,
        upperCenter: upperBox.left + upperBox.width / 2,
        inlineZ: Number(getComputedStyle(inline).zIndex),
        hitZ: Number(getComputedStyle(lowerHit).zIndex),
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

  // Family corners: rounded-square capsule, inset marker line. The journey
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
      `${prefix} ${caseName} thumb is an inset marker line`,
      metrics.thumb.width <= 4 &&
        Math.abs(metrics.thumb.height - (metrics.capsule.height - 8)) <= 0.75 &&
        metrics.thumb.top - metrics.capsule.top >= 1.5 &&
        metrics.capsule.bottom - metrics.thumb.bottom >= 1.5,
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
      // Collision docking (g18.026 family law): the high value docks before
      // the handle instead of straddling the fill boundary.
      check(
        `${prefix} high docked value stays left of the fill boundary`,
        metrics.selectedValue.right <= boundaryX + 1,
        `value right ${metrics.selectedValue.right} boundary ${boundaryX}`,
      );
    }
  }

  // Glyph stability: the label never moves. The value stays at the logical
  // end anchor until the marker enters its reserved box, then docks before
  // the handle (g18.026) rather than becoming illegible.
  const low = await measure(page, `${base} [data-case="slider-low"]`);
  const mid = await measure(page, `${base} [data-case="slider-mid"]`);
  const high = await measure(page, `${base} [data-case="slider-high"]`);
  check(`${prefix} label glyphs never move`, sameX(low.selectedLabel, mid.selectedLabel) && sameX(mid.selectedLabel, high.selectedLabel), `${low.selectedLabel.left}, ${mid.selectedLabel.left}, ${high.selectedLabel.left}`);
  check(
    `${prefix} low and mid keep the value at the logical end anchor`,
    sameX(low.selectedValue, mid.selectedValue),
    `${low.selectedValue.left},${mid.selectedValue.left}`,
  );
  const highBoundaryX = high.capsule.left + (high.capsule.width * 90) / 100;
  check(
    `${prefix} high docks the value before the handle`,
    high.selectedValue.right < mid.selectedValue.left - 4 &&
      highBoundaryX - high.selectedValue.right >= 2 &&
      high.selectedValue.right <= highBoundaryX,
    `value right ${high.selectedValue.right} mid left ${mid.selectedValue.left} boundary ${highBoundaryX}`,
  );
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
    `${prefix} both range endpoints render fixed-width decimals`,
    rangeFractionTexts.join("/") === "0.30/0.85",
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

  // RangeSlider family parity (g18.026): the same capsule, window fill,
  // bounded inset line handles and text-above-handle paint order, composing
  // the shared handle primitive twice.
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
  const rangeMetrics = await measureRange(page, rangeSel);
  const rangeStart = rangeMetrics.capsule.left + rangeMetrics.capsule.width * 0.2;
  const rangeEnd = rangeMetrics.capsule.left + rangeMetrics.capsule.width * 0.8;
  check(
    `${prefix} range window fill spans the pair exactly`,
    Math.abs(rangeMetrics.window.left - rangeStart) <= 1.5 && Math.abs(rangeMetrics.window.right - rangeEnd) <= 1.5,
    `${rangeMetrics.window.left}..${rangeMetrics.window.right} vs ${rangeStart}..${rangeEnd}`,
  );
  for (const [name, thumb] of [
    ["lower", rangeMetrics.lowerThumb],
    ["upper", rangeMetrics.upperThumb],
  ] as const) {
    check(
      `${prefix} range ${name} handle is the shared inset marker line`,
      thumb.width <= 4 &&
        Math.abs(thumb.height - (rangeMetrics.capsule.height - 8)) <= 0.75 &&
        thumb.top >= rangeMetrics.capsule.top + 1.5 &&
        thumb.bottom <= rangeMetrics.capsule.bottom - 1.5,
      `${thumb.width}x${thumb.height}`,
    );
  }
  check(
    `${prefix} range handles sit at their own values`,
    Math.abs(rangeMetrics.lowerCenter - rangeStart) <= 1.5 && Math.abs(rangeMetrics.upperCenter - rangeEnd) <= 1.5,
    `${rangeMetrics.lowerCenter},${rangeMetrics.upperCenter}`,
  );
  check(
    `${prefix} range text paints above the shared handle`,
    rangeMetrics.inlineZ > rangeMetrics.hitZ,
    `${rangeMetrics.inlineZ} vs ${rangeMetrics.hitZ}`,
  );

  const extrema = await measureRange(page, `${base} [data-case="range-extrema"]`);
  check(
    `${prefix} extrema handles are clamped inside the capsule`,
    Math.abs(extrema.lowerCenter - (extrema.capsule.left + 6)) <= 1.5 &&
      Math.abs(extrema.upperCenter - (extrema.capsule.right - 6)) <= 1.5,
    `${extrema.lowerCenter} ${extrema.upperCenter} capsule ${extrema.capsule.left}..${extrema.capsule.right}`,
  );
  check(
    `${prefix} extrema handles are fully inside the capsule`,
    extrema.lowerThumb.left >= extrema.capsule.left - 0.5 &&
      extrema.upperThumb.right <= extrema.capsule.right + 0.5,
    `${extrema.lowerThumb.left} ${extrema.upperThumb.right}`,
  );
  check(
    `${prefix} extrema window fill spans the capsule`,
    Math.abs(extrema.window.left - extrema.capsule.left) <= 1.5 &&
      Math.abs(extrema.window.right - extrema.capsule.right) <= 1.5,
    `${extrema.window.left}..${extrema.window.right}`,
  );

  const equality = await measureRange(page, `${base} [data-case="range-equality"]`);
  const equalityCenter = equality.capsule.left + equality.capsule.width / 2;
  check(
    `${prefix} equality handles meet at the value without crossing`,
    equality.lowerCenter <= equality.upperCenter + 0.5 &&
      Math.abs(equality.lowerCenter - equalityCenter) <= 1.5 &&
      Math.abs(equality.upperCenter - equalityCenter) <= 1.5,
    `${equality.lowerCenter} ${equality.upperCenter} center ${equalityCenter}`,
  );

  const rangeVert = await measureRange(page, `${base} [data-case="range-vertical"]`);
  for (const [name, thumb] of [
    ["lower", rangeVert.lowerThumb],
    ["upper", rangeVert.upperThumb],
  ] as const) {
    check(
      `${prefix} vertical range ${name} handle is the shared inset marker line`,
      thumb.height <= 4 &&
        Math.abs(thumb.width - (rangeVert.capsule.width - 8)) <= 0.75 &&
        thumb.left >= rangeVert.capsule.left + 1.5 &&
        thumb.right <= rangeVert.capsule.right - 1.5,
      `${thumb.width}x${thumb.height}`,
    );
  }
  const rangeVertLowerY = rangeVert.lowerThumb.top + rangeVert.lowerThumb.height / 2;
  const rangeVertUpperY = rangeVert.upperThumb.top + rangeVert.upperThumb.height / 2;
  check(
    `${prefix} vertical range handles clamp inside the rail`,
    rangeVert.capsule.bottom - rangeVertLowerY >= 4 && rangeVertUpperY - rangeVert.capsule.top >= 4,
    `lower ${rangeVertLowerY} upper ${rangeVertUpperY} capsule ${rangeVert.capsule.top}..${rangeVert.capsule.bottom}`,
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
