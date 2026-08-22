/**
 * g15.047 — Svelte/React capture driver. Boots the two vite previews, drives
 * each Button fixture through its capture-only fixture host, and retains a
 * PNG plus a `poodle.button-visual-capture.v1` receipt per runtime.
 *
 * Determinism is a precondition, not a metric: every fixture/runtime pair is
 * captured twice with a fresh navigation between, and the pair must be
 * byte-identical. A mismatch stops the batch — there is no retry and no
 * frame picking. Receipts are verified against the retained PNG bytes before
 * anything is written.
 */

import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

import { chromium, type Browser, type Page } from "playwright";

import type { ButtonFixture } from "../fixtures/button-visual-inventory.ts";
import { pinPage } from "../capture.ts";
import { ensureUp, startPreviews, type PreviewServers } from "../server.ts";
import {
  CaptureIntegrityError,
  isRecoverableTransportError,
  PreviewTransportError,
} from "./capture-set.ts";
import {
  parseButtonCaptureReceipt,
  RECEIPT_SCHEMA,
  sha256Hex,
  verifyReceiptAgainstPng,
  type ButtonCaptureReceipt,
  type RoleEvidence,
  type Srgb,
} from "./receipt.ts";

export type WebCaptureRecord = {
  fixture: string;
  runtime: "svelte" | "react";
  pngPath: string;
  receiptPath: string;
  sha256: string;
  repeatSha256: string;
};

const WEB_RUNTIMES = ["svelte", "react"] as const;

