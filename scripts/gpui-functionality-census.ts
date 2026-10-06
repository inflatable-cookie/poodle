import { createHash } from "node:crypto";
import { execFileSync, execSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import { deriveLiveRoster, EXPECTED_MOUNTED_BEHAVIOUR_TESTS } from "./parity-evidence-ledger";
import { PORTABLE_ROUTE_COUNT, PUBLIC_COMPONENT_COUNT, ROSTER_WEB_ONLY_NAMES } from "./component-denominator";
import { deriveNucleusReceiptRows } from "./nucleus-parity-receipts";

const ROOT = path.resolve(import.meta.dir, "..");
const HEADLESS_TEST_FILE = "packages/gpui/preview/tests/headless_regressions.rs";
const GPUI_LOCKFILE = "packages/gpui/preview/Cargo.lock";
const NATIVE_SELECTOR = "effigy regressions:native";

/** The live release identity for GPUI receipts. The preview crate manifest is
 * the package authority; receipts must never embed a literal that outlives a
 * version bump. */
export const GPUI_PREVIEW_CARGO_MANIFEST = "packages/gpui/preview/Cargo.toml";
export const PREVIEW_PACKAGE = "poodle-gpui-preview";

const SEMVER_RE = /^\d+\.\d+\.\d+(?:[-+][0-9A-Za-z.-]+)?$/;

export const CENSUS_DIR = "docs/evidence/gpui";
export const MANIFEST_PATH = `${CENSUS_DIR}/capability-manifest.json`;
export const RECEIPT_DIR = `${CENSUS_DIR}/mounted-receipts`;
export const CENSUS_JSON_PATH = `${CENSUS_DIR}/gpui-functionality-census.json`;
export const CENSUS_MD_PATH = `${CENSUS_DIR}/gpui-functionality-census.md`;
export const GROUPS_PATH = `${CENSUS_DIR}/missing-capability-groups.json`;
export const EXECUTION_RECORD_PATH = `${CENSUS_DIR}/expected-test-execution.json`;
export const CROSS_RUNTIME_REPORT = "packages/gpui/cross-runtime-parity-report.json";

export const CENSUS_SCHEMA = "poodle.g18-gpui-functionality-census.v1";
export const RECEIPT_SCHEMA = "poodle.g18-gpui-mounted-receipt.v1";
export const EXECUTION_SCHEMA = "poodle.g18-expected-test-execution.v2";
export const MANIFEST_SCHEMA = "poodle.g18-capability-manifest.v1";

export const CENSUS_AXES = ["semantic", "events", "pointer", "keyboard_focus", "accessibility", "visual"] as const;
export type CensusAxis = (typeof CENSUS_AXES)[number];

export const ADMITTED_VIA = ["nucleus-m1", "nucleus-a1", "nucleus-v1", "expected-test"] as const;
export type AdmittedVia = (typeof ADMITTED_VIA)[number];

/// Phrasing that proves a writer is reaching for the A2 platform gate to erase
/// component-level obligations. A not-applicable reason matching this can never
/// be admitted: the upstream publication hold cannot make node semantics,
/// keyboard, focus, role, state, or label gaps disappear.
export const PLATFORM_LANGUAGE =
  /A2|platform tree|accesskit|accesskit_winit|gpui-apple|upstream|0\.2\.2|publication hold|assistive-technology/i;

/// Language that presents construction (or any partial evidence) as whole-catalogue
/// functional completion. Forbidden in every generated census claim.
const OVERCLAIM_LANGUAGE = [
  /fully functional/i,
  /all \d+ [a-z ]*(proven|proved|complete|passing|pass)\b/i,
  /construction (proves|proving|means|confirms) [a-z ]*functional/i,
  /\b\d+\/\d+\b[^.\n]*functional(?! completion)/i,
];

const ASSERT_RE = /\bassert(_eq|_ne)?!\s*\(/;
const PRODUCTION_RENDER_RE = /poodle_render::(?!color::)\w+|node_compat::\w+/;

/// Per-axis source signals for a headless regression body. Every signal must be
/// read as production-backend evidence: admission additionally requires the
/// production mount (run_headless + HeadlessDriver + production renderer), so a
/// renderer unit test or a direct handler call can never satisfy any axis.
/// Matchers must stay formatting-tolerant: rustfmt wraps method chains and
/// argument lists across lines, so a pattern may only rely on token sequences
/// rustfmt never splits (identifiers, `::` paths) or must bridge the wrap with
/// `\s*` / `[^;]`-style classes. `a11y\s*\.` exists because rustfmt split
/// `n.a11y.label` onto separate chain lines and the split once dropped a true
/// accessibility admission (poodle#072 census ruling, 2026-09-30).
export const AXIS_TEST_SIGNALS: Record<CensusAxis, RegExp[]> = {
  semantic: [PRODUCTION_RENDER_RE, ASSERT_RE],
  events: [/counting_handler/i, /payloads?\.\s*lock/i, /assert[^;]*(emit|payload|change|commit|callback)/i],
  pointer: [/pointer_activate|pointer_press|dispatch_pointer|mouse_|simulate_click|\.click\(/i],
  keyboard_focus: [/dispatch_key|focus_element|focus_state_for|focus_handle_for|roving|key_press|press_key|keyboard_/i],
  accessibility: [/a11y\s*\.|NodeToggled|NodeRole|\baria\b|accessible/i, /assert[^;]*announce|announcements\(\)|on_announce/i],
  visual: [/rem_to_px|resolve_color|resolve_space|resolve_opacity|resolve_radius|_geometry|Geometry|computed_rect|dimensions/i],
};

/** Axes whose test signals are present but whose claim the live component does
 * not yet earn. Each entry is a recorded refusal with its reason, never a
 * silent skip; remove the entry when the gap closes. */
export const WITHHELD_AXES: Record<string, Partial<Record<CensusAxis, string>>> = {
  CardToggleGroup: {
    // Planner ruling 2026-10-04 (brief v2): the contract's
    // `min(100%, max(min-width, track width))` clamp stays normative. GPUI
    // cannot express it with today's node vocabulary, so the visual claim is
    // withheld and the shared layout capability is planned as its own task.
    visual:
      "narrow-container clamp needs a shared min(100%, max(…)) layout capability",
  },
  ListGrid: {
    // Same clamp class as the CardToggleGroup ruling above: the contract's
    // `min(minItemWidth, 100%)` / `min(100%, max(…))` track floor stays
    // normative, and the static flex vocabulary cannot clamp the tile floor
    // to the container width, so a sub-floor host overflows where Svelte
    // collapses to one full-width column. Withheld pending the same shared
    // container-relative layout capability; the cap, gap, and token proofs
    // stay asserted in the retained test as regression value.
    visual:
      "narrow-container floor clamp needs a shared container-relative layout capability",
  },
};

const RECEIPT_TEXT_SIGNALS: Record<Exclude<CensusAxis, "semantic">, RegExp> = {
  events: /emit|callback|payload|change|commit|toggle|dismiss|select/i,
  pointer: /pointer|mouse|click|press/i,
  keyboard_focus: /keyboard|focus|tab|roving|enter|space|escape|blur/i,
  accessibility: /role|accessible name|aria|announce|checked|mixed|toggled|label/i,
  visual: /px\b|geometry|token|resolv|dimension|layout|bound/i,
};

export type ManifestNotApplicable = { axis: CensusAxis; reason: string; contractRef: string };
export type ManifestEntry = {
  component: string;
  portable: boolean;
  contract: string;
  substrate: string;
  required: CensusAxis[];
  notApplicable: ManifestNotApplicable[];
};

export type CensusAdmission = { axis: CensusAxis; via: AdmittedVia; ref: string };
export type CensusHold = { axis: CensusAxis; kind: "A2-platform-hold"; note: string; ref: string };
export type CensusRow = {
  component: string;
  portable: boolean;
  contract: string;
  substrate: string;
  required: CensusAxis[];
  admitted: CensusAdmission[];
  missing: CensusAxis[];
  holds: CensusHold[];
  receipts: string[];
  refusals: string[];
};

export type MissingGroup = {
  substrate: string;
  components: string[];
  missingTally: Record<CensusAxis, number>;
  contracts: string[];
};

export type CensusDoc = {
  schema: string;
  task: string;
  source_commit: string;
  denominator: { public: number; portable: number; notApplicable: string[] };
  axes: CensusAxis[];
  manifest: ManifestEntry[];
  rows: CensusRow[];
  summary: {
    admittedRows: number;
    fullyAdmittedRows: number;
    missingTally: Record<CensusAxis, number>;
    holdCount: number;
    refusalCount: number;
  };
  groups: MissingGroup[];
  crossRuntime: { constructionClaim: string; mountedScope: string; note: string };
};

/** The component itself is a static display surface; any interactions belong
 * to composed children. These contract-backed boundaries stay local so they
 * do not relax the census rules for other styled-only components. */
const STATIC_DISPLAY_NOT_APPLICABLE: Record<string, ManifestNotApplicable[]> = {
  Spacer: [
    {
      axis: "keyboard_focus",
      reason: "The contract defines Spacer as decorative layout scaffolding that is never focusable and has no keyboard behavior.",
      contractRef: "docs/contracts/components/spacer.md#6. Accessibility",
    },
  ],
  Skeleton: [
    {
      axis: "keyboard_focus",
      reason: "The contract defines Skeleton placeholders as decorative and never focusable, with no keyboard interaction.",
      contractRef: "docs/contracts/components/skeleton.md#6. Accessibility",
    },
  ],
  Avatar: [
    {
      axis: "events",
      reason: "Avatar is a styled-only display component with no component-owned events.",
      contractRef: "docs/contracts/components/avatar.md#Behavior Machine",
    },
    {
      axis: "pointer",
      reason: "Avatar renders identity content and has no pointer interaction or focusable child.",
      contractRef: "docs/contracts/components/avatar.md#1. Purpose",
    },
    {
      axis: "keyboard_focus",
      reason: "Avatar is styled-only and has no keyboard behavior or component-owned focus stop.",
      contractRef: "docs/contracts/components/avatar.md#Behavior Machine",
    },
  ],
  Icon: [
    {
      axis: "keyboard_focus",
      reason: "Icon is not focusable; it is visual content announced through its parent context.",
      contractRef: "docs/contracts/components/icon.md#6. Accessibility",
    },
  ],
  MetaBar: [
    {
      axis: "events",
      reason: "MetaBar lays out caller-owned children and dispatches no component-owned events.",
      contractRef: "docs/contracts/components/meta-bar.md#4. Behavior",
    },
    {
      axis: "pointer",
      reason: "MetaBar is a layout-only container; pointer interaction belongs to child content.",
      contractRef: "docs/contracts/components/meta-bar.md#6. Accessibility",
    },
    {
      axis: "keyboard_focus",
      reason: "MetaBar has no keyboard behavior or focus stop; interactive children keep their own focus targets.",
      contractRef: "docs/contracts/components/meta-bar.md#6. Accessibility",
    },
  ],
  MetaItem: [
    {
      axis: "events",
      reason: "MetaItem displays caller-owned value content and dispatches no component-owned events.",
      contractRef: "docs/contracts/components/meta-item.md#6. Accessibility",
    },
    {
      axis: "pointer",
      reason: "MetaItem is a display wrapper; pointer interaction belongs to its value content.",
      contractRef: "docs/contracts/components/meta-item.md#6. Accessibility",
    },
    {
      axis: "keyboard_focus",
      reason: "MetaItem has no keyboard behavior; an interactive value remains its own focus target.",
      contractRef: "docs/contracts/components/meta-item.md#9. Keyboard",
    },
  ],
  AudioMeter: [
    {
      axis: "events",
      reason: "The display emits no user events; hosts advance contexts with PUSH_FRAME.",
      contractRef: "docs/contracts/components/audio-meter.md#5. Events",
    },
    {
      axis: "pointer",
      reason: "AudioMeter is a temporal level display with no pointer interaction; clip reset is host-owned.",
      contractRef: "docs/contracts/components/audio-meter.md#6. Accessibility",
    },
    {
      axis: "keyboard_focus",
      reason: "The root exposes meter semantics and is not keyboard-focusable.",
      contractRef: "docs/contracts/components/audio-meter.md#6. Accessibility",
    },
  ],
  GainReductionMeter: [
    {
      axis: "events",
      reason: "Hosts own feed cadence and bindable context observation; the meter has no callbacks.",
      contractRef: "docs/contracts/components/gain-reduction-meter.md#5. Callbacks",
    },
    {
      axis: "pointer",
      reason: "GainReductionMeter is an inverted level display with no pointer interaction.",
      contractRef: "docs/contracts/components/gain-reduction-meter.md#6. Accessibility",
    },
    {
      axis: "keyboard_focus",
      reason: "The root exposes meter semantics from zero to maximum reduction and is not a focus stop.",
      contractRef: "docs/contracts/components/gain-reduction-meter.md#6. Accessibility",
    },
  ],
  ValueReadout: [
    {
      axis: "events",
      reason: "ValueReadout is display-only and emits no component events.",
      contractRef: "docs/contracts/components/value-readout.md#5. Events",
    },
    {
      axis: "pointer",
      reason: "ValueReadout is a formatted read-only output with no pointer interaction.",
      contractRef: "docs/contracts/components/value-readout.md#5. Events",
    },
    {
      axis: "keyboard_focus",
      reason: "The output carries an optional accessible name and is not a keyboard focus stop.",
      contractRef: "docs/contracts/components/value-readout.md#6. Accessibility",
    },
  ],
  IconProvider: [
    {
      axis: "pointer",
      reason: "IconProvider renders no element and produces no visual output; pointer interaction belongs to descendant content.",
      contractRef: "docs/contracts/components/icon-provider.md#6. Accessibility",
    },
    {
      axis: "keyboard_focus",
      reason: "IconProvider emits no element and takes no focus; descendants keep their own focus targets.",
      contractRef: "docs/contracts/components/icon-provider.md#Keyboard",
    },
  ],
  MetricTile: [
    {
      axis: "events",
      reason: "MetricTile is a non-interactive display component with no component-owned events.",
      contractRef: "docs/contracts/components/metric-tile.md#5. Events",
    },
    {
      axis: "pointer",
      reason: "MetricTile declares no click behavior and no pointer interaction; it is a label-value display surface.",
      contractRef: "docs/contracts/components/metric-tile.md#1. Purpose",
    },
  ],
  StateTile: [
    {
      axis: "events",
      reason: "StateTile is wholly static with no internal state or event surface.",
      contractRef: "docs/contracts/components/state-tile.md#Controlled And Uncontrolled",
    },
    {
      axis: "pointer",
      reason: "StateTile renders label, value, trend text and a host-owned sparkline slot with no pointer interaction of its own.",
      contractRef: "docs/contracts/components/state-tile.md#7. Accessibility",
    },
    {
      axis: "keyboard_focus",
      reason: "StateTile has no focus stop and no keyboard behavior; a host wrapper owns any promoted semantics.",
      contractRef: "docs/contracts/components/state-tile.md#7. Accessibility",
    },
  ],
  MediaPreview: [
    {
      axis: "events",
      reason: "MediaPreview declares no component-owned events; state posture is delegated to MediaThumbnail.",
      contractRef: "docs/contracts/components/media-preview.md#6. Events",
    },
    {
      axis: "pointer",
      reason: "MediaPreview is a styled-only composition; pointer interaction belongs to composed media and body content.",
      contractRef: "docs/contracts/components/media-preview.md#Behavior Machine",
    },
    {
      axis: "keyboard_focus",
      reason: "MediaPreview owns no focus stop or keyboard behavior; nested controls keep their own targets.",
      contractRef: "docs/contracts/components/media-preview.md#Behavior Machine",
    },
  ],
  EmptyState: [
    {
      axis: "events",
      reason:
        "EmptyState dispatches no component-owned events; action behavior belongs to slotted host buttons.",
      contractRef: "docs/contracts/components/empty-state.md#5. Events",
    },
  ],
  ErrorBoundary: [
    {
      axis: "events",
      reason:
        "The boundary dispatches no component-owned events: the error state is an EmptyState and the retry press is a host-reset action.",
      contractRef: "docs/contracts/components/error-boundary.md#Composition",
    },
  ],
  EmbedPreview: [
    {
      axis: "events",
      reason: "EmbedPreview is a pure display component with no component-owned events.",
      contractRef: "docs/contracts/components/embed-preview.md#5. Events",
    },
  ],
  InlineListSection: [
    {
      axis: "events",
      reason:
        "The section dispatches no component-owned events; item content, row actions, and status pills stay host-owned.",
      contractRef: "docs/contracts/components/inline-list-section.md#Rules",
    },
  ],
  DetailSection: [
    {
      axis: "events",
      reason: "DetailSection is a grouping composite with no component-owned events; slotted actions own theirs.",
      contractRef: "docs/contracts/components/detail-section.md#5. Events",
    },
  ],
  ListGrid: [
    {
      axis: "events",
      reason: "ListGrid is a styled-only layout primitive with no component-owned events.",
      contractRef: "docs/contracts/components/list-grid.md#Behavior Machine",
    },
    {
      axis: "pointer",
      reason:
        "ListGrid renders layout only and has no pointer interaction; grid items own theirs.",
      contractRef: "docs/contracts/components/list-grid.md#2. Accessibility",
    },
    {
      axis: "keyboard_focus",
      reason: "ListGrid is styled-only and never a focus stop; it has no keyboard behavior.",
      contractRef: "docs/contracts/components/list-grid.md#Behavior Machine",
    },
  ],
  FieldSet: [
    {
      axis: "events",
      reason:
        "FieldSet groups controls and owns no callbacks or events; validation and submission stay host-owned.",
      contractRef: "docs/contracts/components/field-set.md#1. Purpose",
    },
    {
      axis: "pointer",
      reason: "FieldSet owns grouping layout only; pointer interaction belongs to the grouped controls.",
      contractRef: "docs/contracts/components/field-set.md#5. Accessibility",
    },
    {
      axis: "keyboard_focus",
      reason:
        "FieldSet adds no keyboard behavior and is not a focus stop; grouped controls keep their own focus targets.",
      contractRef: "docs/contracts/components/field-set.md#5. Accessibility",
    },
  ],
  UiPresentationProvider: [
    {
      axis: "pointer",
      reason:
        "The provider is not a hit target and intercepts no input; descendants own their pointer behavior.",
      contractRef: "docs/contracts/components/ui-presentation-provider.md#7. Layout",
    },
  ],
  MotionPolicyProvider: [
    {
      axis: "pointer",
      reason:
        "The provider adds no hit target; descendants keep their own pointer behavior.",
      contractRef: "docs/contracts/components/motion-policy-provider.md#7. Layout And Composition",
    },
    {
      axis: "events",
      reason:
        "The provider emits no component event; changing the policy rebuilds descendants without a semantic callback.",
      contractRef: "docs/contracts/components/motion-policy-provider.md#5. Events",
    },
  ],
  AgentMessage: [
    {
      axis: "events",
      reason:
        "The only declared event, onLinkClick, has no native element to attach to: inline nodes flatten to text, a recorded accepted delta.",
      contractRef: "docs/contracts/components/agent-message.md#12. Known Deltas",
    },
    {
      axis: "pointer",
      reason:
        "Link activation is the message's only pointer interaction and the natives draw no link, a recorded accepted delta.",
      contractRef: "docs/contracts/components/agent-message.md#12. Known Deltas",
    },
  ],
  PasswordRequirements: [
    {
      axis: "events",
      reason:
        "Callers own policy fetch and retry behavior; the checklist dispatches no component-owned events.",
      contractRef: "docs/contracts/components/password-requirements.md#5. Boundary",
    },
    {
      axis: "pointer",
      reason:
        "The checklist is display-only with no pointer surface; status reads through wording and indicator.",
      contractRef: "docs/contracts/components/password-requirements.md#6. Accessibility",
    },
  ],
};

export type ExecutionRecord = {
  schema: string;
  command: string;
  source_commit: string;
  /** Commit that first recorded the legacy test-body hashes. */
  body_hash_baseline_commit?: string;
  lockfile: string;
  lockfile_sha256: string;
  run_id: string;
  /** Overrides the default full-selector identity for a named test execution.
   * Named reruns never rewrite the record-wide full-selector identity. */
  results: Record<
    string,
    {
      outcome: "passed" | "failed" | "not-run";
      body_sha256: string;
      run_id?: string;
      command?: string;
      source_commit?: string;
      lockfile_sha256?: string;
    }
  >;
};

export function sha256Hex(text: string): string {
  return createHash("sha256").update(text).digest("hex");
}

function read(root: string, relativePath: string): string {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

/** Parse the preview crate's `[package].version` narrowly and fail closed.
 * Missing, duplicate, and malformed declarations refuse to produce evidence,
 * so a manifest edit can never silently fall back to a stale literal. */
export function parsePreviewPackageVersion(source: string): string {
  const packageHeaders = [...source.matchAll(/^\[package\][ \t]*$/gm)];
  if (packageHeaders.length !== 1) {
    throw new Error(`GPUI preview manifest must declare exactly one [package] table; found ${packageHeaders.length}.`);
  }
  const header = packageHeaders[0];
  const after = source.slice((header.index ?? 0) + header[0].length);
  const nextTable = after.search(/^\[/m);
  const block = nextTable === -1 ? after : after.slice(0, nextTable);
  const declarations = [...block.matchAll(/^[ \t]*version[ \t]*=[ \t]*([^\r\n]*)$/gm)];
  if (declarations.length !== 1) {
    throw new Error(`GPUI preview [package] must declare exactly one version key; found ${declarations.length}.`);
  }
  const raw = declarations[0][1].trim();
  const quoted = /^"([^"]*)"$/.exec(raw);
  if (quoted === null || !SEMVER_RE.test(quoted[1])) {
    throw new Error(`GPUI preview package version must be one quoted semver string; found ${raw === "" ? "nothing" : raw}.`);
  }
  return quoted[1];
}

/** The live preview package version from the checked-in manifest. */
export function loadPreviewPackageVersion(root = ROOT): string {
  return parsePreviewPackageVersion(read(root, GPUI_PREVIEW_CARGO_MANIFEST));
}

/** A mounted receipt's recorded package version must equal the live manifest.
 * Returns the live version so callers can keep using it after validating. */
export function validateReceiptPackageVersion(recorded: unknown, live: string, component: string): string {
  if (recorded !== live) {
    const found = typeof recorded === "string" && recorded.length > 0 ? recorded : "none";
    throw new Error(
      `Mounted receipt for ${component} records package version ${found} but the preview manifest is ${live}; regenerate.`,
    );
  }
  return live;
}

/** The pinned execution commit must exist locally and the working tree must
 * descend from it. Evidence is point-in-time: anything else means history was
 * rewritten or the record belongs to another line, and the census refuses to
 * describe the current tree until the tests are re-run. */
function validatePinAncestry(sourceCommit: string, root: string): void {
  let known = false;
  try {
    execSync(`git cat-file -e ${sourceCommit}^{commit}`, { cwd: root, stdio: "ignore" });
    known = true;
  } catch {
    known = false;
  }
  if (!known) throw new Error(`Execution record pins unknown commit ${sourceCommit}; re-run the expected tests.`);
  let descendant = false;
  try {
    execSync(`git merge-base --is-ancestor ${sourceCommit} HEAD`, { cwd: root, stdio: "ignore" });
    descendant = true;
  } catch {
    descendant = false;
  }
  if (!descendant) {
    throw new Error(`Current HEAD does not descend from the recorded execution commit ${sourceCommit}; re-run the expected tests and regenerate the census.`);
  }
}

/** Extract a top-level test through its own closing brace, or undefined when absent/stale. */
function extractTestBodyFromSource(source: string, testName: string): string | undefined {
  const lines = source.split("\n");
  const start = lines.findIndex((line) => line.startsWith(`fn ${testName}(`));
  if (start < 0) return undefined;
  for (let end = start + 1; end < lines.length; end++) {
    // Rustfmt puts a top-level function's closing brace at column zero.
    // Stop here so comments for a following test (or appended tests at EOF)
    // do not become part of this test's evidence hash.
    if (lines[end] === "}") {
      // Preserve trailing blank lines: the old EOF extraction included the
      // file's final newline, and those existing execution hashes stay stable.
      while (end + 1 < lines.length && lines[end + 1].trim() === "") end++;
      return lines.slice(start, end + 1).join("\n");
    }
  }
  return undefined;
}

export function extractTestBody(root: string, testName: string): string | undefined {
  const file = path.join(root, HEADLESS_TEST_FILE);
  if (!fs.existsSync(file)) return undefined;
  return extractTestBodyFromSource(fs.readFileSync(file, "utf8"), testName);
}

/** True when the test carries #[ignore]: it never executes, so it can never admit evidence. */
export function testIsIgnored(root: string, testName: string): boolean {
  const file = path.join(root, HEADLESS_TEST_FILE);
  if (!fs.existsSync(file)) return true;
  const lines = fs.readFileSync(file, "utf8").split("\n");
  const start = lines.findIndex((line) => line.startsWith(`fn ${testName}(`));
  if (start < 0) return true;
  return lines.slice(Math.max(0, start - 4), start).some((line) => line.trim() === "#[ignore]");
}

export function testBodySha256(root: string, testName: string): string | undefined {
  const body = extractTestBody(root, testName);
  return body === undefined ? undefined : sha256Hex(body);
}

/** Previous extraction retained text through the next test/function boundary. */
function extractLegacyTestBody(source: string, testName: string): string | undefined {
  const lines = source.split("\n");
  const start = lines.findIndex((line) => line.startsWith(`fn ${testName}(`));
  if (start < 0) return undefined;
  let end = lines.length;
  for (let i = start + 1; i < lines.length; i++) {
    if (lines[i].startsWith("#[test]") || lines[i].startsWith("fn ")) {
      end = i;
      break;
    }
  }
  return lines.slice(start, end).join("\n");
}

function testSourceAtCommit(root: string, commit: string): string | undefined {
  try {
    return execSync(`git show ${commit}:${HEADLESS_TEST_FILE}`, {
      cwd: root,
      encoding: "utf8",
      maxBuffer: 32 * 1024 * 1024,
    });
  } catch {
    return undefined;
  }
}

function executionBodyHashMatches(
  root: string,
  testName: string,
  expectedHash: string,
  baselineSource: string | undefined,
): boolean {
  const currentBody = extractTestBody(root, testName);
  if (currentBody === undefined) return false;
  const currentHash = sha256Hex(currentBody);
  if (currentHash === expectedHash) return true;
  if (baselineSource === undefined) return false;
  const legacyBaseline = extractLegacyTestBody(baselineSource, testName);
  const canonicalBaseline = extractTestBodyFromSource(baselineSource, testName);
  return (
    legacyBaseline !== undefined &&
    canonicalBaseline !== undefined &&
    sha256Hex(legacyBaseline) === expectedHash &&
    sha256Hex(canonicalBaseline) === currentHash
  );
}

type TopLevelFn = { name: string; test: boolean; body: string };

function topLevelFns(source: string): TopLevelFn[] {
  const lines = source.split("\n");
  const fns: TopLevelFn[] = [];
  let start = -1;
  const flush = (end: number): void => {
    if (start < 0) return;
    const head = lines[start];
    const name = head.slice(3).split("(")[0].split("<")[0].trim();
    const test = lines.slice(Math.max(0, start - 3), start).some((line) => line.trim() === "#[test]");
    fns.push({ name, test, body: lines.slice(start, end).join("\n") });
    start = -1;
  };
  for (let i = 0; i < lines.length; i++) {
    if (lines[i].startsWith("fn ")) {
      flush(i);
      start = i;
    }
  }
  flush(lines.length);
  return fns;
}

/** Identifiers the test file imports straight from the production renderer.
 * A body calling one of these calls poodle_render by name, not by coincidence. */
export function rendererImportNames(source: string): string[] {
  const names = new Set<string>();
  for (const match of source.matchAll(/use poodle_render::\{([^}]*)\}/gs)) {
    for (const part of match[1].split(",")) {
      const alias = part.trim().split(/\s+as\s+/);
      const name = (alias.length > 1 ? alias[1] : alias[0]).trim();
      if (/^\w+$/.test(name)) names.add(name);
    }
  }
  return [...names].sort();
}

/** Node-compat component types imported directly by mounted regressions. */
export function nodeCompatImportNames(source: string): string[] {
  const names = new Set<string>();
  for (const match of source.matchAll(/use node_compat::\{([^}]*)\}/gs)) {
    for (const part of match[1].split(",")) {
      const alias = part.trim().split(/\s+as\s+/);
      const name = (alias.length > 1 ? alias[1] : alias[0]).trim();
      if (/^\w+$/.test(name)) names.add(name);
    }
  }
  return [...names].sort();
}

/** Non-test helpers that resolve to the production renderer, closed transitively:
 * direct poodle_render/node_compat users plus helpers that only call those. */
export function rendererHelperNames(source: string): string[] {
  const helpers = topLevelFns(source).filter((fn) => !fn.test);
  const imported = rendererImportNames(source);
  const importedCompat = nodeCompatImportNames(source);
  const callsRenderer = (body: string, known: Set<string>): boolean => {
    if (PRODUCTION_RENDER_RE.test(body)) return true;
    if (imported.some((name) => new RegExp(`\\b${name}\\s*\\(`).test(body))) return true;
    if (importedCompat.some((name) => new RegExp(`\\b${name}::\\w+`).test(body))) return true;
    return [...known].some((name) => new RegExp(`\\b${name}\\s*\\(`).test(body));
  };
  const known = new Set<string>();
  let changed = true;
  while (changed) {
    changed = false;
    for (const helper of helpers) {
      if (!known.has(helper.name) && callsRenderer(helper.body, known)) {
        known.add(helper.name);
        changed = true;
      }
    }
  }
  return [...known].sort();
}

/** Non-test helpers that mount through the real backend driver. */
export function mountHelperNames(source: string): string[] {
  return topLevelFns(source)
    .filter((fn) => !fn.test && /HeadlessDriver::\w+/.test(fn.body))
    .map((fn) => fn.name)
    .sort();
}

export function productionMount(body: string, source = "", selfName = ""): boolean {
  if (!/run_headless\s*\(/.test(body)) return false;
  const driverDirect = /HeadlessDriver::\w+/.test(body);
  const rendererDirect = PRODUCTION_RENDER_RE.test(body);
  if (source === "") return driverDirect && rendererDirect && /HeadlessDriver::new/.test(body);
  const rendererHelpers = rendererHelperNames(source).filter((name) => name !== selfName);
  const mountHelpers = mountHelperNames(source).filter((name) => name !== selfName);
  const calls = (name: string): boolean => new RegExp(`\\b${name}\\s*\\(`).test(body);
  const imported = rendererImportNames(source);
  const importedCompat = nodeCompatImportNames(source);
  const renderer =
    rendererDirect ||
    imported.some((name) => calls(name)) ||
    importedCompat.some((name) => new RegExp(`\\b${name}::\\w+`).test(body)) ||
    rendererHelpers.some((name) => calls(name));
  const mount = driverDirect || mountHelpers.some((name) => calls(name));
  return renderer && mount;
}
/** Distinct matched fragments for one signal, longest first, capped for review. */
export function matchedText(body: string, signal: RegExp): string[] {
  const flags = signal.flags.includes("g") ? signal.flags : `${signal.flags}g`;
  const hits = body.match(new RegExp(signal.source, flags)) ?? [];
  return [...new Set(hits)].sort((a, b) => b.length - a.length).slice(0, 3);
}

/** The exact driver construction the body shows: a HeadlessDriver constructor,
 * or the mount helper it goes through. Stored per receipt so the production
 * observation names evidence instead of repeating a canned block. */
export function observedDriver(body: string, source: string, selfName: string): string {
  const direct = body.match(/HeadlessDriver::\w+/);
  if (direct !== null) return direct[0];
  const helper = mountHelperNames(source).filter((name) => name !== selfName).find((name) => new RegExp(`\\b${name}\\s*\\(`).test(body));
  return helper === undefined ? "unknown" : `mount-helper:${helper}`;
}

/** The exact renderer reference the body shows: a qualified production path,
 * an imported poodle_render name, or the fixture helper it goes through. */
export function observedRenderer(body: string, source: string, selfName: string): string {
  const direct = body.match(/poodle_render::(?!color::)\w+|node_compat::\w+/);
  if (direct !== null) return direct[0];
  const imported = rendererImportNames(source).find((name) => new RegExp(`\\b${name}\\s*\\(`).test(body));
  if (imported !== undefined) return `poodle_render-import:${imported}`;
  const importedCompat = nodeCompatImportNames(source).find((name) => new RegExp(`\\b${name}::\\w+`).test(body));
  if (importedCompat !== undefined) return `node_compat-import:${importedCompat}`;
  const helper = rendererHelperNames(source).filter((name) => name !== selfName).find((name) => new RegExp(`\\b${name}\\s*\\(`).test(body));
  return helper === undefined ? "unknown" : `fixture-helper:${helper}`;
}
/** Claim-bound axis admission for one expected-test body. Signals without the
 * production mount admit nothing: renderer unit tests and direct handler calls
 * fail closed here. Pass the file source so fixture-helper indirection
 * (poodle_render imports, node builders, mount helpers) resolves; without it
 * only literal markers count. */
export function admitTestAxes(
  body: string,
  source = "",
  selfName = "",
): {
  production: boolean;
  axes: CensusAxis[];
  signals: Record<CensusAxis, string[]>;
} {
  const production = productionMount(body, source, selfName);
  const signals = {} as Record<CensusAxis, string[]>;
  const axes: CensusAxis[] = [];
  if (!production) {
    for (const axis of CENSUS_AXES) signals[axis] = [];
    return { production, axes, signals };
  }
  signals.semantic = ASSERT_RE.test(body) ? ["assert", "production-mount"] : [];
  if (signals.semantic.length > 0) axes.push("semantic");
  const observed = ASSERT_RE.test(body);
  for (const axis of CENSUS_AXES) {
    if (axis === "semantic") continue;
    // Receipts store what the body actually said, not the pattern that
    // matched: at most three distinct matched fragments per axis, so review
    // reads evidence instead of regex source.
    const matched = observed ? AXIS_TEST_SIGNALS[axis].flatMap((signal) => matchedText(body, signal)) : [];
    signals[axis] = [...new Set(matched)].slice(0, 3);
    if (signals[axis].length >= 1) axes.push(axis);
  }
  return { production, axes, signals };
}

/** Claim-bound axis admission for validated Nucleus receipt prose. The receipt
 * is already execution evidence; this maps only what its recorded actions and
 * assertions actually name. Semantic is the mounted-production-render baseline. */
export function admitReceiptTextAxes(text: string): CensusAxis[] {
  const axes: CensusAxis[] = ["semantic"];
  for (const axis of CENSUS_AXES) {
    if (axis === "semantic") continue;
    if (RECEIPT_TEXT_SIGNALS[axis].test(text)) axes.push(axis);
  }
  return axes;
}

function headingBody(markdown: string, start: RegExp): { heading: string; body: string } {
  const lines = markdown.split("\n");
  const index = lines.findIndex((line) => start.test(line));
  if (index < 0) return { heading: "", body: "" };
  const level = (lines[index].match(/^(#+)/)?.[1] ?? "#").length;
  let end = lines.length;
  for (let i = index + 1; i < lines.length; i++) {
    const match = lines[i].match(/^(#+)\s/);
    if (match !== null && match[1].length <= level) {
      end = i;
      break;
    }
  }
  return { heading: lines[index].replace(/^#+\s*/, "").trim(), body: lines.slice(index + 1, end).join("\n") };
}

const SUBSTRATE_RULES: Array<{ substrate: string; pattern: RegExp }> = [
  { substrate: "layout-primitive", pattern: /layout primitive|neutral \w+ container|non-interactive/i },
  { substrate: "overlay-dismissal", pattern: /\boverlay\b|\bpopover\b|\bdialog\b|\btooltip\b|\bmenu\b|\bdismiss|\bdropdown\b|\bsheet\b|command palette/i },
  { substrate: "drag-resize-reorder", pattern: /\bdrag\b|\bdrop\b|\bresize\b|\breorder\b|\bgrip\b|move control/i },
  { substrate: "selection-navigation", pattern: /\bselect\b|\bradio\b|\btab\b|\btree\b|\bbreadcrumb\b|\bpagination\b|\bsegmented\b|\btoggle\b/i },
  { substrate: "form-editing", pattern: /\binput\b|\beditor\b|\bsegment\b|\bslider\b|\bfader\b|\bknob\b|\bfield\b|\bduration\b|\bnumber\b|\btime\b/i },
  { substrate: "feedback-status", pattern: /\bcallout\b|\bbanner\b|\btoast\b|\bstatus\b|\bspinner\b|\bskeleton\b|\bprogress\b|\bempty\b|\berror\b|\bconfirm\b|\brating\b/i },
  { substrate: "workstation-shell", pattern: /\bdock\b|\bregion\b|\bpanel\b|\bshell\b|\bheader\b|\bsidebar\b|\bpalette\b|\bcenter\b|\bstepper\b/i },
  { substrate: "agent-composites", pattern: /\bagent\b|\btranscript\b|tool call|\bplan\b|\bsubagent\b|\bdiscovery\b/i },
  { substrate: "text-display", pattern: /\btext\b|\bcode\b|\bmarkdown\b|\blabel\b|\bavatar\b|\bicon\b|\bbadge\b|\btag\b|\bchip\b|\bdetail\b/i },
  { substrate: "media-data", pattern: /\baudio\b|\bvideo\b|\bimage\b|\bplayer\b|\bupload\b|\bfile\b|\bcalendar\b|\bdate\b|\bchart\b|\bcolor\b/i },
];

function classifySubstrate(contract: string): string {
  const purpose = headingBody(contract, /^## 1\. /).body;
  const head = purpose.length > 0 ? purpose : contract.split("\n").slice(0, 25).join("\n");
  for (const rule of SUBSTRATE_RULES) {
    if (rule.pattern.test(head)) return rule.substrate;
  }
  return "general-composite";
}

/** Derive the closed capability manifest from contract authority. Every portable
 * component requires semantic, accessibility, and visual proof; events, pointer,
 * and keyboard_focus are required unless the contract itself declares the
 * non-interactive boundary with an exact section reference. Platform state is
 * never a not-applicable reason. */
export function deriveCapabilityManifest(root = ROOT): ManifestEntry[] {
  const roster = deriveLiveRoster(root);
  const entries: ManifestEntry[] = [];
  for (const component of roster) {
    if (!component.portable) continue;
    const contractPath = `docs/contracts/components/${component.slug}.md`;
    const contract = read(root, contractPath);
    // MediaThumbnail has no event section but explicitly declares a passive
    // figure with no component-owned events.
    const staticFigureWithoutEvents =
      component.name === "MediaThumbnail" &&
      /\[Root\].*<figure>/.test(contract) &&
      /No component-owned events\./.test(contract);
    // AgentQuestionRecord has no Events/Keyboard headings. §2 and §6 declare
    // the answered record read-only and never a focus stop — the same N/A
    // partition #81 recorded for PaginationSummary, citing this contract.
    const readOnlyRecordWithoutInteraction =
      component.name === "AgentQuestionRecord" &&
      /This component has no interactive parts\./.test(contract) &&
      /Nothing inside is\s+focusable/.test(contract);
    // These two components declare only composition/layout and no component
    // interaction. Their children remain responsible for their own behavior.
    const layoutOnlyGroup =
      component.name === "DetailSectionGroup" &&
      /the component does not inject section chrome; it only owns layout/i.test(contract) &&
      /root element is a plain `<div>`/i.test(contract);
    const decorativeRegion =
      component.name === "Region" &&
      /Role: `presentation`/.test(contract) &&
      /Region is non-interactive and should not be keyboard-focusable\./.test(contract);
    const events = headingBody(
      contract,
      component.name === "DetailShell"
        ? /^## 6\. Events/
        : staticFigureWithoutEvents || component.name === "AppHeader"
          ? /^## 6\. Events/
          : /^## 5\. /,
    );
    const keyboard = headingBody(contract, /^### Keyboard/);
    const focus = headingBody(contract, /^### Focus/);
    const eventTableKeys = events.body
      .split("\n")
      .filter((line) => /^\s*\|/.test(line))
      .map((line) => line.split("|")[1]?.trim() ?? "")
      .filter((key) => key.length > 0 && !/^:?-{2,}:?$/.test(key));
    const hasDeclaredEvents = eventTableKeys.some((key) => !/^(event|callback|none|[-—–])$/i.test(key));
    const eventsNone =
      readOnlyRecordWithoutInteraction ||
      layoutOnlyGroup ||
      decorativeRegion ||
      (!hasDeclaredEvents &&
        (/^\|\s*none\s*\|/m.test(events.body) ||
          /^\s*None\.\s*$/m.test(events.body) ||
          /^No component-owned events are dispatched\./m.test(events.body) ||
          ((component.name === "AppHeader" || component.name === "DetailShell") &&
            /^No component-owned events\./m.test(events.body)) ||
          (component.name === "PageHeader" && /^No component-owned events beyond child action behavior\./m.test(events.body)) ||
          /layout primitive only|no events/i.test(events.body) ||
          staticFigureWithoutEvents));
    const keyboardRows = keyboard.body
      .split("\n")
      .filter((line) => /^\s*\|/.test(line))
      .map((line) => {
        const cells = line.split("|").slice(1);
        return { key: cells[0]?.trim() ?? "", behavior: cells[1]?.trim() ?? "" };
      })
      .filter(({ key }) => key.length > 0 && !/^:?-{2,}:?$/.test(key) && !/^(key|keys)$/i.test(key));
    const tabBehavior = keyboardRows.find(({ key }) => /^`?Tab`?$/i.test(key))?.behavior ?? "";
    const tabNotFocusable = /not focusable/i.test(tabBehavior);
    const hasKeyboardBehavior = keyboardRows.some(
      ({ key, behavior }) => !/^none$/i.test(key) && !/not focusable|host focus behavior is unaffected/i.test(behavior),
    );
    const keyboardNone =
      readOnlyRecordWithoutInteraction ||
      layoutOnlyGroup ||
      decorativeRegion ||
      (!hasKeyboardBehavior &&
        (keyboardRows.some(({ key }) => /^none$/i.test(key)) ||
          tabNotFocusable ||
          /no keyboard behavior/i.test(keyboard.body) ||
          staticFigureWithoutEvents));
    const focusNeutral =
      /not focusable/i.test(focus.body) ||
      staticFigureWithoutEvents ||
      readOnlyRecordWithoutInteraction ||
      layoutOnlyGroup ||
      decorativeRegion;
    const required: CensusAxis[] = ["semantic", "accessibility", "visual"];
    const notApplicable: ManifestNotApplicable[] = [];
    if (eventsNone) {
      notApplicable.push({
        axis: "events",
        reason: readOnlyRecordWithoutInteraction
          ? "Contract declares the record has no interactive parts and no inputs, so it has no callbacks or events to prove."
          : layoutOnlyGroup
            ? "Contract assigns only responsive section layout to this group; child sections own any callbacks or events."
            : decorativeRegion
              ? "Contract defines a decorative placeholder with no child content or component callbacks."
              : "Contract declares no component callbacks or events.",
        contractRef: readOnlyRecordWithoutInteraction
          ? `${contractPath}#2. Read-Only By Construction`
          : layoutOnlyGroup
            ? `${contractPath}#4. Behavior Rules`
            : decorativeRegion
              ? `${contractPath}#3. Composition`
              : `${contractPath}#${events.heading || "Events"}`,
      });
    } else {
      required.push("events");
    }
    if (eventsNone && keyboardNone) {
      notApplicable.push({
        axis: "pointer",
        reason: staticFigureWithoutEvents
          ? "The contract defines a passive figure with no component-owned events, so it has no pointer interaction to prove."
          : readOnlyRecordWithoutInteraction
            ? "Contract declares the record has no interactive parts, so it has no pointer interaction to prove."
            : layoutOnlyGroup
              ? "The contract assigns only section layout to this plain container; pointer interactions belong to its children."
              : decorativeRegion
                ? "The contract defines a decorative placeholder with no child content, so it has no pointer interaction to prove."
                : "Contract declares the non-interactive boundary: no events and not focusable, so no pointer interaction exists to prove.",
        contractRef: staticFigureWithoutEvents
          ? `${contractPath}#3. Anatomy`
          : readOnlyRecordWithoutInteraction
            ? `${contractPath}#2. Read-Only By Construction`
            : layoutOnlyGroup
              ? `${contractPath}#4. Behavior Rules`
              : decorativeRegion
                ? `${contractPath}#6. Accessibility`
                : `${contractPath}#${keyboard.heading || "Keyboard"}`,
      });
    } else {
      required.push("pointer");
    }
    if (keyboardNone && focusNeutral) {
      notApplicable.push({
        axis: "keyboard_focus",
        reason: staticFigureWithoutEvents
          ? "The contract defines a passive figure that is not a focus stop and has no keyboard behavior."
          : readOnlyRecordWithoutInteraction
            ? "Contract states nothing inside is focusable, so the record never appears in the tab order."
            : layoutOnlyGroup
              ? "The contract defines a plain layout container, not a focus stop; child controls own keyboard behavior."
              : decorativeRegion
                ? "The contract states the decorative placeholder is non-interactive and not keyboard-focusable."
                : "Contract declares the component not focusable with no keyboard behavior.",
        contractRef: staticFigureWithoutEvents
          ? `${contractPath}#3. Anatomy`
          : readOnlyRecordWithoutInteraction
            ? `${contractPath}#6. Accessibility`
            : layoutOnlyGroup || decorativeRegion
              ? `${contractPath}#6. Accessibility`
              : `${contractPath}#${keyboard.heading || "Keyboard"}`,
      });
    } else if (keyboardNone && tabNotFocusable) {
      notApplicable.push({
        axis: "keyboard_focus",
        reason: "Contract states the component is not focusable and Tab leaves host focus behavior unaffected.",
        contractRef: `${contractPath}#${keyboard.heading || "Keyboard"}`,
      });
    } else {
      required.push("keyboard_focus");
    }
    for (const item of STATIC_DISPLAY_NOT_APPLICABLE[component.name] ?? []) {
      const requiredIndex = required.indexOf(item.axis);
      if (requiredIndex >= 0) {
        required.splice(requiredIndex, 1);
        notApplicable.push(item);
      }
    }
    const order = (axis: CensusAxis): number => CENSUS_AXES.indexOf(axis);
    required.sort((a, b) => order(a) - order(b));
    notApplicable.sort((a, b) => order(a.axis) - order(b.axis));
    entries.push({
      component: component.name,
      portable: true,
      contract: contractPath,
      substrate: classifySubstrate(contract),
      required,
      notApplicable,
    });
  }
  entries.sort((a, b) => (a.component < b.component ? -1 : 1));
  return entries;
}

export function validateCapabilityManifest(entries: ManifestEntry[]): void {
  const seen = new Set<string>();
  for (const entry of entries) {
    if (seen.has(entry.component)) throw new Error(`Capability manifest duplicates ${entry.component}.`);
    seen.add(entry.component);
    const combined = [...entry.required, ...entry.notApplicable.map((item) => item.axis)];
    if (
      combined.length !== CENSUS_AXES.length ||
      new Set(combined).size !== CENSUS_AXES.length ||
      !CENSUS_AXES.every((axis) => combined.includes(axis))
    ) {
      throw new Error(`Capability manifest for ${entry.component} must partition the closed axes (required plus not-applicable).`);
    }
    for (const item of entry.notApplicable) {
      if (item.reason.trim().length === 0) throw new Error(`Not-applicable ${item.axis} for ${entry.component} needs a reason.`);
      if (PLATFORM_LANGUAGE.test(item.reason) || PLATFORM_LANGUAGE.test(item.contractRef)) {
        throw new Error(
          `Not-applicable ${item.axis} for ${entry.component} cites platform state; the A2 hold cannot widen into a component exemption.`,
        );
      }
    }
  }
}

export function validateManifestRefs(entries: ManifestEntry[], root: string): void {
  for (const entry of entries) {
    if (!fs.existsSync(path.join(root, entry.contract))) {
      throw new Error(`Capability manifest contract missing for ${entry.component}: ${entry.contract}.`);
    }
    for (const item of entry.notApplicable) {
      const [reference, anchor] = item.contractRef.split("#");
      const absolute = path.join(root, reference);
      if (!fs.existsSync(absolute)) throw new Error(`Not-applicable reference missing: ${item.contractRef}.`);
      if (anchor !== undefined) {
        const source = fs.readFileSync(absolute, "utf8");
        if (!source.includes(anchor)) throw new Error(`Not-applicable reference unresolved: ${item.contractRef}.`);
      }
    }
  }
}

export function loadExecutionRecord(root = ROOT): ExecutionRecord {
  return JSON.parse(read(root, EXECUTION_RECORD_PATH)) as ExecutionRecord;
}

function namedRegressionCommand(test: string): string {
  return `${NATIVE_SELECTOR} ${test} -- --exact`;
}

function sourceTextAtCommit(root: string, relativePath: string, commit: string): string | undefined {
  try {
    return execFileSync("git", ["show", `${commit}:${relativePath}`], {
      cwd: root,
      encoding: "utf8",
      maxBuffer: 32 * 1024 * 1024,
    });
  } catch {
    return undefined;
  }
}

function validateRecordedIdentity(
  sourceCommit: string,
  lockfileSha256: string,
  lockfile: string,
  root: string,
  test: string,
): void {
  if (!/^[0-9a-f]{40}$/.test(sourceCommit)) {
    throw new Error(`Execution record for ${test} needs a 40-hex source commit.`);
  }
  validatePinAncestry(sourceCommit, root);
  const lockText = sourceTextAtCommit(root, lockfile, sourceCommit);
  if (lockText === undefined || sha256Hex(lockText) !== lockfileSha256) {
    throw new Error(`Execution record for ${test} does not match the lockfile at ${sourceCommit}.`);
  }
}

/** Old v1 records stored one shared source/lock identity even for named runs.
 * Recover each differing run's identity from the evidence commit that first
 * recorded its run id, then persist it on that test before validation. */
function migrateNamedRunIdentities(record: ExecutionRecord, root: string): void {
  for (const [test, result] of Object.entries(record.results)) {
    if (result.run_id === undefined || result.run_id === record.run_id || result.source_commit !== undefined) continue;
    const needle = `"run_id": "${result.run_id}"`;
    const commits = execFileSync(
      "git",
      ["log", "--all", "--format=%H", "-S", needle, "--", EXECUTION_RECORD_PATH],
      { cwd: root, encoding: "utf8", maxBuffer: 8 * 1024 * 1024 },
    )
      .trim()
      .split("\n")
      .filter((commit) => /^[0-9a-f]{40}$/.test(commit))
      .reverse();
    let recovered:
      | { source_commit: string; lockfile_sha256: string }
      | undefined;
    for (const commit of commits) {
      const snapshotText = sourceTextAtCommit(root, EXECUTION_RECORD_PATH, commit);
      if (snapshotText === undefined) continue;
      const snapshot = JSON.parse(snapshotText) as ExecutionRecord;
      if (snapshot.results[test]?.run_id !== result.run_id) continue;
      recovered = {
        source_commit: snapshot.source_commit,
        lockfile_sha256: snapshot.lockfile_sha256,
      };
      break;
    }
    if (recovered === undefined) {
      throw new Error(`Cannot recover named execution identity for ${test} (${result.run_id}).`);
    }
    result.command = namedRegressionCommand(test);
    result.source_commit = recovered.source_commit;
    result.lockfile_sha256 = recovered.lockfile_sha256;
  }
}

export function validateExecutionRecord(record: ExecutionRecord, root: string): void {
  if (record.schema !== EXECUTION_SCHEMA) throw new Error(`Execution record schema is ${record.schema}.`);
  if (!/^[0-9a-f]{40}$/.test(record.source_commit)) throw new Error("Execution record needs a 40-hex source commit.");
  if (record.command !== NATIVE_SELECTOR) throw new Error(`Execution record must cite ${NATIVE_SELECTOR}.`);
  validateRecordedIdentity(record.source_commit, record.lockfile_sha256, record.lockfile, root, "default selector");
  const bodyHashBaselineCommit = record.body_hash_baseline_commit ?? record.source_commit;
  if (!/^[0-9a-f]{40}$/.test(bodyHashBaselineCommit)) {
    throw new Error("Execution record needs a 40-hex body-hash baseline commit.");
  }
  validatePinAncestry(bodyHashBaselineCommit, root);
  const baselineSource = testSourceAtCommit(root, bodyHashBaselineCommit);
  const expectedTests = Object.values(EXPECTED_MOUNTED_BEHAVIOUR_TESTS).flatMap((tests) =>
    Array.isArray(tests) ? tests : [tests],
  );
  for (const test of expectedTests) {
    const entry = record.results[test];
    if (entry === undefined) throw new Error(`Execution record has no result for expected test ${test}.`);
    if (entry.outcome !== "passed") throw new Error(`Expected test ${test} did not pass in the recorded execution.`);
    const named = entry.run_id !== undefined && entry.run_id !== record.run_id;
    if (named && (entry.command === undefined || entry.source_commit === undefined || entry.lockfile_sha256 === undefined)) {
      throw new Error(`Named execution for ${test} has no independent command/source/lockfile identity.`);
    }
    const command = entry.command ?? record.command;
    if (command !== record.command && command !== namedRegressionCommand(test)) {
      throw new Error(`Execution command for ${test} is not its full selector or exact named selector.`);
    }
    validateRecordedIdentity(
      entry.source_commit ?? record.source_commit,
      entry.lockfile_sha256 ?? record.lockfile_sha256,
      record.lockfile,
      root,
      test,
    );
    const current = testBodySha256(root, test);
    if (current === undefined) throw new Error(`Expected test ${test} is stale: it no longer exists in ${HEADLESS_TEST_FILE}.`);
    if (testIsIgnored(root, test)) throw new Error(`Expected test ${test} is ignored and never executes.`);
    if (!executionBodyHashMatches(root, test, entry.body_sha256, baselineSource)) {
      throw new Error(`Expected test ${test} changed since the recorded execution; re-run it before admitting claims.`);
    }
  }
}

/** Refresh provenance for retained mounted regressions after their named
 * Effigy runs passed. The selected names must already be expected census
 * tests; this writer never launches tests or edits execution evidence by hand
 * outside the census tool. The complete record is validated before any write. */
export function recordExpectedTestExecution(testNames: string[], runId: string, root = ROOT): ExecutionRecord {
  if (runId.trim().length === 0) throw new Error("Execution recording needs a non-empty run id.");
  if (testNames.length === 0) throw new Error("Execution recording needs at least one expected test.");
  if (new Set(testNames).size !== testNames.length) throw new Error("Execution recording contains a duplicate test name.");

  const expectedTests = new Set(
    Object.values(EXPECTED_MOUNTED_BEHAVIOUR_TESTS).flatMap((tests) => (Array.isArray(tests) ? tests : [tests])),
  );
  for (const test of testNames) {
    if (!/^[A-Za-z0-9_]+$/.test(test) || !expectedTests.has(test)) {
      throw new Error(`Execution recording refuses non-retained expected test ${test}.`);
    }
    if (extractTestBody(root, test) === undefined || testIsIgnored(root, test)) {
      throw new Error(`Execution recording refuses stale or ignored expected test ${test}.`);
    }
  }

  const record = loadExecutionRecord(root);
  record.schema = EXECUTION_SCHEMA;
  migrateNamedRunIdentities(record, root);
  const sourceCommit = execSync("git rev-parse HEAD", { cwd: root, encoding: "utf8" }).trim();
  const lockfile = read(root, GPUI_LOCKFILE);
  record.body_hash_baseline_commit ??= record.source_commit;
  for (const test of testNames) {
    const bodySha = testBodySha256(root, test);
    if (bodySha === undefined) throw new Error(`Expected test ${test} disappeared after execution.`);
    record.results[test] = {
      outcome: "passed",
      body_sha256: bodySha,
      run_id: runId,
      command: namedRegressionCommand(test),
      source_commit: sourceCommit,
      lockfile_sha256: sha256Hex(lockfile),
    };
  }
  validateExecutionRecord(record, root);
  writeFile(root, EXECUTION_RECORD_PATH, `${JSON.stringify(record, null, 2)}\n`);
  return record;
}

/** Restore the last full-selector identity after a legacy named rerun moved
 * the shared source/lock fields. The named results retain their own identity. */
export function restoreDefaultExecutionIdentity(sourceCommit: string, root = ROOT): ExecutionRecord {
  const record = loadExecutionRecord(root);
  const lockText = sourceTextAtCommit(root, record.lockfile, sourceCommit);
  if (lockText === undefined) throw new Error(`Cannot read ${record.lockfile} at ${sourceCommit}.`);
  record.schema = EXECUTION_SCHEMA;
  record.source_commit = sourceCommit;
  record.lockfile_sha256 = sha256Hex(lockText);
  migrateNamedRunIdentities(record, root);
  validateExecutionRecord(record, root);
  writeFile(root, EXECUTION_RECORD_PATH, `${JSON.stringify(record, null, 2)}\n`);
  return record;
}

const OBSERVED_SENTENCES: Record<CensusAxis, string> = {
  semantic: "mounted the production renderer through HeadlessDriver and asserted spec/state meaning",
  events: "asserted emitted payloads or handler delivery through mounted input",
  pointer: "drove real pointer activation through the mounted GPUI node backend",
  keyboard_focus: "drove real keyboard input or backend focus through the mounted tree",
  accessibility: "asserted node-level role, label, state, or focusability in the mounted tree",
  visual: "asserted resolved token values or mounted geometry",
};

export function receiptFileName(component: string, test: string): string {
  const slug = test.replace(/[^a-z0-9]+/gi, "-").replace(/^-+|-+$/g, "").toLowerCase();
  return `${RECEIPT_DIR}/${component}--${slug}.json`;
}

export type ExpectedTestReceiptInput = {
  component: string;
  test: string;
  command: string;
  axes: CensusAxis[];
  signals: Record<CensusAxis, string[]>;
  driver: string;
  renderer: string;
  packageVersion: string;
  sourceCommit: string;
  lockfileSha256: string;
  runId: string;
  bodySha256: string;
};

/** Build one checked-in mounted receipt. The release identity is injected, so
 * the caller must supply the live preview manifest version instead of a
 * literal that silently survives a version bump. */
export function expectedTestReceiptContent(input: ExpectedTestReceiptInput): string {
  return `${JSON.stringify(
    {
      schema: RECEIPT_SCHEMA,
      component: input.component,
      test: input.test,
      selector: NATIVE_SELECTOR,
      scenario_id: null,
      scenario_note:
        "Expected-test rows carry no Nucleus scenario; the named mounted test is the scenario identity.",
      contract_claims: input.axes,
      signals: Object.fromEntries(input.axes.map((axis) => [axis, input.signals[axis]])),
      production_path_observation: {
        observed: true,
        mount: "HeadlessDriver",
        driver: input.driver,
        render_path: "poodle_render -> poodle_gpui_node_backend::to_gpui",
        renderer: input.renderer,
        input_dispatch: "gpui-test-platform-dispatch",
      },
      package: PREVIEW_PACKAGE,
      package_version: input.packageVersion,
      source_commit: input.sourceCommit,
      lockfile: GPUI_LOCKFILE,
      lockfile_sha256: input.lockfileSha256,
      distribution: "workspace",
      execution: {
        command: input.command,
        run_id: input.runId,
        outcome: "passed",
        body_sha256: input.bodySha256,
      },
      observed: input.axes.map((axis) => OBSERVED_SENTENCES[axis]),
      outcome: "passed",
    },
    null,
    2,
  )}\n`;
}

export function generateCensus(root = ROOT): { doc: CensusDoc; receipts: Array<{ file: string; content: string }> } {
  const roster = deriveLiveRoster(root);
  const manifest = deriveCapabilityManifest(root);
  validateCapabilityManifest(manifest);
  validateManifestRefs(manifest, root);
  const manifestByName = new Map(manifest.map((entry) => [entry.component, entry]));
  const nucleusRows = deriveNucleusReceiptRows(root);
  const nucleusByName = new Map(nucleusRows.map((row) => [row.entry.name, row]));
  const record = loadExecutionRecord(root);
  validateExecutionRecord(record, root);
  // Release identity comes from the live preview manifest, never a literal.
  const packageVersion = loadPreviewPackageVersion(root);
  // Evidence identity is record state, never live HEAD: a commit cannot
  // contain its own hash, so embedding the current checkout would make every
  // checked-in artifact disagree with the generator on every later commit.
  const commit = record.source_commit;
  const receipts: Array<{ file: string; content: string }> = [];

  const rows: CensusRow[] = roster.map((component) => {
    const entry = manifestByName.get(component.name);
    const contractPath = `docs/contracts/components/${component.slug}.md`;
    if (!component.portable) {
      return {
        component: component.name,
        portable: false,
        contract: contractPath,
        substrate: "web-only",
        required: [],
        admitted: [],
        missing: [],
        holds: [],
        receipts: [],
        refusals: [],
      };
    }
    if (entry === undefined) throw new Error(`Capability manifest has no row for portable ${component.name}.`);
    const admitted: CensusAdmission[] = [];
    const files: string[] = [];
    const refusals: string[] = [];
    const admit = (axis: CensusAxis, via: AdmittedVia, ref: string): void => {
      if (!entry.required.includes(axis)) return;
      if (admitted.some((item) => item.axis === axis)) return;
      admitted.push({ axis, via, ref });
    };

    const nucleus = nucleusByName.get(component.name);
    if (nucleus?.receipt !== undefined && nucleus.receiptPath !== undefined) {
      const text = [...(nucleus.receipt.actions ?? []), ...(nucleus.receipt.assertions ?? [])].join("\n");
      for (const axis of admitReceiptTextAxes(text)) admit(axis, "nucleus-m1", `${nucleus.receiptPath}#proof_level`);
    } else if (nucleus !== undefined) {
      refusals.push(
        `No validated M1 receipt for ${component.name} (expected ${nucleus.entry.expected_selector} ${nucleus.entry.expected_test ?? "with no named test yet"}); nucleus row stays missing.`,
      );
    }
    if (nucleus?.a1Receipt !== undefined && nucleus.a1ReceiptPath !== undefined) {
      admit("accessibility", "nucleus-a1", `${nucleus.a1ReceiptPath}#accessibility`);
    }
    if (nucleus?.v1Receipt !== undefined && nucleus.v1ReceiptPath !== undefined) {
      admit("visual", "nucleus-v1", `${nucleus.v1ReceiptPath}#proof_level`);
    }

    const expected = EXPECTED_MOUNTED_BEHAVIOUR_TESTS[component.name];
    const tests = expected === undefined ? [] : Array.isArray(expected) ? expected : [expected];
    if (tests.length === 0 && nucleus?.receipt === undefined) {
      refusals.push(`No validated receipt and no retained expected test for ${component.name}; every required capability stays missing.`);
    }
    const headlessSource = read(root, HEADLESS_TEST_FILE);
    for (const test of tests) {
      const body = extractTestBody(root, test);
      if (body === undefined) {
        refusals.push(`Retained expected test ${test} is stale: it no longer exists in ${HEADLESS_TEST_FILE}.`);
        continue;
      }
      const admission = admitTestAxes(body, headlessSource, test);
      if (!admission.production) {
        refusals.push(`Expected test ${test} bypasses the mounted GPUI node backend; it admits nothing.`);
        continue;
      }
      const withheld = WITHHELD_AXES[component.name] ?? {};
      const axes = admission.axes.filter((axis) => entry.required.includes(axis) && withheld[axis] === undefined);
      const skipped = admission.axes.filter((axis) => !entry.required.includes(axis));
      const notApplicableAxes = new Set(entry.notApplicable.map((item) => item.axis));
      for (const axis of admission.axes) {
        const reason = withheld[axis];
        if (reason !== undefined && entry.required.includes(axis)) {
          refusals.push(`Expected test ${test} shows ${axis} signals, but ${axis} is withheld: ${reason}.`);
        }
      }
      for (const axis of skipped) {
        if (notApplicableAxes.has(axis)) continue;
        refusals.push(`Expected test ${test} shows ${axis} signals the contract does not require; not admitted.`);
      }
      const admittedAxes = axes.filter((axis) => !admitted.some((item) => item.axis === axis));
      for (const axis of entry.required) {
        if (!axes.includes(axis) && !admitted.some((item) => item.axis === axis)) {
          refusals.push(`Expected test ${test} proves no ${axis} claim; ${axis} stays missing.`);
        }
      }
      if (axes.length === 0) {
        refusals.push(`Expected test ${test} mounts the backend but proves no required claim.`);
        continue;
      }
      const file = receiptFileName(component.name, test);
      files.push(file);
      for (const axis of axes) admit(axis, "expected-test", `${HEADLESS_TEST_FILE}#${test}`);
      receipts.push({
        file,
        content: expectedTestReceiptContent({
          component: component.name,
          test,
          command: record.results[test].command ?? record.command,
          axes,
          signals: admission.signals,
          driver: observedDriver(body, headlessSource, test),
          renderer: observedRenderer(body, headlessSource, test),
          packageVersion,
          sourceCommit: record.results[test].source_commit ?? record.source_commit,
          lockfileSha256: record.results[test].lockfile_sha256 ?? record.lockfile_sha256,
          runId: record.results[test].run_id ?? record.run_id,
          bodySha256: record.results[test].body_sha256,
        }),
      });
      void admittedAxes;
    }

    const order = (axis: CensusAxis): number => CENSUS_AXES.indexOf(axis);
    admitted.sort((a, b) => order(a.axis) - order(b.axis));
    const missing = entry.required.filter((axis) => !admitted.some((item) => item.axis === axis));
    return {
      component: component.name,
      portable: true,
      contract: contractPath,
      substrate: entry.substrate,
      required: [...entry.required],
      admitted,
      missing,
      holds: [
        {
          axis: "accessibility",
          kind: "A2-platform-hold",
          note: "Assistive-technology projection waits on the published gpui-apple crate and live platform-tree proof; node-level semantics above are still payable and proved where admitted.",
          ref: "docs/contracts/003-native-accessibility.md",
        },
      ],
      receipts: files,
      refusals,
    };
  });

  const missingTally = {} as Record<CensusAxis, number>;
  for (const axis of CENSUS_AXES) missingTally[axis] = 0;
  let admittedRows = 0;
  let fullyAdmittedRows = 0;
  let refusalCount = 0;
  for (const row of rows) {
    if (row.portable && row.admitted.length > 0) admittedRows += 1;
    if (row.portable && row.required.length > 0 && row.missing.length === 0) fullyAdmittedRows += 1;
    for (const axis of row.missing) missingTally[axis] += 1;
    refusalCount += row.refusals.length;
  }

  const groups: MissingGroup[] = (() => {
    const bySubstrate = new Map<string, CensusRow[]>();
    for (const row of rows) {
      if (!row.portable || row.missing.length === 0) continue;
      const list = bySubstrate.get(row.substrate) ?? [];
      list.push(row);
      bySubstrate.set(row.substrate, list);
    }
    return [...bySubstrate.entries()]
      .map(([substrate, members]) => {
        const tally = {} as Record<CensusAxis, number>;
        for (const axis of CENSUS_AXES) tally[axis] = 0;
        for (const member of members) for (const axis of member.missing) tally[axis] += 1;
        return {
          substrate,
          components: members.map((member) => member.component).sort(),
          missingTally: tally,
          contracts: members.map((member) => member.contract).sort(),
        };
      })
      .sort((a, b) => b.components.length - a.components.length);
  })();

  const doc: CensusDoc = {
    schema: CENSUS_SCHEMA,
    task: "g18.001",
    source_commit: commit,
    denominator: {
      public: PUBLIC_COMPONENT_COUNT,
      portable: PORTABLE_ROUTE_COUNT,
      notApplicable: [...ROSTER_WEB_ONLY_NAMES],
    },
    axes: [...CENSUS_AXES],
    manifest,
    rows,
    summary: {
      admittedRows,
      fullyAdmittedRows,
      missingTally,
      holdCount: rows.filter((row) => row.portable).length,
      refusalCount,
    },
    groups,
    crossRuntime: {
      constructionClaim: "Every portable component route constructs through the headless GPUI specimen probe.",
      mountedScope:
        "bounded named regression set, not a roster-wide behaviour pass; the g18.001 capability census is the compilation input for repair tranches",
      note: "Construction is not functional completion. A passing route, a test name, or one passing test never marks a component complete.",
    },
  };
  return { doc, receipts };
}

export function validateCensusDoc(doc: CensusDoc): void {
  if (doc.schema !== CENSUS_SCHEMA) throw new Error(`Census schema is ${doc.schema}.`);
  if (doc.task !== "g18.001") throw new Error(`Census task is ${doc.task}.`);
  if (!/^[0-9a-f]{40}$/.test(doc.source_commit)) throw new Error("Census needs a 40-hex source commit.");
  if ([...doc.axes].sort().join(",") !== [...CENSUS_AXES].sort().join(",")) {
    throw new Error("Census must use exactly the closed capability axes.");
  }
  if (doc.denominator.public !== PUBLIC_COMPONENT_COUNT || doc.denominator.portable !== PORTABLE_ROUTE_COUNT) {
    throw new Error(
      `Census denominator must stay ${PUBLIC_COMPONENT_COUNT} public / ${PORTABLE_ROUTE_COUNT} portable, found ${doc.denominator.public}/${doc.denominator.portable}.`,
    );
  }
  if (
    doc.denominator.notApplicable.length !== ROSTER_WEB_ONLY_NAMES.length ||
    !ROSTER_WEB_ONLY_NAMES.every((name) => doc.denominator.notApplicable.includes(name))
  ) {
    throw new Error(`${ROSTER_WEB_ONLY_NAMES.join(", ")} must be the single contract-approved non-portable row.`);
  }
  const seen = new Set<string>();
  for (const row of doc.rows) {
    if (seen.has(row.component)) throw new Error(`Census duplicates ${row.component}.`);
    seen.add(row.component);
    if (row.portable) {
      const combined = [...row.admitted.map((item) => item.axis), ...row.missing];
      if (
        combined.length !== row.required.length ||
        new Set(combined).size !== row.required.length ||
        !row.required.every((axis) => combined.includes(axis))
      ) {
        throw new Error(`Census row for ${row.component} must partition its required axes into admitted plus missing.`);
      }
      for (const item of row.admitted) {
        if (!ADMITTED_VIA.includes(item.via)) throw new Error(`Census admission via ${item.via} is not closed.`);
      }
      for (const hold of row.holds) {
        if (hold.axis !== "accessibility" || hold.kind !== "A2-platform-hold") {
          throw new Error(`Census holds stay narrow: accessibility only, A2 platform hold only (${row.component}).`);
        }
        if (hold.ref !== "docs/contracts/003-native-accessibility.md") {
          throw new Error(`Census holds must cite the native accessibility contract (${row.component}).`);
        }
      }
    } else if (row.component !== "MeterSurface") {
      throw new Error(`Only MeterSurface may be non-portable, found ${row.component}.`);
    }
  }
  validateCapabilityManifest(doc.manifest);
  for (const claim of [doc.crossRuntime.constructionClaim, doc.crossRuntime.mountedScope, doc.crossRuntime.note]) {
    for (const pattern of OVERCLAIM_LANGUAGE) {
      if (pattern.test(claim)) throw new Error(`Census cross-runtime summary overclaims: ${claim}`);
    }
  }
  if (!doc.crossRuntime.mountedScope.includes("bounded") || !doc.crossRuntime.mountedScope.includes("not a roster-wide behaviour pass")) {
    throw new Error("Census mounted scope must stay bounded and refuse roster-wide promotion.");
  }
  if (doc.rows.length !== doc.denominator.public) {
    throw new Error(`Census must carry exactly ${doc.denominator.public} rows, found ${doc.rows.length}.`);
  }
}

export function censusMarkdown(doc: CensusDoc): string {
  const lines: string[] = [];
  lines.push("# g18.001 — Contract-bound GPUI functionality census");
  lines.push("");
  lines.push(`Default full-selector source commit (individual mounted receipts carry their own execution identity): \`${doc.source_commit}\``);
  lines.push(`Denominator: **${doc.denominator.public}** public / **${doc.denominator.portable}** portable; \`${doc.denominator.notApplicable[0]}\` is the single contract-approved non-portable row.`);
  lines.push("");
  lines.push("<!-- g18-census-method -->");
  lines.push("## Method");
  lines.push("");
  lines.push("Capability axes are closed: `semantic`, `events`, `pointer`, `keyboard_focus`, `accessibility`, `visual`.");
  lines.push("Each portable row requires the axes its contract declares; `not-applicable` needs an exact contract section and can never cite platform state.");
  lines.push("Admitted capabilities trace to validated Nucleus M1/A1/V1 receipts or to retained expected tests whose individual execution records identify the source and dependency they ran against, mount the production renderer plus GPUI node backend, and show the claimed axis signals in their bodies.");
  lines.push("Construction is not functional completion. A passing route, a test name, or one passing test never marks a component complete.");
  lines.push("");
  lines.push("<!-- g18-census-summary -->");
  lines.push("## Summary");
  lines.push("");
  lines.push(`Rows with at least one admitted capability: **${doc.summary.admittedRows}**/${doc.denominator.portable}.`);
  lines.push(`Fully admitted rows: **${doc.summary.fullyAdmittedRows}**/${doc.denominator.portable}.`);
  lines.push(`Missing by axis: ${CENSUS_AXES.map((axis) => `${axis} ${doc.summary.missingTally[axis]}`).join("; ")}.`);
  lines.push(`Accessibility platform holds (A2, narrow): **${doc.summary.holdCount}**. Refusals recorded: **${doc.summary.refusalCount}**.`);
  lines.push("");
  lines.push("## Rows");
  lines.push("");
  lines.push("| Component | Required | Admitted | Missing | Holds | Receipts |");
  lines.push("| --- | --- | --- | --- | --- | --- |");
  for (const row of doc.rows) {
    if (!row.portable) {
      lines.push(`| ${row.component} | not-applicable (web-only) | — | — | — | — |`);
      continue;
    }
    const admitted = row.admitted.length === 0 ? "—" : row.admitted.map((item) => `${item.axis} (${item.via})`).join("; ");
    const missing = row.missing.length === 0 ? "—" : row.missing.join("; ");
    const holds = row.holds.length === 0 ? "—" : row.holds.map((hold) => `${hold.axis} (${hold.kind})`).join("; ");
    const receipts = row.receipts.length === 0 ? "—" : row.receipts.map((file) => `\`${file}\``).join("; ");
    lines.push(`| ${row.component} | ${row.required.join("; ")} | ${admitted} | ${missing} | ${holds} | ${receipts} |`);
  }
  lines.push("");
  lines.push("<!-- g18-census-refusals -->");
  lines.push("## Refusals");
  lines.push("");
  const refused = doc.rows.filter((row) => row.refusals.length > 0);
  if (refused.length === 0) {
    lines.push("None.");
  } else {
    for (const row of refused) {
      for (const refusal of row.refusals) lines.push(`- ${row.component}: ${refusal}`);
    }
  }
  lines.push("");
  lines.push("## Missing-capability groups");
  lines.push("");
  lines.push("Shared-substrate groupings for repair-tranche compilation. Grouping only; no tranche is planned here.");
  lines.push("");
  for (const group of doc.groups) {
    lines.push(
      `- ${group.substrate}: ${group.components.length} components (${group.components.join(", ")}); missing ${CENSUS_AXES.map((axis) => `${axis} ${group.missingTally[axis]}`).join("; ")}`,
    );
  }
  lines.push("");
  return lines.join("\n");
}

function writeFile(root: string, relativePath: string, content: string): void {
  const absolute = path.join(root, relativePath);
  fs.mkdirSync(path.dirname(absolute), { recursive: true });
  fs.writeFileSync(absolute, content);
}

export function writeCensusArtifacts(root = ROOT): { rows: number; admitted: number; receipts: number } {
  const { doc, receipts } = generateCensus(root);
  validateCensusDoc(doc, root);
  writeFile(root, MANIFEST_PATH, `${JSON.stringify({ schema: MANIFEST_SCHEMA, task: "g18.001", entries: doc.manifest }, null, 2)}\n`);
  const existing = fs.existsSync(path.join(root, RECEIPT_DIR)) ? fs.readdirSync(path.join(root, RECEIPT_DIR)) : [];
  for (const file of existing) {
    if (!receipts.some((receipt) => receipt.file === `${RECEIPT_DIR}/${file}`)) {
      fs.rmSync(path.join(root, RECEIPT_DIR, file));
    }
  }
  for (const receipt of receipts) writeFile(root, receipt.file, receipt.content);
  writeFile(root, CENSUS_JSON_PATH, `${JSON.stringify({ ...doc, manifest: undefined }, null, 2)}\n`);
  writeFile(root, CENSUS_MD_PATH, censusMarkdown(doc));
  writeFile(
    root,
    GROUPS_PATH,
    `${JSON.stringify({ schema: "poodle.g18-missing-capability-groups.v1", task: "g18.001", groups: doc.groups }, null, 2)}\n`,
  );
  return { rows: doc.rows.length, admitted: doc.summary.admittedRows, receipts: receipts.length };
}

function validateReceiptFile(content: string, root: string, baselineSource: string | undefined): void {
  const receipt = JSON.parse(content) as {
    schema?: string;
    component?: string;
    test?: string;
    selector?: string;
    contract_claims?: string[];
    package_version?: unknown;
    source_commit?: string;
    lockfile_sha256?: string;
    execution?: { command?: string; outcome?: string; body_sha256?: string };
    production_path_observation?: { driver?: unknown; renderer?: unknown };
  };
  if (receipt.schema !== RECEIPT_SCHEMA) throw new Error(`Mounted receipt schema is ${receipt.schema}.`);
  if (receipt.selector !== NATIVE_SELECTOR) throw new Error(`Mounted receipt selector must be ${NATIVE_SELECTOR}.`);
  if (typeof receipt.component !== "string" || typeof receipt.test !== "string") {
    throw new Error("Mounted receipt needs a component and test.");
  }
  if (!Array.isArray(receipt.contract_claims) || receipt.contract_claims.length === 0) {
    throw new Error(`Mounted receipt for ${receipt.component} names no contract claims.`);
  }
  for (const axis of receipt.contract_claims) {
    if (!(CENSUS_AXES as readonly string[]).includes(axis)) throw new Error(`Mounted receipt claims unknown axis ${axis}.`);
  }
  validateReceiptPackageVersion(receipt.package_version, loadPreviewPackageVersion(root), receipt.component);
  if (!/^[0-9a-f]{40}$/.test(receipt.source_commit ?? "")) throw new Error("Mounted receipt needs a 40-hex source commit.");
  const command = receipt.execution?.command;
  if (command !== NATIVE_SELECTOR && command !== namedRegressionCommand(receipt.test)) {
    throw new Error(`Mounted receipt for ${receipt.component} has no full or exact named Effigy selector.`);
  }
  validateRecordedIdentity(
    receipt.source_commit,
    receipt.lockfile_sha256 ?? "",
    GPUI_LOCKFILE,
    root,
    receipt.test,
  );
  if (receipt.execution?.outcome !== "passed") {
    throw new Error(`Mounted receipt for ${receipt.component} has no passing execution.`);
  }
  const body = extractTestBody(root, receipt.test);
  if (body === undefined) throw new Error(`Mounted receipt test ${receipt.test} is stale.`);
  if (testIsIgnored(root, receipt.test)) throw new Error(`Mounted receipt test ${receipt.test} is ignored.`);
  const admission = admitTestAxes(body, read(root, HEADLESS_TEST_FILE), receipt.test);
  if (!admission.production) throw new Error(`Mounted receipt test ${receipt.test} bypasses the mounted backend.`);
  const observation = receipt.production_path_observation;
  if (typeof observation?.driver !== "string" || observation.driver === "unknown" || typeof observation?.renderer !== "string" || observation.renderer === "unknown") {
    throw new Error(`Mounted receipt for ${receipt.component} has no resolved production observation.`);
  }
  for (const axis of receipt.contract_claims) {
    if (!admission.axes.includes(axis as CensusAxis)) {
      throw new Error(`Mounted receipt for ${receipt.component} claims ${axis} its test body does not show.`);
    }
  }
  if (!executionBodyHashMatches(root, receipt.test, receipt.execution.body_sha256, baselineSource)) {
    throw new Error(`Mounted receipt test ${receipt.test} changed since its recorded execution.`);
  }
}

function validateCrossRuntimeReport(root: string): void {
  const report = JSON.parse(read(root, CROSS_RUNTIME_REPORT)) as {
    generation?: string;
    construction?: { claim?: string };
    mountedBehaviour?: { scope?: string; testFile?: string; namedTests?: string[] };
  };
  if (report.generation !== "g16.001") throw new Error("Cross-runtime report must keep targeting g16.001.");
  const claim = report.construction?.claim ?? "";
  for (const pattern of OVERCLAIM_LANGUAGE) {
    if (pattern.test(claim)) throw new Error(`Cross-runtime construction claim overclaims: ${claim}`);
  }
  const scope = report.mountedBehaviour?.scope ?? "";
  if (!scope.includes("bounded") || !scope.includes("not a roster-wide behaviour pass")) {
    throw new Error("Cross-runtime mounted scope must stay bounded and refuse roster-wide promotion.");
  }
  if (!scope.includes("g18.001")) throw new Error("Cross-runtime mounted scope must cite the g18.001 census.");
  const testFile = report.mountedBehaviour?.testFile ?? "";
  if (!fs.existsSync(path.join(root, testFile))) throw new Error("Cross-runtime mounted test file is missing.");
  const source = read(root, testFile);
  for (const test of report.mountedBehaviour?.namedTests ?? []) {
    if (!source.includes(test)) throw new Error(`Cross-runtime mounted regression is unresolved: ${test}.`);
  }
}

/** Fail-closed check of every checked-in census artifact. Regenerates the
 * document in memory, validates it, and byte-compares the manifest, census,
 * and group files; re-validates each admitted receipt and the execution
 * record against the current tree. */
export function checkCensusArtifacts(root = ROOT): void {
  const { doc, receipts } = generateCensus(root);
  validateCensusDoc(doc, root);
  const expectedManifest = `${JSON.stringify({ schema: MANIFEST_SCHEMA, task: "g18.001", entries: doc.manifest }, null, 2)}\n`;
  const expectedJson = `${JSON.stringify({ ...doc, manifest: undefined }, null, 2)}\n`;
  const expectedMd = censusMarkdown(doc);
  const expectedGroups = `${JSON.stringify({ schema: "poodle.g18-missing-capability-groups.v1", task: "g18.001", groups: doc.groups }, null, 2)}\n`;
  const record = loadExecutionRecord(root);
  const baselineCommit = record.body_hash_baseline_commit ?? record.source_commit;
  const baselineSource = testSourceAtCommit(root, baselineCommit);
  const compare = (relativePath: string, expected: string): void => {
    const actual = read(root, relativePath);
    if (actual !== expected) throw new Error(`Checked-in ${relativePath} disagrees with the generator; regenerate.`);
  };
  compare(MANIFEST_PATH, expectedManifest);
  compare(CENSUS_JSON_PATH, expectedJson);
  compare(CENSUS_MD_PATH, expectedMd);
  compare(GROUPS_PATH, expectedGroups);
  const onDisk = fs.existsSync(path.join(root, RECEIPT_DIR)) ? fs.readdirSync(path.join(root, RECEIPT_DIR)).sort() : [];
  const generated = receipts.map((receipt) => path.basename(receipt.file)).sort();
  if (onDisk.join("\n") !== generated.join("\n")) {
    throw new Error(`Checked-in ${RECEIPT_DIR} disagrees with the generator; regenerate.`);
  }
  for (const receipt of receipts) {
    const actual = read(root, receipt.file);
    // Validate the checked-in receipt first so a stale package version fails
    // with its own provenance message, not only a generic byte mismatch.
    validateReceiptFile(actual, root, baselineSource);
    if (actual !== receipt.content) throw new Error(`Checked-in ${receipt.file} disagrees with the generator; regenerate.`);
  }
  validateManifestRefs(doc.manifest, root);
  validateCrossRuntimeReport(root);
}

function main(): void {
  const args = process.argv.slice(2);
  if (args[0] === "--restore-default-selector") {
    const [sourceCommit] = args.slice(1);
    if (sourceCommit === undefined) {
      throw new Error("Usage: gpui-functionality-census.ts --restore-default-selector <full-selector-source-commit>");
    }
    const record = restoreDefaultExecutionIdentity(sourceCommit);
    const stats = writeCensusArtifacts();
    checkCensusArtifacts();
    console.log(
      `gpui-functionality-census: restored default selector ${record.command} at ${record.source_commit}; ${stats.rows} rows, ${stats.admitted} admitted, ${stats.receipts} mounted receipts.`,
    );
    return;
  }
  if (args[0] === "--record-passed") {
    const [runId, ...testNames] = args.slice(1);
    if (runId === undefined) throw new Error("Usage: gpui-functionality-census.ts --record-passed <run-id> <expected-test>...");
    const record = recordExpectedTestExecution(testNames, runId);
    const stats = writeCensusArtifacts();
    checkCensusArtifacts();
    console.log(
      `gpui-functionality-census: recorded ${testNames.length} expected tests as ${runId}; ${stats.rows} rows, ${stats.admitted} admitted, ${stats.receipts} mounted receipts.`,
    );
    return;
  }
  const check = process.argv.includes("--check");
  if (check) {
    checkCensusArtifacts();
    console.log("gpui-functionality-census: checked-in artifacts match the generator and all oracles hold.");
  } else {
    const stats = writeCensusArtifacts();
    checkCensusArtifacts();
    console.log(
      `gpui-functionality-census: ${stats.rows} rows, ${stats.admitted} rows with admitted capabilities, ${stats.receipts} mounted receipts.`,
    );
  }
}

if (import.meta.main) main();
