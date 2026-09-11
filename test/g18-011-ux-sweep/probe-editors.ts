/**
 * g18.011 UX acceptance sweep — RichTextEditor, RichTextRenderer,
 * MarkdownRenderer real-browser journeys.
 *
 * Drives the real public specimen routes on both preview applications
 * (Svelte :4173 `#/components/<slug>`, React :4180 `#components/<slug>`)
 * through ordinary editing, configuration, keyboard, pointer, theme and
 * constrained-layout journeys, capturing screenshots and a JSON report.
 * Evidence-only: this sweep never repairs product code.
 *
 * Run from the repository root:
 *   effigy test:g18-011-ux-sweep-editors
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
  surface: string,
  framework: string,
  severity: Severity,
  action: string,
  expected: string,
  observed: string,
  evidence: string[] = [],
): void {
  findings.push({ id, surface, framework, severity, action, expected, observed, evidence });
}

function ok(label: string): void {
  passes += 1;
  console.log(`  ok   ${label}`);
}

function fail(
  id: string,
  surface: string,
  framework: string,
  severity: Severity,
  action: string,
  expected: string,
  observed: string,
  evidence: string[] = [],
): void {
  record(id, surface, framework, severity, action, expected, observed, evidence);
  console.error(`  FAIL ${surface}/${id} [${severity}] ${framework}: ${expected} — ${observed}`);
}

const MODED = process.platform === "darwin" ? "Meta" : "Control";

function specimenUrl(base: string, framework: "svelte" | "react", slug: string, theme = "eclipse", density = ""): string {
  const hash = framework === "svelte" ? `#/components/${slug}` : `#components/${slug}`;
  const densityParam = density ? `&density=${density}` : "";
  return `${base}/?theme=${theme}${densityParam}${hash}`;
}

async function settle(page: Page, ms = 350): Promise<void> {
  await page.evaluate(
    (ms) =>
      new Promise<void>((resolve) => {
        requestAnimationFrame(() => window.setTimeout(resolve, ms));
      }),
    ms,
  );
}

async function shoot(page: Page, name: string, outDir: string): Promise<string> {
  const file = `${outDir}/${name}.png`;
  await page.screenshot({ path: file, fullPage: false });
  return file;
}

/** Evaluate a scoped query inside one data-part group. */
async function inPart<T>(
  page: Page,
  part: string,
  fn: string,
): Promise<T> {
  return page.evaluate(
    ([part, body]) => {
      // eslint-disable-next-line no-new-func
      return new Function(`return (${body})`)().call(null, document.querySelector(`[data-part='${part}']`));
    },
    [part, fn],
  ) as Promise<T>;
}

/* ================= RichTextEditor ================= */

interface RTEState {
  prosemirrorMounted: boolean;
  toolbarCommands: string[];
  boldPressed: string | null;
}

async function readRTEState(page: Page, part: string): Promise<RTEState> {
  return page.evaluate((part) => {
    const frame = document.querySelector(`[data-part='${part}']`);
    const pm = frame?.querySelector(".ProseMirror");
    const commands = frame
      ? [...frame.querySelectorAll("[data-command]")].map((el) => el.getAttribute("data-command") ?? "")
      : [];
    const bold = frame?.querySelector('[data-command="bold"] button');
    return {
      prosemirrorMounted: pm !== null,
      toolbarCommands: commands,
      boldPressed: bold?.getAttribute("aria-pressed") ?? null,
    };
  }, part);
}

async function hostReadout(page: Page): Promise<string> {
  return page.evaluate(
    () => document.querySelector("[data-part='host-document']")?.textContent ?? "",
  );
}

async function headingTriggerLabel(page: Page, part: string): Promise<string | null> {
  return page.evaluate(
    (part) =>
      document
        .querySelector(`[data-part='${part}'] .poodle-rich-text-editor__heading-value`)
        ?.textContent?.trim() ?? null,
    part,
  );
}

async function openHeadingMenu(page: Page, part: string): Promise<void> {
  await page.locator(`[data-part='${part}'] [data-command="heading-select"] button`).first().click();
  await page.waitForSelector(".poodle-select__listbox", { timeout: 5000 });
  await settle(page, 200);
}

async function openHeadingOptions(page: Page): Promise<string[]> {
  return page.evaluate(() =>
    [...document.querySelectorAll(".poodle-select__option")].map(
      (o) => o.getAttribute("data-value") ?? "",
    ),
  );
}

async function chooseHeading(page: Page, value: string): Promise<void> {
  await page.locator(`.poodle-select__option[data-value="${value}"]`).first().click();
  await settle(page, 300);
}

async function blockTags(page: Page, part: string): Promise<string[]> {
  return page.evaluate(
    (part) =>
      [...document.querySelectorAll(`[data-part='${part}'] .ProseMirror > *`)].map(
        (el) => el.tagName.toLowerCase(),
      ),
    part,
  );
}

