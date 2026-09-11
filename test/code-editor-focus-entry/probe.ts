/**
 * Headless Chromium + WebKit proof that CodeEditor's outer focus treatment is
 * a keyboard-entry affordance that dismisses on the first editing intent
 * while the document input modality stays truthful. jsdom cannot prove
 * computed styles or real focus movement.
 *
 *   bun test/code-editor-focus-entry/probe.ts --browser=chromium
 *   bun test/code-editor-focus-entry/probe.ts --browser=webkit
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
const port = 4193;
const url = `http://127.0.0.1:${port}/`;

let failures = 0;

type Ring = {
  outlineWidth: string;
  outlineStyle: string;
  outlineColor: string;
  entry: string | null;
  modality: string;
};

function check(label: string, ok: boolean, detail = ""): void {
  if (!ok) failures += 1;
  console.log(`${ok ? "  ok  " : "  FAIL"}  ${label}${detail ? `  — ${detail}` : ""}`);
}

function formatRing(ring: Ring): string {
  return `modality=${ring.modality} entry=${ring.entry ?? "none"} outline=${ring.outlineStyle} ${ring.outlineWidth} ${ring.outlineColor}`;
}

function armed(ring: Ring): boolean {
  return ring.entry === "keyboard" && ring.outlineStyle !== "none" && ring.outlineWidth !== "0px";
}

function disarmed(ring: Ring): boolean {
  return ring.entry === null && (ring.outlineStyle === "none" || ring.outlineWidth === "0px");
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
  throw new Error(`code-editor focus-entry fixture on :${port} did not start`);
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

async function readRing(page: Page, framework: "svelte" | "react"): Promise<Ring> {
  return page.evaluate((fw) => {
    const root = document.querySelector(`[data-framework="${fw}"] .poodle-code-editor`);
    if (!(root instanceof HTMLElement)) {
      throw new Error(`missing ${fw} CodeEditor root`);
    }
    const style = getComputedStyle(root);
    return {
      outlineWidth: style.outlineWidth,
      outlineStyle: style.outlineStyle,
      outlineColor: style.outlineColor,
      entry: root.getAttribute("data-focus-entry"),
      modality: document.documentElement.getAttribute("data-poodle-input-modality") ?? "",
    };
  }, framework);
}

/** Dispatch a Mod-chord keydown on the editing surface (platform-correct Mod). */
async function pressModChord(page: Page, framework: "svelte" | "react", key: string): Promise<void> {
  await page.evaluate(
    ([fw, chordKey]) => {
      const surface = document.querySelector(`[data-framework="${fw}"] .cm-content`);
      if (!(surface instanceof HTMLElement)) throw new Error("missing editing surface");
      surface.dispatchEvent(
        new KeyboardEvent("keydown", {
          bubbles: true,
          cancelable: true,
          key: chordKey,
          // CodeMirror reads metaKey for Mod on macOS and ctrlKey elsewhere.
          ...(navigator.platform.includes("Mac") ? { metaKey: true } : { ctrlKey: true }),
        }),
      );
    },
    [framework, key],
  );
}

