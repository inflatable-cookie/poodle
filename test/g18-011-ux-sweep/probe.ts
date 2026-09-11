/**
 * g18.011 CodeEditor UX acceptance sweep — focused real-browser journey.
 *
 * Drives the real public `#/components/code-editor` specimen route on both
 * preview applications (Svelte :4173, React :4180) through ordinary editing,
 * configuration, keyboard, focus, theme and layout journeys, capturing
 * screenshots and a JSON report. Evidence-only: this sweep never repairs
 * product code.
 *
 * Run from the repository root:
 *   effigy test:g18-011-ux-sweep-code-editor
 */
import { chromium, webkit, type Browser, type BrowserType, type Page } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

import { startPreviews } from "../visual/server";

const browserFlag =
  process.argv.find((a) => a.startsWith("--browser="))?.slice("--browser=".length) ?? "";

const OUT_BASE = fileURLToPath(new URL("./out", import.meta.url));

type Severity = "blocking" | "follow-up" | "accepted";
interface Finding {
  id: string;
  surface: string;
  framework: string;
  severity: Severity;
  action: string;
  expected: string;
  observed: string;
  evidence: string[];
}
const findings: Finding[] = [];
let passes = 0;

function record(
  id: string,
  framework: string,
  severity: Severity,
  action: string,
  expected: string,
  observed: string,
  evidence: string[] = [],
): void {
  findings.push({ id, surface: "CodeEditor", framework, severity, action, expected, observed, evidence });
}

function ok(label: string): void {
  passes += 1;
  console.log(`  ok   ${label}`);
}

function fail(
  id: string,
  framework: string,
  severity: Severity,
  action: string,
  expected: string,
  observed: string,
  evidence: string[] = [],
): void {
  record(id, framework, severity, action, expected, observed, evidence);
  console.error(`  FAIL ${label(action)} [${severity}]`);
}

function label(action: string): string {
  return action.length > 100 ? `${action.slice(0, 97)}...` : action;
}

const MODED = process.platform === "darwin" ? "Meta" : "Control";

interface Summary {
  editorCount: number;
  role: string | null;
  ariaLabel: string | null;
  contentEditable: string | null;
  tokenSpans: number;
  coloredSpans: number;
  lineColors: string[];
  baseColor: string | null;
}

async function readEditor(page: Page, part: string): Promise<Summary> {
  return page.evaluate((part) => {
    const frame = document.querySelector(`[data-part='${part}']`);
    const root = frame?.querySelector(".poodle-code-editor");
    const content = root?.querySelector(".cm-content");
    const spans = content ? [...content.querySelectorAll("span")] : [];
    const base = content ? getComputedStyle(content).color : null;
    const lines = content ? [...content.querySelectorAll(".cm-line")] : [];
    const lineColors = [
      ...new Set(lines.map((l) => getComputedStyle(l).color)),
    ];
    let colored = 0;
    if (base) {
      for (const span of spans) {
        if (getComputedStyle(span).color !== base) colored += 1;
      }
    }
    return {
      editorCount: document.querySelectorAll(".poodle-code-editor").length,
      role: content?.getAttribute("role") ?? null,
      ariaLabel: content?.getAttribute("aria-label") ?? null,
      contentEditable: content?.getAttribute("contenteditable") ?? null,
      tokenSpans: spans.length,
      coloredSpans: colored,
      lineColors,
      baseColor: base,
    };
  }, part);
}

async function settle(page: Page, ms = 400): Promise<void> {
  await page.evaluate(
    (ms) =>
      new Promise<void>((resolve) => {
        requestAnimationFrame(() => window.setTimeout(resolve, ms));
      }),
    ms,
  );
}

async function focusedIn(page: Page, part: string): Promise<boolean> {
  return page.evaluate((part) => {
    const frame = document.querySelector(`[data-part='${part}']`);
    return frame instanceof HTMLElement && frame.contains(document.activeElement);
  }, part);
}

async function clickPart(page: Page, part: string): Promise<void> {
  await page.locator(`[data-part='${part}']`).first().click();
}

async function shoot(page: Page, name: string, outDir: string): Promise<string> {
  const file = `${outDir}/${name}.png`;
  await page.screenshot({ path: file, fullPage: false });
  return file;
}

