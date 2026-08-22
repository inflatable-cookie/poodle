/**
 * g15.046 — Button visual fixture inventory: TypeScript loader and validator.
 *
 * This module parses exactly one checked-in file,
 * `button-visual-inventory.json`, and nothing else. Its Rust sibling
 * (`packages/gpui/preview/tests/visual_fixture_inventory.rs`) parses the same
 * bytes. Neither is generated from the other.
 *
 * What this is: a frozen list of named Button visual cases with every input
 * resolved on every row. A fixture identity is a *name for a case*, so that
 * Svelte, React, and GPUI can later be asked for the same case by the same
 * name.
 *
 * What this is not, and must not become: a component API schema, a props
 * registry, a scene, a render tree, an action language, or a place to record
 * expected renderer output. It carries no bounds, colors, hashes, or
 * thresholds — those belong to the g15.047 comparator.
 *
 * Test tooling only. No published Poodle package imports this file.
 */

import { readFileSync } from "node:fs";

// Imported by workspace-relative path, not by package specifier: this suite
// runs under `bun test` at the repo root, where the workspace packages are not
// linked into node_modules.
import {
  controlSizes,
  densityModes,
  themes,
} from "../../../packages/core/src/tokens/index.ts";

import defaultIcons from "../../../packages/core/src/icons/default-icons.json" with { type: "json" };

/**
 * Versioned discriminator. Local to the Button inventory by design: a generic
 * component-name registry is a stop condition for this lane, not a feature.
 */
export const INVENTORY_SCHEMA = "poodle.button-visual-inventory.v1";

/**
 * The frozen first batch. This roster is authority for the denominator: the
 * canonical file must contain these names, all of them, once each, and no
 * others. The Rust loader holds the same 18 strings — one duplicated registry,
 * recorded honestly rather than generated away.
 */
export const BUTTON_FIXTURE_NAMES = [
  "button/rest-secondary",
  "button/variant-primary",
  "button/variant-ghost",
  "button/tone-danger",
  "button/tone-success",
  "button/tone-warning",
  "button/size-xs",
  "button/size-sm",
  "button/size-lg",
  "button/size-xl",
  "button/density-compact",
  "button/density-comfortable",
  "button/state-disabled",
  "button/state-loading",
  "button/state-pressed",
  "button/content-leading-icon",
  "button/content-icon-only",
  "button/theme-iceberg",
] as const;

/**
 * Renderer-neutral geometry landmarks for later receipts. Names only: this
 * file never says where a landmark is or how big it should be.
 */
export const LANDMARKS = ["root", "content", "icon", "spinner"] as const;

/**
 * Semantic roles a later comparator may report beside pixels. Again names
 * only — no resolved token values, no expected colors.
 */
export const REPORT_ROLES = ["fill", "border", "text", "shadow", "focus-ring"] as const;

/**
 * `g15.045` measured the adopted GPUI revision's headless window at a
 * hardcoded 2x scale factor. A fixture at any other scale cannot be captured
 * in every runtime, so it is rejected here rather than silently approximated
 * later.
 */
export const SUPPORTED_CAPTURE_SCALES = [2] as const;

/**
 * Portable Button contract domain. HTML form-only props never enter a fixture,
 * and the legacy Rust `ButtonVariant::Danger` compatibility arm is not one of
 * the contract's three variants.
 */
export const VARIANTS = ["primary", "secondary", "ghost"] as const;
export const TONES = ["default", "danger", "success", "warning"] as const;

/**
 * Visual states name rendering inputs that are already true of the frame, not
 * the interactions that produce them. `hover`, `active`, and `focus` are
 * therefore absent: they are driver work for a later card, not fixture data.
 */
export const STATES = ["rest", "disabled", "loading", "pressed"] as const;

export const CONTENT_KINDS = ["label", "leading-icon", "icon-only"] as const;

/**
 * Sentinel strings that would mean "some runtime resolves this later". The
 * inventory's whole point is that nothing is left to a runtime default.
 */
const UNRESOLVED_MARKERS = new Set(["inherit", "default-value", "__default__", ""]);

const THEME_IDS: readonly string[] = Object.keys(themes);
const SIZE_IDS: readonly string[] = Object.keys(controlSizes);
const DENSITY_IDS: readonly string[] = Object.keys(densityModes);
const ICON_IDS: readonly string[] = (defaultIcons as { icons: string[] }).icons;

