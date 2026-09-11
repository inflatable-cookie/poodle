/**
 * Headless Chromium + WebKit proof that CodeEditor's `lineNumbers` prop
 * reconfigures the mounted CodeMirror gutter live. jsdom cannot prove the
 * rendered gutter, real focus, or real history commands.
 *
 *   bun test/code-editor-line-numbers/probe.ts --browser=chromium
 *   bun test/code-editor-line-numbers/probe.ts --browser=webkit
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
const port = 4197;
const url = `http://127.0.0.1:${port}/`;

const INITIAL_DOC = "const answer = 42;\nconst unused = 0;\n";

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
  throw new Error(`code-editor line-numbers fixture on :${port} did not start`);
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

async function readState(page: Page, framework: string): Promise<{
  gutter: boolean;
  pressed: string | null;
  doc: string;
  marked: boolean;
  message: string | null;
  diagnosticMark: boolean;
  focused: boolean;
}> {
  return page.evaluate((fw) => {
    const section = document.querySelector(`[data-framework="${fw}"]`);
    if (!(section instanceof HTMLElement)) throw new Error(`missing ${fw} section`);
    const editor = section.querySelector(".cm-editor");
    const content = section.querySelector(".cm-content");
    const message = section.querySelector(".poodle-code-editor__diagnostic-message");
    return {
      gutter: section.querySelector(".cm-gutters .cm-lineNumbers") !== null,
      pressed: section.querySelector("[data-part='line-numbers-toggle']")?.getAttribute("aria-pressed") ?? null,
      doc: content
        ? [...content.querySelectorAll(".cm-line")].map((line) => line.textContent ?? "").join("\n")
        : "",
      marked: editor !== null && editor.hasAttribute("data-probe-id"),
      message: message?.textContent ?? null,
      diagnosticMark: section.querySelector("[data-poodle-diagnostic='d1']") !== null,
      focused: content !== null && document.activeElement === content,
    };
  }, framework);
}

/** Mark the current editor root so a remount would lose the marker. */
async function markEditor(page: Page, framework: string): Promise<void> {
  await page.evaluate((fw) => {
    const editor = document.querySelector(`[data-framework="${fw}"] .cm-editor`);
    if (!(editor instanceof HTMLElement)) throw new Error(`missing ${fw} editor`);
    editor.setAttribute("data-probe-id", "mounted");
  }, framework);
}

/**
 * Toggle the host line-numbers state without moving focus: a real mouse
 * click focuses the button, which would confound the focus-survival proof.
 * The dispatched click drives the same control the specimen renders.
 */
async function toggleLineNumbers(page: Page, framework: string): Promise<void> {
  await page.evaluate((fw) => {
    const button = document.querySelector(
      `[data-framework="${fw}"] [data-part='line-numbers-toggle']`,
    );
    if (!(button instanceof HTMLElement)) throw new Error(`missing ${fw} toggle`);
    button.dispatchEvent(new MouseEvent("click", { bubbles: true }));
  }, framework);
}

/** Move focus from the leading button onto the editing surface. */
async function focusEditor(page: Page, framework: string): Promise<void> {
  const section = `[data-framework="${framework}"]`;
  await page.locator(`${section} [data-before]`).click();
  await settle(page);
  // Tab order differs by engine: WebKit skips buttons entirely, Chromium
  // visits the line-numbers toggle first. Walk forward until the editing
  // surface holds focus.
  for (let tabs = 0; tabs < 4; tabs += 1) {
    await page.keyboard.press("Tab");
    await settle(page);
    const state = await readState(page, framework);
    if (state.focused) return;
  }
  throw new Error(`[${framework}] Tab never reached the editing surface`);
}

async function pressModChord(page: Page, framework: string, key: string): Promise<void> {
  await page.evaluate(
    ([fw, chordKey]) => {
      const surface = document.querySelector(`[data-framework="${fw}"] .cm-content`);
      if (!(surface instanceof HTMLElement)) throw new Error("missing editing surface");
      surface.dispatchEvent(
        new KeyboardEvent("keydown", {
          bubbles: true,
          cancelable: true,
          key: chordKey,
          ...(navigator.platform.includes("Mac") ? { metaKey: true } : { ctrlKey: true }),
        }),
      );
    },
    [framework, key],
  );
}

