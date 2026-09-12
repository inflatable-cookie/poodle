/**
 * g18.025 preview header control sizing — paired real-browser geometry proof.
 *
 *   bun test/g18-025-preview-header-sizing/probe.ts --browser=chromium
 *   bun test/g18-025-preview-header-sizing/probe.ts --browser=webkit
 *
 * Law under test, on both live preview applications (Svelte, React):
 * - every painted header control resolves the selected `controlSize` stop
 *   through the size context;
 * - all five painted controls (ThemeSelect, both ToggleGroups, block Slider,
 *   search TextInput) measure the selected shared ladder and share top and
 *   bottom edges per row;
 * - ToggleGroup uses the same shared ladder outside the header too,
 *   identically in both frameworks;
 * - selecting specimen xs–xl moves the catalogue, pills, and header together;
 * - density changes never alter the selected size geometry;
 * - Size, Density, Theme, Contrast, Search, wrapping, and keyboard journeys
 *   keep working while the five controls stay aligned.
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

// ── Shared size oracle ────────────────────────────────────────────────
// The shared control-size ladder in px, by ToggleGroup data-size.
const SIZE_LADDER_PX: Record<string, number> = { xs: 24, sm: 28, md: 36, lg: 44, xl: 52 };

/** Assert the selected chrome geometry for one measured header. */
function assertChromeGeometry(prefix: string, measure: HeaderMeasure, expectedSize: string): void {
  const expectedHeight = SIZE_LADDER_PX[expectedSize];
  if (expectedHeight === undefined) throw new Error(`unknown expected header size ${expectedSize}`);
  const names = measure.controls.map((control) => control.name).sort().join(",");
  check(`${prefix} all five painted controls measured`, measure.controls.length === 5, names);
  for (const { name, box, size } of measure.controls) {
    check(`${prefix} ${name} resolves the ${expectedSize} stop`, size === expectedSize, `data-size=${size}`);
    check(
      `${prefix} ${name} paints the ${expectedHeight}px ${expectedSize} ladder`,
      near(box.height, expectedHeight),
      `${box.height}px`,
    );
  }
  // Bucket controls into visual rows by their group's top edge, then align
  // within each row: shared tops and shared bottoms beneath equal labels.
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
    const bottoms = row.controls.map((control) => control.box.bottom);
    check(
      `${prefix} row ${index} controls share bottom edges`,
      Math.max(...bottoms) - Math.min(...bottoms) <= 2,
      `bottoms ${bottoms.map((bottom) => bottom.toFixed(1)).join("/")}`,
    );
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
): Promise<void> {
  await page.setViewportSize({ width: 1280, height: 900 });
  await page.goto(base, { waitUntil: "domcontentloaded", timeout: 120_000 });
  await page.locator(HEADER).waitFor({ timeout: 60_000 });
  await page.locator(".poodle-app-top-bar__pills .poodle-pill").first().waitFor({ timeout: 30_000 });

  // Start from the shell default and prove all five controls resolve sm.
  await selectSize(page, framework, "sm");
  const first = await measureHeader(page);
  assertChromeGeometry(`${engine} ${framework} sm stop`, first, "sm");

  for (const size of SIZES) {
    await selectSize(page, framework, size);
    const measure = await measureHeader(page);
    assertChromeGeometry(`${engine} ${framework} ${size} stop`, measure, size);
  }

  // Density journey: reset to sm, then prove density changes app state without
  // changing the selected size geometry.
  await selectSize(page, framework, "sm");
  const beforeDensity = await measureHeader(page);
  await page.locator('.poodle-toggle-group[aria-label="Density"] [data-toggle-value="comfortable"]').click();
  const densityPill = page.locator(".poodle-app-top-bar__pills .poodle-pill").nth(1);
  await densityPill.filter({ hasText: "comfortable" }).waitFor({ timeout: 5000 });
  const comfortable = await measureHeader(page);
  assertChromeGeometry(`${engine} ${framework} comfortable density`, comfortable, "sm");
  check(
    `${engine} ${framework} density change preserves sm header geometry`,
    near(comfortable.header.height, beforeDensity.header.height, 1),
    `height ${comfortable.header.height} vs ${beforeDensity.header.height}`,
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

  // Contrast journey: keyboard moves the slider without disturbing alignment.
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
  assertChromeGeometry(`${engine} ${framework} after contrast keypress`, afterContrast, "sm");

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
  assertChromeGeometry(`${engine} ${framework} after search journey`, afterSearch, "sm");

  // Wrapping: a narrow viewport must wrap rows without stretching controls.
  await page.setViewportSize({ width: 560, height: 900 });
  const wrapped = await measureHeader(page);
  const rows = new Set(wrapped.controls.map((control) => Math.round(control.rowTop / 4)));
  check(`${engine} ${framework} narrow viewport wraps the header`, rows.size > 1, `${rows.size} rows`);
  assertChromeGeometry(`${engine} ${framework} wrapped`, wrapped, "sm");
  await page.setViewportSize({ width: 1280, height: 900 });

}

/** ToggleGroup uses the same ladder in ordinary catalogue content too. */
async function probeToggleGroupLadder(page: Page, engine: string, framework: string, base: string): Promise<void> {
  const route = framework === "svelte" ? "#/components/toggle-group" : "#components/toggle-group";
  await page.goto(`${base}/${route}`, { waitUntil: "domcontentloaded", timeout: 120_000 });
  await page.locator(".poodle-toggle-group__item").first().waitFor({ timeout: 60_000 });
  const samples = await page.evaluate(() => {
    const items = [...document.querySelectorAll<HTMLElement>(".poodle-toggle-group__item")]
      .filter((item) => !item.closest(".poodle-display-controls"));
    return items.slice(0, 6).map((item) => {
      const group = item.closest<HTMLElement>(".poodle-toggle-group")!;
      return {
        size: group.getAttribute("data-size") ?? "unknown",
        height: item.getBoundingClientRect().height,
      };
    });
  });
  check(
    `${engine} ${framework} found catalogue toggle items outside the header`,
    samples.length > 0,
    `${samples.length} items`,
  );
  for (const [index, sample] of samples.entries()) {
    const ladder = SIZE_LADDER_PX[sample.size];
    if (ladder === undefined) {
      check(`${engine} ${framework} catalogue item ${index} has a known size stop`, false, sample.size);
      continue;
    }
    check(
      `${engine} ${framework} catalogue item ${index} (${sample.size}) paints the shared ladder (${ladder}px)`,
      near(sample.height, ladder),
      `${sample.height}px`,
    );
  }
}

let servers: Awaited<ReturnType<typeof startPreviews>> | undefined;

try {
  servers = await startPreviews();
  for (const [name, engine] of engines) {
    console.log(`\n[g18-025-preview-header-sizing] ${name}`);
    const browser = await engine.launch({ headless: true });
    const page = await browser.newPage({ viewport: { width: 1280, height: 900 } });
    // One measured header per framework at the default sm stop, for parity.
    await page.setViewportSize({ width: 1280, height: 900 });
    await page.goto(servers.urls.svelte, { waitUntil: "domcontentloaded", timeout: 120_000 });
    await page.locator(HEADER).waitFor({ timeout: 60_000 });
    const svelte = await measureHeader(page);
    await page.goto(servers.urls.react, { waitUntil: "domcontentloaded", timeout: 120_000 });
    await page.locator(HEADER).waitFor({ timeout: 60_000 });
    const react = await measureHeader(page);
    assertFrameworkParity(svelte, react, `${name} parity`);
    // Full journeys per framework.
    await probe(page, name, "svelte", servers.urls.svelte);
    await probe(page, name, "react", servers.urls.react);
    await probeToggleGroupLadder(page, name, "svelte", servers.urls.svelte);
    await probeToggleGroupLadder(page, name, "react", servers.urls.react);
    await browser.close();
  }
} finally {
  // Stops only the servers this run spawned; reused previews stay up (the
  // visual gate contract in test/visual/server.ts).
  await servers?.stop();
}

process.exit(failures === 0 ? 0 : 1);