interface NetLog {
  urls: string[];
}

function grammarHits(net: NetLog): { javascript: number; json: number; other: number } {
  const hits = { javascript: 0, json: 0, other: 0 };
  for (const url of net.urls) {
    if (url.includes("lang-javascript")) hits.javascript += 1;
    else if (url.includes("lang-json")) hits.json += 1;
    else if (url.includes("legacy-modes") || url.includes("lang-")) hits.other += 1;
  }
  return hits;
}

/** Svelte router accepts `#/components/x`; React router only `#components/x`. */
function specimenUrl(base: string, framework: "svelte" | "react", slug: string, theme = "eclipse", density = ""): string {
  const hash = framework === "svelte" ? `#/components/${slug}` : `#components/${slug}`;
  const densityParam = density ? `&density=${density}` : "";
  return `${base}/?theme=${theme}${densityParam}${hash}`;
}

async function runFramework(
  browser: Browser,
  framework: "svelte" | "react",
  base: string,
  outDir: string,
): Promise<void> {
  const context = await browser.newContext({ viewport: { width: 1280, height: 900 } });
  const page = await context.newPage();
  const net: NetLog = { urls: [] };
  page.on("request", (r) => net.urls.push(r.url()));

  // ---- journey 0: deep-link parity between the two preview harnesses ----
  const canonicalHash = `#/components/code-editor`;
  await page.goto(`${base}/?theme=eclipse${canonicalHash}`, { waitUntil: "load", timeout: 60_000 });
  await settle(page, 2500);
  const deepLinkMounted = await page.evaluate(
    () => document.querySelectorAll(".poodle-code-editor").length,
  );
  if (deepLinkMounted >= 4)
    ok(`${framework}: canonical deep link ${canonicalHash} opens the specimen`);
  else
    record(
      "F12-deep-link",
      framework,
      "follow-up",
      `open ${base}/?theme=eclipse#/components/code-editor (the hash form the Svelte preview accepts and the visual gate uses)`,
      "the code-editor specimen mounts",
      `specimen did not mount (${deepLinkMounted} editors); the catalogue landing rendered instead — only the hash form without the leading slash routes`,
      [],
    );

  const url = specimenUrl(base, framework, "code-editor");
  net.urls = []; // network assertions below are per-load
  await page.goto(url, { waitUntil: "load", timeout: 60_000 });
  await page.waitForSelector(".poodle-code-editor", { timeout: 30_000 });
  await settle(page, 1200);
  console.log(`\n=== ${framework} (${url}) ===`);

  // ---- journey 1: mount + editing surface semantics ----
  const live = await readEditor(page, "live-editor");
  if (live.editorCount >= 4) ok(`${framework}: four editor postures mounted`);
  else fail("F-mount", framework, "follow-up", "open the code-editor specimen", "at least 4 editor postures", `found ${live.editorCount}`);
  if (live.role === "textbox" && live.contentEditable === "true" && live.ariaLabel)
    ok(`${framework}: editing surface exposes multiline textbox with accessible name`);
  else
    fail(
      "F-a11y-surface",
      framework,
      "blocking",
      "inspect the live editor surface",
      "role=textbox, contenteditable, aria-label present",
      `role=${live.role} contenteditable=${live.contentEditable} label=${live.ariaLabel}`,
    );

  // ---- journey 2: syntax visibility (the sweep dimension) ----
  await settle(page, 2500); // allow the lazy typescript grammar to resolve
  const ts = await readEditor(page, "live-editor");
  if (ts.tokenSpans > 0 && ts.coloredSpans > 0)
    ok(`${framework}: TypeScript tokens painted (${ts.coloredSpans}/${ts.tokenSpans} spans colored)`);
  else
    fail(
      "F1-syntax-visibility",
      framework,
      "blocking",
      "mount the live TypeScript editor (language=typescript, registry with typescript+json) and wait for the lazy grammar to resolve",
      "syntax tokens visibly colored: code spans carry token classes/styles with non-default color (performanceMode=full tokenization observable)",
      `zero visible tokenization: ${ts.tokenSpans} spans in .cm-content, ${ts.coloredSpans} colored; all ${ts.lineColors.length} line color value(s) identical (${[...ts.lineColors].join(", ")})`,
      [await shoot(page, `${framework}-1-typescript-no-tokens`, outDir)],
    );

  // language switching still works at the configuration level
  await clickPart(page, "language-json");
  await settle(page, 1500);
  const jsonRead = await readEditor(page, "config-editor");
  const jsonText = await page.evaluate(
    () => document.querySelector("[data-part='config-editor'] .cm-content")?.textContent ?? "",
  );
  if (jsonText.includes('"'))
    ok(`${framework}: JSON language selection swaps the configured source`);
  else fail("F-lang-switch", framework, "follow-up", "select the JSON language", "JSON source rendered", `content=${jsonText.slice(0, 40)}`);
  if (jsonRead.tokenSpans > 0 && jsonRead.coloredSpans > 0)
    ok(`${framework}: JSON tokens painted`);
  else
    fail(
      "F1-syntax-visibility",
      framework,
      "blocking",
      "switch the configured editor to language=json and wait for the lazy grammar",
      "JSON syntax tokens visibly colored",
      `zero visible tokenization: ${jsonRead.tokenSpans} spans, ${jsonRead.coloredSpans} colored`,
      [await shoot(page, `${framework}-2-json-no-tokens`, outDir)],
    );

  const pressed = await page.evaluate(
    () =>
      ["plain-text", "typescript", "json"].map((id) => ({
        id,
        pressed: document.querySelector(`[data-part='language-${id}']`)?.getAttribute("aria-pressed") ?? null,
      })),
  );
  if (pressed.find((p) => p.id === "json")?.pressed === "true")
    ok(`${framework}: language buttons report aria-pressed truthfully`);
  else fail("F-lang-pressed", framework, "follow-up", "inspect language buttons", "active language aria-pressed=true", JSON.stringify(pressed));

  // plain-text posture: selection state moves and the editor keeps rendering
  await clickPart(page, "language-plain-text");
  await settle(page, 600);
  const plain = await readEditor(page, "config-editor");
  const plainPressed = await page.evaluate(() => ({
    plain: document.querySelector("[data-part='language-plain-text']")?.getAttribute("aria-pressed") ?? null,
    typescript: document.querySelector("[data-part='language-typescript']")?.getAttribute("aria-pressed") ?? null,
    configText: (document.querySelector("[data-part='config-editor'] .cm-content")?.textContent ?? "").length,
  }));
  if (plainPressed.plain === "true" && plainPressed.typescript === "false" && plainPressed.configText > 0)
    ok(`${framework}: plain-text selection moves aria-pressed and the editor keeps rendering its source`);
  else
    fail("F-lang-plain", framework, "follow-up", "select plain-text", "plain pressed, typescript unpressed, source still rendered", JSON.stringify(plainPressed));

  // back to typescript for later journeys
  await clickPart(page, "language-typescript");
  await settle(page, 800);

  // ---- journey 3: lazy grammar loading honesty (network-level) ----
  const hits = grammarHits(net);
  console.log(`       grammar network requests (this load): javascript=${hits.javascript} json=${hits.json} other=${hits.other}`);
  if (hits.javascript === 1) ok(`${framework}: typescript grammar loaded lazily exactly once per editor mount`);
  else fail("F2-lazy-load", framework, "follow-up", "load the page with a typescript default editor", "exactly one lang-javascript fetch per mount", `${hits.javascript} fetches`);
  if (hits.json <= 1) ok(`${framework}: the journey's first JSON selection fetched the json grammar at most once`);
  else fail("F2-lazy-load", framework, "follow-up", "first json selection", "one lang-json fetch", `${hits.json} fetches`);
  await clickPart(page, "language-plain-text");
  await settle(page, 800);
  const plainHits = grammarHits(net);
  if (plainHits.json === hits.json && plainHits.javascript === hits.javascript)
    ok(`${framework}: selecting plain-text fetches no grammar (built-in)`);
  else fail("F2-lazy-load", framework, "follow-up", "select plain-text", "no grammar fetch", JSON.stringify(plainHits));
  await clickPart(page, "language-json");
  await settle(page, 1000);
  const memoHits = grammarHits(net);
  if (memoHits.json === hits.json) ok(`${framework}: re-selecting json triggers no new grammar fetch (memoized)`);
  else fail("F2-memo", framework, "follow-up", "re-select json after plain-text", "no new lang-json request", `grew to ${memoHits.json}`);
  await clickPart(page, "language-typescript");
  await settle(page, 300);

  // ---- journey 4: live editing, controlled echo, undo/redo ----
  const before = await page.evaluate(
    () => document.querySelector("[data-part='host-value']")?.textContent ?? "",
  );
  await page.locator("[data-part='live-editor'] .cm-content").click();
  await page.keyboard.press(`${MODED}+ArrowDown`);
  await page.keyboard.press("End");
  await page.keyboard.type(" // swept");
  await settle(page, 400);
  const afterType = await page.evaluate(
    () => document.querySelector("[data-part='host-value']")?.textContent ?? "",
  );
  if (afterType.includes("// swept") && afterType.startsWith(before))
    ok(`${framework}: typing updates the host-controlled value (echo, exact prefix)`);
  else
    fail(
      "F3-echo",
      framework,
      "blocking",
      "click into the live editor, move to end, type ' // swept'",
      "host readout appends the typed text (controlled echo)",
      `before=${JSON.stringify(before.slice(-30))} after=${JSON.stringify(afterType.slice(-30))}`,
    );
  await page.keyboard.press(`${MODED}+z`);
  await settle(page, 300);
  const afterUndo = await page.evaluate(
    () => document.querySelector("[data-part='host-value']")?.textContent ?? "",
  );
  if (afterUndo === before) ok(`${framework}: platform undo reverts the host value exactly`);
  else fail("F3-undo", framework, "follow-up", "press Mod+Z after typing", `readout returns to ${JSON.stringify(before.slice(-30))}`, `got ${JSON.stringify(afterUndo.slice(-30))}`);
  await page.keyboard.press(`${MODED}+Shift+z`);
  await settle(page, 300);
  const afterRedo = await page.evaluate(
    () => document.querySelector("[data-part='host-value']")?.textContent ?? "",
  );
  if (afterRedo.includes("// swept")) ok(`${framework}: redo re-applies the edit`);
  else fail("F3-redo", framework, "follow-up", "press Mod+Shift+Z", "typed text returns", JSON.stringify(afterRedo.slice(-30)));
  // clean back
  await page.keyboard.press(`${MODED}+z`);
  await settle(page, 200);

  // ---- journey 5: line numbers live reconfiguration ----
  const guttersBefore = await page.evaluate(
    () => document.querySelector("[data-part='config-editor'] .cm-gutters .cm-lineNumbers") !== null,
  );
  await clickPart(page, "line-numbers-toggle");
  await settle(page, 400);
  const guttersAfter = await page.evaluate(
    () => document.querySelector("[data-part='config-editor'] .cm-gutters .cm-lineNumbers") !== null,
  );
  if (guttersBefore && !guttersAfter) ok(`${framework}: line-number gutter reconfigures live (off)`);
  else fail("F4-gutter", framework, "follow-up", "toggle line numbers off", "gutter unmounts without remount", `before=${guttersBefore} after=${guttersAfter}`);
  await clickPart(page, "line-numbers-toggle");
  await settle(page, 400);
  const guttersRestored = await page.evaluate(
    () => document.querySelector("[data-part='config-editor'] .cm-gutters .cm-lineNumbers") !== null,
  );
  if (guttersRestored) ok(`${framework}: line-number gutter reconfigures live (on)`);
  else fail("F4-gutter", framework, "follow-up", "toggle line numbers back on", "gutter returns", "gutter still missing");

  // ---- journey 6: diagnostics ----
  const diagMark = await page.evaluate(
    () => document.querySelectorAll("[data-part='diagnostics-editor'] [data-poodle-diagnostic]").length,
  );
  if (diagMark >= 1) ok(`${framework}: diagnostic decorations present`);
  else fail("F5-diagnostics", framework, "follow-up", "open diagnostics editor", "diagnostic marks mounted", "no [data-poodle-diagnostic] element");
  await page.locator("[data-part='diagnostics-editor'] .cm-content").click();
  await page.keyboard.press("F8");
  await settle(page, 400);
  const diagMsg = await page.evaluate(
    () => document.querySelector("[data-part='diagnostics-editor'] .poodle-code-editor__diagnostic-message")?.textContent ?? "",
  );
  if (diagMsg.trim().length > 0) ok(`${framework}: diagnostic navigation announces a message`);
  else fail("F5-diagnostics", framework, "follow-up", "press F8 in the diagnostics editor", "active diagnostic message rendered", "message empty");

  // ---- journey 7: read-only refusal ----
  const roTextBefore = await page.evaluate(
    () => document.querySelector("[data-part='read-only-editor'] .cm-content")?.textContent ?? "",
  );
  await page.locator("[data-part='read-only-editor'] .cm-content").click();
  await page.keyboard.press(`${MODED}+a`);
  await settle(page, 200);
  const roSelection = await page.evaluate(() => window.getSelection()?.toString().length ?? 0);
  if (roSelection > 0)
    ok(`${framework}: read-only keeps the selection available for copy (select-all held ${roSelection} chars)`);
  else
    fail("F6-readonly", framework, "follow-up", "select all in the read-only editor", "selection preserved for copy", "empty selection");
  await page.keyboard.type("MUTATED");
  await settle(page, 400);
  const roTextAfter = await page.evaluate(
    () => document.querySelector("[data-part='read-only-editor'] .cm-content")?.textContent ?? "",
  );
  if (roTextAfter === roTextBefore) ok(`${framework}: read-only refuses text mutation`);
  else fail("F6-readonly", framework, "blocking", "select all and type in the read-only editor", "text unchanged", "text mutated");

  // ---- journey 8: Tab escape ----
  await page.locator("[data-part='live-editor'] .cm-content").click();
  await page.keyboard.press("Tab");
  await settle(page, 300);
  const tabLeft = await page.evaluate(() => {
    const active = document.activeElement;
    return !(active instanceof HTMLElement && !!active.closest("[data-part='live-editor'] .cm-content"));
  });
  if (tabLeft) ok(`${framework}: Tab leaves the editor (tabBehavior focus)`);
  else fail("F7-tab", framework, "blocking", "press Tab inside the editor", "focus leaves the editing surface", "focus stayed inside the editor");

  // ---- journey 9: find panel ----
  await page.locator("[data-part='live-editor'] .cm-content").click();
  await page.keyboard.press(`${MODED}+f`);
  await settle(page, 500);
  const findVisible = await page.evaluate(() => {
    const input = document.querySelector("[data-part='live-editor'] .cm-search input[name='search'], [data-part='live-editor'] .cm-panel input[name='search']");
    return input instanceof HTMLElement && document.activeElement === input;
  });
  if (findVisible) ok(`${framework}: Mod+F opens the find panel with focus in its input`);
  else fail("F8-find", framework, "follow-up", "press Mod+F", "find panel input focused", "panel not open or not focused");
  await page.keyboard.type("Point");
  await settle(page, 300);
  await page.keyboard.press("Enter");
  await settle(page, 200);
  const matchSelected = await page.evaluate(() => {
    const sel = window.getSelection();
    return sel ? sel.toString().length > 0 : false;
  });
  if (matchSelected) ok(`${framework}: Enter selects the first match`);
  else fail("F8-find", framework, "follow-up", "press Enter in find", "match selected", "no selection");
  await page.keyboard.press("Escape");
  await settle(page, 300);
  const panelClosed = await page.evaluate(
    () => document.querySelector("[data-part='live-editor'] .cm-search, [data-part='live-editor'] .cm-panel") === null,
  );
  const focusBack = await focusedIn(page, "live-editor");
  if (panelClosed && focusBack) ok(`${framework}: Escape closes find and returns focus to the surface`);
  else fail("F8-find", framework, "follow-up", "press Escape in find", "panel closed + focus returned", `closed=${panelClosed} focusBack=${focusBack}`);

  // ---- journey 10: focus treatment (g18.010) ----
  const modality0 = await page.evaluate(() => document.documentElement.getAttribute("data-poodle-input-modality"));
  // pointer entry: click into the diagnostics editor
  await page.locator("[data-part='diagnostics-editor'] .cm-content").click();
  await settle(page, 300);
  const pointerEntry = await page.evaluate(
    () => document.querySelector("[data-part='diagnostics-editor'] .poodle-code-editor")?.getAttribute("data-focus-entry") ?? null,
  );
  if (pointerEntry === null) ok(`${framework}: pointer entry paints no outer treatment`);
  else fail("F9-focus", framework, "follow-up", "click into an editor", "no treatment on pointer entry", `treatment value=${pointerEntry}`);
  // keyboard entry: blur everything, then Tab-walk from the document origin
  // until the editing surface is reached — the same journey a keyboard user
  // takes. The treatment must arm on that entry.
  await page.evaluate(() => (document.activeElement as HTMLElement | null)?.blur());
  let kbEntry: { entry: string | null; focused: boolean } = { entry: null, focused: false };
  for (let i = 0; i < 60; i += 1) {
    await page.keyboard.press("Tab");
    await settle(page, 120);
    kbEntry = await page.evaluate(() => ({
      entry: document.querySelector("[data-part='diagnostics-editor'] .poodle-code-editor")?.getAttribute("data-focus-entry") ?? null,
      focused: (() => {
        const el = document.activeElement;
        return el instanceof HTMLElement && !!el.closest("[data-part='diagnostics-editor'] .cm-content");
      })(),
    }));
    if (kbEntry.focused) break;
  }
  if (kbEntry.focused && kbEntry.entry === "keyboard")
    ok(`${framework}: keyboard entry arms the outer treatment`);
  else
    fail("F9-focus", framework, "follow-up", "Tab from the document origin into the editing surface", "data-focus-entry=keyboard on keyboard entry", `focused=${kbEntry.focused} entry=${kbEntry.entry}`);
  const modality1 = await page.evaluate(() => document.documentElement.getAttribute("data-poodle-input-modality"));
  if (modality0 === modality1 && (modality1 === "keyboard" || modality1 === "pointer"))
    ok(`${framework}: the editor never writes the document modality (${modality0} → ${modality1})`);
  else
    fail("F9-focus", framework, "follow-up", "compare the document modality across the editor journeys", "unchanged and always a known modality", `${modality0} → ${modality1}`);

  // ---- journey 11: constrained layout at Desktop-like width ----
  await page.setViewportSize({ width: 900, height: 700 });
  await settle(page, 500);
  const overflow = await page.evaluate(
    () => document.documentElement.scrollWidth - document.documentElement.clientWidth,
  );
  if (overflow <= 1) ok(`${framework}: no page-level horizontal overflow at 900px`);
  else fail("F10-layout", framework, "blocking", "view the code-editor specimen at 900px width", "no horizontal page overflow", `overflow of ${overflow}px`);
  await shoot(page, `${framework}-3-constrained-900`, outDir);
  await page.setViewportSize({ width: 1280, height: 900 });

  await context.close();
}

