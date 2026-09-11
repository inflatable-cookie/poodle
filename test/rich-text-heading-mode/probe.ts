/**
 * Headless Chromium + WebKit proof for g18.020: the RichTextEditor's heading
 * levels are consumer-configurable document modes projected as one Poodle
 * text-mode Select. happy-dom can only approximate a real listbox and a real
 * contenteditable selection, so this probe drives a live browser through the
 * paired Svelte and React fixtures in `test/rich-text-heading-mode`.
 *
 *   bun test/rich-text-heading-mode/probe.ts --browser=chromium
 *   bun test/rich-text-heading-mode/probe.ts --browser=webkit
 */

import { chromium, webkit, type Browser, type BrowserType, type Page } from "playwright";
import { fileURLToPath } from "node:url";

const browserFlag = process.argv
  .find((arg) => arg.startsWith("--browser="))
  ?.slice("--browser=".length);
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
const port = 4199;
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
  throw new Error(`rich-text heading-mode fixture on :${port} did not start`);
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

async function settle(page: Page): Promise<void> {
  await page.evaluate(
    () =>
      new Promise<void>((resolve) => {
        window.setTimeout(resolve, 120);
      }),
  );
}

function sectionOf(framework: string): string {
  return `[data-framework="${framework}"]`;
}

function editorOf(framework: string, part: string): string {
  return `${sectionOf(framework)} [data-part="${part}"]`;
}

function triggerOf(framework: string, part: string): string {
  return `${editorOf(framework, part)} [data-command="heading-select"] button.poodle-select__trigger`;
}

async function triggerState(
  page: Page,
  framework: string,
  part: string,
): Promise<{ mode: string | null; label: string; ariaLabel: string | null }> {
  return page.evaluate(
    (payload) => {
      const root = document.querySelector(
        `[data-framework="${payload.framework}"] [data-part="${payload.part}"]`,
      );
      const value = root?.querySelector(".poodle-rich-text-editor__heading-value");
      return {
        mode: value?.getAttribute("data-heading-mode") ?? null,
        label: value?.textContent?.trim() ?? "",
        ariaLabel: root
          ?.querySelector('[data-command="heading-select"] button.poodle-select__trigger')
          ?.getAttribute("aria-label") ?? null,
      };
    },
    { framework, part },
  );
}

async function optionValues(page: Page): Promise<string[]> {
  return page.evaluate(() =>
    [...document.querySelectorAll(".poodle-select__option")].map(
      (option) => option.getAttribute("data-value") ?? "",
    ),
  );
}

async function listboxOpen(page: Page): Promise<boolean> {
  return page.evaluate(() => document.querySelector(".poodle-select__listbox") !== null);
}

async function blockTagOf(
  page: Page,
  framework: string,
  text: string,
): Promise<string | null> {
  return page.evaluate(
    (payload) => {
      const surface = document.querySelector(
        `[data-framework="${payload.framework}"] [data-part="heading-editor"] .ProseMirror`,
      );
      if (!surface) return null;
      const block = [...surface.querySelectorAll("h1,h2,h3,h4,h5,h6,p")].find(
        (element) => element.textContent === payload.text,
      );
      return block?.tagName ?? null;
    },
    { framework, text },
  );
}

async function changeCount(page: Page, framework: string): Promise<number> {
  return page.evaluate((fw) => {
    const el = document.querySelector(`[data-framework="${fw}"] [data-part="change-count"]`);
    return Number(el?.getAttribute("data-count") ?? "-1");
  }, framework);
}

async function chooseOption(
  page: Page,
  framework: string,
  part: string,
  value: string,
): Promise<void> {
  await page.locator(triggerOf(framework, part)).click();
  await settle(page);
  await page.locator(`.poodle-select__option[data-value="${value}"]`).click();
  await settle(page);
}

async function closeListbox(page: Page): Promise<void> {
  if (await listboxOpen(page)) {
    await page.keyboard.press("Escape");
    await settle(page);
  }
}