async function runFramework(page: Page, framework: string, browserName: string): Promise<void> {
  const section = `[data-framework="${framework}"]`;
  const before = page.locator(`${section} [data-before]`);
  const toggle = page.locator(`${section} [data-part="line-numbers-toggle"]`);
  const content = page.locator(`${section} .cm-content`);
  await content.waitFor();

  await markEditor(page, framework);
  const initial = await readState(page, framework);
  check(
    `${browserName} ${framework} initial mount shows the gutter with the button on`,
    initial.gutter && initial.pressed === "true" && initial.doc === INITIAL_DOC && initial.marked,
    `gutter=${initial.gutter} pressed=${initial.pressed} doc=${JSON.stringify(initial.doc)}`,
  );

  // 1. Live off: the gutter disappears without a new editor instance.
  await toggle.click();
  await settle(page);
  const off = await readState(page, framework);
  check(
    `${browserName} ${framework} live off removes the gutter and flips the button`,
    !off.gutter && off.pressed === "false" && off.doc === INITIAL_DOC && off.marked,
    `gutter=${off.gutter} pressed=${off.pressed} marked=${off.marked}`,
  );

  // 2. Live on: the same view restores the exact logical-line gutter.
  await toggle.click();
  await settle(page);
  const on = await readState(page, framework);
  check(
    `${browserName} ${framework} live on restores the gutter on the same editor`,
    on.gutter && on.pressed === "true" && on.doc === INITIAL_DOC && on.marked,
    `gutter=${on.gutter} pressed=${on.pressed} marked=${on.marked}`,
  );

  // 3. Focus, selection, and undo history survive both transitions.
  await focusEditor(page, framework);
  const entered = await readState(page, framework);
  check(`${browserName} ${framework} Tab lands on the editing surface`, entered.focused);
  await page.keyboard.press("End");
  await toggleLineNumbers(page, framework);
  await settle(page);
  await toggleLineNumbers(page, framework);
  await settle(page);
  const afterToggle = await readState(page, framework);
  check(
    `${browserName} ${framework} focus survives both transitions`,
    afterToggle.focused && afterToggle.gutter && afterToggle.marked,
    `focused=${afterToggle.focused} gutter=${afterToggle.gutter}`,
  );
  await page.keyboard.press("Enter");
  await settle(page);
  const afterEnter = await readState(page, framework);
  const enterDoc = "const answer = 42;\n\nconst unused = 0;\n";
  check(
    `${browserName} ${framework} the caret held its line-1 end position through the toggle`,
    afterEnter.doc === enterDoc,
    `doc=${JSON.stringify(afterEnter.doc)}`,
  );
  await pressModChord(page, framework, "z");
  await settle(page);
  const afterUndo = await readState(page, framework);
  check(
    `${browserName} ${framework} undo history survives the presentation toggle`,
    afterUndo.doc === INITIAL_DOC,
    `doc=${JSON.stringify(afterUndo.doc)}`,
  );

  // 4. An active diagnostic and its navigation state survive the toggle.
  await page.keyboard.press("F8");
  await settle(page);
  const diagnosticOn = await readState(page, framework);
  check(
    `${browserName} ${framework} F8 announces the diagnostic`,
    diagnosticOn.message === "warning 2:7 — unused binding" && diagnosticOn.diagnosticMark,
    `message=${JSON.stringify(diagnosticOn.message)}`,
  );
  await toggleLineNumbers(page, framework);
  await settle(page);
  const diagnosticOff = await readState(page, framework);
  check(
    `${browserName} ${framework} the active diagnostic and its mark survive live off`,
    !diagnosticOff.gutter &&
      diagnosticOff.message === "warning 2:7 — unused binding" &&
      diagnosticOff.diagnosticMark,
    `gutter=${diagnosticOff.gutter} message=${JSON.stringify(diagnosticOff.message)} mark=${diagnosticOff.diagnosticMark}`,
  );
  await page.keyboard.press("F8");
  await settle(page);
  const diagnosticWrap = await readState(page, framework);
  check(
    `${browserName} ${framework} F8 navigation stays coherent after the toggle`,
    diagnosticWrap.message === "warning 2:7 — unused binding" && diagnosticWrap.diagnosticMark,
    `message=${JSON.stringify(diagnosticWrap.message)}`,
  );
  await toggleLineNumbers(page, framework);
  await settle(page);
  const restored = await readState(page, framework);
  check(
    `${browserName} ${framework} live on restores the gutter with the diagnostic intact`,
    restored.gutter && restored.message === "warning 2:7 — unused binding",
    `gutter=${restored.gutter} message=${JSON.stringify(restored.message)}`,
  );

  // 5. Rapid controlled updates: the latest host value wins.
  await toggleLineNumbers(page, framework); // off
  await toggleLineNumbers(page, framework); // on
  await settle(page);
  const rapidOn = await readState(page, framework);
  check(
    `${browserName} ${framework} rapid off/on resolves to on`,
    rapidOn.gutter && rapidOn.pressed === "true",
    `gutter=${rapidOn.gutter} pressed=${rapidOn.pressed}`,
  );
  await toggleLineNumbers(page, framework); // off
  await toggleLineNumbers(page, framework); // on
  await toggleLineNumbers(page, framework); // off
  await settle(page);
  const rapidOff = await readState(page, framework);
  check(
    `${browserName} ${framework} rapid on/off/on/off resolves to off`,
    !rapidOff.gutter && rapidOff.pressed === "false" && rapidOff.doc === INITIAL_DOC,
    `gutter=${rapidOff.gutter} pressed=${rapidOff.pressed}`,
  );
  await toggleLineNumbers(page, framework);
  await settle(page);
  const final = await readState(page, framework);
  check(
    `${browserName} ${framework} final toggle restores the initial presentation`,
    final.gutter && final.pressed === "true" && final.doc === INITIAL_DOC && final.marked,
    `gutter=${final.gutter} pressed=${final.pressed}`,
  );
}

try {
  for (const [browserName, browserType] of engines) {
    let browser: Browser | undefined;
    try {
      browser = await browserType.launch({ headless: true });
      const page = await browser.newPage({ viewport: { width: 960, height: 720 } });
      await page.goto(url, { waitUntil: "networkidle" });
      await page.waitForSelector('[data-framework="svelte"] .poodle-code-editor');
      await page.waitForSelector('[data-framework="react"] .poodle-code-editor');
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

console.log("\nall CodeEditor line-numbers checks passed");