export type ButtonFixtureName = (typeof BUTTON_FIXTURE_NAMES)[number];
export type Landmark = (typeof LANDMARKS)[number];
export type ReportRole = (typeof REPORT_ROLES)[number];
export type ButtonVariantName = (typeof VARIANTS)[number];
export type ButtonToneName = (typeof TONES)[number];
export type VisualStateName = (typeof STATES)[number];
export type ContentKind = (typeof CONTENT_KINDS)[number];

export type Viewport = { width: number; height: number };

export type FixtureContent =
  | { kind: "label"; label: string }
  | { kind: "leading-icon"; label: string; icon: string }
  | { kind: "icon-only"; icon: string; ariaLabel: string };

export type ButtonFixture = {
  name: ButtonFixtureName;
  group: string;
  theme: string;
  size: string;
  density: string;
  viewport: Viewport;
  scale: number;
  variant: ButtonVariantName;
  tone: ButtonToneName;
  content: FixtureContent;
  state: VisualStateName;
  landmarks: Landmark[];
};

export type ButtonVisualInventory = {
  schema: typeof INVENTORY_SCHEMA;
  component: "button";
  batch: string;
  captureScales: number[];
  reportRoles: ReportRole[];
  fixtures: ButtonFixture[];
};

/** Every failure the parse found, each naming its exact offender. */
export class ButtonInventoryError extends Error {
  readonly problems: string[];

  constructor(problems: string[]) {
    super(`button visual inventory is invalid:\n  - ${problems.join("\n  - ")}`);
    this.name = "ButtonInventoryError";
    this.problems = problems;
  }
}

const ROOT_KEYS = ["schema", "component", "batch", "captureScales", "reportRoles", "fixtures"];
const FIXTURE_KEYS = [
  "name",
  "group",
  "theme",
  "size",
  "density",
  "viewport",
  "scale",
  "variant",
  "tone",
  "content",
  "state",
  "landmarks",
];
const CONTENT_KEYS: Record<ContentKind, string[]> = {
  label: ["kind", "label"],
  "leading-icon": ["kind", "label", "icon"],
  "icon-only": ["kind", "icon", "ariaLabel"],
};

