/**
 * Headless Chromium + WebKit proof of the g18.012 CodeEditor language
 * registry and the g18.021 token-bound syntax presentation. jsdom covers the
 * semantic suite; this probe proves the parts only a real browser can: real
 * dynamic ESM grammar loading, controlled switching without remount,
 * focus/undo survival across switches, fail-closed rejected-load behavior in
 * a live page, real mount-time unknown-id refusal through each framework's
 * boundary mechanism, and — for g18.021 — visible token-bound syntax with
 * distinct computed Poodle colours for TypeScript and JSON, live theme
 * response, danger treatment for parser errors, and zero spans in plain
 * mode.
 *
 *   bun test/code-editor-language-registry/probe.ts --browser=chromium
 *   bun test/code-editor-language-registry/probe.ts --browser=webkit
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
const port = 4199;
const url = `http://127.0.0.1:${port}/`;

const DOC = `// ledger
const answer = 42;
const label = "hello";
`;
const JSON_DOC = `{\n  "answer": 42,\n  "label": "hello",\n  "live": true\n}\n`;
const INVALID_DOC = `// ledger
const answer = 42;
### oops ###
const broken = ;
`;

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
  throw new Error(`code-editor language-registry fixture on :${port} did not start`);
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
        requestAnimationFrame(() => {
          window.setTimeout(resolve, 200);
        });
      }),
  );
}

type Counters = { typescript: number; json: number; broken: number };

/**
 * g18.021: computed reference colours for the semantic tokens the private
 * highlight style consumes. Captured live so theme switches are compared
 * against the variables the mounted editor actually resolves.
 */
const TOKEN_VARIABLES = [
  "text-secondary",
  "accent-base",
  "status-success",
  "status-info",
  "status-warning",
  "status-danger",
] as const;
type TokenColors = Record<(typeof TOKEN_VARIABLES)[number], string>;

async function tokenColors(page: Page): Promise<TokenColors> {
  return page.evaluate((names) => {
    const probe = document.createElement("span");
    probe.setAttribute(
      "style",
      "position:absolute;visibility:hidden;pointer-events:none",
    );
    document.body.appendChild(probe);
    const out: Partial<Record<(typeof names)[number], string>> = {};
    for (const name of names) {
      probe.style.color = `var(--poodle-color-${name})`;
      out[name] = getComputedStyle(probe).color;
    }
    probe.remove();
    return out as TokenColors;
  }, [...TOKEN_VARIABLES]);
}

type StyledSpan = { text: string; color: string; invalid: boolean };

/** Every span inside the editor content that carries non-default colour. */
async function styledSpans(page: Page, framework: string): Promise<StyledSpan[]> {
  return page.evaluate((fw) => {
    const frame = document.querySelector(`[data-framework="${fw}"] [data-part='main-editor']`);
    const content = frame?.querySelector(".cm-content");
    if (!(content instanceof HTMLElement)) throw new Error(`missing ${fw} editor content`);
    const base = getComputedStyle(content).color;
    return [...content.querySelectorAll("span")].flatMap((span) => {
      const color = getComputedStyle(span).color;
      const invalid = span.classList.contains("poodle-code-editor__syntax-invalid");
      if (color === base && !invalid) return [];
      return [{ text: span.textContent ?? "", color, invalid }];
    });
  }, framework);
}

async function clickPart(page: Page, framework: string, part: string): Promise<void> {
  await page.evaluate(
    ([fw, id]) => {
      const button = document.querySelector(`[data-framework="${fw}"] [data-part='${id}']`);
      if (!(button instanceof HTMLElement)) throw new Error(`missing ${fw} ${id} button`);
      button.dispatchEvent(new MouseEvent("click", { bubbles: true }));
    },
    [framework, part],
  );
}

async function setTheme(page: Page, theme: string): Promise<void> {
  await page.evaluate((name) => {
    document.documentElement.dataset.theme = name;
  }, theme);
}

async function counters(page: Page, framework: string): Promise<Counters> {
  return page.evaluate((fw) => {
    const all = (window as unknown as { __registryCounters: Record<string, Counters> }).__registryCounters;
    return { ...all[fw] };
  }, framework);
}

async function selectLanguage(page: Page, framework: string, language: string): Promise<void> {
  await clickPart(page, framework, `language-${language}`);
}

