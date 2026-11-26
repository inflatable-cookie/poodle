/**
 * Headless Chromium + WebKit proof that TokenInput's composite focus
 * treatment follows input modality. jsdom cannot prove computed styles.
 *
 *   bun test/focus-ring-modality/probe.ts --browser=chromium
 *   bun test/focus-ring-modality/probe.ts --browser=webkit
 */

import type { Page } from "playwright";

import {
  check,
  finish,
  selectedEngines,
  settle,
  withBrowser,
  withFixture,
} from "../browser-probe";

const engines = selectedEngines();

type Chrome = {
  boxShadow: string;
  borderColor: string;
  backgroundColor: string;
  modality: string;
};

function formatChrome(chrome: Chrome): string {
  return `modality=${chrome.modality} box-shadow=${JSON.stringify(chrome.boxShadow)} border-color=${JSON.stringify(chrome.borderColor)} background=${JSON.stringify(chrome.backgroundColor)}`;
}

function chromeEqual(a: Chrome, b: Chrome): boolean {
  return a.boxShadow === b.boxShadow && a.borderColor === b.borderColor && a.backgroundColor === b.backgroundColor;
}

async function readChrome(page: Page, framework: "svelte" | "react"): Promise<Chrome> {
  return page.evaluate((fw) => {
    const root = document.querySelector(`[data-framework="${fw}"] .poodle-token-input`);
    if (!(root instanceof HTMLElement)) {
      throw new Error(`missing ${fw} TokenInput root`);
    }
    const style = getComputedStyle(root);
    return {
      boxShadow: style.boxShadow,
      borderColor: style.borderColor,
      backgroundColor: style.backgroundColor,
      modality: document.documentElement.getAttribute("data-poodle-input-modality") ?? "",
    };
  }, framework);
}

async function runFramework(page: Page, framework: "svelte" | "react", browserName: string): Promise<void> {
  const section = `[data-framework="${framework}"]`;
  const input = page.locator(`${section} .poodle-token-input__control`);
  const before = page.locator(`${section} [data-before]`);
  await input.waitFor();

  await before.click();
  await settle(page, 200);
  const resting = await readChrome(page, framework);
  console.log(`  evidence  ${browserName} ${framework} resting  ${formatChrome(resting)}`);

  await input.click();
  await settle(page, 200);
  const pointer = await readChrome(page, framework);
  console.log(`  evidence  ${browserName} ${framework} pointer  ${formatChrome(pointer)}`);
  check(
    `${browserName} ${framework} pointer focus paints no composite treatment`,
    chromeEqual(pointer, resting) && pointer.modality === "pointer",
    formatChrome(pointer),
  );

  await before.click();
  await settle(page, 200);
  await page.keyboard.press("Tab");
  await settle(page, 200);
  const keyboard = await readChrome(page, framework);
  console.log(`  evidence  ${browserName} ${framework} keyboard  ${formatChrome(keyboard)}`);
  const focused = await page.evaluate((fw) => {
    const control = document.querySelector(`[data-framework="${fw}"] .poodle-token-input__control`);
    return document.activeElement === control;
  }, framework);
  check(`${browserName} ${framework} Tab lands on the live input`, focused);
  check(
    `${browserName} ${framework} keyboard focus paints the composite treatment`,
    keyboard.modality === "keyboard" &&
      !chromeEqual(keyboard, resting) &&
      keyboard.boxShadow !== resting.boxShadow &&
      keyboard.borderColor !== resting.borderColor,
    formatChrome(keyboard),
  );
}

await withFixture(import.meta.url, async (origin) => {
  for (const [browserName, browserType] of engines) {
    await withBrowser(browserType, async (browser) => {
      const page = await browser.newPage({ viewport: { width: 960, height: 720 } });
      await page.goto(`${origin}/`, { waitUntil: "networkidle" });
      await page.waitForSelector('[data-framework="svelte"] .poodle-token-input');
      await page.waitForSelector('[data-framework="react"] .poodle-token-input');
      await runFramework(page, "svelte", browserName);
      await runFramework(page, "react", browserName);
    });
  }
});

finish("focus-ring input-modality");