async function themeJourney(browser: Browser, framework: "svelte" | "react", base: string, outDir: string): Promise<void> {
  const context = await browser.newContext({ viewport: { width: 1280, height: 900 } });
  const page = await context.newPage();
  const bg: Record<string, string> = {};
  for (const theme of ["eclipse", "clay", "nord"]) {
    await page.goto(specimenUrl(base, framework, "code-editor", theme), { waitUntil: "load" });
    await page.waitForSelector(".poodle-code-editor", { timeout: 30_000 });
    await settle(page, 800);
    bg[theme] = await page.evaluate(() => {
      const root = document.querySelector("[data-part='live-editor'] .poodle-code-editor");
      return root ? getComputedStyle(root).backgroundColor : "missing";
    });
    await shoot(page, `${framework}-theme-${theme}`, outDir);
  }
  const distinct = new Set(Object.values(bg)).size;
  if (distinct >= 2) ok(`${framework}: editor surface follows themes (${JSON.stringify(bg)})`);
  else fail("F11-theme", framework, "follow-up", "load the specimen under eclipse/clay/nord", "surface background varies by theme", JSON.stringify(bg));
  await context.close();
}

/** The route must honor the preview's density configuration (card work item 2). */
async function densityJourney(browser: Browser, framework: "svelte" | "react", base: string, outDir: string): Promise<void> {
  const context = await browser.newContext({ viewport: { width: 1280, height: 900 } });
  const page = await context.newPage();
  const seen: Array<{ density: string; shellAttr: string | null; panelX: string; editors: number; editorHeight: number }> = [];
  for (const density of ["comfortable", "compact"]) {
    await page.goto(specimenUrl(base, framework, "code-editor", "eclipse", density), { waitUntil: "load" });
    await page.waitForSelector(".poodle-code-editor", { timeout: 30_000 });
    await settle(page, 700);
    seen.push(
      await page.evaluate((density) => {
        const shell = document.querySelector(".poodle-app-shell") ?? document.documentElement;
        const cs = shell ? getComputedStyle(shell) : null;
        const editor = document.querySelector(".poodle-code-editor");
        return {
          density,
          shellAttr: shell?.getAttribute("data-density") ?? null,
          panelX: cs?.getPropertyValue("--poodle-space-panel-x").trim() ?? "",
          editors: document.querySelectorAll(".poodle-code-editor").length,
          editorHeight: editor ? Math.round(editor.getBoundingClientRect().height) : 0,
        };
      }, density),
    );
    await shoot(page, `${framework}-density-${density}`, outDir);
  }
  const [a, b] = seen;
  if (
    a && b &&
    a.shellAttr === "comfortable" && b.shellAttr === "compact" &&
    a.panelX !== b.panelX &&
    a.editors >= 4 && b.editors >= 4 &&
    a.editorHeight > 0 && b.editorHeight > 0
  )
    ok(`${framework}: route honors density (${a.density}: panel-x ${a.panelX}, ${b.density}: panel-x ${b.panelX}) with all editors mounted`);
  else
    fail("F13-density", framework, "follow-up", "load the code-editor specimen under comfortable and compact densities", "density attribute and density-driven spacing change while every editor stays mounted", JSON.stringify(seen));
  await context.close();
}