async function readEditor(page: Page, framework: string): Promise<{
  doc: string;
  marked: boolean;
  focused: boolean;
}> {
  return page.evaluate((fw) => {
    const section = document.querySelector(`[data-framework="${fw}"]`);
    if (!(section instanceof HTMLElement)) throw new Error(`missing ${fw} section`);
    const frame = section.querySelector("[data-part='main-editor']");
    const editor = frame?.querySelector(".cm-editor");
    const content = frame?.querySelector(".cm-content");
    return {
      doc: content
        ? [...content.querySelectorAll(".cm-line")].map((line) => line.textContent ?? "").join("\n")
        : "",
      marked: editor != null && editor.hasAttribute("data-probe-id"),
      focused: content !== null && document.activeElement === content,
    };
  }, framework);
}

async function markEditor(page: Page, framework: string): Promise<void> {
  await page.evaluate((fw) => {
    const editor = document.querySelector(
      `[data-framework="${fw}"] [data-part='main-editor'] .cm-editor`,
    );
    if (!(editor instanceof HTMLElement)) throw new Error(`missing ${fw} editor`);
    editor.setAttribute("data-probe-id", "mounted");
  }, framework);
}

async function typeText(page: Page, framework: string, text: string): Promise<void> {
  await page.locator(`[data-framework="${framework}"] [data-part='main-editor'] .cm-content`).click();
  // The trailing newline leaves the caret on the empty last line; move it to
  // the end of line 1 deterministically.
  await page.keyboard.press("ArrowUp");
  await page.keyboard.press("End");
  await page.keyboard.type(text);
  await settle(page);
}

/** Undo until the document is exactly `doc`, bounded for flake safety. */
async function undoUntil(page: Page, framework: string, doc: string): Promise<boolean> {
  for (let steps = 0; steps < 20; steps += 1) {
    const current = await readEditor(page, framework);
    if (current.doc === doc) return true;
    await page.keyboard.press("ControlOrMeta+z");
    await settle(page);
  }
  return (await readEditor(page, framework)).doc === doc;
}

