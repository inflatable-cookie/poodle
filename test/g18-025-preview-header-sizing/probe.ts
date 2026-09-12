/**
 * g18.025 preview header control sizing — paired real-browser geometry proof.
 *
 *   bun test/g18-025-preview-header-sizing/probe.ts --browser=chromium
 *   bun test/g18-025-preview-header-sizing/probe.ts --browser=webkit
 *
 * Law under test, on both live preview applications (Svelte, React):
 * - every painted header control resolves the fixed `md` stop through the
 *   size context — the specimen `controlSize` axis never reaches the header;
 * - ThemeSelect, the block Slider, and the search TextInput paint the 36px
 *   md ladder; the two ToggleGroups paint their documented item contract
 *   (the 36px ladder minus the 0.25rem item inset in
 *   docs/contracts/components/toggle-group.md), identically in both
 *   frameworks;
 * - controls on the same row share top edges; full-height controls share
 *   bottom edges;
 * - selecting specimen xs–xl moves the catalogue/pills, never the header;
 * - density changes never inflate the md chrome;
 * - Size, Density, Theme, Contrast, Search, wrapping, and keyboard journeys
 *   keep working with the chrome pinned.
 */
import { chromium, webkit, type BrowserType, type Page } from "playwright";

import { startPreviews } from "../visual/server";

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

let failures = 0;

function check(label: string, ok: boolean, detail = ""): void {
  if (!ok) failures += 1;
  console.log(`${ok ? "  ok  " : "  FAIL"}  ${label}${detail ? `  — ${detail}` : ""}`);
}

function near(a: number, b: number, tolerance = 1.5): boolean {
  return Math.abs(a - b) <= tolerance;
}

const HEADER = ".poodle-display-controls";
const SIZES = ["xs", "sm", "md", "lg", "xl"] as const;
const MD_PX = 36;

type Box = { height: number; top: number; bottom: number };

type HeaderMeasure = {
  header: Box;
  controls: Array<{ name: string; box: Box; rowTop: number; size: string | null }>;
};

async function measureHeader(page: Page): Promise<HeaderMeasure> {
  return page.evaluate(() => {
    const header = document.querySelector<HTMLElement>(".poodle-display-controls");
    if (!header) throw new Error("header did not mount");
    const pick = (selector: string): { box: Box; rowTop: number; size: string | null } | null => {
      const el = header.querySelector<HTMLElement>(selector);
      if (!el) return null;
      const r = el.getBoundingClientRect();
      const group = el.closest<HTMLElement>(".poodle-display-controls__group");
      const gr = (group ?? el).getBoundingClientRect();
      // The painted control's resolved size stop: the nearest data-size root.
      const sized = el.closest<HTMLElement>("[data-size]");
      return {
        box: { height: r.height, top: r.top, bottom: r.bottom },
        rowTop: gr.top,
        size: sized?.getAttribute("data-size") ?? null,
      };
    };
    const picks: Array<[string, string]> = [
      ["theme", ".poodle-theme-select__trigger"],
      ["density", '.poodle-toggle-group[aria-label="Density"] .poodle-toggle-group__item'],
      ["size", '.poodle-toggle-group[aria-label="Control size"] .poodle-toggle-group__item'],
      ["contrast", ".poodle-slider"],
      ["search", ".poodle-text-input"],
    ];
    const controls = picks.flatMap(([name, selector]) => {
      const picked = pick(selector);
      return picked ? [{ name, ...picked }] : [];
    });
    const hr = header.getBoundingClientRect();
    return { header: { height: hr.height, top: hr.top, bottom: hr.bottom }, controls };
  });
}

// The md ladder is 36px. ToggleGroup items paint the ladder minus the
// documented 0.25rem item inset (docs/contracts/components/toggle-group.md,
// "Item .toggle-group__item: min-height = calc(height - 0.25rem)") — a
// reusable-component contract this repair must not override.
const EXPECTED_MD_HEIGHT: Record<string, number> = {
  theme: MD_PX,
  density: MD_PX - 4,
  size: MD_PX - 4,
  contrast: MD_PX,
  search: MD_PX,
};

