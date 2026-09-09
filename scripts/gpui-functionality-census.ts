import { createHash } from "node:crypto";
import { execSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import { deriveLiveRoster, EXPECTED_MOUNTED_BEHAVIOUR_TESTS } from "./parity-evidence-ledger";
import { deriveNucleusReceiptRows } from "./nucleus-parity-receipts";

const ROOT = path.resolve(import.meta.dir, "..");
const HEADLESS_TEST_FILE = "packages/gpui/preview/tests/headless_regressions.rs";
const GPUI_LOCKFILE = "packages/gpui/preview/Cargo.lock";
const NATIVE_SELECTOR = "effigy regressions:native";

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
export const EXECUTION_SCHEMA = "poodle.g18-expected-test-execution.v1";
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
  /all 175 [a-z ]*(proven|proved|complete|passing|pass)\b/i,
  /construction (proves|proving|means|confirms) [a-z ]*functional/i,
  /\b175\/175\b[^.\n]*functional(?! completion)/i,
];

const ASSERT_RE = /\bassert(_eq|_ne)?!\s*\(/;
const PRODUCTION_RENDER_RE = /poodle_render::(?!color::)\w+|node_compat::\w+/;

/// Per-axis source signals for a headless regression body. Every signal must be
/// read as production-backend evidence: admission additionally requires the
/// production mount (run_headless + HeadlessDriver + production renderer), so a
/// renderer unit test or a direct handler call can never satisfy any axis.
export const AXIS_TEST_SIGNALS: Record<CensusAxis, RegExp[]> = {
  semantic: [PRODUCTION_RENDER_RE, ASSERT_RE],
  events: [/counting_handler/i, /payloads?\.\s*lock/i, /assert[^;]*(emit|payload|change|commit|callback)/i],
  pointer: [/pointer_activate|pointer_press|dispatch_pointer|mouse_|simulate_click|\.click\(/i],
  keyboard_focus: [/dispatch_key|focus_element|focus_state_for|focus_handle_for|roving|key_press|press_key|keyboard_/i],
  accessibility: [/a11y\.|NodeToggled|NodeRole|\baria\b|accessible|announce/i],
  visual: [/rem_to_px|resolve_color|resolve_space|resolve_opacity|resolve_radius|_geometry|Geometry|computed_rect|dimensions/i],
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

export type ExecutionRecord = {
  schema: string;
  command: string;
  source_commit: string;
  lockfile: string;
  lockfile_sha256: string;
  run_id: string;
  results: Record<string, { outcome: "passed" | "failed" | "not-run"; body_sha256: string }>;
};

export function sha256Hex(text: string): string {
  return createHash("sha256").update(text).digest("hex");
}

function read(root: string, relativePath: string): string {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function headCommit(root: string): string {
  return execSync("git rev-parse HEAD", { cwd: root, encoding: "utf8" }).trim();
}

/** Extract the top-level test function body, or undefined when the test is absent (renamed/stale). */
export function extractTestBody(root: string, testName: string): string | undefined {
  const file = path.join(root, HEADLESS_TEST_FILE);
  if (!fs.existsSync(file)) return undefined;
  const lines = fs.readFileSync(file, "utf8").split("\n");
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

/** Non-test helpers that resolve to the production renderer, closed transitively:
 * direct poodle_render/node_compat users plus helpers that only call those. */
export function rendererHelperNames(source: string): string[] {
  const helpers = topLevelFns(source).filter((fn) => !fn.test);
  const imported = rendererImportNames(source);
  const callsRenderer = (body: string, known: Set<string>): boolean => {
    if (PRODUCTION_RENDER_RE.test(body)) return true;
    if (imported.some((name) => new RegExp(`\\b${name}\\s*\\(`).test(body))) return true;
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
  const renderer =
    rendererDirect ||
    imported.some((name) => calls(name)) ||
    rendererHelpers.some((name) => calls(name));
  const mount = driverDirect || mountHelpers.some((name) => calls(name));
  return renderer && mount;
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
    const matched = observed
      ? AXIS_TEST_SIGNALS[axis].filter((signal) => signal.test(body)).map((signal) => String(signal))
      : [];
    signals[axis] = matched;
    if (matched.length >= 1) axes.push(axis);
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
    const events = headingBody(contract, /^## 5\. /);
    const keyboard = headingBody(contract, /^### Keyboard/);
    const focus = headingBody(contract, /^### Focus/);
    const eventsNone = /^\|\s*none\s*\|/m.test(events.body) || /layout primitive only|no events/i.test(events.body);
    const keyboardNone = /^\|\s*none\s*\|/m.test(keyboard.body);
    const focusNeutral = /not focusable/i.test(focus.body);
    const required: CensusAxis[] = ["semantic", "accessibility", "visual"];
    const notApplicable: ManifestNotApplicable[] = [];
    if (eventsNone) {
      notApplicable.push({
        axis: "events",
        reason: "Contract declares no events; layout or display primitive only.",
        contractRef: `${contractPath}#${events.heading || "5. Events"}`,
      });
    } else {
      required.push("events");
    }
    if (eventsNone && keyboardNone) {
      notApplicable.push({
        axis: "pointer",
        reason: "Contract declares the non-interactive boundary: no events and not focusable, so no pointer interaction exists to prove.",
        contractRef: `${contractPath}#${keyboard.heading || "Keyboard"}`,
      });
    } else {
      required.push("pointer");
    }
    if (keyboardNone && focusNeutral) {
      notApplicable.push({
        axis: "keyboard_focus",
        reason: "Contract declares the control not focusable with no keyboard behavior.",
        contractRef: `${contractPath}#${keyboard.heading || "Keyboard"}`,
      });
    } else {
      required.push("keyboard_focus");
    }
    const order = (axis: CensusAxis): number => CENSUS_AXES.indexOf(axis);
    required.sort((a, b) => order(a) - order(b));
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

export function validateExecutionRecord(record: ExecutionRecord, root: string): void {
  if (record.schema !== EXECUTION_SCHEMA) throw new Error(`Execution record schema is ${record.schema}.`);
  if (!/^[0-9a-f]{40}$/.test(record.source_commit)) throw new Error("Execution record needs a 40-hex source commit.");
  if (record.command !== NATIVE_SELECTOR) throw new Error(`Execution record must cite ${NATIVE_SELECTOR}.`);
  const lockfile = read(root, record.lockfile);
  if (sha256Hex(lockfile) !== record.lockfile_sha256) {
    throw new Error("GPUI lockfile changed since the recorded execution; re-run the expected tests and regenerate the census.");
  }
  const expectedTests = Object.values(EXPECTED_MOUNTED_BEHAVIOUR_TESTS).flatMap((tests) =>
    Array.isArray(tests) ? tests : [tests],
  );
  for (const test of expectedTests) {
    const entry = record.results[test];
    if (entry === undefined) throw new Error(`Execution record has no result for expected test ${test}.`);
    if (entry.outcome !== "passed") throw new Error(`Expected test ${test} did not pass in the recorded execution.`);
    const current = testBodySha256(root, test);
    if (current === undefined) throw new Error(`Expected test ${test} is stale: it no longer exists in ${HEADLESS_TEST_FILE}.`);
    if (testIsIgnored(root, test)) throw new Error(`Expected test ${test} is ignored and never executes.`);
    if (current !== entry.body_sha256) {
      throw new Error(`Expected test ${test} changed since the recorded execution; re-run it before admitting claims.`);
    }
  }
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
  const commit = headCommit(root);
  const lockText = read(root, GPUI_LOCKFILE);
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
      const axes = admission.axes.filter((axis) => entry.required.includes(axis));
      const skipped = admission.axes.filter((axis) => !entry.required.includes(axis));
      for (const axis of skipped) {
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
        content: `${JSON.stringify(
          {
            schema: RECEIPT_SCHEMA,
            component: component.name,
            test,
            selector: NATIVE_SELECTOR,
            scenario_id: null,
            scenario_note:
              "Expected-test rows carry no Nucleus scenario; the named mounted test is the scenario identity.",
            contract_claims: axes,
            signals: Object.fromEntries(axes.map((axis) => [axis, admission.signals[axis]])),
            production_path_observation: {
              observed: true,
              mount: "HeadlessDriver",
              render_path: "poodle_render -> poodle_gpui_node_backend::to_gpui",
              input_dispatch: "gpui-test-platform-dispatch",
            },
            package: "poodle-gpui-preview",
            package_version: "0.3.0",
            source_commit: record.source_commit,
            lockfile: GPUI_LOCKFILE,
            lockfile_sha256: record.lockfile_sha256,
            distribution: "workspace",
            execution: {
              command: record.command,
              run_id: record.run_id,
              outcome: "passed",
              body_sha256: record.results[test].body_sha256,
            },
            observed: axes.map((axis) => OBSERVED_SENTENCES[axis]),
            outcome: "passed",
          },
          null,
          2,
        )}\n`,
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
    denominator: { public: 176, portable: 175, notApplicable: ["MeterSurface"] },
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
        "bounded named regression set, not a 175-component behaviour pass; the g18.001 capability census is the compilation input for repair tranches",
      note: "Construction is not functional completion. A passing route, a test name, or one passing test never marks a component complete.",
    },
  };
  void lockText;
  return { doc, receipts };
}

export function validateCensusDoc(doc: CensusDoc): void {
  if (doc.schema !== CENSUS_SCHEMA) throw new Error(`Census schema is ${doc.schema}.`);
  if (doc.task !== "g18.001") throw new Error(`Census task is ${doc.task}.`);
  if (!/^[0-9a-f]{40}$/.test(doc.source_commit)) throw new Error("Census needs a 40-hex source commit.");
  if ([...doc.axes].sort().join(",") !== [...CENSUS_AXES].sort().join(",")) {
    throw new Error("Census must use exactly the closed capability axes.");
  }
  if (doc.denominator.public !== 176 || doc.denominator.portable !== 175) {
    throw new Error("Census denominator must stay 176 public / 175 portable.");
  }
  if (doc.denominator.notApplicable.length !== 1 || doc.denominator.notApplicable[0] !== "MeterSurface") {
    throw new Error("MeterSurface is the single contract-approved non-portable row.");
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
  if (!doc.crossRuntime.mountedScope.includes("bounded") || !doc.crossRuntime.mountedScope.includes("not a 175-component behaviour pass")) {
    throw new Error("Census mounted scope must stay bounded and refuse roster-wide promotion.");
  }
  if (doc.rows.length !== 176) throw new Error(`Census must carry exactly 176 rows, found ${doc.rows.length}.`);
}

export function censusMarkdown(doc: CensusDoc): string {
  const lines: string[] = [];
  lines.push("# g18.001 — Contract-bound GPUI functionality census");
  lines.push("");
  lines.push(`Source commit: \`${doc.source_commit}\``);
  lines.push(`Denominator: **${doc.denominator.public}** public / **${doc.denominator.portable}** portable; \`${doc.denominator.notApplicable[0]}\` is the single contract-approved non-portable row.`);
  lines.push("");
  lines.push("<!-- g18-census-method -->");
  lines.push("## Method");
  lines.push("");
  lines.push("Capability axes are closed: `semantic`, `events`, `pointer`, `keyboard_focus`, `accessibility`, `visual`.");
  lines.push("Each portable row requires the axes its contract declares; `not-applicable` needs an exact contract section and can never cite platform state.");
  lines.push("Admitted capabilities trace to validated Nucleus M1/A1/V1 receipts or to retained expected tests that ran green on the recorded source and dependency identity, mount the production renderer plus GPUI node backend, and show the claimed axis signals in their bodies.");
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

function validateReceiptFile(content: string, root: string): void {
  const receipt = JSON.parse(content) as {
    schema?: string;
    component?: string;
    test?: string;
    selector?: string;
    contract_claims?: string[];
    source_commit?: string;
    lockfile_sha256?: string;
    execution?: { outcome?: string; body_sha256?: string };
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
  if (!/^[0-9a-f]{40}$/.test(receipt.source_commit ?? "")) throw new Error("Mounted receipt needs a 40-hex source commit.");
  const lockText = read(root, GPUI_LOCKFILE);
  if (sha256Hex(lockText) !== receipt.lockfile_sha256) {
    throw new Error(`Mounted receipt for ${receipt.component} predates the current GPUI lockfile; regenerate.`);
  }
  if (receipt.execution?.outcome !== "passed") throw new Error(`Mounted receipt for ${receipt.component} has no passing execution.`);
  const body = extractTestBody(root, receipt.test);
  if (body === undefined) throw new Error(`Mounted receipt test ${receipt.test} is stale.`);
  if (testIsIgnored(root, receipt.test)) throw new Error(`Mounted receipt test ${receipt.test} is ignored.`);
  const admission = admitTestAxes(body, read(root, HEADLESS_TEST_FILE), receipt.test);
  if (!admission.production) throw new Error(`Mounted receipt test ${receipt.test} bypasses the mounted backend.`);
  for (const axis of receipt.contract_claims) {
    if (!admission.axes.includes(axis as CensusAxis)) {
      throw new Error(`Mounted receipt for ${receipt.component} claims ${axis} its test body does not show.`);
    }
  }
  if (sha256Hex(body) !== receipt.execution.body_sha256) {
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
  if (!scope.includes("bounded") || !scope.includes("not a 175-component behaviour pass")) {
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
    if (actual !== receipt.content) throw new Error(`Checked-in ${receipt.file} disagrees with the generator; regenerate.`);
    validateReceiptFile(actual, root);
  }
  validateManifestRefs(doc.manifest, root);
  validateCrossRuntimeReport(root);
}

function main(): void {
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