/** Mirrors the pinning rules in ../capture.ts, minus the catalogue-chrome hiding. */
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
`;

function fixtureFileStem(name: string): string {
  return name.replace("/", "--");
}

function fixtureUrl(base: string, fixture: ButtonFixture): string {
  const params = new URLSearchParams({
    fixture: fixture.name,
    theme: fixture.theme,
    size: fixture.size,
    density: fixture.density,
    variant: fixture.variant,
    tone: fixture.tone,
    state: fixture.state,
    contentKind: fixture.content.kind,
  });
  if (fixture.content.kind === "label" || fixture.content.kind === "leading-icon") {
    params.set("label", fixture.content.label);
  }
  if (fixture.content.kind === "leading-icon" || fixture.content.kind === "icon-only") {
    params.set("icon", fixture.content.icon);
  }
  if (fixture.content.kind === "icon-only") {
    params.set("ariaLabel", fixture.content.ariaLabel);
  }
  return `${base}/?${params.toString()}`;
}

type MeasuredScene = {
  landmarks: ButtonCaptureReceipt["landmarks"];
  roles: RoleEvidence;
};

/**
 * In-page measurement: landmark bounds from getBoundingClientRect, role
 * evidence from getComputedStyle on the rendered button. Runs entirely in the
 * browser so the numbers are the renderer's own.
 */
function measureScene(): MeasuredScene {
  type Bounds = { x: number; y: number; width: number; height: number };

  function boundsOf(element: Element): Bounds {
    const rect = element.getBoundingClientRect();
    return { x: rect.x, y: rect.y, width: rect.width, height: rect.height };
  }

  /** Standard OKLab → sRGB (IEC 61966-2-1) conversion, clamped to gamut. */
  function oklabToSrgb(L: number, a: number, b: number): [number, number, number] {
    const l_ = L + 0.3963377774 * a + 0.2158037573 * b;
    const m_ = L - 0.1055613458 * a - 0.0638541728 * b;
    const s_ = L - 0.0894841775 * a - 1.2914855480 * b;
    const l = l_ ** 3;
    const m = m_ ** 3;
    const s = s_ ** 3;
    const linear = [
      +4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
      -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
      -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s,
    ];
    return linear.map((channel) => {
      const clamped = Math.min(Math.max(channel, 0), 1);
      return clamped <= 0.0031308 ? 12.92 * clamped : 1.055 * clamped ** (1 / 2.4) - 0.055;
    }) as [number, number, number];
  }

  function parseColor(value: string): Srgb {
    const rgb = value.match(/^rgba?\(([^)]+)\)$/);
    if (rgb) {
      // Legacy comma serialization: `rgb(255, 0, 0)` / `rgba(255, 0, 0, 0.5)`.
      const parts = rgb[1].split(",").map((part) => part.trim());
      const channel = (part: string): number =>
        part.endsWith("%") ? parseFloat(part) / 100 : parseFloat(part) / 255;
      const alpha = (part: string | undefined): number => {
        if (part === undefined) return 1;
        return part.endsWith("%") ? parseFloat(part) / 100 : parseFloat(part);
      };
      return [channel(parts[0]), channel(parts[1]), channel(parts[2]), alpha(parts[3])];
    }
    // Chromium serializes computed oklch-specified colors in OKLab/OKLCH:
    // `oklab(0.454 -0.004 -0.009)` or `oklch(0.454 0.01 250 / 0.8)`.
    const ok = value.match(/^ok(lab|lch)\(([^)]+)\)$/);
    if (ok) {
      const [componentsPart, alphaPart] = ok[2].split("/").map((part) => part.trim());
      const components = componentsPart.split(/\s+/);
      const scaled = (part: string, percentScale: number): number =>
        part.endsWith("%") ? (parseFloat(part) / 100) * percentScale : parseFloat(part);
      const lightness = scaled(components[0], 1);
      let a: number;
      let b: number;
      if (ok[1] === "lab") {
        a = scaled(components[1], 0.4);
        b = scaled(components[2], 0.4);
      } else {
        const chroma = scaled(components[1], 0.4);
        const hue = (parseFloat(components[2]) * Math.PI) / 180;
        a = chroma * Math.cos(hue);
        b = chroma * Math.sin(hue);
      }
      let alpha = 1;
      if (alphaPart !== undefined) {
        alpha = alphaPart.endsWith("%") ? parseFloat(alphaPart) / 100 : parseFloat(alphaPart);
      }
      const [r, g, blue] = oklabToSrgb(lightness, a, b);
      return [r, g, blue, alpha];
    }
    // Wide-gamut serialization, e.g. `color(srgb 1 0 0 / 0.5)`.
    const colorFn = value.match(
      /^color\(srgb\s+([^\s/]+)\s+([^\s/]+)\s+([^\s/]+?)(?:\s*\/\s*([^\s%)]+)(%)?)?\)$/,
    );
    if (colorFn) {
      let alpha = 1;
      if (colorFn[4] !== undefined) {
        alpha = parseFloat(colorFn[4]);
        if (colorFn[5] === "%") alpha /= 100;
      }
      return [parseFloat(colorFn[1]), parseFloat(colorFn[2]), parseFloat(colorFn[3]), alpha];
    }
    throw new Error(`unrecognized computed color '${value}'`);
  }

  /**
   * Resolve a raw token value (hex, oklch, var(...) reference) to a computed
   * sRGB color without a canvas: paint it onto a throwaway element and read
   * the computed style back. Custom properties inherit from the themed
   * document element, so `var(...)` references resolve on the probe.
   */
  function resolveColor(raw: string): Srgb {
    const probe = document.createElement("span");
    probe.style.color = raw;
    if (probe.style.color === "") {
      throw new Error(`focus-ring color token '${raw}' is not a usable CSS color`);
    }
    document.body.appendChild(probe);
    const resolved = getComputedStyle(probe).color;
    probe.remove();
    return parseColor(resolved);
  }

  /** Resolve a raw length token (px, rem, var(...)) to a px float. */
  function resolveLength(raw: string): number {
    const probe = document.createElement("div");
    probe.style.position = "absolute";
    probe.style.width = raw;
    if (probe.style.width === "") {
      throw new Error(`focus-ring width token '${raw}' is not a usable CSS length`);
    }
    document.body.appendChild(probe);
    const px = parseFloat(getComputedStyle(probe).width);
    probe.remove();
    if (!Number.isFinite(px)) {
      throw new Error(`focus-ring width token '${raw}' did not resolve to a px length`);
    }
    return px;
  }

  function parseShadowLayers(value: string): RoleEvidence["shadow"]["layers"] {
    if (value === "none") return [];
    // Split layers on commas that are not inside a color function's parens.
    const layers: string[] = [];
    let depth = 0;
    let current = "";
    for (const char of value) {
      if (char === "(") depth += 1;
      if (char === ")") depth -= 1;
      if (char === "," && depth === 0) {
        layers.push(current);
        current = "";
      } else {
        current += char;
      }
    }
    layers.push(current);

    return layers.map((layer) => {
      const colorMatch = layer.match(/rgba?\([^)]*\)|color\([^)]*\)/);
      if (!colorMatch) throw new Error(`shadow layer '${layer}' has no color`);
      const rest = `${layer.slice(0, colorMatch.index)} ${layer.slice((colorMatch.index ?? 0) + colorMatch[0].length)}`;
      const tokens = rest.trim().split(/\s+/).filter(Boolean);
      const inset = tokens.includes("inset");
      const lengths = tokens.filter((token) => token !== "inset").map(parseFloat);
      const [offsetX = 0, offsetY = 0, blur = 0, spread = 0] = lengths;
      return { inset, offsetX, offsetY, blur, spread, color: parseColor(colorMatch[0]) };
    });
  }

  const button = document.querySelector(".poodle-button");
  if (!button) throw new Error("no .poodle-button in the fixture host");

  // Landmarks measure the rendered parts, not web-only scaffolding: the icon
  // landmark is the glyph (`svg`), the spinner landmark is the spinner host —
  // both sit centered inside a 16px wrapper span that exists only to reserve
  // layout space (its contract force is observable through root/content
  // geometry, so the wrapper itself is not a separate landmark).
  const landmarks: MeasuredScene["landmarks"] = { root: boundsOf(button) };
  const label = button.querySelector(".poodle-button__label");
  const icon = button.querySelector(".poodle-button__icon svg");
  const spinner = button.querySelector(".poodle-button__spinner .poodle-spinner");
  // Icon-only rule: with no label, content reuses the icon bounds.
  if (label) landmarks.content = boundsOf(label);
  else if (icon) landmarks.content = boundsOf(icon);
  if (icon) landmarks.icon = boundsOf(icon);
  if (spinner) landmarks.spinner = boundsOf(spinner);

  const styles = getComputedStyle(button);
  const focusRingColorRaw = styles.getPropertyValue("--poodle-color-accent-focusRing").trim();
  const focusRingWidthRaw = styles.getPropertyValue("--poodle-border-width-focus").trim();
  // Dormant means reachable-in-this-state: the values the runtime would paint
  // on keyboard focus *in the captured state*. A disabled button is
  // unfocusable in HTML (loading implies disabled on this component), so no
  // ring can ever paint for it and the control declares none — even though
  // getComputedStyle still resolves the unreachable :focus-visible tokens.
  const focusable = !(button as HTMLButtonElement).disabled;

  const roles: RoleEvidence = {
    fill: { color: parseColor(styles.backgroundColor) },
    border: {
      color: parseColor(styles.borderColor),
      width: parseFloat(styles.borderTopWidth),
    },
    text: { color: parseColor(styles.color) },
    shadow: { layers: parseShadowLayers(styles.boxShadow) },
    // Dormant declared evidence: what the runtime would paint on keyboard
    // focus. No fixture in this batch captures a focused frame.
    "focus-ring": {
      color: !focusable || focusRingColorRaw === "" ? null : resolveColor(focusRingColorRaw),
      width: !focusable || focusRingWidthRaw === "" ? null : resolveLength(focusRingWidthRaw),
      status: "dormant",
    },
  };

  return { landmarks, roles };
}

type CapturedScene = {
  png: Buffer;
  landmarks: ButtonCaptureReceipt["landmarks"];
  roles: RoleEvidence;
};

async function captureOnce(page: Page, url: string): Promise<CapturedScene> {
  // The navigation/marker phase is the only transport-failure surface: a
  // degraded page or dead preview fails here, before any frame exists, and
  // the batch may recover from exactly this class (once, on a fresh page).
  // Everything after it — measurement, screenshot, hashing — is evidence and
  // fails the batch without retry.
  try {
    // about:blank first: the fixture host reads its params once on mount, and a
    // search-only change does not reload the page.
    await page.goto("about:blank");
    await page.goto(url, { waitUntil: "load", timeout: 60_000 });

    // The host reports param problems in-page; either marker wins the race.
    await page.waitForSelector("[data-fixture-error], [data-fixture-ready]", { timeout: 60_000 });
  } catch (error) {
    throw new PreviewTransportError(
      `preview navigation failed for ${url}: ${(error as Error).message}`,
      error,
    );
  }
  const errorElement = await page.$("[data-fixture-error]");
  if (errorElement) {
    const text = await errorElement.textContent();
    throw new Error(`fixture host rejected the params: ${text}`);
  }

  const button = page.locator(".poodle-button");
  await button.waitFor({ state: "visible", timeout: 60_000 });

  await page.addStyleTag({ content: FREEZE_CSS });
  await page.evaluate(() => document.fonts.ready);
  await page.waitForFunction(() => document.fonts.status === "loaded");
  // One frame past the freeze stylesheet so in-flight transitions snap to
  // their end state before measurement and capture.
  await page.evaluate(
    () => new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(resolve))),
  );

  const scene = await page.evaluate(measureScene);
  const png = await page.screenshot({ animations: "disabled" });
  return { png, landmarks: scene.landmarks, roles: scene.roles };
}

async function captureFixtureRuntime(
  page: Page,
  browser: Browser,
  base: string,
  fixture: ButtonFixture,
  runtime: (typeof WEB_RUNTIMES)[number],
  outDir: string,
): Promise<WebCaptureRecord> {
  const url = fixtureUrl(base, fixture);
  const id = `${fixture.name} [${runtime}]`;

  const first = await captureOnce(page, url);
  // Fresh navigation for the repeat: byte-identical output from the same
  // fixed input is the determinism contract, not a sampled average.
  const repeat = await captureOnce(page, url);

  const sha256 = sha256Hex(first.png);
  const repeatSha256 = sha256Hex(repeat.png);
  if (!first.png.equals(repeat.png)) {
    throw new CaptureIntegrityError(
      `repeat captures differ for ${id}: ${sha256} vs ${repeatSha256} — ` +
        "fixed input must render byte-identically; the batch stops rather than choosing a frame",
    );
  }

  const receipt: ButtonCaptureReceipt = {
    schema: RECEIPT_SCHEMA,
    fixture: fixture.name,
    runtime,
    logicalViewport: { width: fixture.viewport.width, height: fixture.viewport.height },
    scale: fixture.scale,
    deviceDimensions: {
      width: fixture.viewport.width * fixture.scale,
      height: fixture.viewport.height * fixture.scale,
    },
    pngSha256: sha256,
    environment: { kind: "chromium", version: browser.version() },
    landmarks: first.landmarks,
    roles: first.roles,
  };

  // Verify before anything hits disk: a receipt that cannot parse or does not
  // match its PNG is a failed capture, not an artifact.
  const parsedReceipt = parseButtonCaptureReceipt(receipt, { fixture, runtime });
  const problems = verifyReceiptAgainstPng(parsedReceipt, first.png);
  if (problems.length > 0) {
    throw new CaptureIntegrityError(`web receipt does not verify for ${id}:\n  - ${problems.join("\n  - ")}`);
  }

  const dir = join(outDir, runtime);
  mkdirSync(dir, { recursive: true });
  const stem = fixtureFileStem(fixture.name);
  const pngPath = join(dir, `${stem}.png`);
  const receiptPath = join(dir, `${stem}.json`);
  writeFileSync(pngPath, first.png);
  writeFileSync(receiptPath, `${JSON.stringify(parsedReceipt, null, 2)}\n`);

  return { fixture: fixture.name, runtime, pngPath, receiptPath, sha256, repeatSha256 };
}

export async function captureWebBatch(
  fixtures: ButtonFixture[],
  outDir: string,
): Promise<WebCaptureRecord[]> {
  if (fixtures.length === 0) return [];
  const viewport = fixtures[0].viewport;
  const scale = fixtures[0].scale;
  for (const fixture of fixtures) {
    if (
      fixture.viewport.width !== viewport.width ||
      fixture.viewport.height !== viewport.height ||
      fixture.scale !== scale
    ) {
      throw new Error(
        `fixture ${fixture.name} has viewport ${fixture.viewport.width}x${fixture.viewport.height} @${fixture.scale}; ` +
          `the batch shares one browser context at ${viewport.width}x${viewport.height} @${scale}`,
      );
    }
  }

  let servers: PreviewServers | null = null;
  let browser: Browser | null = null;
  try {
    servers = await startPreviews();
    browser = await chromium.launch();
    const context = await browser.newContext({
      deviceScaleFactor: scale,
      viewport: { width: viewport.width, height: viewport.height },
      reducedMotion: "reduce",
      colorScheme: "dark",
    });

    // A page degrades after a dozen or so SPA navigations (vite client state
    // accumulates) until navigations stop settling — the same failure the
    // g12.009 gate recycles pages for. Recycle on a fixed cadence, and on a
    // PRE-CAPTURE transport failure restart a dead preview and retry once on
    // a young page. Determinism, receipt, and host-rejection failures are
    // evidence failures: they rethrow immediately and stop the batch. This is
    // infrastructure recovery, never frame picking — the byte-identity rule
    // inside captureFixtureRuntime can never reach this branch.
    const RECYCLE_AFTER = 12;
    let page = await context.newPage();
    // Fixed clock + seeded Math.random; the freeze stylesheet lands per
    // navigation inside captureOnce.
    await pinPage(page);
    let capturesOnPage = 0;

    const recycle = async () => {
      await page.close();
      page = await context.newPage();
      await pinPage(page);
      capturesOnPage = 0;
    };

    const records: WebCaptureRecord[] = [];
    for (const fixture of fixtures) {
      for (const runtime of WEB_RUNTIMES) {
        if (capturesOnPage >= RECYCLE_AFTER) await recycle();
        capturesOnPage += 1;
        try {
          records.push(
            await captureFixtureRuntime(page, browser, servers.urls[runtime], fixture, runtime, outDir),
          );
        } catch (error) {
          if (!isRecoverableTransportError(error)) throw error;
          if (servers && (await ensureUp(runtime))) {
            console.log(`  restarted ${runtime} preview after capture failure`);
          }
          await recycle();
          records.push(
            await captureFixtureRuntime(page, browser, servers.urls[runtime], fixture, runtime, outDir),
          );
          console.log(`  recovered ${fixture.name} [${runtime}] after: ${(error as Error).message.split("\n")[0]}`);
        }
      }
    }
    return records;
  } finally {
    if (browser) await browser.close();
    if (servers) await servers.stop();
  }
}
