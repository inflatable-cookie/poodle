/**
 * Headless Chromium + WebKit proof of the g18.012 CodeEditor language
 * registry. jsdom covers the semantic suite; this probe proves the parts only
 * a real browser can: real dynamic ESM grammar loading, controlled switching
 * without remount, focus/undo survival across switches, fail-closed
 * rejected-load behavior in a live page, and real mount-time unknown-id
 * refusal through each framework's boundary mechanism.
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

const DOC = "const answer = 42;\n";

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

async function counters(page: Page, framework: string): Promise<Counters> {
  return page.evaluate((fw) => {
    const all = (window as unknown as { __registryCounters: Record<string, Counters> }).__registryCounters;
    return { ...all[fw] };
  }, framework);
}

async function selectLanguage(page: Page, framework: string, language: string): Promise<void> {
  await page.evaluate(
    ([fw, id]) => {
      const button = document.querySelector(
        `[data-framework="${fw}"] [data-part='language-${id}']`,
      );
      if (!(button instanceof HTMLElement)) throw new Error(`missing ${fw} ${id} button`);
      button.dispatchEvent(new MouseEvent("click", { bubbles: true }));
    },
    [framework, language],
  );
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
      marked: editor !== null && editor.hasAttribute("data-probe-id"),
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

  // 4. Editing survives switching: typed text persists and undo history works
  //    across a plain-text -> typescript -> json round trip.
  await typeText(page, framework, "\nconst live = true;");
  const typed = await readEditor(page, framework);
  check(
    `${browserName} ${framework} typing lands in the plain-text editor`,
    typed.doc === "const answer = 42;\nconst live = true;\n",
    `doc=${JSON.stringify(typed.doc)}`,
  );
  await selectLanguage(page, framework, "typescript");
  await settle(page);
  await selectLanguage(page, framework, "json");
  await settle(page);
  const afterSwitch = await readEditor(page, framework);
  check(
    `${browserName} ${framework} typed text survives two language switches`,
    afterSwitch.doc === "const answer = 42;\nconst live = true;\n" && afterSwitch.marked,
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
