import { chromium } from "playwright";

import { pinPage, specimenUrl } from "./capture";
import { SMOKE_AXES } from "./config";
import { startPreviews } from "./server";

/**
 * Containment probe for g12.011 (anchored overlay portalling):
 *
 *   bun test/visual/overlay-portal-probe.ts
 *
 * The structural gates cannot see this class of bug — the markup is identical
 * whether or not an ancestor clips the surface. So this drives the real preview
 * and asserts the three failure modes portalling exists to fix:
 *
 *   1. clipped by a scrolling ancestor
 *   2. trapped by a transformed ancestor (a containing block defeats `fixed`)
 *   3. painted under a sibling stacking context
 *
 * plus the behaviour portalling costs us and has to hand back: the surface
 * hides when its anchor scrolls out of the pane.
 *
 * "Visible" is asserted with `elementFromPoint` at the surface's own centre —
 * the only check that actually proves nothing is covering or cutting it.
 */

type Case = {
  slug: string;
  trigger: string;
  surface: string;
};

const CASES: Case[] = [
  { slug: "model-picker", trigger: ".poodle-model-picker__trigger", surface: ".poodle-model-picker__surface" },
  { slug: "ref-select", trigger: ".poodle-ref-select__trigger", surface: ".poodle-ref-select__surface" },
  { slug: "select", trigger: ".poodle-select--custom .poodle-select__trigger-area", surface: ".poodle-select__listbox" },
  { slug: "popover", trigger: ".poodle-popover__trigger", surface: ".poodle-popover__surface" },
  { slug: "order-by", trigger: ".poodle-order-by__trigger", surface: ".poodle-order-by__surface" },
  { slug: "date-picker", trigger: ".poodle-date-picker__trigger", surface: ".poodle-date-picker__surface" },
];

/** Wrap the specimen in a clipping, transformed, low-z-index ancestor. */
const HOSTILE_ANCESTOR = `
  (() => {
    const section = document.querySelector(".poodle-component-page__section");
    if (!section) return "no section";
    const pane = document.createElement("div");
    pane.id = "hostile-pane";
    pane.style.cssText = [
      "overflow: auto",          // clips absolutely positioned descendants
      "transform: translateZ(0)",// containing block: traps position: fixed
      "filter: saturate(1)",     // second containing-block trigger
      "position: relative",
      "z-index: 0",              // stacking context capped below its sibling
      "height: 220px",
      "padding: 40px 12px",
    ].join(";");
    section.parentElement.insertBefore(pane, section);
    pane.appendChild(section);

    // Enough scroll range to lift the anchor clear of the pane's top edge on a
    // short specimen; without it scrollTop clamps before the anchor leaves.
    const spacer = document.createElement("div");
    spacer.style.height = "1200px";
    pane.appendChild(spacer);

    const rival = document.createElement("div");
    rival.id = "hostile-rival";
    rival.style.cssText = "position: relative; z-index: 5; height: 400px; background: rgba(255,0,0,0.35)";
    pane.parentElement.insertBefore(rival, pane.nextSibling);
    return "ok";
  })()
`;

const servers = await startPreviews();
const browser = await chromium.launch();
const context = await browser.newContext({ viewport: { width: 1280, height: 900 } });

let failures = 0;

function check(label: string, ok: boolean, detail = ""): void {
  if (!ok) failures += 1;
  console.log(`${ok ? "  ok  " : "  FAIL"}  ${label}${detail ? `  — ${detail}` : ""}`);
}

for (const framework of ["svelte", "react"] as const) {
  console.log(`\n=== ${framework} ===`);

  for (const testCase of CASES) {
    const page = await context.newPage();
    await pinPage(page);
    await page.goto(specimenUrl(servers.urls[framework], testCase.slug, SMOKE_AXES[0]), {
      waitUntil: "load",
    });
    await page.locator(".poodle-component-page__section").first().waitFor();

    const wrapped = await page.evaluate(HOSTILE_ANCESTOR);
    if (wrapped !== "ok") {
      check(`${testCase.slug}: hostile ancestor`, false, String(wrapped));
      await page.close();
      continue;
    }

    const trigger = page.locator(testCase.trigger).first();
    await trigger.click();

    const surface = page.locator(testCase.surface).first();
    await surface.waitFor({ state: "attached", timeout: 5000 }).catch(() => {});

    const report = await page.evaluate(
      ([surfaceSel]) => {
        const el = document.querySelector(surfaceSel as string) as HTMLElement | null;
        if (!el) return { found: false } as const;

        const rect = el.getBoundingClientRect();
        const cx = rect.left + rect.width / 2;
        const cy = rect.top + rect.height / 2;
        const hit = document.elementFromPoint(cx, cy);

        return {
          found: true,
          portalled: el.dataset.poodleAnchored === "true",
          insidePane: Boolean(el.closest("#hostile-pane")),
          position: getComputedStyle(el).position,
          rect: { top: rect.top, left: rect.left, width: rect.width, height: rect.height },
          // The topmost painted element at the surface's centre must be the
          // surface itself (or something inside it).
          topmostIsSurface: Boolean(hit && (hit === el || el.contains(hit))),
          hitTag: hit ? `${hit.tagName.toLowerCase()}.${(hit.className || "").toString().split(" ")[0]}` : null,
          withinViewport:
            rect.top >= 0 &&
            rect.left >= 0 &&
            rect.bottom <= window.innerHeight &&
            rect.right <= window.innerWidth,
        } as const;
      },
      [testCase.surface],
    );

    const label = `${testCase.slug}`;
    if (!report.found) {
      check(label, false, "surface never rendered");
      await page.close();
      continue;
    }

    check(`${label}: portalled out of the clipping pane`, report.portalled && !report.insidePane);
    check(`${label}: fixed positioning`, report.position === "fixed", report.position);
    check(`${label}: fits the viewport`, report.withinViewport, JSON.stringify(report.rect));
    check(
      `${label}: topmost at its own centre`,
      report.topmostIsSurface,
      report.topmostIsSurface ? "" : `hit ${report.hitTag}`,
    );

    // Scroll the anchor out of the pane: the surface must hide rather than
    // float over unrelated content.
    const hidden = await page.evaluate(
      ([surfaceSel, triggerSel]) => {
        const pane = document.getElementById("hostile-pane");
        const anchor = document.querySelector(triggerSel as string);
        if (!pane || !anchor) return "no pane";

        // Scroll by exactly enough to lift the anchor clear of the pane's top
        // edge. Scrolling to the end is not enough on a short specimen, where
        // the anchor stays partly visible — and a partly visible anchor is
        // meant to keep its surface.
        const paneTop = pane.getBoundingClientRect().top;
        pane.scrollTop += anchor.getBoundingClientRect().bottom - paneTop + 4;
        pane.dispatchEvent(new Event("scroll", { bubbles: true }));

        return new Promise<string>((resolve) => {
          requestAnimationFrame(() => {
            const el = document.querySelector(surfaceSel as string) as HTMLElement | null;
            resolve(el ? String(el.dataset.anchorHidden) : "gone");
          });
        });
      },
      [testCase.surface, testCase.trigger],
    );
    check(`${label}: hides when its anchor scrolls out`, hidden === "true" || hidden === "gone", String(hidden));

    await page.close();
  }
}

await browser.close();
await servers.stop();

console.log(failures === 0 ? "\nall checks passed" : `\n${failures} check(s) failed`);
process.exit(failures === 0 ? 0 : 1);
