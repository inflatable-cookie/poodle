import type { Page } from "playwright";

import { FIXED_TIME, SPECIMEN_SELECTOR, VIEWPORT, type Axis } from "./config";

/**
 * Determinism pinning + specimen capture.
 *
 * Screenshot gates fail on noise unless the page is pinned. Everything here
 * exists because an unpinned preview produces a different image on the second
 * run: transitions mid-flight, fonts swapping in, blinking carets, wall-clock
 * text, and focus rings left over from the previous navigation.
 */

const FREEZE_CSS = `
  *, *::before, *::after {
    transition-duration: 0s !important;
    transition-delay: 0s !important;
    animation-duration: 0s !important;
    animation-delay: 0s !important;
    animation-iteration-count: 1 !important;
    caret-color: transparent !important;
  }
  html { scroll-behavior: auto !important; }

  /*
   * Capture only the specimen. The page hero and the Import/UsageDocs sections
   * below it are preview chrome, and their text can land on a fractional pixel
   * boundary — which shifts the specimen's origin by a sub-pixel amount and
   * makes every glyph below it rasterise differently between the two apps.
   */
  .poodle-component-page__hero { display: none !important; }
  .poodle-component-page__section ~ .poodle-component-page__section {
    display: none !important;
  }
`;

export async function pinPage(page: Page): Promise<void> {
  // setFixedTime, not install(): install() pauses the clock, which starves
  // timer-driven work (React's scheduler, specimen effects) and makes renders
  // land differently between runs. This pins Date/now while timers keep firing.
  await page.clock.setFixedTime(FIXED_TIME);
  await page.addInitScript(() => {
    // Deterministic content: specimens that seed data from Math.random would
    // otherwise differ between the two apps and between runs.
    let seed = 0x2f6e2b1;
    Math.random = () => {
      seed = (seed * 1664525 + 1013904223) % 0xffffffff;
      return seed / 0xffffffff;
    };
  });
}

export function specimenUrl(base: string, slug: string, axis: Axis): string {
  const params = new URLSearchParams({
    theme: axis.theme,
    density: axis.density,
    controlSize: axis.controlSize,
  });
  return `${base}/?${params.toString()}#components/${slug}`;
}

export type CaptureResult =
  | { ok: true; png: Buffer; width: number; height: number }
  | { ok: false; error: string };

async function waitForStableHeight(page: Page, samples = 3, gapMs = 80): Promise<void> {
  let stable = 0;
  let last = -1;
  for (let attempt = 0; attempt < 25 && stable < samples; attempt += 1) {
    const height = await page.evaluate(
      (selector) =>
        Math.round(
          document.querySelector(selector)?.getBoundingClientRect().height ?? -1,
        ),
      SPECIMEN_SELECTOR,
    );
    stable = height === last ? stable + 1 : 0;
    last = height;
    if (stable < samples) await page.waitForTimeout(gapMs);
  }
}

export async function captureSpecimen(
  page: Page,
  base: string,
  slug: string,
  axis: Axis,
): Promise<CaptureResult> {
  const errors: string[] = [];
  const onPageError = (error: Error) => errors.push(error.message);
  page.on("pageerror", onPageError);

  try {
    // Reset the viewport: the previous capture may have grown it to fit a tall
    // specimen, and vh-sized chrome would then lay out differently.
    await page.setViewportSize(VIEWPORT);

    // about:blank first: a hash-only change does not reload, and the preview
    // shells read their axis params once on mount.
    await page.goto("about:blank");
    await page.goto(specimenUrl(base, slug, axis), { waitUntil: "load", timeout: 60_000 });

    const specimen = page.locator(SPECIMEN_SELECTOR).first();
    // Generous: vite compiles heavy specimens (ListCard, DataTable) on first
    // request, and both previews are being driven at once.
    await specimen.waitFor({ state: "visible", timeout: 60_000 });

    await page.addStyleTag({ content: FREEZE_CSS });
    await page.evaluate(
      (contrast) => {
        const shell = document.querySelector<HTMLElement>(".poodle-app-shell");
        if (shell) {
          if (contrast === 0.5) shell.style.removeProperty("--poodle-contrast");
          else shell.style.setProperty("--poodle-contrast", String(contrast));
        }
        (document.activeElement as HTMLElement | null)?.blur?.();
      },
      axis.contrast,
    );

    // Both previews scroll their content inside the app shell, not the
    // document. An element screenshot of a specimen taller than the viewport
    // then captures blank space where the clipped content should be, so unwind
    // every scrolling ancestor and grow the viewport to fit.
    const specimenHeight = await page.evaluate((selector) => {
      const el = document.querySelector<HTMLElement>(selector);
      if (!el) return 0;

      for (let node = el.parentElement; node; node = node.parentElement) {
        const styles = getComputedStyle(node);
        if (styles.overflow !== "visible" || styles.overflowY !== "visible") {
          node.style.setProperty("overflow", "visible", "important");
          node.style.setProperty("height", "auto", "important");
          node.style.setProperty("max-height", "none", "important");
        }
      }
      document.documentElement.style.setProperty("overflow", "visible", "important");
      document.body.style.setProperty("overflow", "visible", "important");

      return Math.ceil(el.getBoundingClientRect().height);
    }, SPECIMEN_SELECTOR);

    await page.setViewportSize({
      width: VIEWPORT.width,
      height: Math.min(Math.max(specimenHeight + 200, VIEWPORT.height), 12_000),
    });

    // Snap the specimen to an integer y. A fractional origin rasterises text
    // half a pixel off, which reads as a whole-image diff.
    await page.evaluate((selector) => {
      window.scrollTo(0, 0);
      const el = document.querySelector<HTMLElement>(selector);
      if (!el) return;
      const fraction = el.getBoundingClientRect().top % 1;
      if (fraction !== 0) el.style.marginTop = `${-fraction}px`;
    }, SPECIMEN_SELECTOR);

    await page.evaluate(() => document.fonts.ready);
    await page.waitForFunction(() => document.fonts.status === "loaded");
    // One frame after the freeze stylesheet lands, so any in-flight transition
    // has snapped to its end state.
    await page.evaluate(
      () => new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(resolve))),
    );

    // Settle: hold until the specimen's height stops changing, then require two
    // identical captures. Async specimen state (debounced filters, deferred
    // effects) otherwise lands mid-render in one run and not the next.
    await waitForStableHeight(page);
    let png = await specimen.screenshot({ animations: "disabled" });
    const confirm = await specimen.screenshot({ animations: "disabled" });
    if (!png.equals(confirm)) {
      await page.waitForTimeout(500);
      png = await specimen.screenshot({ animations: "disabled" });
      const second = await specimen.screenshot({ animations: "disabled" });
      if (!png.equals(second)) {
        return { ok: false, error: "specimen never settled (two unstable captures)" };
      }
    }
    const box = await specimen.boundingBox();

    if (errors.length > 0) {
      return { ok: false, error: `page error: ${errors[0]}` };
    }

    return {
      ok: true,
      png,
      width: Math.round(box?.width ?? 0),
      height: Math.round(box?.height ?? 0),
    };
  } catch (error) {
    return { ok: false, error: (error as Error).message };
  } finally {
    page.off("pageerror", onPageError);
  }
}