async function runRichTextEditor(
  browser: Browser,
  framework: "svelte" | "react",
  base: string,
  outDir: string,
): Promise<void> {
  const context = await browser.newContext({ viewport: { width: 1280, height: 900 } });
  const page = await context.newPage();
  const url = specimenUrl(base, framework, "rich-text-editor");
  await page.goto(url, { waitUntil: "load", timeout: 60_000 });
  await page.waitForSelector("[data-part='live-editor'] .ProseMirror", { timeout: 30_000 });
  await settle(page, 600);
  console.log(`\n=== ${framework} RichTextEditor (${url}) ===`);
  const fw = framework;

  // ---- mount + toolbar projection ----
  const live = await readRTEState(page, "live-editor");
  if (live.prosemirrorMounted && live.toolbarCommands.includes("bold"))
    ok(`${fw}: live editor mounts with a projected toolbar`);
  else fail("F20-mount", "RichTextEditor", fw, "blocking", "open the rich-text-editor specimen", "ProseMirror surface + toolbar mounted", JSON.stringify(live));

  // ---- live typing echoes to the host JSON ----
  await page.locator("[data-part='live-editor'] .ProseMirror").click();
  await page.keyboard.press(`${MODED}+Home`);
  await page.keyboard.type("Swept ");
  await settle(page, 400);
  const readout1 = await hostReadout(page);
  if (readout1.includes("Swept ")) ok(`${fw}: typing echoes into the host document JSON`);
  else fail("F21-echo", "RichTextEditor", fw, "blocking", "type 'Swept ' at the document start", "host readout JSON contains the typed text", `readout head=${readout1.slice(0, 80)}`);

  // ---- undo reverts through the host echo ----
  let undoCount = 0;
  let readout2 = readout1;
  while (readout2.includes("Swept") && undoCount < 12) {
    await page.keyboard.press(`${MODED}+z`);
    undoCount += 1;
    await settle(page, 250);
    readout2 = await hostReadout(page);
  }
  if (!readout2.includes("Swept")) ok(`${fw}: undo reverts the document through the host echo`);
  else fail("F21-undo", "RichTextEditor", fw, "follow-up", "Mod+Z after typing", "typed text removed from host JSON", `still present after ${undoCount} undos`);

  // ---- selection drives toolbar state ----
  await page.locator("[data-part='live-editor'] .ProseMirror strong").first().click();
  await settle(page, 400);
  const boldOn = (await readRTEState(page, "live-editor")).boldPressed;
  await page.locator("[data-part='live-editor'] .ProseMirror p").first().click({ position: { x: 4, y: 4 } });
  await settle(page, 400);
  const boldOff = (await readRTEState(page, "live-editor")).boldPressed;
  if (boldOn === "true" && boldOff === "false")
    ok(`${fw}: selection drives the Bold pressed state (in bold text: ${boldOn}, plain text: ${boldOff})`);
  else fail("F22-toolbar-state", "RichTextEditor", fw, "blocking", "click into the bold word, then into a plain paragraph", "bold aria-pressed true inside bold, false outside", `in-bold=${boldOn} in-plain=${boldOff}`);

  // ---- table context commands ----
  await page.locator("[data-part='live-editor'] .ProseMirror th").first().click();
  await settle(page, 400);
  const tableCommands = await page.evaluate(() => {
    const frame = document.querySelector("[data-part='live-editor']");
    const states = [...frame.querySelectorAll("[data-command]")] .map((el) => ({
      command: el.getAttribute("data-command"),
      disabled: el.querySelector("button")?.hasAttribute("disabled") ?? null,
    }));
    return states.filter((s) => s.command?.startsWith("add-") || s.command === "delete-table" || s.command === "insert-table");
  });
  const tableEnabled = tableCommands.filter((c) => c.command !== "insert-table" && c.disabled === false);
  if (tableEnabled.length >= 3)
    ok(`${fw}: table context admits row/column/delete actions (${tableEnabled.length} enabled)`);
  else fail("F23-table", "RichTextEditor", fw, "blocking", "click into a table header cell", "row/column/delete table commands enabled", JSON.stringify(tableCommands));

  // ---- Mixed heading state across a block boundary ----
  const bodyStart = page.locator("[data-part='heading-mode-editor'] .ProseMirror p").first();
  await bodyStart.click();
  await page.keyboard.press("Shift+ArrowUp");
  await settle(page, 400);
  const mixedLabel = await headingTriggerLabel(page, "heading-mode-editor");
  if ((mixedLabel ?? "").toLowerCase().includes("mixed"))
    ok(`${fw}: a selection spanning paragraph and heading reads Mixed (${JSON.stringify(mixedLabel)})`);
  else fail("F24-mixed", "RichTextEditor", fw, "blocking", "select from the body paragraph back into Heading six", "trigger reads Mixed", `label=${JSON.stringify(mixedLabel)}`);
  await page.locator("[data-part='heading-mode-editor'] .ProseMirror h1").click();
  await settle(page, 250);

  // ---- full text-mode selector ----
  await openHeadingMenu(page, "heading-mode-editor");
  const fullOptions = await openHeadingOptions(page);
  if (fullOptions.length === 7 && fullOptions.includes("normal") && ["heading-1","heading-2","heading-3","heading-4","heading-5","heading-6"].every((v) => fullOptions.includes(v)))
    ok(`${fw}: full selector offers Normal text + H1–H6 exactly`);
  else fail("F25-full-select", "RichTextEditor", fw, "blocking", "open the full text-mode selector", "options normal + heading-1..6 (7)", JSON.stringify(fullOptions));
  await chooseHeading(page, "heading-6");
  const tags1 = await blockTags(page, "heading-mode-editor");
  const label6 = await headingTriggerLabel(page, "heading-mode-editor");
  if (tags1.includes("h6") && (label6 ?? "").toLowerCase().includes("heading 6"))
    ok(`${fw}: choosing Heading 6 produces a real h6 block and trigger label`);
  else fail("F25-full-select", "RichTextEditor", fw, "blocking", "choose Heading 6", "h6 element + Heading 6 trigger", `tags=${JSON.stringify(tags1)} label=${label6}`);
  // set-instead-of-toggle: re-choose the active level
  await openHeadingMenu(page, "heading-mode-editor");
  await chooseHeading(page, "heading-6");
  const tags2 = await blockTags(page, "heading-mode-editor");
  if (tags2.includes("h6")) ok(`${fw}: re-choosing the active level keeps it (no toggle-off)`);
  else fail("F25-full-select", "RichTextEditor", fw, "blocking", "re-choose Heading 6 while h6 active", "block stays h6", `tags=${JSON.stringify(tags2)}`);
  // Normal converts back: the block that was h6 (the selection's block) must
  // now be a paragraph. The fixture also carries its own h6 block, so assert
  // on the first block rather than the absence of all h6.
  await openHeadingMenu(page, "heading-mode-editor");
  await chooseHeading(page, "normal");
  const tags3 = await blockTags(page, "heading-mode-editor");
  if (tags3[0] === "p" && !tags3.includes("h1"))
    ok(`${fw}: Normal text converts the selected heading back to a paragraph`);
  else fail("F25-full-select", "RichTextEditor", fw, "follow-up", "choose Normal text with the selected h6 active", "the selected block becomes p", `tags=${JSON.stringify(tags3)}`);

  // ---- sparse selector ----
  await openHeadingMenu(page, "sparse-heading-editor");
  const sparseOptions = await openHeadingOptions(page);
  if (sparseOptions.length === 3 && sparseOptions.join() === ["normal","heading-2","heading-4"].join())
    ok(`${fw}: sparse selector offers exactly Normal + H2 + H4`);
  else fail("F26-sparse-select", "RichTextEditor", fw, "blocking", "open the sparse text-mode selector", "exactly normal, heading-2, heading-4", JSON.stringify(sparseOptions));
  await chooseHeading(page, "heading-4");
  const sparseTags = await blockTags(page, "sparse-heading-editor");
  if (sparseTags.includes("h4")) ok(`${fw}: sparse H4 choice produces a real h4 block`);
  else fail("F26-sparse-select", "RichTextEditor", fw, "blocking", "choose Heading 4 in the sparse selector", "h4 element appears", JSON.stringify(sparseTags));

  // ---- subset toolbar ----
  const subsetCommands = (await readRTEState(page, "subset-editor")).toolbarCommands;
  if (subsetCommands.length === 3 && subsetCommands.join() === ["bold","italic","link"].join())
    ok(`${fw}: subset toolbar projects exactly bold/italic/link`);
  else fail("F27-subset", "RichTextEditor", fw, "follow-up", "inspect the subset toolbar", "exactly bold, italic, link", JSON.stringify(subsetCommands));

  // ---- disabled state ----
  const disabled = await page.evaluate(() => {
    const frame = document.querySelector("[data-part='disabled-editor']");
    const root = frame?.querySelector(".poodle-rich-text-editor");
    const pm = frame?.querySelector(".ProseMirror");
    const anyButton = frame?.querySelector("button");
    return {
      dataDisabled: root?.hasAttribute("data-disabled") ?? false,
      inert: root?.hasAttribute("inert") ?? false,
      contentEditable: pm?.getAttribute("contenteditable") ?? null,
      buttonDisabled: anyButton?.hasAttribute("disabled") ?? null,
    };
  });
  if (disabled.dataDisabled && disabled.inert && disabled.contentEditable === "false" && disabled.buttonDisabled === true)
    ok(`${fw}: disabled editor is inert, non-editable, and removes its controls`);
  else fail("F28-disabled", "RichTextEditor", fw, "blocking", "inspect the disabled editor", "data-disabled + inert + contenteditable=false + disabled controls", JSON.stringify(disabled));

  // ---- image policy ----
  await page.locator("[data-part='images-toggle']").click();
  await page.waitForSelector("[data-part='image-policy-editor'] .ProseMirror", { timeout: 10_000 });
  await settle(page, 600);
  const seeded = await page.evaluate(() => {
    const img = document.querySelector("[data-part='image-policy-editor'] img");
    const rect = img?.getBoundingClientRect();
    return {
      count: document.querySelectorAll("[data-part='image-policy-editor'] img").length,
      natural: img ? `${img.naturalWidth}x${img.naturalHeight}` : null,
      box: rect ? `${Math.round(rect.width)}x${Math.round(rect.height)}` : null,
      srcIsData: img?.getAttribute("src")?.startsWith("data:image/") ?? false,
      alt: img?.getAttribute("alt") ?? null,
      requests: document.querySelector("[data-part='image-request-count']")?.getAttribute("data-count") ?? null,
    };
  });
  if (seeded.count === 1 && seeded.srcIsData && seeded.natural === "96x48" && seeded.box && !seeded.box.startsWith("0x") && seeded.requests === "0")
    ok(`${fw}: images-on mounts the seeded offline raster (96x48 decoded, alt=${JSON.stringify(seeded.alt)})`);
  else fail("F29-image-seed", "RichTextEditor", fw, "blocking", "toggle images on", "one seeded data-URL image decoded with a visible box and zero host requests", JSON.stringify(seeded));

  // insert image at retained selection
  await page.locator("[data-part='image-policy-editor'] .ProseMirror p").last().click();
  await page.locator("[data-part='image-policy-editor'] [data-command='insert-image'] button").click();
  await settle(page, 1200); // host picker delay is 300ms
  const inserted = await page.evaluate(() => ({
    images: document.querySelectorAll("[data-part='image-policy-editor'] img").length,
    alts: [...document.querySelectorAll("[data-part='image-policy-editor'] img")].map((i) => i.getAttribute("alt")),
    requests: document.querySelector("[data-part='image-request-count']")?.getAttribute("data-count") ?? null,
    changes: document.querySelector("[data-part='image-change-count']")?.getAttribute("data-count") ?? null,
  }));
  if (inserted.images === 2 && inserted.requests === "1" && inserted.alts?.includes("Revenue chart (host pick)"))
    ok(`${fw}: one host request inserts exactly one picked image at the selection (host images=${inserted.images}, changes=${inserted.changes})`);
  else fail("F29-image-insert", "RichTextEditor", fw, "blocking", "press Insert image with a host picker", "exactly one more image, requests=1", JSON.stringify(inserted));

  // toggle off retains the host document
  await page.locator("[data-part='images-toggle']").click();
  await settle(page, 500);
  const retained = await page.evaluate(() => ({
    editorImages: document.querySelectorAll("[data-part='image-policy-editor'] img").length,
    hostHasPicked: (document.querySelector("[data-part='image-host-document']")?.textContent ?? "").includes("Revenue chart (host pick)"),
  }));
  if (retained.editorImages === 0 && retained.hostHasPicked)
    ok(`${fw}: images-off swaps the editor back while the host document is retained`);
  else fail("F29-image-retention", "RichTextEditor", fw, "follow-up", "toggle images off", "editor without images; host JSON keeps the picked image", JSON.stringify(retained));

  // ---- paste sanitization ----
  await page.locator("[data-part='live-editor'] .ProseMirror").click();
  await page.keyboard.press(`${MODED}+Home`);
  await page.keyboard.press("ArrowDown");
  const pasteResult = await page.evaluate(() => {
    const pm = document.querySelector("[data-part='live-editor'] .ProseMirror");
    if (!pm) return { ok: false };
    const dt = new DataTransfer();
    dt.setData(
      "text/html",
      `<p onclick="window.__pwned=1">Pasted <b>hostile</b></p><script>window.__pwned=1<\/script><img src="x" onerror="window.__pwned=1">`,
    );
    const ev = new ClipboardEvent("paste", { clipboardData: dt, bubbles: true, cancelable: true });
    (pm as HTMLElement).dispatchEvent(ev);
    return { ok: true };
  });
  await settle(page, 500);
  const afterPaste = await page.evaluate(() => ({
    text: document.querySelector("[data-part='live-editor'] .ProseMirror")?.textContent ?? "",
    pwned: (window as unknown as { __pwned?: number }).__pwned ?? null,
    images: document.querySelectorAll("[data-part='live-editor'] img").length,
    readout: document.querySelector("[data-part='host-document']")?.textContent ?? "",
  }));
  if (
    pasteResult.ok &&
    afterPaste.text.includes("Pasted") &&
    afterPaste.text.includes("hostile") &&
    afterPaste.pwned === null &&
    afterPaste.images === 0 &&
    !afterPaste.readout.includes("onerror") &&
    !afterPaste.readout.includes("script")
  )
    ok(`${fw}: pasted HTML is sanitized (text kept, handler/script/img dropped, no execution)`);
  else fail("F30-paste", "RichTextEditor", fw, "blocking", "paste hostile HTML (handler + script + img)", "prose text kept; executable markup dropped without running", JSON.stringify({ dispatch: pasteResult, afterPaste: { ...afterPaste, readout: afterPaste.readout.slice(0, 60) } }));

  // ---- keyboard: table cell Tab navigation + Escape then Tab escape ----
  await page.locator("[data-part='live-editor'] .ProseMirror th").first().click();
  await settle(page, 250);
  await page.keyboard.press("Tab");
  await settle(page, 300);
  const inNextCell = await page.evaluate(() => {
    const sel = window.getSelection();
    const anchor = sel?.anchorNode;
    const cell = anchor instanceof Node ? (anchor.parentElement?.closest("th,td") ?? null) : null;
    return cell ? (cell.textContent ?? "").trim() : null;
  });
  if (inNextCell === "Status") ok(`${fw}: Tab in a table moves to the next cell`);
  else fail("F31-table-keys", "RichTextEditor", fw, "follow-up", "Tab from the first table header cell", "selection in the Status cell", `cell=${JSON.stringify(inNextCell)}`);
  await page.keyboard.press("Escape");
  await page.keyboard.press("Tab");
  await settle(page, 300);
  const leftEditor = await page.evaluate(() => {
    const active = document.activeElement;
    return !(active instanceof HTMLElement && !!active.closest("[data-part='live-editor']"));
  });
  if (leftEditor) ok(`${fw}: Escape then Tab always leaves the editor from a table`);
  else fail("F31-table-keys", "RichTextEditor", fw, "blocking", "Escape then Tab inside a table", "focus leaves the editor", "focus stayed inside");

  // ---- constrained layout ----
  await page.setViewportSize({ width: 900, height: 700 });
  await settle(page, 500);
  const overflow = await page.evaluate(
    () => document.documentElement.scrollWidth - document.documentElement.clientWidth,
  );
  if (overflow <= 1) ok(`${fw}: no page-level horizontal overflow at 900px`);
  else fail("F32-layout", "RichTextEditor", fw, "blocking", "view the rich-text-editor specimen at 900px", "no horizontal page overflow", `${overflow}px overflow`);
  await shoot(page, `${fw}-rte-constrained-900`, outDir);
  await page.setViewportSize({ width: 1280, height: 900 });

  await context.close();
}

