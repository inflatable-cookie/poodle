/**
 * g15.047 — Button visual capture receipt: the typed evidence every runtime
 * emits beside its PNG, and the verifier that fails closed on a missing,
 * stale, aliased, or hash-mismatched pair.
 *
 * The receipt is Button-only and closed: it names one of the 18 accepted
 * fixtures, one of the three runtimes, the declared landmark bounds, and the
 * five fixed visual roles. It may not carry a component name, an arbitrary
 * landmark, an arbitrary role, or a free-form property bag.
 *
 * Three producers write this schema (the Svelte/React capture harness and the
 * Rust `poodle-window-capture --fixture` target); this verifier is the one
 * reader. Cross-language duplication of the key sets is recorded in the
 * g15.047 log, the same posture as the inventory loaders.
 */

import { createHash } from "node:crypto";

import { PNG } from "pngjs";

import {
  BUTTON_FIXTURE_NAMES,
  LANDMARKS,
  type ButtonFixture,
  type Landmark,
} from "../fixtures/button-visual-inventory.ts";

// v2 (g16.005): the GPUI environment changed shape when the fork-only
// offscreen readback was replaced by a real non-activating window. It now
// names a published crate rather than a Git revision, says what the transport
// actually is, and carries the run's own frontmost-application evidence.
export const RECEIPT_SCHEMA = "poodle.button-visual-capture.v2";

export const RUNTIMES = ["svelte", "react", "gpui"] as const;
export type RuntimeName = (typeof RUNTIMES)[number];

/** Non-premultiplied sRGB channels, normalized to 0..1. */
export type Srgb = [number, number, number, number];

/** Logical-pixel bounds of one declared landmark, viewport-relative. */
export type LandmarkBounds = { x: number; y: number; width: number; height: number };

export type ShadowLayer = {
  inset: boolean;
  offsetX: number;
  offsetY: number;
  blur: number;
  spread: number;
  color: Srgb;
};

export type RoleEvidence = {
  fill: { color: Srgb };
  border: { color: Srgb; width: number };
  text: { color: Srgb };
  shadow: { layers: ShadowLayer[] };
  /**
   * No fixture in this batch captures a focused frame, so the focus ring is
   * dormant declared evidence: the values the runtime would paint on keyboard
   * focus. `color`/`width` are `null` when the runtime declares no distinct
   * focus-ring channel — an honest absence, not a pass.
   */
  "focus-ring": { color: Srgb | null; width: number | null; status: "dormant" };
};

export type WebEnvironment = { kind: "chromium"; version: string };
/**
 * What the capture process observed about the frontmost application for the
 * whole of its own run. `changed: true` is a contract violation, not a
 * warning: the capture binary refuses to publish in that case, so a receipt
 * that carries it should not exist.
 */
export type ForegroundEvidence = {
  baseline: string | null;
  observed: string[];
  samples: number;
  changed: boolean;
};

export type GpuiEnvironment = {
  kind: "macos-window-server-nonactivating";
  os: string;
  arch: string;
  gpuiSource: "crates.io";
  gpuiVersion: string;
  foreground: ForegroundEvidence;
};
export type CaptureEnvironment = WebEnvironment | GpuiEnvironment;

export type ButtonCaptureReceipt = {
  schema: typeof RECEIPT_SCHEMA;
  fixture: (typeof BUTTON_FIXTURE_NAMES)[number];
  runtime: RuntimeName;
  logicalViewport: { width: number; height: number };
  scale: number;
  deviceDimensions: { width: number; height: number };
  pngSha256: string;
  environment: CaptureEnvironment;
  landmarks: Partial<Record<Landmark, LandmarkBounds>>;
  roles: RoleEvidence;
};

export class ReceiptError extends Error {
  readonly problems: string[];

  constructor(problems: string[]) {
    super(`button capture receipt is invalid:\n  - ${problems.join("\n  - ")}`);
    this.name = "ReceiptError";
    this.problems = problems;
  }
}

const RECEIPT_KEYS = [
  "schema",
  "fixture",
  "runtime",
  "logicalViewport",
  "scale",
  "deviceDimensions",
  "pngSha256",
  "environment",
  "landmarks",
  "roles",
] as const;

const BOUNDS_KEYS = ["x", "y", "width", "height"] as const;
const ROLE_KEYS = ["fill", "border", "text", "shadow", "focus-ring"] as const;
const SHADOW_LAYER_KEYS = ["inset", "offsetX", "offsetY", "blur", "spread", "color"] as const;
const WEB_ENV_KEYS = ["kind", "version"] as const;
const GPUI_ENV_KEYS = ["kind", "os", "arch", "gpuiSource", "gpuiVersion", "foreground"] as const;
const FOREGROUND_KEYS = ["baseline", "observed", "samples", "changed"] as const;
export const GPUI_TRANSPORT = "macos-window-server-nonactivating";

