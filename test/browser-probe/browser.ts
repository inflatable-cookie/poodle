/** Engine selection, browser lifetime and page-timing helpers for the browser probes. */
import { chromium, webkit, type Browser, type BrowserType, type LaunchOptions, type Page } from "playwright";

export type EngineName = "chromium" | "webkit";

/** The engine named by `--browser=`, or both without the flag. */
export function selectedEngines(): Array<[EngineName, BrowserType]> {
  const flag = process.argv.find((arg) => arg.startsWith("--browser="))?.slice("--browser=".length);
  const engines: Array<[EngineName, BrowserType]> = [
    ["chromium", chromium],
    ["webkit", webkit],
  ];
  const selected = engines.filter(([name]) => flag === undefined || flag === name);
  if (selected.length === 0) throw new Error(`Unknown --browser=${flag}`);
  return selected;
}

/** Launches `engine` for the duration of `body` and always closes it. */
export async function withBrowser<T>(
  engine: BrowserType,
  body: (browser: Browser) => Promise<T>,
  options?: LaunchOptions,
): Promise<T> {
  const browser = await engine.launch(options);
  try {
    return await body(browser);
  } finally {
    // A close failure must not replace the body's own result or error.
    await browser.close().catch((error) => console.warn(`  browser close failed: ${error}`));
  }
}

/** Waits for `count` animation frames. */
export async function frames(page: Page, count: number): Promise<void> {
  await page.evaluate((n) => {
    return new Promise<void>((resolve) => {
      const tick = (left: number) => {
        if (left <= 0) {
          resolve();
          return;
        }
        requestAnimationFrame(() => tick(left - 1));
      };
      tick(n);
    });
  }, count);
}

/** Waits one animation frame, then `ms` for timer-driven follow-up work. */
export async function settle(page: Page, ms: number): Promise<void> {
  await page.evaluate(
    (delay) =>
      new Promise<void>((resolve) => {
        requestAnimationFrame(() => window.setTimeout(resolve, delay));
      }),
    ms,
  );
}

/** Rejects with `message` if `work` has not settled within `ms`. */
export async function deadline<T>(ms: number, message: string, work: Promise<T>): Promise<T> {
  let timer: ReturnType<typeof setTimeout> | undefined;
  const expired = new Promise<never>((_, reject) => {
    timer = setTimeout(() => reject(new Error(message)), ms);
  });
  try {
    return await Promise.race([work, expired]);
  } finally {
    clearTimeout(timer);
  }
}