/* ================= RichTextRenderer ================= */

async function runRichTextRenderer(
  browser: Browser,
  framework: "svelte" | "react",
  base: string,
  outDir: string,
): Promise<void> {
  const context = await browser.newContext({ viewport: { width: 1280, height: 900 } });
  const page = await context.newPage();
  const url = specimenUrl(base, framework, "rich-text-renderer");
  await page.goto(url, { waitUntil: "load", timeout: 60_000 });
  await page.waitForSelector("[data-part='standard-renderer']", { timeout: 30_000 });
  await settle(page, 500);
  console.log(`\n=== ${framework} RichTextRenderer (${url}) ===`);
  const fw = framework;

  // ---- inert rendering ----
  const inert = await page.evaluate(() => ({
    editable: document.querySelectorAll("[contenteditable='true']").length,
    prosemirror: document.querySelectorAll(".ProseMirror").length,
    toolbar: document.querySelectorAll("[data-command]").length,
  }));
  if (inert.editable === 0 && inert.prosemirror === 0 && inert.toolbar === 0)
    ok(`${fw}: renderer mounts no contenteditable, no editor instance, no toolbar`);
  else fail("F40-inert", "RichTextRenderer", fw, "blocking", "inspect the rich-text-renderer route", "zero editing mechanics", JSON.stringify(inert));

  // ---- faithful semantics for the standard document ----
  const sem = await page.evaluate(() => {
    const root = document.querySelector("[data-part='standard-renderer']");
    const q = (sel: string) => root?.querySelector(sel) ?? null;
    return {
      strong: q("strong")?.textContent ?? null,
      em: q("em")?.textContent ?? null,
      link: q("a")?.getAttribute("href") ?? null,
      linkText: q("a")?.textContent ?? null,
      list: q("ul li") !== null,
      quote: q("blockquote") !== null,
      code: q("pre code")?.textContent ?? null,
      rule: q("hr") !== null,
      tableTh: q("table th")?.textContent ?? null,
      tableTd: q("table td")?.textContent ?? null,
      tableBox: (() => { const t = q("table"); if (!t) return null; const r = t.getBoundingClientRect(); return `${Math.round(r.width)}x${Math.round(r.height)}`; })(),
      heading: q("h1")?.textContent ?? null,
    };
  });
  const semOk =
    sem.strong === "formatted" &&
    sem.em === "italic" &&
    sem.link === "https://example.com" &&
    sem.linkText === "link" &&
    sem.list &&
    sem.quote &&
    (sem.code ?? "").includes("const shipped = true;") &&
    sem.rule &&
    sem.tableTh === "Surface" &&
    sem.tableTd === "Web editor" &&
    sem.tableBox !== null &&
    !sem.tableBox.startsWith("0x") &&
    sem.heading === "Release notes";
  if (semOk) ok(`${fw}: standard document renders faithful semantic structure (headings, marks, link, list, quote, code, rule, table)`);
  else fail("F41-semantics", "RichTextRenderer", fw, "blocking", "inspect the standard renderer group", "faithful semantic output for every admitted feature", JSON.stringify(sem));

  // ---- heading levels ----
  const headingLevels = await page.evaluate(() => {
    const root = document.querySelector("[data-part='heading-renderer']");
    return ["h1","h2","h3","h4","h5","h6"].map((tag) => ({
      tag,
      present: !!root?.querySelector(tag),
      text: root?.querySelector(tag)?.textContent ?? null,
    }));
  });
  if (headingLevels.every((h) => h.present)) ok(`${fw}: renderer projects real H1–H6 document structure`);
  else fail("F42-headings", "RichTextRenderer", fw, "blocking", "inspect the heading-levels renderer group", "h1..h6 all present", JSON.stringify(headingLevels));

  // ---- image ----
  const image = await page.evaluate(() => {
    const img = document.querySelector("[data-part='image-renderer'] img");
    const rect = img?.getBoundingClientRect();
    return {
      count: document.querySelectorAll("[data-part='image-renderer'] img").length,
      srcIsData: img?.getAttribute("src")?.startsWith("data:image/") ?? false,
      natural: img ? `${img.naturalWidth}x${img.naturalHeight}` : null,
      box: rect ? `${Math.round(rect.width)}x${Math.round(rect.height)}` : null,
      alt: img?.getAttribute("alt") ?? null,
    };
  });
  if (image.count === 1 && image.srcIsData && image.natural === "96x48" && image.box && !image.box.startsWith("0x") && image.alt === "Revenue chart")
    ok(`${fw}: renderer projects the seeded image with geometry and alt text`);
  else fail("F43-image", "RichTextRenderer", fw, "blocking", "inspect the images-on renderer group", "one decoded data-URL image with alt", JSON.stringify(image));

  // ---- constrained layout ----
  await page.setViewportSize({ width: 900, height: 700 });
  await settle(page, 400);
  const overflow = await page.evaluate(
    () => document.documentElement.scrollWidth - document.documentElement.clientWidth,
  );
  if (overflow <= 1) ok(`${fw}: no page-level horizontal overflow at 900px`);
  else fail("F44-layout", "RichTextRenderer", fw, "blocking", "view the rich-text-renderer at 900px", "no horizontal page overflow", `${overflow}px overflow`);
  await shoot(page, `${fw}-rtr-constrained-900`, outDir);
  await context.close();
}