function isPlainObject(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function keyProblems(
  problems: string[],
  where: string,
  value: Record<string, unknown>,
  expected: readonly string[],
): void {
  const actual = Object.keys(value);
  for (const key of expected) {
    if (!actual.includes(key)) problems.push(`${where} is missing '${key}'`);
  }
  for (const key of actual) {
    if (!expected.includes(key)) problems.push(`${where} has unknown key '${key}'`);
  }
}

function isFiniteNumber(value: unknown): value is number {
  return typeof value === "number" && Number.isFinite(value);
}

function checkSrgb(problems: string[], where: string, value: unknown): void {
  if (!Array.isArray(value) || value.length !== 4) {
    problems.push(`${where} must be a 4-channel normalized sRGB array, got ${JSON.stringify(value)}`);
    return;
  }
  for (const [index, channel] of value.entries()) {
    if (!isFiniteNumber(channel) || channel < 0 || channel > 1) {
      problems.push(
        `${where} channel ${index} must be a finite number in 0..1, got ${JSON.stringify(channel)}`,
      );
    }
  }
}

function checkBounds(problems: string[], where: string, value: unknown): void {
  if (!isPlainObject(value)) {
    problems.push(`${where} must be an object, got ${JSON.stringify(value)}`);
    return;
  }
  keyProblems(problems, where, value, BOUNDS_KEYS);
  for (const key of BOUNDS_KEYS) {
    const side = value[key];
    if (side === undefined) continue;
    if (!isFiniteNumber(side)) {
      problems.push(`${where}.${key} must be a finite number, got ${JSON.stringify(side)}`);
    }
  }
  if (isFiniteNumber(value.width) && value.width <= 0) {
    problems.push(`${where}.width must be positive, got ${value.width}`);
  }
  if (isFiniteNumber(value.height) && value.height <= 0) {
    problems.push(`${where}.height must be positive, got ${value.height}`);
  }
}

function checkShadow(problems: string[], where: string, value: unknown): void {
  if (!isPlainObject(value)) {
    problems.push(`${where} must be an object, got ${JSON.stringify(value)}`);
    return;
  }
  keyProblems(problems, where, value, ["layers"]);
  if (!Array.isArray(value.layers)) {
    problems.push(`${where}.layers must be an array, got ${JSON.stringify(value.layers)}`);
    return;
  }
  for (const [index, layer] of value.layers.entries()) {
    const layerWhere = `${where}.layers[${index}]`;
    if (!isPlainObject(layer)) {
      problems.push(`${layerWhere} must be an object, got ${JSON.stringify(layer)}`);
      continue;
    }
    keyProblems(problems, layerWhere, layer, SHADOW_LAYER_KEYS);
    if (layer.inset !== undefined && typeof layer.inset !== "boolean") {
      problems.push(`${layerWhere}.inset must be a boolean, got ${JSON.stringify(layer.inset)}`);
    }
    for (const key of ["offsetX", "offsetY", "blur", "spread"] as const) {
      if (layer[key] !== undefined && !isFiniteNumber(layer[key])) {
        problems.push(`${layerWhere}.${key} must be a finite number, got ${JSON.stringify(layer[key])}`);
      }
    }
    if (layer.color !== undefined) checkSrgb(problems, `${layerWhere}.color`, layer.color);
  }
}

function checkRoles(problems: string[], value: unknown): void {
  const where = "receipt roles";
  if (!isPlainObject(value)) {
    problems.push(`${where} must be an object, got ${JSON.stringify(value)}`);
    return;
  }
  keyProblems(problems, where, value, ROLE_KEYS);

  if (isPlainObject(value.fill)) {
    keyProblems(problems, `${where} fill`, value.fill, ["color"]);
    checkSrgb(problems, `${where} fill.color`, value.fill.color);
  } else if (value.fill !== undefined) {
    problems.push(`${where} fill must be an object`);
  }

  if (isPlainObject(value.border)) {
    keyProblems(problems, `${where} border`, value.border, ["color", "width"]);
    checkSrgb(problems, `${where} border.color`, value.border.color);
    if (!isFiniteNumber(value.border.width) || value.border.width < 0) {
      problems.push(`${where} border.width must be a finite non-negative number`);
    }
  } else if (value.border !== undefined) {
    problems.push(`${where} border must be an object`);
  }

  if (isPlainObject(value.text)) {
    keyProblems(problems, `${where} text`, value.text, ["color"]);
    checkSrgb(problems, `${where} text.color`, value.text.color);
  } else if (value.text !== undefined) {
    problems.push(`${where} text must be an object`);
  }

  if (value.shadow !== undefined) checkShadow(problems, `${where} shadow`, value.shadow);

  const ring = value["focus-ring"];
  if (isPlainObject(ring)) {
    keyProblems(problems, `${where} focus-ring`, ring, ["color", "width", "status"]);
    if (ring.status !== "dormant") {
      problems.push(
        `${where} focus-ring.status must be 'dormant' — no fixture in this batch captures focus, got ${JSON.stringify(ring.status)}`,
      );
    }
    if (ring.color !== null) checkSrgb(problems, `${where} focus-ring.color`, ring.color);
    if (ring.width !== null && (!isFiniteNumber(ring.width) || ring.width < 0)) {
      problems.push(`${where} focus-ring.width must be null or a finite non-negative number`);
    }
  } else if (ring !== undefined) {
    problems.push(`${where} focus-ring must be an object`);
  }
}

function checkForeground(problems: string[], where: string, value: unknown): void {
  if (!isPlainObject(value)) {
    problems.push(`${where} must be an object, got ${JSON.stringify(value)}`);
    return;
  }
  keyProblems(problems, where, value, FOREGROUND_KEYS);
  if (value.baseline !== null && typeof value.baseline !== "string") {
    problems.push(`${where}.baseline must be a string or null`);
  }
  if (!Array.isArray(value.observed) || value.observed.some((app) => typeof app !== "string")) {
    problems.push(`${where}.observed must be an array of strings`);
  }
  if (typeof value.samples !== "number" || !Number.isInteger(value.samples) || value.samples < 1) {
    problems.push(`${where}.samples must be a positive integer`);
  }
  if (value.changed !== false) {
    problems.push(
      `${where}.changed must be false — a capture that changed the frontmost application is not evidence`,
    );
  }
}

function checkEnvironment(problems: string[], runtime: RuntimeName, value: unknown): void {
  const where = "receipt environment";
  if (!isPlainObject(value)) {
    problems.push(`${where} must be an object, got ${JSON.stringify(value)}`);
    return;
  }
  if (runtime === "gpui") {
    keyProblems(problems, where, value, GPUI_ENV_KEYS);
    if (value.kind !== GPUI_TRANSPORT) {
      problems.push(`${where}.kind must be '${GPUI_TRANSPORT}' for gpui, got ${JSON.stringify(value.kind)}`);
    }
    if (typeof value.os !== "string" || value.os.length === 0) {
      problems.push(`${where}.os must be a non-empty string`);
    }
    if (typeof value.arch !== "string" || value.arch.length === 0) {
      problems.push(`${where}.arch must be a non-empty string`);
    }
    // The public source boundary, asserted rather than assumed: a receipt
    // produced against a forked GPUI is not evidence about what consumers get.
    if (value.gpuiSource !== "crates.io") {
      problems.push(`${where}.gpuiSource must be 'crates.io', got ${JSON.stringify(value.gpuiSource)}`);
    }
    if (typeof value.gpuiVersion !== "string" || !/^\d+\.\d+\.\d+$/.test(value.gpuiVersion)) {
      problems.push(`${where}.gpuiVersion must be a published semver version`);
    }
    checkForeground(problems, `${where}.foreground`, value.foreground);
  } else {
    keyProblems(problems, where, value, WEB_ENV_KEYS);
    if (value.kind !== "chromium") {
      problems.push(`${where}.kind must be 'chromium' for ${runtime}, got ${JSON.stringify(value.kind)}`);
    }
    if (typeof value.version !== "string" || value.version.length === 0) {
      problems.push(`${where}.version must be a non-empty string`);
    }
  }
  // No hostname, username, or absolute path may ride in the environment.
  for (const field of Object.values(value)) {
    if (typeof field === "string" && (field.startsWith("/") || field.includes("\\Users\\"))) {
      problems.push(`${where} must not contain absolute paths or user identifiers`);
    }
  }
}

/**
 * Validate a decoded receipt against the fixture it claims to capture. The
 * landmark set must be exactly the fixture's declared set — a missing icon or
 * spinner landmark is as much a failure as an extra one.
 */
export function parseButtonCaptureReceipt(
  raw: unknown,
  expected: { fixture: ButtonFixture; runtime: RuntimeName },
): ButtonCaptureReceipt {
  const problems: string[] = [];
  const where = `receipt for ${expected.fixture.name} [${expected.runtime}]`;

  if (!isPlainObject(raw)) {
    throw new ReceiptError([`${where} must be an object, got ${JSON.stringify(raw)}`]);
  }
  keyProblems(problems, where, raw, RECEIPT_KEYS);

  if (raw.schema !== RECEIPT_SCHEMA) {
    problems.push(`${where}: schema must be '${RECEIPT_SCHEMA}', got ${JSON.stringify(raw.schema)}`);
  }
  if (raw.fixture !== expected.fixture.name) {
    problems.push(
      `${where}: fixture must be '${expected.fixture.name}', got ${JSON.stringify(raw.fixture)}`,
    );
  }
  if (raw.runtime !== expected.runtime) {
    problems.push(
      `${where}: runtime must be '${expected.runtime}', got ${JSON.stringify(raw.runtime)}`,
    );
  }

  if (isPlainObject(raw.logicalViewport)) {
    keyProblems(problems, `${where} logicalViewport`, raw.logicalViewport, ["width", "height"]);
    if (raw.logicalViewport.width !== expected.fixture.viewport.width) {
      problems.push(
        `${where}: logicalViewport.width must be ${expected.fixture.viewport.width}, got ${JSON.stringify(raw.logicalViewport.width)}`,
      );
    }
    if (raw.logicalViewport.height !== expected.fixture.viewport.height) {
      problems.push(
        `${where}: logicalViewport.height must be ${expected.fixture.viewport.height}, got ${JSON.stringify(raw.logicalViewport.height)}`,
      );
    }
  } else if (raw.logicalViewport !== undefined) {
    problems.push(`${where}: logicalViewport must be an object`);
  }

  if (raw.scale !== expected.fixture.scale) {
    problems.push(`${where}: scale must be ${expected.fixture.scale}, got ${JSON.stringify(raw.scale)}`);
  }

  if (isPlainObject(raw.deviceDimensions)) {
    keyProblems(problems, `${where} deviceDimensions`, raw.deviceDimensions, ["width", "height"]);
    const expectedWidth = expected.fixture.viewport.width * expected.fixture.scale;
    const expectedHeight = expected.fixture.viewport.height * expected.fixture.scale;
    if (raw.deviceDimensions.width !== expectedWidth) {
      problems.push(
        `${where}: deviceDimensions.width must be ${expectedWidth} (viewport × scale), got ${JSON.stringify(raw.deviceDimensions.width)}`,
      );
    }
    if (raw.deviceDimensions.height !== expectedHeight) {
      problems.push(
        `${where}: deviceDimensions.height must be ${expectedHeight} (viewport × scale), got ${JSON.stringify(raw.deviceDimensions.height)}`,
      );
    }
  } else if (raw.deviceDimensions !== undefined) {
    problems.push(`${where}: deviceDimensions must be an object`);
  }

  if (typeof raw.pngSha256 !== "string" || !/^[0-9a-f]{64}$/.test(raw.pngSha256)) {
    problems.push(`${where}: pngSha256 must be a 64-character lowercase hex digest`);
  }

  if (raw.environment !== undefined && (raw.runtime === "svelte" || raw.runtime === "react" || raw.runtime === "gpui")) {
    checkEnvironment(problems, raw.runtime as RuntimeName, raw.environment);
  }

  if (isPlainObject(raw.landmarks)) {
    keyProblems(problems, `${where} landmarks`, raw.landmarks, expected.fixture.landmarks);
    for (const landmark of LANDMARKS) {
      const bounds = raw.landmarks[landmark];
      if (bounds !== undefined) checkBounds(problems, `${where} landmarks.${landmark}`, bounds);
    }
  } else if (raw.landmarks !== undefined) {
    problems.push(`${where}: landmarks must be an object`);
  }

  if (raw.roles !== undefined) checkRoles(problems, raw.roles);

  if (problems.length > 0) throw new ReceiptError(problems);
  return raw as unknown as ButtonCaptureReceipt;
}

export function sha256Hex(bytes: Buffer): string {
  return createHash("sha256").update(bytes).digest("hex");
}

/**
 * Verify a receipt against the PNG bytes it claims to describe: the digest
 * must match and the PNG's real dimensions must equal the declared device
 * dimensions. A tampered, truncated, or swapped PNG fails here before any
 * pair comparison sees it.
 */
export function verifyReceiptAgainstPng(receipt: ButtonCaptureReceipt, png: Buffer): string[] {
  const problems: string[] = [];
  const digest = sha256Hex(png);
  if (digest !== receipt.pngSha256) {
    problems.push(
      `PNG SHA-256 ${digest} does not match receipt ${receipt.pngSha256} (${receipt.fixture} [${receipt.runtime}])`,
    );
  }
  let decoded: PNG;
  try {
    decoded = PNG.sync.read(png);
  } catch (error) {
    problems.push(`PNG does not decode: ${(error as Error).message}`);
    return problems;
  }
  if (decoded.width !== receipt.deviceDimensions.width || decoded.height !== receipt.deviceDimensions.height) {
    problems.push(
      `PNG is ${decoded.width}x${decoded.height}, receipt declares ${receipt.deviceDimensions.width}x${receipt.deviceDimensions.height} (${receipt.fixture} [${receipt.runtime}])`,
    );
  }
  return problems;
}