async function runFramework(page: Page, framework: string, browserName: string): Promise<void> {
  const headingEditor = editorOf(framework, "heading-editor");
  const sparseEditor = editorOf(framework, "sparse-editor");
  await page.locator(`${headingEditor} .ProseMirror`).waitFor();
  await page.locator(`${sparseEditor} .ProseMirror`).waitFor();
  await settle(page);

  // 1. Projection: one selector, zero separate heading buttons, exact levels.
  const projection = await page.evaluate(
    (fw) => {
      const root = document.querySelector(`[data-framework="${fw}"] [data-part="heading-editor"]`);
      const select = root?.querySelector('[data-command="heading-select"]');
      const buttons = [1, 2, 3, 4, 5, 6].filter((level) =>
        root?.querySelector(`[data-command="heading-${level}"]`),
      );
      return {
        select: select !== null && select !== undefined,
        buttons: buttons.length,
        headingCommands: select?.getAttribute("data-heading-commands") ?? null,
      };
    },
    framework,
  );
  check(
    `${browserName} ${framework} heading levels project as one selector and never as buttons`,
    projection.select && projection.buttons === 0,
    `select=${projection.select} buttons=${projection.buttons}`,
  );
  check(
    `${browserName} ${framework} the selector carries all six configured levels`,
    projection.headingCommands === "heading-1 heading-2 heading-3 heading-4 heading-5 heading-6",
    `headingCommands=${projection.headingCommands}`,
  );

  // 2. Full menu: Normal text plus exactly H1–H6.
  await page.locator(triggerOf(framework, "heading-editor")).click();
  await settle(page);
  const fullOptions = await optionValues(page);
  check(
    `${browserName} ${framework} the full menu is Normal text plus H1–H6`,
    fullOptions.join(",") ===
      "normal,heading-1,heading-2,heading-3,heading-4,heading-5,heading-6",
    `options=${fullOptions.join(",")}`,
  );
  await closeListbox(page);

  // 3. Sparse configuration: exactly the admitted levels, never the full set.
  await page.locator(triggerOf(framework, "sparse-editor")).click();
  await settle(page);
  const sparseOptions = await optionValues(page);
  check(
    `${browserName} ${framework} a sparse toolbar offers Normal text plus only H2 and H4`,
    sparseOptions.join(",") === "normal,heading-2,heading-4",
    `options=${sparseOptions.join(",")}`,
  );
  await closeListbox(page);

  // 4. Truthful trigger: one active heading, Normal text, and Mixed.
  await page.locator(`${headingEditor} .ProseMirror h2`).click();
  await settle(page);
  const headingState = await triggerState(page, framework, "heading-editor");
  check(
    `${browserName} ${framework} a caret in H2 reports the active level`,
    headingState.mode === "heading" && headingState.label === "Heading 2",
    `mode=${headingState.mode} label=${headingState.label}`,
  );
  check(
    `${browserName} ${framework} the trigger's accessible name carries the active mode`,
    (headingState.ariaLabel ?? "").includes("Heading 2"),
    `aria-label=${headingState.ariaLabel}`,
  );

  await page.locator(`${headingEditor} .ProseMirror p`).click();
  await settle(page);
  const normalState = await triggerState(page, framework, "heading-editor");
  check(
    `${browserName} ${framework} a caret in the paragraph reports Normal text`,
    normalState.mode === "normal" && normalState.label === "Normal text",
    `mode=${normalState.mode} label=${normalState.label}`,
  );

  await page.locator(`${headingEditor} .ProseMirror h2`).click();
  await page.keyboard.press("End");
  await page.keyboard.press("Shift+ArrowDown");
  await settle(page);
  const mixedState = await triggerState(page, framework, "heading-editor");
  check(
    `${browserName} ${framework} a selection spanning block modes reports Mixed`,
    mixedState.mode === "mixed" && mixedState.label === "Mixed",
    `mode=${mixedState.mode} label=${mixedState.label}`,
  );

  // 5. Geometry: stable trigger height, bounded option rows, no page overflow.
  await page.locator(`${headingEditor} .ProseMirror p`).click();
  await settle(page);
  const normalTrigger = await page
    .locator(triggerOf(framework, "heading-editor"))
    .boundingBox();
  await page.locator(`${headingEditor} .ProseMirror h2`).click();
  await settle(page);
  const headingTrigger = await page
    .locator(triggerOf(framework, "heading-editor"))
    .boundingBox();
  check(
    `${browserName} ${framework} the trigger height is stable across text modes`,
    normalTrigger !== null &&
      headingTrigger !== null &&
      Math.abs(normalTrigger.height - headingTrigger.height) <= 1,
    `normal=${normalTrigger?.height} heading=${headingTrigger?.height}`,
  );

  await page.locator(triggerOf(framework, "heading-editor")).click();
  await settle(page);
  const optionGeometry = await page.evaluate(() => {
    const rows = [...document.querySelectorAll(".poodle-select__option")];
    const preview = (value: string) =>
      rows
        .find((row) => row.querySelector(`[data-heading-level="${value}"]`))
        ?.querySelector(".poodle-rich-text-editor__heading-option-label") ?? null;
    const maxRow = Math.max(
      ...rows.map((row) => row.getBoundingClientRect().height),
      0,
    );
    return {
      maxRow,
      normalFont: Number.parseFloat(getComputedStyle(preview("normal") ?? rows[0]).fontSize),
      h1Font: Number.parseFloat(getComputedStyle(preview("1") ?? rows[0]).fontSize),
    };
  });
  check(
    `${browserName} ${framework} menu rows stay bounded while previewing the document scale`,
    optionGeometry.maxRow > 0 && optionGeometry.maxRow <= 44,
    `maxRow=${optionGeometry.maxRow}`,
  );
  check(
    `${browserName} ${framework} the H1 option previews a larger type scale than Normal text`,
    optionGeometry.h1Font > optionGeometry.normalFont,
    `h1=${optionGeometry.h1Font} normal=${optionGeometry.normalFont}`,
  );
  await closeListbox(page);

  const overflow = await page.evaluate(() => ({
    scrollWidth: document.documentElement.scrollWidth,
    innerWidth: window.innerWidth,
    constrainedWidth: document
      .querySelector('[data-part="sparse-editor"] .poodle-rich-text-editor')
      ?.getBoundingClientRect().width,
  }));
  check(
    `${browserName} ${framework} a constrained pane never forces page-width overflow`,
    overflow.scrollWidth <= overflow.innerWidth + 1 &&
      (overflow.constrainedWidth ?? 0) > 0 &&
      (overflow.constrainedWidth ?? 0) <= 21 * 16,
    `scrollWidth=${overflow.scrollWidth} innerWidth=${overflow.innerWidth} constrained=${overflow.constrainedWidth}`,
  );

  // 6. Exact setting: one change, same-level stays, focus and caret survive the echo.
  await page.locator(`${headingEditor} .ProseMirror p`).click();
  await settle(page);
  const before = await changeCount(page, framework);
  await chooseOption(page, framework, "heading-editor", "heading-4");
  const setCount = await changeCount(page, framework);
  const bodyAfterSet = await blockTagOf(page, framework, "Body copy");
  check(
    `${browserName} ${framework} choosing Heading 4 emits exactly one change and makes the block real H4`,
    setCount === before + 1 && bodyAfterSet === "H4",
    `before=${before} after=${setCount} tag=${bodyAfterSet}`,
  );
  const afterSet = await page.evaluate((fw) => {
    const surface = document.querySelector(
      `[data-framework="${fw}"] [data-part="heading-editor"] .ProseMirror`,
    );
    const selection = window.getSelection();
    const anchor = selection?.anchorNode ?? null;
    const element = anchor instanceof Element ? anchor : anchor?.parentElement ?? null;
    const block = element?.closest("h1,h2,h3,h4,h5,h6,p") ?? null;
    return {
      focused: document.activeElement === surface,
      blockText: block?.textContent ?? null,
    };
  }, framework);
  check(
    `${browserName} ${framework} focus and the caret survive the controlled echo`,
    afterSet.focused && afterSet.blockText === "Body copy",
    `focused=${afterSet.focused} block=${afterSet.blockText}`,
  );

  await chooseOption(page, framework, "heading-editor", "heading-4");
  const sameLevelCount = await changeCount(page, framework);
  const bodyAfterSameLevel = await blockTagOf(page, framework, "Body copy");
  check(
    `${browserName} ${framework} choosing the active level keeps it instead of toggling off`,
    sameLevelCount === setCount && bodyAfterSameLevel === "H4",
    `changes=${sameLevelCount} tag=${bodyAfterSameLevel}`,
  );

  // 7. Undo/redo run through the echo exactly once per history step.
  await page.locator(`${headingEditor} [data-command="undo"] button`).click();
  await settle(page);
  const afterUndo = await blockTagOf(page, framework, "Body copy");
  check(
    `${browserName} ${framework} undo reverts the heading through the accepted echo`,
    afterUndo === "P",
    `tag=${afterUndo}`,
  );
  await page.locator(`${headingEditor} [data-command="redo"] button`).click();
  await settle(page);
  const afterRedo = await blockTagOf(page, framework, "Body copy");
  check(
    `${browserName} ${framework} redo restores the heading through the accepted echo`,
    afterRedo === "H4",
    `tag=${afterRedo}`,
  );

  // 8. Keyboard ownership: one roving stop; an open listbox owns its keys.
  const trigger = page.locator(triggerOf(framework, "heading-editor"));
  await trigger.focus();
  await page.keyboard.press("ArrowRight");
  await settle(page);
  const roved = await page.evaluate(
    (fw) =>
      document.activeElement?.closest(
        `[data-framework="${fw}"] [data-part="heading-editor"] [data-command="heading-select"]`,
      ) === null,
    framework,
  );
  check(
    `${browserName} ${framework} the closed trigger roves on to the next toolbar stop`,
    roved,
    `activeElement=${await page.evaluate(() => document.activeElement?.getAttribute("aria-label"))}`,
  );

  await trigger.focus();
  await page.keyboard.press("Enter");
  await settle(page);
  check(
    `${browserName} ${framework} Enter on the trigger opens the listbox`,
    await listboxOpen(page),
  );
  await page.keyboard.press("ArrowDown");
  await settle(page);
  const highlightedOpen = await page.evaluate(() => ({
    focusedTrigger: document.activeElement?.classList.contains("poodle-select__trigger") ?? false,
    highlighted: document.querySelector('.poodle-select__option[data-highlighted="true"]')?.getAttribute("data-value") ?? null,
  }));
  check(
    `${browserName} ${framework} ArrowDown highlights inside the listbox without stealing focus`,
    highlightedOpen.focusedTrigger && highlightedOpen.highlighted !== null,
    `focusedTrigger=${highlightedOpen.focusedTrigger} highlighted=${highlightedOpen.highlighted}`,
  );
  await page.keyboard.press("End");
  await settle(page);
  const endHighlight = await page.evaluate(
    () =>
      document
        .querySelector('.poodle-select__option[data-highlighted="true"]')
        ?.getAttribute("data-value") ?? null,
  );
  await page.keyboard.press("Home");
  await settle(page);
  const homeHighlight = await page.evaluate(
    () =>
      document
        .querySelector('.poodle-select__option[data-highlighted="true"]')
        ?.getAttribute("data-value") ?? null,
  );
  check(
    `${browserName} ${framework} Home and End move the listbox highlight to its bounds`,
    homeHighlight === "normal" && endHighlight === "heading-6",
    `home=${homeHighlight} end=${endHighlight}`,
  );
  await page.keyboard.press("ArrowRight");
  await settle(page);
  const stillOnTrigger = await page.evaluate(() => ({
    trigger: document.activeElement?.classList.contains("poodle-select__trigger") ?? false,
    open: document.querySelector(".poodle-select__listbox") !== null,
  }));
  check(
    `${browserName} ${framework} an open listbox keeps the toolbar from roving behind it`,
    stillOnTrigger.trigger && stillOnTrigger.open,
    `onTrigger=${stillOnTrigger.trigger} open=${stillOnTrigger.open}`,
  );
  await page.keyboard.press("Escape");
  await settle(page);
  check(
    `${browserName} ${framework} Escape closes the listbox`,
    !(await listboxOpen(page)),
  );
  await page.keyboard.press("ArrowRight");
  await settle(page);
  const rovedAfterClose = await page.evaluate(
    (fw) =>
      document.activeElement?.closest(
        `[data-framework="${fw}"] [data-part="heading-editor"] [data-command="heading-select"]`,
      ) === null,
    framework,
  );
  check(
    `${browserName} ${framework} closing restores the ordinary roving journey`,
    rovedAfterClose,
  );
}

try {
  for (const [browserName, browserType] of engines) {
    let browser: Browser | undefined;
    try {
      browser = await browserType.launch({ headless: true });
      const page = await browser.newPage({ viewport: { width: 960, height: 720 } });
      await page.goto(url, { waitUntil: "networkidle" });
      await page.waitForSelector('[data-framework="svelte"] .poodle-rich-text-editor');
      await page.waitForSelector('[data-framework="react"] .poodle-rich-text-editor');
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

console.log("\nall RichTextEditor heading-mode checks passed");