function isPlainObject(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function keyDiff(actual: string[], expected: string[]): { missing: string[]; extra: string[] } {
  const expectedSet = new Set(expected);
  const actualSet = new Set(actual);
  return {
    missing: expected.filter((key) => !actualSet.has(key)),
    extra: actual.filter((key) => !expectedSet.has(key)),
  };
}

/**
 * Compare a declared array against an expected one element by element.
 *
 * Deliberately not `join()` and not a filter: joining accepts a collapsed
 * element (`["root content"]` reading as `["root", "content"]`), and filtering
 * non-strings out would silently accept an inserted number or null. Either
 * would let this loader and its Rust sibling disagree about the same bytes,
 * which is the one thing a shared fixture parser must not do.
 *
 * Returns a problem string, or `null` when the array matches exactly.
 */
function exactStringArrayProblem(
  label: string,
  value: unknown,
  expected: readonly string[],
): string | null {
  if (!Array.isArray(value)) {
    return `${label} must be an array of strings, got ${JSON.stringify(value)}`;
  }
  if (value.length !== expected.length) {
    return `${label} must be exactly [${expected.join(", ")}] (${expected.length} entries), got ${JSON.stringify(value)}`;
  }
  for (const [index, entry] of value.entries()) {
    if (typeof entry !== "string") {
      return `${label} entry ${index} must be the string '${expected[index]}', got ${JSON.stringify(entry)}`;
    }
    if (entry !== expected[index]) {
      return `${label} entry ${index} must be '${expected[index]}', got ${JSON.stringify(entry)}`;
    }
  }
  return null;
}

/**
 * The landmark set is derived, not free text: a later receipt can only ask for
 * a landmark the case actually renders. `icon` exists when the content shape
 * carries one; `spinner` exists when the state is `loading`.
 */
export function expectedLandmarks(content: FixtureContent, state: VisualStateName): Landmark[] {
  const landmarks: Landmark[] = ["root", "content"];
  if (content.kind === "leading-icon" || content.kind === "icon-only") landmarks.push("icon");
  if (state === "loading") landmarks.push("spinner");
  return landmarks;
}

type Checker = {
  problems: string[];
  fail(message: string): void;
};

function requireResolvedString(
  check: Checker,
  where: string,
  field: string,
  value: unknown,
  domain: readonly string[],
): string | null {
  if (value === undefined) {
    check.fail(`${where}: missing required field '${field}'`);
    return null;
  }
  if (value === null) {
    check.fail(`${where}: field '${field}' is null; every value must be resolved in the file`);
    return null;
  }
  if (typeof value !== "string") {
    check.fail(`${where}: field '${field}' must be a string, got ${JSON.stringify(value)}`);
    return null;
  }
  if (UNRESOLVED_MARKERS.has(value)) {
    check.fail(
      `${where}: field '${field}' is the unresolved-default marker '${value}'; no runtime may supply it`,
    );
    return null;
  }
  if (!domain.includes(value)) {
    check.fail(
      `${where}: field '${field}' value '${value}' is outside the domain [${domain.join(", ")}]`,
    );
    return null;
  }
  return value;
}

function checkViewport(check: Checker, where: string, value: unknown): void {
  if (value === undefined || value === null) {
    check.fail(`${where}: missing required field 'viewport'`);
    return;
  }
  if (!isPlainObject(value)) {
    check.fail(`${where}: field 'viewport' must be an object, got ${JSON.stringify(value)}`);
    return;
  }
  const { missing, extra } = keyDiff(Object.keys(value), ["width", "height"]);
  for (const key of missing) check.fail(`${where}: viewport is missing '${key}'`);
  for (const key of extra) check.fail(`${where}: viewport has unknown key '${key}'`);
  for (const key of ["width", "height"] as const) {
    const side = value[key];
    if (side === undefined) continue;
    if (typeof side !== "number" || !Number.isInteger(side) || side <= 0) {
      check.fail(
        `${where}: viewport.${key} must be a positive whole number of logical pixels, got ${JSON.stringify(side)}`,
      );
    }
  }
}

function checkContent(check: Checker, where: string, value: unknown): FixtureContent | null {
  if (value === undefined || value === null) {
    check.fail(`${where}: missing required field 'content'`);
    return null;
  }
  if (!isPlainObject(value)) {
    check.fail(`${where}: field 'content' must be an object, got ${JSON.stringify(value)}`);
    return null;
  }
  const kind = requireResolvedString(check, where, "content.kind", value.kind, CONTENT_KINDS);
  if (kind === null) return null;

  const { missing, extra } = keyDiff(Object.keys(value), CONTENT_KEYS[kind as ContentKind]);
  for (const key of missing) check.fail(`${where}: content '${kind}' is missing '${key}'`);
  for (const key of extra) check.fail(`${where}: content '${kind}' has unknown key '${key}'`);
  if (missing.length > 0 || extra.length > 0) return null;

  for (const key of ["label", "ariaLabel"] as const) {
    if (!(key in value)) continue;
    const text = value[key];
    if (typeof text !== "string" || text.length === 0) {
      check.fail(
        `${where}: content.${key} must be a non-empty string, got ${JSON.stringify(text)}`,
      );
      return null;
    }
  }
  if ("icon" in value) {
    const icon = value.icon;
    if (typeof icon !== "string" || !ICON_IDS.includes(icon)) {
      check.fail(
        `${where}: content.icon '${String(icon)}' is not in the default icon registry (packages/core/src/icons/default-icons.json)`,
      );
      return null;
    }
  }
  return value as unknown as FixtureContent;
}

/**
 * Validate an already-decoded value. Every problem is collected so a planted
 * fault is reported by exact fixture name rather than aborting on the first
 * one.
 */
export function parseButtonVisualInventory(raw: unknown): ButtonVisualInventory {
  const problems: string[] = [];
  const check: Checker = { problems, fail: (message) => problems.push(message) };

  if (!isPlainObject(raw)) {
    throw new ButtonInventoryError([
      `inventory root must be an object, got ${JSON.stringify(raw)}`,
    ]);
  }

  const rootKeys = keyDiff(Object.keys(raw), ROOT_KEYS);
  for (const key of rootKeys.missing) check.fail(`inventory root is missing '${key}'`);
  for (const key of rootKeys.extra) check.fail(`inventory root has unknown key '${key}'`);

  if (raw.schema !== INVENTORY_SCHEMA) {
    check.fail(`inventory schema must be '${INVENTORY_SCHEMA}', got ${JSON.stringify(raw.schema)}`);
  }
  if (raw.component !== "button") {
    check.fail(
      `inventory component must be 'button' — this batch is Button-only, got ${JSON.stringify(raw.component)}`,
    );
  }
  if (typeof raw.batch !== "string" || raw.batch.length === 0) {
    check.fail(`inventory batch must be a non-empty string, got ${JSON.stringify(raw.batch)}`);
  }

  const captureScales = Array.isArray(raw.captureScales) ? raw.captureScales : [];
  if (!Array.isArray(raw.captureScales) || raw.captureScales.length === 0) {
    check.fail(
      `inventory captureScales must be a non-empty array, got ${JSON.stringify(raw.captureScales)}`,
    );
  }
  for (const scale of captureScales) {
    if (
      typeof scale !== "number" ||
      !(SUPPORTED_CAPTURE_SCALES as readonly number[]).includes(scale)
    ) {
      check.fail(
        `inventory captureScales entry ${JSON.stringify(scale)} is outside the supported set [${SUPPORTED_CAPTURE_SCALES.join(", ")}]`,
      );
    }
  }

  const rolesProblem = exactStringArrayProblem("inventory reportRoles", raw.reportRoles, REPORT_ROLES);
  if (rolesProblem !== null) check.fail(rolesProblem);

  if (!Array.isArray(raw.fixtures)) {
    problems.push(`inventory fixtures must be an array, got ${JSON.stringify(raw.fixtures)}`);
    throw new ButtonInventoryError(problems);
  }

  const roster = new Set<string>(BUTTON_FIXTURE_NAMES);
  const seen = new Set<string>();
  const fixtures: ButtonFixture[] = [];

  raw.fixtures.forEach((entry, index) => {
    if (!isPlainObject(entry)) {
      check.fail(`fixture at index ${index} must be an object, got ${JSON.stringify(entry)}`);
      return;
    }
    const rawName = entry.name;
    const where = typeof rawName === "string" ? `fixture '${rawName}'` : `fixture at index ${index}`;

    if (typeof rawName !== "string" || rawName.length === 0) {
      check.fail(`${where}: 'name' must be a non-empty string, got ${JSON.stringify(rawName)}`);
      return;
    }
    if (!roster.has(rawName)) {
      check.fail(
        `unknown fixture name '${rawName}': not one of the ${BUTTON_FIXTURE_NAMES.length} g15.046 identities`,
      );
    } else if (seen.has(rawName)) {
      check.fail(`duplicate fixture name '${rawName}'`);
    }
    seen.add(rawName);

    const keys = keyDiff(Object.keys(entry), FIXTURE_KEYS);
    for (const key of keys.missing) check.fail(`${where}: missing required field '${key}'`);
    for (const key of keys.extra) check.fail(`${where}: unknown field '${key}'`);

    if (typeof entry.group !== "string" || entry.group.length === 0) {
      check.fail(`${where}: 'group' must be a non-empty string, got ${JSON.stringify(entry.group)}`);
    }

    const theme = requireResolvedString(check, where, "theme", entry.theme, THEME_IDS);
    const size = requireResolvedString(check, where, "size", entry.size, SIZE_IDS);
    const density = requireResolvedString(check, where, "density", entry.density, DENSITY_IDS);
    const variant = requireResolvedString(check, where, "variant", entry.variant, VARIANTS);
    const tone = requireResolvedString(check, where, "tone", entry.tone, TONES);
    const state = requireResolvedString(check, where, "state", entry.state, STATES);
    checkViewport(check, where, entry.viewport);

    if (entry.scale === undefined || entry.scale === null) {
      check.fail(`${where}: missing required field 'scale'`);
    } else if (
      typeof entry.scale !== "number" ||
      !(captureScales as unknown[]).includes(entry.scale)
    ) {
      check.fail(
        `${where}: scale ${JSON.stringify(entry.scale)} is not one of the inventory captureScales [${captureScales.join(", ")}]`,
      );
    }

    const content = checkContent(check, where, entry.content);

    if (content !== null && state !== null) {
      const expected = expectedLandmarks(content, state as VisualStateName);
      const landmarkProblem = exactStringArrayProblem(
        `${where}: landmarks`,
        entry.landmarks,
        expected,
      );
      if (landmarkProblem !== null) {
        check.fail(`${landmarkProblem} — content '${content.kind}', state '${state}'`);
      }
    }

    if (theme && size && density && variant && tone && state && content) {
      fixtures.push(entry as unknown as ButtonFixture);
    }
  });

  for (const name of BUTTON_FIXTURE_NAMES) {
    if (!seen.has(name)) check.fail(`missing fixture name '${name}'`);
  }

  if (problems.length > 0) throw new ButtonInventoryError(problems);
  return raw as unknown as ButtonVisualInventory;
}

export const INVENTORY_PATH = new URL("./button-visual-inventory.json", import.meta.url);

/** Read and validate the one canonical file. */
export function loadButtonVisualInventory(): ButtonVisualInventory {
  return parseButtonVisualInventory(JSON.parse(readFileSync(INVENTORY_PATH, "utf8")));
}