/** Assert the md-chrome geometry for one measured header. */
function assertChromeGeometry(prefix: string, measure: HeaderMeasure): void {
  const names = measure.controls.map((control) => control.name).sort().join(",");
  check(`${prefix} all five painted controls measured`, measure.controls.length === 5, names);
  for (const { name, box, size } of measure.controls) {
    const expected = EXPECTED_MD_HEIGHT[name]!;
    check(`${prefix} ${name} resolves the md stop`, size === "md", `data-size=${size}`);
    check(
      `${prefix} ${name} paints its md-contract height (${expected}px)`,
      near(box.height, expected),
      `${box.height}px`,
    );
  }
  // Bucket controls into visual rows by their group's top edge, then align
  // within each row: shared tops; bottoms shared by the full-height controls
  // with the toggle items inset by exactly the documented 4px contract.
  const rows: Array<{ rowTop: number; controls: HeaderMeasure["controls"] }> = [];
  for (const control of measure.controls) {
    const row = rows.find((candidate) => near(candidate.rowTop, control.rowTop, 2));
    if (row) row.controls.push(control);
    else rows.push({ rowTop: control.rowTop, controls: [control] });
  }
  for (const [index, row] of rows.entries()) {
    const tops = row.controls.map((control) => control.box.top);
    check(
      `${prefix} row ${index} controls share top edges`,
      Math.max(...tops) - Math.min(...tops) <= 2,
      `tops ${tops.map((top) => top.toFixed(1)).join("/")}`,
    );
    const full = row.controls.filter((control) => EXPECTED_MD_HEIGHT[control.name] === MD_PX);
    const inset = row.controls.filter((control) => EXPECTED_MD_HEIGHT[control.name] === MD_PX - 4);
    if (full.length > 0) {
      const bottoms = full.map((control) => control.box.bottom);
      check(
        `${prefix} row ${index} full-height controls share bottom edges`,
        Math.max(...bottoms) - Math.min(...bottoms) <= 2,
        `bottoms ${bottoms.map((bottom) => bottom.toFixed(1)).join("/")}`,
      );
      for (const control of inset) {
        const delta = bottoms[0]! - control.box.bottom;
        check(
          `${prefix} row ${index} ${control.name} item sits at the documented 4px contract inset`,
          near(delta, 4),
          `inset ${delta.toFixed(1)}px`,
        );
      }
    }
  }
}

/** Cross-framework parity: every painted box matches the other framework. */
function assertFrameworkParity(
  svelte: HeaderMeasure,
  react: HeaderMeasure,
  label: string,
): void {
  for (const control of svelte.controls) {
    const twin = react.controls.find((candidate) => candidate.name === control.name);
    check(
      `${label} ${control.name} height matches across frameworks`,
      twin !== undefined && near(twin.box.height, control.box.height),
      `svelte ${control.box.height} react ${twin?.box.height.toFixed(1) ?? "missing"}`,
    );
  }
}

async function selectSize(page: Page, framework: string, size: string): Promise<void> {
  await page
    .locator(`.poodle-toggle-group[aria-label="Control size"] [data-toggle-value="${size}"]`)
    .click();
  // The selection must reach app state: the top bar's size pill shows it.
  const pill = page.locator(".poodle-app-top-bar__pills .poodle-pill").nth(2);
  await pill.filter({ hasText: size }).waitFor({ timeout: 5000 });
  check(`${framework} size pill reflects ${size}`, true);
}