/* ================= MarkdownRenderer ================= */

async function runMarkdownRenderer(
  browser: Browser,
  framework: "svelte" | "react",
  base: string,
  outDir: string,
): Promise<void> {
  const context = await browser.newContext({ viewport: { width: 1280, height: 900 } });
  const page = await context.newPage();
  const url = specimenUrl(base, framework, "markdown-renderer");
  await page.goto(url, { waitUntil: "load", timeout: 60_000 });
  await page.waitForSelector(".poodle-md-renderer", { timeout: 30_000 });
  await settle(page, 500);
  console.log(`\n=== ${framework} MarkdownRenderer (${url}) ===`);
  const fw = framework;

  // ---- no editor mechanics anywhere on the route ----
  const mechanics = await page.evaluate(() => ({
    textareas: document.querySelectorAll("textarea").length,
    editable: document.querySelectorAll("[contenteditable='true']").length,
    toolbars: document.querySelectorAll(".poodle-md-editor__toolbar, [data-md-toolbar]").length,
  }));
  if (mechanics.textareas === 0 && mechanics.editable === 0 && mechanics.toolbars === 0)
    ok(`${fw}: route carries no textarea, no contenteditable, no markdown-editor toolbar`);
  else fail("F50-inert", "MarkdownRenderer", fw, "blocking", "inspect the markdown-renderer route", "no editor mechanics", JSON.stringify(mechanics));

  // ---- safe document semantics ----
  const safe = await page.evaluate(() => {
    const all = [...document.querySelectorAll(".poodle-specimen-group")];
    const g = all.find(
      (el) =>
        !el.querySelector(".poodle-specimen-group") && (el.textContent ?? "").includes("Safe document"),
    );
    const root = g?.querySelector(".poodle-md-renderer__content");
    if (!root) return null;
    return {
      h1: root.querySelector("h1")?.textContent ?? null,
      h2: root.querySelector("h2")?.textContent ?? null,
      strong: root.querySelector("strong")?.textContent ?? null,
      em: root.querySelector("em")?.textContent ?? null,
      code: root.querySelector("code")?.textContent ?? null,
      listItems: root.querySelectorAll("ul li").length,
      quote: !!root.querySelector("blockquote"),
      link: root.querySelector("a")?.getAttribute("href") ?? null,
    };
  });
  if (safe && safe.h1 === "Release notes" && safe.h2 === "Highlights" && safe.strong === "bold" && safe.em === "italic" && safe.listItems >= 3 && safe.quote && safe.link)
    ok(`${fw}: safe markdown renders full prose semantics (h1/h2, marks, list, quote, link ${JSON.stringify(safe.link)})`);
  else fail("F51-safe", "MarkdownRenderer", fw, "blocking", "inspect the safe-document group", "complete prose semantics", JSON.stringify(safe));

  // ---- trusted policy ----
  const trusted = await page.evaluate(() => {
    const warning = document.querySelector("[data-policy-label='trusted']");
    const div = document.querySelector("[data-trusted-document]");
    return {
      warningPresent: !!warning,
      divPresent: !!div,
      mark: div?.querySelector("mark")?.textContent ?? null,
      trustedH2: div?.querySelector("h2")?.textContent ?? null,
    };
  });
  if (trusted.warningPresent && trusted.divPresent && trusted.mark === "markup" && trusted.trustedH2 === "Trusted HTML")
    ok(`${fw}: explicit trusted policy renders raw HTML under a visible warning label`);
  else fail("F52-trusted", "MarkdownRenderer", fw, "blocking", "inspect the trusted group", "warning label + unsanitized div/mark/h2", JSON.stringify(trusted));

  // ---- custom parser output stays on the safe path ----
  const custom = await page.evaluate(() => {
    const all = [...document.querySelectorAll(".poodle-specimen-group")];
    const g = all.find(
      (el) =>
        !el.querySelector(".poodle-specimen-group") && (el.textContent ?? "").includes("Custom parser"),
    );
    const root = g?.querySelector(".poodle-md-renderer__content");
    return {
      h2: root?.querySelector("h2")?.textContent ?? null,
      strong: root?.querySelector("strong")?.textContent ?? null,
    };
  });
  if (custom && custom.h2 === "Custom parser section" && custom.strong === "renderHtml")
    ok(`${fw}: custom renderHtml output renders through the same safe prose path`);
  else fail("F53-custom", "MarkdownRenderer", fw, "blocking", "inspect the custom-parser group", "custom h2 + strong rendered", JSON.stringify(custom));

  // ---- empty document ----
  const empty = await page.evaluate(() => {
    const all = [...document.querySelectorAll(".poodle-specimen-group")];
    const g = all.find(
      (el) =>
        !el.querySelector(".poodle-specimen-group") && (el.textContent ?? "").includes("Empty document"),
    );
    const root = g?.querySelector(".poodle-md-renderer__content");
    return {
      rootPresent: !!root,
      text: (root?.textContent ?? "").trim(),
      editorEmptyCopy: (g?.textContent ?? "").includes("Nothing to preview"),
    };
  });
  if (empty.rootPresent && empty.text === "" && !empty.editorEmptyCopy)
    ok(`${fw}: empty value renders an empty root with no editor placeholder copy`);
  else fail("F54-empty", "MarkdownRenderer", fw, "blocking", "inspect the empty-document group", "empty root, no 'Nothing to preview'", JSON.stringify(empty));

  // ---- constrained long-token wrap ----
  const constrained = await page.evaluate(() => {
    const all = [...document.querySelectorAll(".poodle-specimen-group")];
    const g = all.find(
      (el) =>
        !el.querySelector(".poodle-specimen-group") && (el.textContent ?? "").includes("Constrained width"),
    );
    const container = g?.querySelector("div[style] > .poodle-md-renderer") ?? g?.querySelector(".poodle-md-renderer");
    const content = g?.querySelector(".poodle-md-renderer__content");
    const cw = (container ?? content)?.getBoundingClientRect().width ?? 0;
    const contentW = content?.scrollWidth ?? 0;
    return {
      containerWidth: Math.round(cw),
      contentScrollWidth: contentW,
      fits: contentW <= Math.ceil(cw) + 2,
    };
  });
  if (constrained.fits && constrained.containerWidth > 0)
    ok(`${fw}: long unbroken token wraps inside the constrained container (${constrained.contentScrollWidth}px content in ${constrained.containerWidth}px)`);
  else fail("F55-wrap", "MarkdownRenderer", fw, "blocking", "view the constrained-width group", "content never spills its container", JSON.stringify(constrained));

  // ---- page layout at Desktop-like width ----
  await page.setViewportSize({ width: 900, height: 700 });
  await settle(page, 400);
  const overflow = await page.evaluate(
    () => document.documentElement.scrollWidth - document.documentElement.clientWidth,
  );
  if (overflow <= 1) ok(`${fw}: no page-level horizontal overflow at 900px`);
  else fail("F56-layout", "MarkdownRenderer", fw, "blocking", "view the markdown-renderer at 900px", "no horizontal page overflow", `${overflow}px overflow`);
  await shoot(page, `${fw}-mdr-constrained-900`, outDir);

  // ---- themes on a typography surface ----
  await page.setViewportSize({ width: 1280, height: 900 });
  const bg: Record<string, string> = {};
  for (const theme of ["eclipse", "clay"]) {
    await page.goto(specimenUrl(base, framework, "markdown-renderer", theme), { waitUntil: "load" });
    await page.waitForSelector(".poodle-md-renderer__content", { timeout: 30_000 });
    await settle(page, 500);
    bg[theme] = await page.evaluate(() => {
      const el = document.querySelector(".poodle-md-renderer__content h1");
      return el ? getComputedStyle(el).color : "missing";
    });
    await shoot(page, `${fw}-mdr-theme-${theme}`, outDir);
  }
  if (Object.values(bg)[0] !== Object.values(bg)[1] && !Object.values(bg).includes("missing"))
    ok(`${fw}: renderer typography follows themes (${JSON.stringify(bg)})`);
  else fail("F57-theme", "MarkdownRenderer", fw, "follow-up", "load under eclipse and clay", "heading color varies", JSON.stringify(bg));

  await context.close();
}

