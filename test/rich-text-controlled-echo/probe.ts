/**
 * Headless Chromium + WebKit proof that an accepted controlled echo of a
 * RichTextEditor user transaction is a true no-op in both web wrappers.
 * happy-dom can only approximate a real contenteditable: real caret motion,
 * real typing, real IME text insertion, and real history commands are what the
 * operator-visible defect moved, so this probe drives a live browser.
 *
 *   bun test/rich-text-controlled-echo/probe.ts --browser=chromium
 *   bun test/rich-text-controlled-echo/probe.ts --browser=webkit
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
const port = 4198;
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
  throw new Error(`rich-text controlled-echo fixture on :${port} did not start`);
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

interface EditorState {
  paragraph: string;
  caretText: string | null;
  caretOffset: number;
  focused: boolean;
  host: string;
}

async function readState(page: Page, framework: string): Promise<EditorState> {
  return page.evaluate((fw) => {
    const section = document.querySelector(`[data-framework="${fw}"]`);
    if (!(section instanceof HTMLElement)) throw new Error(`missing ${fw} section`);
    const surface = section.querySelector(".ProseMirror");
    const paragraph = surface?.querySelector("p");
    const selection = window.getSelection();
    return {
      paragraph: paragraph?.textContent ?? "",
      caretText: selection?.anchorNode?.nodeValue ?? null,
      caretOffset: selection?.anchorOffset ?? -1,
      focused: surface !== null && document.activeElement === surface,
      host: section.querySelector("[data-part='host-document']")?.textContent ?? "",
    };
  }, framework);
}

/** Real pointer click into the first paragraph, then a placed DOM caret. */
async function focusSurface(page: Page, framework: string): Promise<void> {
  await page.locator(`[data-framework="${framework}"] .ProseMirror p`).first().click();
  await settle(page);
}

/**
 * Place a real DOM caret inside the first paragraph's text node. Chromium and
 * WebKit disagree about `Home` and about where a click lands, so the caret is
 * installed at an exact offset the way a browser selection natively is. The
 * engine reads it from the DOM on the next typed character.
 */
async function placeCaret(
  page: Page,
  framework: string,
  from: number,
  to: number = from,
): Promise<void> {
  await page.evaluate(
    (payload) => {
      const surface = document.querySelector(`[data-framework="${payload.framework}"] .ProseMirror`);
      if (!(surface instanceof HTMLElement)) throw new Error("missing editing surface");
      surface.focus();
      const paragraph = surface.querySelector("p");
      const textNode = paragraph?.firstChild;
      if (!textNode) throw new Error("missing paragraph text node");
      const range = document.createRange();
      range.setStart(textNode, payload.from);
      range.setEnd(textNode, payload.to);
      const selection = window.getSelection();
      selection?.removeAllRanges();
      selection?.addRange(range);
    },
    { framework, from, to },
  );
  await settle(page);
}

async function runFramework(page: Page, framework: string, browserName: string): Promise<void> {
  const section = `[data-framework="${framework}"]`;
  await page.locator(`${section} .ProseMirror`).waitFor();
  await settle(page);

  // 1. Repeated middle-of-block typing through immediate controlled echoes.
  await focusSurface(page, framework);
  await placeCaret(page, framework, 5);
  const entered = await readState(page, framework);
  check(
    `${browserName} ${framework} entered the paragraph mid-block with focus`,
    entered.focused && entered.caretOffset === 5 && entered.paragraph === "alpha beta",
    `focused=${entered.focused} offset=${entered.caretOffset} paragraph=${JSON.stringify(entered.paragraph)}`,
  );

  let inserted = "";
  let expectedOffset = 5;
  for (const character of ["X", "Y", "Z"]) {
    await page.keyboard.type(character);
    await settle(page);
    inserted += character;
    expectedOffset += 1;
    const state = await readState(page, framework);
    const expectedText = `alpha${inserted} beta`;
    check(
      `${browserName} ${framework} caret stays after "${character}" through the echo`,
      state.focused && state.caretOffset === expectedOffset && state.paragraph === expectedText,
      `focused=${state.focused} offset=${state.caretOffset} paragraph=${JSON.stringify(state.paragraph)}`,
    );
  }
  const typed = await readState(page, framework);
  check(
    `${browserName} ${framework} the host document echoes the fully typed text`,
    typed.host.includes("alphaXYZ beta"),
    `host=${typed.host.slice(0, 120)}`,
  );

  // 2. IME text insertion commits once and keeps its place.
  await page.keyboard.insertText("\u6f22");
  await settle(page);
  const composed = await readState(page, framework);
  check(
    `${browserName} ${framework} an IME commit keeps the caret at the input position`,
    composed.focused &&
      composed.caretOffset === expectedOffset + 1 &&
      composed.paragraph === "alphaXYZ\u6f22 beta" &&
      composed.host.includes("alphaXYZ\u6f22 beta"),
    `focused=${composed.focused} offset=${composed.caretOffset} paragraph=${JSON.stringify(composed.paragraph)}`,
  );

  // 3. Undo and redo history survive every accepted echo.
  const undo = page.locator(`${section} [data-command="undo"] button`);
  await undo.click();
  await settle(page);
  const undone = await readState(page, framework);
  check(
    `${browserName} ${framework} undo reverts the echoed typing and restores focus`,
    undone.focused && undone.paragraph !== "alphaXYZ\u6f22 beta" && undone.paragraph.length < 15,
    `focused=${undone.focused} paragraph=${JSON.stringify(undone.paragraph)}`,
  );
  const redo = page.locator(`${section} [data-command="redo"] button`);
  await redo.click();
  await settle(page);
  const redone = await readState(page, framework);
  check(
    `${browserName} ${framework} redo restores the echoed typing`,
    redone.focused && redone.paragraph === "alphaXYZ\u6f22 beta",
    `focused=${redone.focused} paragraph=${JSON.stringify(redone.paragraph)}`,
  );

  // 4. Replacing a non-collapsed selection keeps the resulting caret.
  await focusSurface(page, framework);
  await placeCaret(page, framework, 5, 10);
  await page.keyboard.type("B");
  await settle(page);
  const replaced = await readState(page, framework);
  check(
    `${browserName} ${framework} a non-collapsed replacement keeps the collapsed caret`,
    replaced.focused &&
      replaced.caretOffset === 6 &&
      replaced.paragraph === "alphaBbeta" &&
      replaced.host.includes("alphaBbeta"),
    `focused=${replaced.focused} offset=${replaced.caretOffset} paragraph=${JSON.stringify(replaced.paragraph)}`,
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

console.log("\nall RichTextEditor controlled-echo checks passed");
