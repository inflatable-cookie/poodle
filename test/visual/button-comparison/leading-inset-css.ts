/**
 * g16.106 — CSS-side Button leading-inset inventory.
 *
 * happy-dom cannot cascade stylesheets at computed-value time, so this is
 * the headless stand-in for "the CSS values the Svelte fixture computes":
 * it reads the shipped `button.css`, the density custom-property files the
 * fixture host applies, and the frozen visual-fixture rows. It does not
 * re-state rem literals of its own.
 *
 * Test tooling only. The committed JSON sibling is the artifact the Rust
 * node inventory consumes.
 */

import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

import {
  loadButtonVisualInventory,
  type ButtonFixture,
} from "../fixtures/button-visual-inventory.ts";

const here = dirname(fileURLToPath(import.meta.url));
const repoRoot = join(here, "../../..");

export const LEADING_INSET_CSS_SCHEMA = "poodle.button-leading-inset-css.v1";
export const REM_BASE_PX = 16;

/** The two lab fixtures plus the no-leading negative. */
export const LEADING_INSET_CSS_FIXTURES = [
  "button/content-leading-icon",
  "button/state-loading",
  "button/rest-secondary",
] as const;

export type LeadingInsetCssFixture = (typeof LEADING_INSET_CSS_FIXTURES)[number];

export type LeadingInsetCssRow = {
  fixture: LeadingInsetCssFixture;
  size: string;
  density: string;
  hasLeading: boolean;
  padX: number;
  padLeft: number;
  padRight: number;
  inset: number;
};

export type LeadingInsetCssInventory = {
  schema: typeof LEADING_INSET_CSS_SCHEMA;
  remBasePx: number;
  sources: {
    buttonCss: string;
    densityCss: string;
    inventory: string;
  };
  fixtures: LeadingInsetCssRow[];
};

const BUTTON_CSS_PATH = "packages/core/src/styles/button.css";
const DENSITY_CSS_DIR = "packages/core/src/tokens/generated/css";
const INVENTORY_PATH = "test/visual/fixtures/button-visual-inventory.json";

function readRepo(relative: string): string {
  return readFileSync(join(repoRoot, relative), "utf8");
}

function remToPx(rem: number): number {
  return rem * REM_BASE_PX;
}

function parseRem(value: string): number {
  const match = value.trim().match(/^(-?[\d.]+)rem$/);
  if (!match) {
    throw new Error(`expected a rem length, got '${value}'`);
  }
  return Number(match[1]);
}

function cssCustomProperty(css: string, name: string): string {
  const pattern = new RegExp(`${name.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}\\s*:\\s*([^;]+);`);
  const match = css.match(pattern);
  if (!match) {
    throw new Error(`missing custom property ${name}`);
  }
  return match[1].trim();
}

function controlXPx(density: string): number {
  const relative = `${DENSITY_CSS_DIR}/poodle-density-${density}.css`;
  const css = readRepo(relative);
  return remToPx(parseRem(cssCustomProperty(css, "--poodle-space-control-x")));
}

function escapeSelector(selector: string): string {
  return selector.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

function ruleBody(css: string, selector: string): string | null {
  const match = css.match(new RegExp(`${escapeSelector(selector)}\\s*\\{([^}]+)\\}`));
  return match ? match[1] : null;
}

function declaration(body: string, property: string): string | null {
  const match = body.match(new RegExp(`${property}\\s*:\\s*([^;]+);`));
  return match ? match[1].trim() : null;
}

function evalLength(expr: string, controlX: number): number {
  const value = expr.trim();
  if (value === "var(--poodle-space-control-x)") return controlX;
  if (value === "0") return 0;
  const calc = value.match(
    /^calc\(\s*var\(--poodle-space-control-x\)\s*([+-])\s*(-?[\d.]+)rem\s*\)$/,
  );
  if (calc) {
    const delta = remToPx(Number(calc[2]));
    return calc[1] === "-" ? controlX - delta : controlX + delta;
  }
  throw new Error(`cannot evaluate CSS length '${expr}'`);
}

function evalHorizontalPadding(expr: string, controlX: number): { left: number; right: number } {
  const parts = expr.trim().split(/\s+/);
  if (parts.length === 1) {
    const px = evalLength(parts[0], controlX);
    return { left: px, right: px };
  }
  if (parts.length === 2 && parts[0] === "0") {
    const px = evalLength(parts[1], controlX);
    return { left: px, right: px };
  }
  throw new Error(`cannot evaluate CSS padding '${expr}'`);
}

function sizePadding(buttonCss: string, size: string, controlX: number): { left: number; right: number } {
  const sizeBody = ruleBody(buttonCss, `.poodle-button[data-size="${size}"]`);
  const rootBody = ruleBody(buttonCss, ".poodle-button");
  const body = sizeBody ?? rootBody;
  if (!body) throw new Error("missing .poodle-button padding rule");
  const padding = declaration(body, "padding");
  if (!padding) throw new Error(`missing padding on size '${size}'`);
  return evalHorizontalPadding(padding, controlX);
}

function leadingPaddingLeft(
  buttonCss: string,
  size: string,
  controlX: number,
): number {
  const specific = ruleBody(buttonCss, `.poodle-button[data-has-leading][data-size="${size}"]`);
  const generic = ruleBody(buttonCss, ".poodle-button[data-has-leading]");
  const body = specific ?? generic;
  if (!body) throw new Error("missing [data-has-leading] padding-left rule");
  const value = declaration(body, "padding-left");
  if (!value) throw new Error("missing padding-left on [data-has-leading]");
  return evalLength(value, controlX);
}

function fixtureHasLeading(fixture: ButtonFixture): boolean {
  return fixture.content.kind === "leading-icon" || fixture.state === "loading";
}

export function deriveLeadingInsetCssInventory(): LeadingInsetCssInventory {
  const buttonCss = readRepo(BUTTON_CSS_PATH);
  const inventory = loadButtonVisualInventory();
  const fixtures: LeadingInsetCssRow[] = LEADING_INSET_CSS_FIXTURES.map((name) => {
    const fixture = inventory.fixtures.find((row) => row.name === name);
    if (!fixture) throw new Error(`fixture '${name}' is not in the visual inventory`);
    const padX = controlXPx(fixture.density);
    const hasLeading = fixtureHasLeading(fixture);
    const base = sizePadding(buttonCss, fixture.size, padX);
    const padLeft = hasLeading ? leadingPaddingLeft(buttonCss, fixture.size, padX) : base.left;
    const padRight = base.right;
    return {
      fixture: name,
      size: fixture.size,
      density: fixture.density,
      hasLeading,
      padX,
      padLeft,
      padRight,
      inset: padX - padLeft,
    };
  });
  return {
    schema: LEADING_INSET_CSS_SCHEMA,
    remBasePx: REM_BASE_PX,
    sources: {
      buttonCss: BUTTON_CSS_PATH,
      densityCss: `${DENSITY_CSS_DIR}/poodle-density-*.css`,
      inventory: INVENTORY_PATH,
    },
    fixtures,
  };
}

export function loadCommittedLeadingInsetCssInventory(): LeadingInsetCssInventory {
  return JSON.parse(readFileSync(join(here, "leading-inset-css.json"), "utf8")) as LeadingInsetCssInventory;
}