/** The renderer routes must honor the preview's density configuration (card work item 5). */
async function densityJourney(browser: Browser, framework: "svelte" | "react", base: string, outDir: string): Promise<void> {
  const context = await browser.newContext({ viewport: { width: 1280, height: 900 } });
  const page = await context.newPage();
  const seen: Array<{ route: string; density: string; shellAttr: string | null; panelX: string; mounted: boolean }> = [];
  for (const slug of ["rich-text-renderer", "markdown-renderer"]) {
    for (const density of ["comfortable", "compact"]) {
      await page.goto(specimenUrl(base, framework, slug, "eclipse", density), { waitUntil: "load" });
      const selector = slug === "markdown-renderer" ? ".poodle-md-renderer" : "[data-part='standard-renderer']";
      await page.waitForSelector(selector, { timeout: 30_000 });
      await settle(page, 500);
      seen.push(
        await page.evaluate(
          ([slug, density]) => {
            const shell = document.querySelector(".poodle-app-shell") ?? document.documentElement;
            const cs = shell ? getComputedStyle(shell) : null;
            return {
              route: slug as string,
              density: density as string,
              shellAttr: shell?.getAttribute("data-density") ?? null,
              panelX: cs?.getPropertyValue("--poodle-space-panel-x").trim() ?? "",
              mounted: true,
            };
          },
          [slug, density] as [string, string],
        ),
      );
    }
  }
  await page.goto(specimenUrl(base, framework, "markdown-renderer", "eclipse", "compact"), { waitUntil: "load" });
  await page.waitForSelector(".poodle-md-renderer", { timeout: 30_000 });
  await settle(page, 400);
  await shoot(page, `${framework}-renderers-density-compact`, outDir);
  const allDensityApplied = seen.every((s) => s.shellAttr === s.density);
  const compact = seen.find((s) => s.density === "compact");
  const comfortable = seen.find((s) => s.density === "comfortable");
  if (allDensityApplied && compact && comfortable && compact.panelX !== comfortable.panelX && seen.every((s) => s.mounted))
    ok(`${framework}: renderer routes honor density (panel-x ${comfortable.panelX} comfortable vs ${compact.panelX} compact) with content mounted`);
  else
    fail("F58-density", "renderers", framework, "follow-up", "load the renderer routes under comfortable and compact densities", "density attribute and spacing change while content stays mounted", JSON.stringify(seen));
  await context.close();
}