const BASES: Record<"svelte" | "react", string> = {
  svelte: "",
  react: "",
};

/** Launch exactly the engine(s) named by --browser (sibling-probe convention). */
const ENGINES: Array<[string, BrowserType]> = (
  [
    ["chromium", chromium],
    ["webkit", webkit],
  ] as Array<[string, BrowserType]>
).filter(([name]) => !browserFlag || browserFlag === name);

if (ENGINES.length === 0) {
  console.error(`unknown --browser=${browserFlag}`);
  process.exit(2);
}

const servers = await startPreviews();
BASES.svelte = servers.urls.svelte;
BASES.react = servers.urls.react;

try {
  for (const [engineName, engine] of ENGINES) {
    const outDir = `${OUT_BASE}/${engineName}`;
    mkdirSync(outDir, { recursive: true });
    const passesBefore = passes;
    const findingsBefore = findings.length;
    const browser = await engine.launch();
    try {
      for (const fw of ["svelte", "react"] as const) {
        await runFramework(browser, fw, BASES[fw], outDir);
        await themeJourney(browser, fw, BASES[fw], outDir);
        await densityJourney(browser, fw, BASES[fw], outDir);
      }
    } finally {
      await browser.close();
    }
    const engineFindings = findings.slice(findingsBefore);
    const engineBlocking = engineFindings.filter((f) => f.severity === "blocking").length;
    writeFileSync(
      `${outDir}/report.json`,
      JSON.stringify(
        { browser: engineName, passes: passes - passesBefore, findings: engineFindings, blockingCount: engineBlocking },
        null,
        2,
      ),
    );
    console.log(`\n=== sweep summary (${engineName}) ===`);
    console.log(`passes: ${passes - passesBefore}`);
    console.log(`findings: ${engineFindings.length} (blocking: ${engineBlocking})`);
    for (const f of engineFindings) {
      console.log(`\n[${f.severity.toUpperCase()}] ${f.id} — ${f.framework}`);
      console.log(`  action:   ${f.action}`);
      console.log(`  expected: ${f.expected}`);
      console.log(`  observed: ${f.observed}`);
    }
  }
} finally {
  await servers.stop();
}

const blocking = findings.filter((f) => f.severity === "blocking");
if (blocking.length > 0) {
  console.error(`\n${blocking.length} blocking finding(s) across ${ENGINES.map(([n]) => n).join(", ")}`);
  process.exit(1);
}
console.log(`\nall CodeEditor UX sweep checks passed (${ENGINES.map(([n]) => n).join(", ")})`);