async function runFramework(page: Page, framework: "svelte" | "react", browserName: string): Promise<void> {
  const section = `[data-framework="${framework}"]`;
  const before = page.locator(`${section} [data-before]`);
  const content = page.locator(`${section} .cm-content`);
  await content.waitFor();

  await before.click();
  await settle(page);
  const resting = await readRing(page, framework);
  console.log(`  evidence  ${browserName} ${framework} resting  ${formatRing(resting)}`);

  // 1. Pointer entry paints no ring and arms nothing.
  await content.click();
  await settle(page);
  const pointer = await readRing(page, framework);
  console.log(`  evidence  ${browserName} ${framework} pointer  ${formatRing(pointer)}`);
  check(
    `${browserName} ${framework} pointer entry paints no outer treatment`,
    pointer.modality === "pointer" && disarmed(pointer),
    formatRing(pointer),
  );

  // 2. Typing after pointer entry must not paint the ring; the document
  //    modality flips to keyboard while the local treatment stays off.
  await page.keyboard.press("x");
  await settle(page);
  const pointerTyping = await readRing(page, framework);
  console.log(`  evidence  ${browserName} ${framework} pointer+type  ${formatRing(pointerTyping)}`);
  check(
    `${browserName} ${framework} typing after pointer entry paints no outer treatment`,
    pointerTyping.modality === "keyboard" && disarmed(pointerTyping),
    formatRing(pointerTyping),
  );

  // 3. Leave, then re-enter by keyboard: Tab arms the local treatment.
  await before.click();
  await settle(page);
  await page.keyboard.press("Tab");
  await settle(page);
  const keyboard = await readRing(page, framework);
  console.log(`  evidence  ${browserName} ${framework} keyboard entry  ${formatRing(keyboard)}`);
  check(
    `${browserName} ${framework} keyboard entry arms the outer treatment`,
    keyboard.modality === "keyboard" && armed(keyboard),
    formatRing(keyboard),
  );
  const focused = await page.evaluate((fw) => {
    const surface = document.querySelector(`[data-framework="${fw}"] .cm-content`);
    return document.activeElement === surface;
  }, framework);
  check(`${browserName} ${framework} Tab lands on the editing surface`, focused);

  // 4. Navigation-only keys keep the entry treatment.
  await page.keyboard.press("ArrowDown");
  await page.keyboard.press("End");
  await settle(page);
  const navigation = await readRing(page, framework);
  check(
    `${browserName} ${framework} navigation-only keys preserve the treatment`,
    armed(navigation),
    formatRing(navigation),
  );

  // 5. Non-edit chords (copy, select all) keep it too: the document is
  //    untouched and the contract's dismissal list names editing intent only.
  await pressModChord(page, framework, "c");
  await pressModChord(page, framework, "a");
  await settle(page);
  const chords = await readRing(page, framework);
  check(
    `${browserName} ${framework} copy and select-all chords preserve the treatment`,
    armed(chords),
    formatRing(chords),
  );

  // 6. Find panel traversal (Mod+F, Enter for next result) keeps it: the
  //    document is untouched.
  await pressModChord(page, framework, "f");
  await settle(page);
  const panel = page.locator(`${section} .cm-search input[name="search"]`);
  await panel.waitFor();
  await panel.focus();
  await page.keyboard.press("Enter");
  await settle(page);
  const traversal = await readRing(page, framework);
  console.log(`  evidence  ${browserName} ${framework} search traversal  ${formatRing(traversal)}`);
  check(
    `${browserName} ${framework} find-panel traversal preserves the treatment`,
    armed(traversal),
    formatRing(traversal),
  );

  // 7. Close find (focus returns to the editing surface per the contract).
  await page.keyboard.press("Escape");
  await settle(page);
  const closed = await readRing(page, framework);
  check(`${browserName} ${framework} closing find keeps the treatment armed`, armed(closed), formatRing(closed));

  // 8. Collapse the select-all range (navigation-only), then the first
  //    editing intent dismisses it; global modality stays keyboard.
  await page.keyboard.press("ArrowDown");
  await settle(page);
  await page.keyboard.press("y");
  await settle(page);
  const editing = await readRing(page, framework);
  console.log(`  evidence  ${browserName} ${framework} editing  ${formatRing(editing)}`);
  check(
    `${browserName} ${framework} first edit dismisses the treatment with document modality still keyboard`,
    editing.modality === "keyboard" && disarmed(editing),
    formatRing(editing),
  );
  const caretVisible = await page.evaluate((fw) => {
    const surface = document.querySelector(`[data-framework="${fw}"] .cm-content`);
    return surface instanceof HTMLElement && getComputedStyle(surface).caretColor !== "transparent";
  }, framework);
  check(`${browserName} ${framework} caret remains visible after dismissal`, caretVisible);

  // 9. A binding that repurposes navigation keys still counts as an edit:
  //    Shift+Alt+ArrowDown runs copyLineDown and duplicates the line.
  await page.keyboard.press("Shift+Alt+ArrowDown");
  await settle(page);
  const lineCopy = await readRing(page, framework);
  console.log(`  evidence  ${browserName} ${framework} copyLineDown  ${formatRing(lineCopy)}`);
  const copiedDoc = await page.evaluate((fw) => {
    const surface = document.querySelector(`[data-framework="${fw}"] .cm-content`);
    if (!(surface instanceof HTMLElement)) return null;
    return [...surface.querySelectorAll(".cm-line")].map((line) => line.textContent ?? "").join("\n");
  }, framework);
  const lineCopied = copiedDoc === "const answer = 42;xy\nconst answer = 42;xy";
  console.log(`  evidence  ${browserName} ${framework} copyLineDown doc  ${JSON.stringify(copiedDoc)}`);
  check(`${browserName} ${framework} copyLineDown duplicates the document line`, lineCopied);
  check(
    `${browserName} ${framework} a committed line command dismisses the treatment`,
    disarmed(lineCopy),
    formatRing(lineCopy),
  );

  // 10. Leaving and re-entering by keyboard restores the treatment.
  await before.click();
  await settle(page);
  const exited = await readRing(page, framework);
  check(`${browserName} ${framework} exit resets the local state`, disarmed(exited), formatRing(exited));
  await page.keyboard.press("Tab");
  await settle(page);
  const reentry = await readRing(page, framework);
  console.log(`  evidence  ${browserName} ${framework} re-entry  ${formatRing(reentry)}`);
  check(
    `${browserName} ${framework} keyboard re-entry restores the treatment`,
    reentry.modality === "keyboard" && armed(reentry),
    formatRing(reentry),
  );

  // 11. A pointer press inside the armed editor yields the affordance; the
  //     document modality agrees.
  await content.click();
  await settle(page);
  const pointerPress = await readRing(page, framework);
  check(
    `${browserName} ${framework} pointer press inside the armed editor clears it`,
    pointerPress.modality === "pointer" && disarmed(pointerPress),
    formatRing(pointerPress),
  );

  // 12. Keyboard re-entry after the pointer press re-arms, and typing
  //     dismisses again.
  await before.click();
  await settle(page);
  await page.keyboard.press("Tab");
  await settle(page);
  const rearmed = await readRing(page, framework);
  check(`${browserName} ${framework} keyboard re-entry re-arms after pointer press`, armed(rearmed), formatRing(rearmed));
  await page.keyboard.press("q");
  await settle(page);
  const dismissedAgain = await readRing(page, framework);
  check(`${browserName} ${framework} typing after re-entry dismisses again`, disarmed(dismissedAgain), formatRing(dismissedAgain));
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

console.log("\nall CodeEditor focus-entry checks passed");