async function runFramework(page: Page, framework: string, browserName: string): Promise<void> {
  const section = `[data-framework="${framework}"]`;
  await page.locator(`${section} [data-part='main-editor'] .cm-content`).waitFor();

  // Collect unhandled rejections so a rejected load is observable in-page.
  await page.evaluate(() => {
    const w = window as unknown as { __rejections: string[] };
    w.__rejections = [];
    window.addEventListener("unhandledrejection", (event) => {
      w.__rejections.push(
        event.reason instanceof Error ? event.reason.message : String(event.reason),
      );
    });
  });

  await markEditor(page, framework);
  const initial = await readEditor(page, framework);
  const initialCounters = await counters(page, framework);
  check(
    `${browserName} ${framework} initial mount loads exactly the selected language`,
    initial.doc === DOC && initial.marked && initialCounters.typescript === 1 && initialCounters.json === 0 && initialCounters.broken === 0,
    `doc=${JSON.stringify(initial.doc)} counters=${JSON.stringify(initialCounters)}`,
  );

  // 0b. g18.021: token-bound syntax presentation is visible for the real
  //     TypeScript grammar. Representative spans carry the exact Poodle
  //     semantic colours resolved from the live theme variables.
  const eclipseRefs = await tokenColors(page);
  const tsSpans = await styledSpans(page, framework);
  const tsExpectations: Array<[string, keyof TokenColors]> = [
    ["ledger", "text-secondary"],
    ["const", "accent-base"],
    ["answer", "status-warning"],
    ["42", "status-info"],
    ['"hello"', "status-success"],
  ];
  for (const [needle, token] of tsExpectations) {
    const span = tsSpans.find((candidate) => candidate.text.includes(needle));
    check(
      `${browserName} ${framework} typescript "${needle}" span uses --poodle-color-${token}`,
      Boolean(span) && span?.color === eclipseRefs[token],
      span ? `color=${span.color} expected=${eclipseRefs[token]}` : "no styled span",
    );
  }
  check(
    `${browserName} ${framework} full typescript renders syntax spans`,
    tsSpans.length >= tsExpectations.length,
    JSON.stringify(tsSpans),
  );

  // 1. Controlled switch to the second consumer language: one additional lazy
  //    load, same editor instance, document preserved.
  await selectLanguage(page, framework, "json");
  await settle(page);
  const jsonState = await readEditor(page, framework);
  const jsonCounters = await counters(page, framework);
  check(
    `${browserName} ${framework} switching loads the second language once on the same editor`,
    jsonState.doc === DOC && jsonState.marked && jsonCounters.typescript === 1 && jsonCounters.json === 1,
    `counters=${JSON.stringify(jsonCounters)} marked=${jsonState.marked}`,
  );

  // 1b. g18.021: switching language keeps the document — the JSON sample
  //     token proof and the parser-error proof use the dedicated samples
  //     below.
  const switchSpans = await styledSpans(page, framework);
  check(
    `${browserName} ${framework} the json reconfiguration carries the private presentation`,
    switchSpans.length > 0,
    JSON.stringify(switchSpans.slice(0, 5)),
  );

  // 2. Switching back reuses the memoized load: no new loader invocation.
  await selectLanguage(page, framework, "typescript");
  await settle(page);
  const backState = await readEditor(page, framework);
  const backCounters = await counters(page, framework);
  check(
    `${browserName} ${framework} re-selection reuses the memoized load`,
    backState.doc === DOC && backState.marked && backCounters.typescript === 1 && backCounters.json === 1,
    `counters=${JSON.stringify(backCounters)}`,
  );

  // 2b. g18.021: a real JSON sample tokenizes with its own semantic palette.
  //     Property names stay ordinary primary text; strings read as success and
  //     numbers/booleans as info.
  await clickPart(page, framework, "sample-json");
  await settle(page);
  const jsonSampleSpans = await styledSpans(page, framework);
  const jsonColors = new Set(jsonSampleSpans.map((span) => span.color));
  const jsonNumber = jsonSampleSpans.find((span) => span.text.includes("42"));
  const jsonString = jsonSampleSpans.find((span) => span.text.includes('"hello"'));
  check(
    `${browserName} ${framework} json number span uses --poodle-color-status-info`,
    Boolean(jsonNumber) && jsonNumber?.color === eclipseRefs["status-info"],
    jsonNumber ? `color=${jsonNumber.color} expected=${eclipseRefs["status-info"]}` : "no styled span",
  );
  check(
    `${browserName} ${framework} json string span uses --poodle-color-status-success`,
    Boolean(jsonString) && jsonString?.color === eclipseRefs["status-success"],
    jsonString ? `color=${jsonString.color} expected=${eclipseRefs["status-success"]}` : "no styled span",
  );
  check(
    `${browserName} ${framework} json renders at least two distinct token colours`,
    jsonColors.size >= 2,
    JSON.stringify([...jsonColors]),
  );
  check(
    `${browserName} ${framework} json property names stay ordinary text`,
    !jsonSampleSpans.some((span) => span.text.includes("answer") && !span.invalid),
    JSON.stringify(jsonSampleSpans.filter((span) => span.text.includes("answer"))),
  );
  await clickPart(page, framework, "sample-typescript");
  await settle(page);

  // 3. Plain text is built in: no registry entry, no loader, same editor.
  await selectLanguage(page, framework, "plain-text");
  await settle(page);
  const plainState = await readEditor(page, framework);
  const plainCounters = await counters(page, framework);
  check(
    `${browserName} ${framework} plain-text switches without consulting the registry`,
    plainState.doc === DOC && plainState.marked && plainCounters.typescript === 1 && plainCounters.json === 1 && plainCounters.broken === 0,
    `counters=${JSON.stringify(plainCounters)}`,
  );
  const plainLanguageSpans = await styledSpans(page, framework);
  check(
    `${browserName} ${framework} plain-text renders zero syntax spans`,
    plainLanguageSpans.length === 0,
    JSON.stringify(plainLanguageSpans),
  );

  // 4. Editing survives switching: typed text persists and undo history works
  //    across a plain-text -> typescript -> json round trip.
  await typeText(page, framework, "\nconst live = true;");
  const typed = await readEditor(page, framework);
  check(
    `${browserName} ${framework} typing lands in the plain-text editor`,
    typed.doc === `${DOC}const live = true;\n`,
    `doc=${JSON.stringify(typed.doc)}`,
  );
  await selectLanguage(page, framework, "typescript");
  await settle(page);
  await selectLanguage(page, framework, "json");
  await settle(page);
  const afterSwitch = await readEditor(page, framework);
  check(
    `${browserName} ${framework} typed text survives two language switches`,
    afterSwitch.doc === `${DOC}const live = true;\n` && afterSwitch.marked,
    `doc=${JSON.stringify(afterSwitch.doc)}`,
  );
  const undoReached = await undoUntil(page, framework, DOC);
  const afterUndo = await readEditor(page, framework);
  check(
    `${browserName} ${framework} undo history survives language switching`,
    undoReached && afterUndo.doc === DOC && afterUndo.marked,
    `doc=${JSON.stringify(afterUndo.doc)}`,
  );

  // 5. A rejected load fails closed: the rejection surfaces, the previous
  //    language stays active, and nothing remounts.
  const beforeBroken = await counters(page, framework);
  await selectLanguage(page, framework, "broken");
  await settle(page);
  await settle(page);
  const brokenState = await readEditor(page, framework);
  const brokenCounters = await counters(page, framework);
  const rejections = await page.evaluate(() => {
    const w = window as unknown as { __rejections: string[] };
    return [...w.__rejections];
  });
  check(
    `${browserName} ${framework} the rejected load surfaces as an unhandled rejection`,
    rejections.some((message) => message.includes("grammar exploded")),
    `rejections=${JSON.stringify(rejections)}`,
  );
  check(
    `${browserName} ${framework} the failed switch loads only the broken loader and keeps the editor`,
    brokenCounters.broken === beforeBroken.broken + 1 && brokenState.marked && brokenState.doc === DOC,
    `counters=${JSON.stringify(brokenCounters)} marked=${brokenState.marked}`,
  );

  // 6. Recovery: reselecting an admitted id reconfigures through the memoized
  //    registry without remounting.
  await selectLanguage(page, framework, "typescript");
  await settle(page);
  const recovered = await readEditor(page, framework);
  const recoveredCounters = await counters(page, framework);
  check(
    `${browserName} ${framework} the editor recovers through the memoized registry after a failed load`,
    recovered.marked && recoveredCounters.typescript === 1 && recoveredCounters.broken === beforeBroken.broken + 1,
    `counters=${JSON.stringify(recoveredCounters)}`,
  );

  // 7. A mounted unknown id is refused, never silently downgraded to plain
  //    text: the boundary catches the real mount error and no editor mounts.
  await page.evaluate((fw) => {
    const button = document.querySelector(
      `[data-framework="${fw}"] [data-part='mount-invalid']`,
    );
    if (!(button instanceof HTMLElement)) throw new Error(`missing ${fw} mount-invalid button`);
    button.dispatchEvent(new MouseEvent("click", { bubbles: true }));
  }, framework);
  await settle(page);
  const refusal = await page.evaluate((fw) => {
    const scope = document.querySelector(`[data-framework="${fw}"]`);
    if (!(scope instanceof HTMLElement)) throw new Error(`missing ${fw} section`);
    return {
      message: scope.querySelector("[data-part='mount-refusal']")?.textContent ?? "",
      mounted: scope.querySelectorAll("[data-part='refusal-editor'] .cm-editor").length,
    };
  }, framework);
  check(
    `${browserName} ${framework} an unknown id is refused at mount with the exact failure`,
    /unsupported language "cobol"/.test(refusal.message),
    `message=${JSON.stringify(refusal.message)}`,
  );
  check(
    `${browserName} ${framework} the refused mount never presents an editor`,
    refusal.mounted === 0,
    `mounted=${refusal.mounted}`,
  );
  const stillThere = await readEditor(page, framework);
  check(
    `${browserName} ${framework} the refused mount leaves the working editor untouched`,
    stillThere.marked && stillThere.doc === DOC,
    `marked=${stillThere.marked}`,
  );

  // 8. g18.021: full -> plain performance mode drops token presentation on the
  //    same mounted editor without consulting the registry; full restores it.
  await clickPart(page, framework, "mode-plain");
  await settle(page);
  const plainModeState = await readEditor(page, framework);
  const plainModeCounters = await counters(page, framework);
  const plainModeSpans = await styledSpans(page, framework);
  check(
    `${browserName} ${framework} plain performance mode drops syntax spans on the same editor`,
    plainModeState.doc === DOC && plainModeState.marked && plainModeSpans.length === 0,
    `doc=${JSON.stringify(plainModeState.doc)} spans=${JSON.stringify(plainModeSpans)} marked=${plainModeState.marked}`,
  );
  check(
    `${browserName} ${framework} plain performance mode loads no grammar`,
    plainModeCounters.typescript === 1 && plainModeCounters.json === 1,
    `counters=${JSON.stringify(plainModeCounters)}`,
  );
  await clickPart(page, framework, "mode-full");
  await settle(page);
  const restoredModeState = await readEditor(page, framework);
  const restoredModeSpans = await styledSpans(page, framework);
  const restoredKeyword = restoredModeSpans.find((span) => span.text.includes("const"));
  check(
    `${browserName} ${framework} returning to full mode reinstates token presentation without remounting`,
    restoredModeState.doc === DOC && restoredModeState.marked &&
      Boolean(restoredKeyword) && restoredKeyword?.color === eclipseRefs["accent-base"],
    `color=${restoredKeyword?.color} expected=${eclipseRefs["accent-base"]} marked=${restoredModeState.marked}`,
  );

  // 9. g18.021: the malformed TypeScript sample plants parser error nodes.
  await clickPart(page, framework, "sample-invalid");
  await settle(page);
  const invalidState = await readEditor(page, framework);
  const invalidRefs = await tokenColors(page);
  const invalidSpans = await styledSpans(page, framework);
  const invalidMarks = invalidSpans.filter((span) => span.invalid);
  check(
    `${browserName} ${framework} parser error nodes receive the danger treatment`,
    invalidState.marked && invalidMarks.length > 0 &&
      invalidMarks.every((span) => span.color === invalidRefs["status-danger"]),
    JSON.stringify(invalidMarks.slice(0, 5)),
  );
  await clickPart(page, framework, "sample-typescript");
  await settle(page);
  const cleanedSpans = await styledSpans(page, framework);
  check(
    `${browserName} ${framework} repairing the document removes the danger marks`,
    cleanedSpans.every((span) => !span.invalid),
    JSON.stringify(cleanedSpans.filter((span) => span.invalid)),
  );

  // 10. g18.021: a live theme switch restyles the same mounted editor through
  //     the CSS-variable rules, with no remount.
  await page.evaluate((fw) => {
    const editor = document.querySelector(
      `[data-framework="${fw}"] [data-part='main-editor'] .cm-editor`,
    );
    if (!(editor instanceof Element)) throw new Error(`missing ${fw} editor`);
    (editor as unknown as { __probeMounted?: boolean }).__probeMounted = true;
  }, framework);
  await setTheme(page, "iceberg");
  await settle(page);
  const icebergRefs = await tokenColors(page);
  const icebergSpans = await styledSpans(page, framework);
  const icebergKeyword = icebergSpans.find((span) => span.text.includes("const"));
  const icebergComment = icebergSpans.find((span) => span.text.includes("ledger"));
  const identityKept = await page.evaluate((fw) => {
    const editor = document.querySelector(
      `[data-framework="${fw}"] [data-part='main-editor'] .cm-editor`,
    );
    return (
      editor !== null &&
      (editor as unknown as { __probeMounted?: boolean }).__probeMounted === true &&
      editor.hasAttribute("data-probe-id")
    );
  }, framework);
  check(
    `${browserName} ${framework} iceberg theme restyles keyword tokens without remounting`,
    identityKept && Boolean(icebergKeyword) &&
      icebergKeyword?.color === icebergRefs["accent-base"] &&
      icebergKeyword?.color !== eclipseRefs["accent-base"],
    `color=${icebergKeyword?.color} expected=${icebergRefs["accent-base"]} was=${eclipseRefs["accent-base"]} identity=${identityKept}`,
  );
  check(
    `${browserName} ${framework} iceberg theme restyles comment tokens without remounting`,
    Boolean(icebergComment) && icebergComment?.color === icebergRefs["text-secondary"] &&
      icebergComment?.color !== eclipseRefs["text-secondary"],
    `color=${icebergComment?.color} expected=${icebergRefs["text-secondary"]}`,
  );
  await setTheme(page, "eclipse");
  await settle(page);
  const eclipseAgain = await styledSpans(page, framework);
  const eclipseKeyword = eclipseAgain.find((span) => span.text.includes("const"));
  check(
    `${browserName} ${framework} returning to eclipse restores the original token colours`,
    Boolean(eclipseKeyword) && eclipseKeyword?.color === eclipseRefs["accent-base"],
    `color=${eclipseKeyword?.color} expected=${eclipseRefs["accent-base"]}`,
  );
}

try {
  for (const [browserName, browserType] of engines) {
    let browser: Browser | undefined;
    try {
      browser = await browserType.launch({ headless: true });
      const page = await browser.newPage({ viewport: { width: 960, height: 720 } });
      await page.goto(url, { waitUntil: "networkidle" });
      await page.waitForSelector('[data-framework="svelte"] [data-part="main-editor"] .poodle-code-editor');
      await page.waitForSelector('[data-framework="react"] [data-part="main-editor"] .poodle-code-editor');
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

console.log("\nall CodeEditor language-registry checks passed");