async function probe(
  page: Page,
  engine: string,
  framework: string,
  base: string,
  baseline: { headerHeight: number } | undefined,
): Promise<{ headerHeight: number }> {
  await page.setViewportSize({ width: 1280, height: 900 });
  await page.goto(base, { waitUntil: "domcontentloaded", timeout: 120_000 });
  await page.locator(HEADER).waitFor({ timeout: 60_000 });
  await page.locator(".poodle-app-top-bar__pills .poodle-pill").first().waitFor({ timeout: 30_000 });

  // Header stability baseline: the header height at the first size stop must
  // survive every later stop, density change, and theme change.
  await selectSize(page, framework, "md");
  const first = await measureHeader(page);
  assertChromeGeometry(`${engine} ${framework} md stop`, first);
  const headerHeight = baseline?.headerHeight ?? first.header.height;

  for (const size of SIZES) {
    await selectSize(page, framework, size);
    const measure = await measureHeader(page);
    assertChromeGeometry(`${engine} ${framework} ${size} stop`, measure);
    check(
      `${engine} ${framework} header height stable through ${size}`,
      near(measure.header.height, headerHeight, 1),
      `height ${measure.header.height} vs ${headerHeight}`,
    );
  }

  // Density journey: the ambient density axis still flows (the app state
  // changes) and the md chrome never inflates.
  await page.locator('.poodle-toggle-group[aria-label="Density"] [data-toggle-value="comfortable"]').click();
  const densityPill = page.locator(".poodle-app-top-bar__pills .poodle-pill").nth(1);
  await densityPill.filter({ hasText: "comfortable" }).waitFor({ timeout: 5000 });
  const comfortable = await measureHeader(page);
  assertChromeGeometry(`${engine} ${framework} comfortable density`, comfortable);
  check(
    `${engine} ${framework} density change never inflates the md chrome`,
    near(comfortable.header.height, headerHeight, 1),
    `height ${comfortable.header.height} vs ${headerHeight}`,
  );
  await page.locator('.poodle-toggle-group[aria-label="Density"] [data-toggle-value="compact"]').click();
  await densityPill.filter({ hasText: "compact" }).waitFor({ timeout: 5000 });

  // Theme journey: the select opens, a swatch selection reaches app state,
  // and Escape closes the surface.
  const trigger = page.locator(".poodle-theme-select__trigger");
  await trigger.click();
  await page.locator(".poodle-theme-select__surface").waitFor({ timeout: 5000 });
  check(`${engine} ${framework} theme popover opens`, (await trigger.getAttribute("aria-expanded")) === "true");
  await page.keyboard.press("Escape");
  await page.waitForFunction(
    () => document.querySelector<HTMLElement>(".poodle-theme-select__trigger")?.getAttribute("aria-expanded") === "false",
    undefined,
    { timeout: 5000 },
  );
  check(`${engine} ${framework} theme popover closes on Escape`, true);
  const before = await page.locator(".poodle-app-top-bar__pills .poodle-pill").first().innerText();
  await trigger.click();
  await page.locator(".poodle-theme-select__surface .poodle-theme-select__tile").first().click();
  const after = await page.locator(".poodle-app-top-bar__pills .poodle-pill").first().innerText();
  check(`${engine} ${framework} theme selection reaches the shell`, before !== after, `${before} -> ${after}`);

  // Contrast journey: keyboard moves the slider, chrome stays pinned.
  const handle = page.locator(`${HEADER} [role="slider"]`);
  const beforeValue = Number(await handle.getAttribute("aria-valuenow"));
  await handle.focus();
  await page.keyboard.press("ArrowRight");
  await page.waitForFunction(
    (previous) => Number(document.querySelector('[role="slider"]')?.getAttribute("aria-valuenow")) > previous,
    beforeValue,
    { timeout: 5000 },
  );
  check(`${engine} ${framework} contrast ArrowRight moves the value`, true, `${beforeValue} ->`);
  const afterContrast = await measureHeader(page);
  assertChromeGeometry(`${engine} ${framework} after contrast keypress`, afterContrast);

  // Search journey: typing swaps the catalogue to results and back.
  const familyCountBefore = await page.locator("[data-catalogue-family]").count();
  const input = page.locator("input.poodle-text-input__control");
  await input.fill("button");
  await page.locator('[data-catalogue-search="true"]').waitFor({ timeout: 10_000 });
  check(`${engine} ${framework} search swaps the catalogue to results`, true);
  await input.fill("");
  await page.locator("[data-catalogue-family]").first().waitFor({ timeout: 10_000 });
  const familyCountAfter = await page.locator("[data-catalogue-family]").count();
  check(
    `${engine} ${framework} clearing search restores the catalogue`,
    familyCountAfter === familyCountBefore,
    `${familyCountBefore} -> ${familyCountAfter}`,
  );
  const afterSearch = await measureHeader(page);
  assertChromeGeometry(`${engine} ${framework} after search journey`, afterSearch);

  // Wrapping: a narrow viewport must wrap rows without stretching controls.
  await page.setViewportSize({ width: 560, height: 900 });
  const wrapped = await measureHeader(page);
  const rows = new Set(wrapped.controls.map((control) => Math.round(control.rowTop / 4)));
  check(`${engine} ${framework} narrow viewport wraps the header`, rows.size > 1, `${rows.size} rows`);
  assertChromeGeometry(`${engine} ${framework} wrapped`, wrapped);
  await page.setViewportSize({ width: 1280, height: 900 });

  return { headerHeight };
}

let servers: Awaited<ReturnType<typeof startPreviews>> | undefined;

try {
  servers = await startPreviews();
  for (const [name, engine] of engines) {
    console.log(`\n[g18-025-preview-header-sizing] ${name}`);
    const browser = await engine.launch({ headless: true });
    const page = await browser.newPage({ viewport: { width: 1280, height: 900 } });
    // One measured header per framework at the md stop, for parity.
    await page.setViewportSize({ width: 1280, height: 900 });
    await page.goto(servers.urls.svelte, { waitUntil: "domcontentloaded", timeout: 120_000 });
    await page.locator(HEADER).waitFor({ timeout: 60_000 });
    const svelte = await measureHeader(page);
    await page.goto(servers.urls.react, { waitUntil: "domcontentloaded", timeout: 120_000 });
    await page.locator(HEADER).waitFor({ timeout: 60_000 });
    const react = await measureHeader(page);
    assertFrameworkParity(svelte, react, `${name} parity`);
    // Full journeys per framework, sharing one stability baseline per engine.
    const svelteRun = await probe(page, name, "svelte", servers.urls.svelte, undefined);
    await probe(page, name, "react", servers.urls.react, svelteRun);
    await browser.close();
  }
} finally {
  // Stops only the servers this run spawned; reused previews stay up (the
  // visual gate contract in test/visual/server.ts).
  await servers?.stop();
}

process.exit(failures === 0 ? 0 : 1);