/* ================= main ================= */

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
const BASES: Record<"svelte" | "react", string> = {
  svelte: servers.urls.svelte,
  react: servers.urls.react,
};

try {
  for (const [engineName, engine] of ENGINES) {
    const outDir = `${OUT_BASE}/${engineName}`;
    mkdirSync(outDir, { recursive: true });
    const passesBefore = passes;
    const findingsBefore = findings.length;
    const browser = await engine.launch();
    try {
      for (const fw of ["svelte", "react"] as const) {
        await runRichTextEditor(browser, fw, BASES[fw], outDir);
        await runRichTextRenderer(browser, fw, BASES[fw], outDir);
        await runMarkdownRenderer(browser, fw, BASES[fw], outDir);
        await densityJourney(browser, fw, BASES[fw], outDir);
      }
    } finally {
      await browser.close();
    }
    const engineFindings = findings.slice(findingsBefore);
    const engineBlocking = engineFindings.filter((f) => f.severity === "blocking").length;
    writeFileSync(
      `${outDir}/editors-report.json`,
      JSON.stringify(
        { browser: engineName, passes: passes - passesBefore, findings: engineFindings, blockingCount: engineBlocking },
        null,
        2,
      ),
    );
    console.log(`\n=== editors sweep summary (${engineName}) ===`);
    console.log(`passes: ${passes - passesBefore}`);
    console.log(`findings: ${engineFindings.length} (blocking: ${engineBlocking})`);
    for (const f of engineFindings) {
      console.log(`\n[${f.severity.toUpperCase()}] ${f.surface}/${f.id} — ${f.framework}`);
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
console.log(`\nall editor UX sweep checks passed (${ENGINES.map(([n]) => n).join(", ")})`);
