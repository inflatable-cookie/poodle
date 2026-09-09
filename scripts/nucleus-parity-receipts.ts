import { createHash } from "node:crypto";
import { execFileSync } from "node:child_process";
import { existsSync, lstatSync, mkdirSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import {
  A1_GPUI_RUNTIME,
  A1_SNAPSHOT_SCHEMA,
  A1_SVELTE_RUNTIME,
  diffSnapshotNodes,
  GPUI_RUN_RECORD,
  readScenario,
  sha256Hex,
  SVELTE_RUN_RECORD,
  type A1Exclusion,
  type SnapshotFile,
} from "../test/nucleus-a11y/contract";
import { GEOMETRY, PIXELS, ROLES } from "../test/visual/button-comparison/policy";

export const NUCLEUS_MANIFEST_PATH = "docs/evidence/nucleus/nucleus-parity-manifest.json";
export const NUCLEUS_MANIFEST_SCHEMA_PATH = "docs/evidence/nucleus/nucleus-parity-manifest.schema.json";
export const NUCLEUS_SCHEMA_PATH = "docs/evidence/nucleus/nucleus-parity-receipt.schema.json";
export const NUCLEUS_RECEIPT_DIR = "docs/evidence/nucleus/nucleus-parity-receipts";
export const NUCLEUS_RECEIPT_SCHEMA = "poodle.g16-nucleus-parity-receipt.v1";
export const NUCLEUS_RUNTIME = "gpui-headless";
export const NUCLEUS_COMMAND = "effigy regressions:native";

const ROOT = path.resolve(import.meta.dir, "..");
const SOURCE_PATHS = ["packages/gpui/preview", "packages/gpui/adapter", "packages/render", "packages/contracts"];

export type NucleusEntry = {
  id: string;
  name: string;
  rendered?: boolean;
  scenario_id: string;
  direct_dependencies: string[];
  expected_selector: string;
  expected_test: string | null;
};

export type NucleusResolution = {
  package: string;
  version: string;
  source_commit: string;
  lockfile: string;
  lockfile_sha256: string;
  distribution: "workspace";
  lock_resolution: Array<{
    name: string;
    version: string;
    source: "crates.io" | "workspace";
    checksum?: string;
  }>;
};

export type NucleusArtifact = {
  path: string;
  sha256: string;
};

export type NucleusManifest = {
  $schema: string;
  schema: string;
  program: "g16.062";
  rendered_component_count: 29;
  prerequisites: NucleusEntry[];
  resolution: NucleusResolution;
  components: NucleusEntry[];
};

export type NucleusProofLevel = "M1" | "A1" | "V1";

/// g16.111 A1: the paired accessibility record. Both snapshots are committed
/// artifacts; the diff is empty for a pass.
export type NucleusAccessibilityBlock = {
  scenario_path: string;
  scenario_sha256: string;
  gpui_snapshot_path: string;
  gpui_snapshot_sha256: string;
  svelte_snapshot_path: string;
  svelte_snapshot_sha256: string;
  web_only_exclusions: A1Exclusion[];
  diff: unknown[];
};

export type NucleusReceipt = {
  schema: typeof NUCLEUS_RECEIPT_SCHEMA;
  component: string;
  scenario_id: string;
  proof_level: "M1" | "A1";
  runtime: typeof NUCLEUS_RUNTIME;
  command: typeof NUCLEUS_COMMAND;
  package: string;
  package_version: string;
  source_commit: string;
  lockfile: string;
  lockfile_sha256: string;
  lock_resolution: NucleusResolution["lock_resolution"];
  distribution: "workspace";
  production_path_observation: {
    observed: true;
    mount: "HeadlessDriver";
    render_path: "poodle_render -> poodle_gpui_node_backend::to_gpui";
    input_dispatch: "gpui-test-platform-dispatch";
  };
  actions: string[];
  assertions: string[];
  outcome: "passed";
  artifact_paths: NucleusArtifact[];
  accessibility?: NucleusAccessibilityBlock;
};

export type NucleusReceiptRow = {
  entry: NucleusEntry;
  /// The validated M1 receipt, when one exists.
  receiptPath?: string;
  receipt?: NucleusReceipt;
  /// The validated A1 receipt, when one exists (g16.111).
  a1ReceiptPath?: string;
  a1Receipt?: NucleusReceipt;
  /// The validated V1 receipt, when one exists (g17.001).
  v1ReceiptPath?: string;
  v1Receipt?: NucleusV1Receipt;
};

function rootPath(root: string, relativePath: string): string {
  return path.join(root, relativePath);
}

function readJson<T>(root: string, relativePath: string): T {
  return JSON.parse(readFileSync(rootPath(root, relativePath), "utf8")) as T;
}

function assert(condition: unknown, message: string, errors: string[]): void {
  if (!condition) errors.push(message);
}

type JsonObject = Record<string, unknown>;

function isJsonObject(value: unknown): value is JsonObject {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function assertExactObject(
  value: unknown,
  label: string,
  required: readonly string[],
  optional: readonly string[],
  errors: string[],
): value is JsonObject {
  if (!isJsonObject(value)) {
    errors.push(`${label} must be an object`);
    return false;
  }
  const allowed = new Set([...required, ...optional]);
  for (const key of Object.keys(value)) {
    if (!allowed.has(key)) errors.push(`${label} has unexpected property ${key}`);
  }
  for (const key of required) {
    if (!Object.hasOwn(value, key)) errors.push(`${label} is missing required property ${key}`);
  }
  return Object.keys(value).every((key) => allowed.has(key)) && required.every((key) => Object.hasOwn(value, key));
}

function assertArray(value: unknown, label: string, errors: string[]): value is unknown[] {
  if (!Array.isArray(value)) {
    errors.push(`${label} must be an array`);
    return false;
  }
  return true;
}

const ENTRY_REQUIRED_KEYS = ["id", "name", "scenario_id", "direct_dependencies", "expected_selector", "expected_test"];
const ENTRY_OPTIONAL_KEYS = ["rendered"];
const RESOLUTION_REQUIRED_KEYS = ["package", "version", "source_commit", "lockfile", "lockfile_sha256", "distribution", "lock_resolution"];
const LOCKED_PACKAGE_REQUIRED_KEYS = ["name", "version", "source"];
const LOCKED_PACKAGE_OPTIONAL_KEYS = ["checksum"];
const RECEIPT_REQUIRED_KEYS = [
  "schema",
  "component",
  "scenario_id",
  "proof_level",
  "runtime",
  "command",
  "package",
  "package_version",
  "source_commit",
  "lockfile",
  "lockfile_sha256",
  "lock_resolution",
  "distribution",
  "production_path_observation",
  "actions",
  "assertions",
  "outcome",
  "artifact_paths",
];
const RECEIPT_OPTIONAL_KEYS = ["accessibility"];
const OBSERVATION_REQUIRED_KEYS = ["observed", "mount", "render_path", "input_dispatch"];
const ARTIFACT_REQUIRED_KEYS = ["path", "sha256"];
const ACCESSIBILITY_REQUIRED_KEYS = [
  "scenario_path",
  "scenario_sha256",
  "gpui_snapshot_path",
  "gpui_snapshot_sha256",
  "svelte_snapshot_path",
  "svelte_snapshot_sha256",
  "web_only_exclusions",
  "diff",
];
const EXCLUSION_REQUIRED_KEYS = ["attribute", "reason"];

function manifestShapeErrors(manifest: unknown): string[] {
  const errors: string[] = [];
  if (!assertExactObject(manifest, "manifest", ["$schema", "schema", "program", "rendered_component_count", "prerequisites", "resolution", "components"], [], errors)) {
    return errors;
  }

  for (const [label, value] of [["manifest prerequisites", manifest.prerequisites], ["manifest components", manifest.components]] as const) {
    if (!assertArray(value, label, errors)) continue;
    value.forEach((entry, index) => assertExactObject(entry, `${label}[${index}]`, ENTRY_REQUIRED_KEYS, ENTRY_OPTIONAL_KEYS, errors));
  }
  if (assertExactObject(manifest.resolution, "manifest resolution", RESOLUTION_REQUIRED_KEYS, [], errors)) {
    if (assertArray(manifest.resolution.lock_resolution, "manifest resolution lock_resolution", errors)) {
      manifest.resolution.lock_resolution.forEach((packageEntry, index) =>
        assertExactObject(packageEntry, `manifest resolution lock_resolution[${index}]`, LOCKED_PACKAGE_REQUIRED_KEYS, LOCKED_PACKAGE_OPTIONAL_KEYS, errors),
      );
    }
  }
  return errors;
}

function receiptShapeErrors(receipt: unknown): string[] {
  if (isJsonObject(receipt) && receipt.proof_level === "V1") return v1ReceiptShapeErrors(receipt);
  const errors: string[] = [];
  if (!assertExactObject(receipt, "receipt", RECEIPT_REQUIRED_KEYS, RECEIPT_OPTIONAL_KEYS, errors)) return errors;
  if (assertArray(receipt.lock_resolution, "receipt lock_resolution", errors)) {
    receipt.lock_resolution.forEach((packageEntry, index) =>
      assertExactObject(packageEntry, `receipt lock_resolution[${index}]`, LOCKED_PACKAGE_REQUIRED_KEYS, LOCKED_PACKAGE_OPTIONAL_KEYS, errors),
    );
  }
  assertExactObject(receipt.production_path_observation, "receipt production_path_observation", OBSERVATION_REQUIRED_KEYS, [], errors);
  assertArray(receipt.actions, "receipt actions", errors);
  assertArray(receipt.assertions, "receipt assertions", errors);
  if (assertArray(receipt.artifact_paths, "receipt artifact_paths", errors)) {
    receipt.artifact_paths.forEach((artifact, index) =>
      assertExactObject(artifact, `receipt artifact_paths[${index}]`, ARTIFACT_REQUIRED_KEYS, [], errors),
    );
  }
  if (Object.hasOwn(receipt, "accessibility")) {
    if (assertExactObject(receipt.accessibility, "receipt accessibility", ACCESSIBILITY_REQUIRED_KEYS, [], errors)) {
      if (assertArray(receipt.accessibility.web_only_exclusions, "receipt accessibility web_only_exclusions", errors)) {
        receipt.accessibility.web_only_exclusions.forEach((exclusion, index) =>
          assertExactObject(exclusion, `receipt accessibility web_only_exclusions[${index}]`, EXCLUSION_REQUIRED_KEYS, [], errors),
        );
      }
      assertArray(receipt.accessibility.diff, "receipt accessibility diff", errors);
    }
  }
  return errors;
}

function sourceCommitIsValid(value: string): boolean {
  return /^[0-9a-f]{40}$/.test(value);
}

function sha256File(filePath: string): string {
  return createHash("sha256").update(readFileSync(filePath)).digest("hex");
}

function cargoPackage(root: string): { name: string; version: string } {
  const source = readFileSync(rootPath(root, "packages/gpui/preview/Cargo.toml"), "utf8");
  const packageBlock = source.match(/\[package\]([\s\S]*?)(?=\n\[|$)/)?.[1] ?? "";
  const name = packageBlock.match(/^name\s*=\s*"([^"]+)"/m)?.[1];
  const version = packageBlock.match(/^version\s*=\s*"([^"]+)"/m)?.[1];
  if (name === undefined || version === undefined) throw new Error("GPUI preview Cargo package identity is incomplete.");
  return { name, version };
}

function lockPackages(lockfile: string): Map<string, { version: string; source?: string; checksum?: string }> {
  const packages = new Map<string, { version: string; source?: string; checksum?: string }>();
  for (const block of lockfile.split("\n\n")) {
    if (!block.startsWith("[[package]]")) continue;
    const name = block.match(/^name = "([^"]+)"/m)?.[1];
    const version = block.match(/^version = "([^"]+)"/m)?.[1];
    if (name === undefined || version === undefined) continue;
    packages.set(name, {
      version,
      source: block.match(/^source = "([^"]+)"/m)?.[1],
      checksum: block.match(/^checksum = "([^"]+)"/m)?.[1],
    });
  }
  return packages;
}

function normalizedLockSource(source: string | undefined): "crates.io" | "workspace" {
  return source?.startsWith("registry+") ? "crates.io" : "workspace";
}

export function loadNucleusManifest(root = ROOT): NucleusManifest {
  const manifest = readJson<NucleusManifest>(root, NUCLEUS_MANIFEST_PATH);
  validateNucleusManifest(manifest, root);
  return manifest;
}

export function validateNucleusManifest(manifest: NucleusManifest, root = ROOT): void {
  const shapeErrors = manifestShapeErrors(manifest);
  if (shapeErrors.length > 0) throw new Error(shapeErrors.join("\n"));
  const errors: string[] = [];
  assert(manifest.$schema === "./nucleus-parity-manifest.schema.json", "manifest $schema is not the manifest schema", errors);
  assert(manifest.schema === "poodle.g16.062-nucleus-parity-manifest.v1", "manifest schema is not current", errors);
  assert(manifest.program === "g16.062", "manifest program is not g16.062", errors);
  assert(manifest.rendered_component_count === 29, `manifest rendered denominator is ${manifest.rendered_component_count}, expected 29`, errors);
  assert(manifest.prerequisites.length === 1, "manifest must contain one prerequisite", errors);
  const prerequisite = manifest.prerequisites[0];
  assert(prerequisite?.name === "IconProvider", "IconProvider must be the separate prerequisite", errors);
  assert(prerequisite?.rendered === false, "IconProvider must not be a rendered denominator row", errors);
  assert(manifest.components.length === 29, `manifest has ${manifest.components.length} rendered rows, expected 29`, errors);

  const entries = [...manifest.prerequisites, ...manifest.components];
  const ids = entries.map((entry) => entry.id);
  const names = entries.map((entry) => entry.name);
  const scenarios = entries.map((entry) => entry.scenario_id);
  assert(new Set(ids).size === ids.length, "manifest entry ids must be unique", errors);
  assert(new Set(names).size === names.length, "manifest entry names must be unique", errors);
  assert(new Set(scenarios).size === scenarios.length, "manifest scenario ids must be unique", errors);
  const knownIds = new Set(ids);
  for (const entry of entries) {
    assert(/^[a-z0-9-]+$/.test(entry.id), `invalid manifest id ${entry.id}`, errors);
    assert(entry.expected_selector === NUCLEUS_COMMAND, `${entry.name} expected selector is not ${NUCLEUS_COMMAND}`, errors);
    assert(entry.expected_test === null || /^[a-z0-9_]+$/.test(entry.expected_test), `${entry.name} expected test is not a Rust test name`, errors);
    for (const dependency of entry.direct_dependencies) {
      assert(knownIds.has(dependency), `${entry.name} has unmanifested dependency ${dependency}`, errors);
      assert(dependency !== entry.id, `${entry.name} depends on itself`, errors);
    }
  }

  const packageIdentity = cargoPackage(root);
  assert(packageIdentity.name === manifest.resolution.package, `manifest package ${manifest.resolution.package} does not match Cargo.toml ${packageIdentity.name}`, errors);
  assert(packageIdentity.version === manifest.resolution.version, `manifest version ${manifest.resolution.version} does not match Cargo.toml ${packageIdentity.version}`, errors);
  assert(sourceCommitIsValid(manifest.resolution.source_commit), "manifest source_commit must be a 40-character lowercase commit", errors);
  assert(manifest.resolution.lockfile === "packages/gpui/preview/Cargo.lock", "manifest lockfile must be the GPUI preview lockfile", errors);
  const lockfilePath = rootPath(root, manifest.resolution.lockfile);
  assert(existsSync(lockfilePath), `manifest lockfile is missing: ${manifest.resolution.lockfile}`, errors);
  if (existsSync(lockfilePath)) {
    assert(sha256File(lockfilePath) === manifest.resolution.lockfile_sha256, "manifest lockfile SHA-256 does not match", errors);
    const locked = lockPackages(readFileSync(lockfilePath, "utf8"));
    for (const expected of manifest.resolution.lock_resolution) {
      const actual = locked.get(expected.name);
      assert(actual !== undefined, `manifest lock resolution is missing ${expected.name}`, errors);
      if (actual === undefined) continue;
      assert(actual.version === expected.version, `${expected.name} lock version differs: expected ${expected.version}, found ${actual.version}`, errors);
      assert(normalizedLockSource(actual.source) === expected.source, `${expected.name} lock source differs`, errors);
      if (expected.checksum !== undefined) assert(actual.checksum === expected.checksum, `${expected.name} lock checksum differs`, errors);
    }
  }

  if (errors.length > 0) throw new Error(errors.join("\n"));
}

function assertReceiptStringArray(value: unknown, label: string, errors: string[]): void {
  assert(Array.isArray(value) && value.length > 0 && value.every((item) => typeof item === "string" && item.length > 0), `${label} must be a non-empty string array`, errors);
}

function repositoryRelativeArtifactPath(root: string, artifactPath: unknown): string | undefined {
  if (
    typeof artifactPath !== "string" ||
    artifactPath.length === 0 ||
    path.isAbsolute(artifactPath) ||
    /^[A-Za-z]:/.test(artifactPath) ||
    artifactPath.includes("\\") ||
    artifactPath.includes("\0") ||
    artifactPath.split("/").some((segment) => segment.length === 0 || segment === "." || segment === "..")
  ) {
    return undefined;
  }
  const repositoryRoot = path.resolve(root);
  const resolved = path.resolve(repositoryRoot, artifactPath);
  const relativeToRoot = path.relative(repositoryRoot, resolved);
  if (relativeToRoot === ".." || relativeToRoot.startsWith(`..${path.sep}`) || path.isAbsolute(relativeToRoot)) return undefined;
  return resolved;
}

function validateArtifact(artifact: unknown, index: number, root: string, errors: string[]): void {
  const label = `receipt artifact_paths[${index}]`;
  if (!isJsonObject(artifact)) return;
  const artifactPath = artifact.path;
  const hash = artifact.sha256;
  const filePath = repositoryRelativeArtifactPath(root, artifactPath);
  assert(filePath !== undefined, `${label} path must be repository-relative`, errors);
  assert(typeof hash === "string" && /^[0-9a-f]{64}$/.test(hash), `${label} SHA-256 must be 64 lowercase hex characters`, errors);
  if (filePath === undefined || typeof hash !== "string" || !/^[0-9a-f]{64}$/.test(hash)) return;
  assert(existsSync(filePath), `${label} path does not exist: ${artifactPath}`, errors);
  if (!existsSync(filePath)) return;
  try {
    const file = lstatSync(filePath);
    assert(file.isFile(), `${label} path is not a regular file: ${artifactPath}`, errors);
    if (file.isFile()) assert(sha256File(filePath) === hash, `${label} SHA-256 does not match: ${artifactPath}`, errors);
  } catch {
    errors.push(`${label} path cannot be read: ${artifactPath}`);
  }
}

export function validateNucleusReceipt(receipt: NucleusReceipt | NucleusV1Receipt, manifest = loadNucleusManifest(), root = ROOT): void {
  const shapeErrors = receiptShapeErrors(receipt);
  if (shapeErrors.length > 0) throw new Error(shapeErrors.join("\n"));
  const errors: string[] = [];
  const entry = [...manifest.prerequisites, ...manifest.components].find((candidate) => candidate.name === receipt.component);
  assert(receipt.schema === NUCLEUS_RECEIPT_SCHEMA, "receipt schema is not current", errors);
  assert(entry !== undefined && entry.rendered !== false, `receipt component is not a rendered manifest entry: ${receipt.component}`, errors);
  if (entry !== undefined) assert(receipt.scenario_id === entry.scenario_id, `${receipt.component} receipt scenario does not match the manifest`, errors);
  assert(
    receipt.proof_level === "M1" || receipt.proof_level === "A1" || receipt.proof_level === "V1",
    "receipt proof level must be M1, A1, or V1",
    errors,
  );
  if (receipt.proof_level === "V1") {
    validateV1Receipt(receipt, manifest, root, errors);
    if (errors.length > 0) throw new Error(errors.join("\n"));
    return;
  }
  if (receipt.proof_level === "M1") {
    assert(receipt.accessibility === undefined, "an M1 receipt carries no accessibility block", errors);
  } else if (receipt.proof_level === "A1") {
    assert(receipt.accessibility !== undefined, "an A1 receipt requires an accessibility block", errors);
    if (receipt.accessibility !== undefined) validateAccessibilityBlock(receipt, root, errors);
  }
  assert(receipt.runtime === NUCLEUS_RUNTIME, `receipt runtime must be ${NUCLEUS_RUNTIME}`, errors);
  assert(receipt.command === NUCLEUS_COMMAND, `receipt command must be ${NUCLEUS_COMMAND}`, errors);
  assert(receipt.package === manifest.resolution.package, "receipt package does not match manifest resolution", errors);
  assert(receipt.package_version === manifest.resolution.version, "receipt package version does not match manifest resolution", errors);
  assert(receipt.source_commit === manifest.resolution.source_commit, "receipt source commit does not match manifest resolution", errors);
  assert(receipt.lockfile === manifest.resolution.lockfile, "receipt lockfile does not match manifest resolution", errors);
  assert(receipt.lockfile_sha256 === manifest.resolution.lockfile_sha256, "receipt lockfile SHA-256 does not match manifest resolution", errors);
  assert(JSON.stringify(receipt.lock_resolution) === JSON.stringify(manifest.resolution.lock_resolution), "receipt lock resolution does not match manifest resolution", errors);
  assert(receipt.distribution === manifest.resolution.distribution, "receipt distribution does not match manifest resolution", errors);
  const observation = receipt.production_path_observation;
  assert(observation?.observed === true, "receipt lacks an observed production-path mount", errors);
  assert(observation?.mount === "HeadlessDriver", "receipt mount is not HeadlessDriver", errors);
  assert(observation?.render_path === "poodle_render -> poodle_gpui_node_backend::to_gpui", "receipt render path is not the production path", errors);
  assert(observation?.input_dispatch === "gpui-test-platform-dispatch", "receipt input was not dispatched through the GPUI test platform", errors);
  assertReceiptStringArray(receipt.actions, "receipt actions", errors);
  assertReceiptStringArray(receipt.assertions, "receipt assertions", errors);
  assert(receipt.outcome === "passed", "receipt outcome is not passed", errors);
  for (const [index, artifact] of receipt.artifact_paths.entries()) validateArtifact(artifact, index, root, errors);
  assert(sourceCommitIsValid(receipt.source_commit), "receipt source_commit must be a 40-character lowercase commit", errors);
  const encoded = JSON.stringify(receipt);
  assert(!encoded.includes("/Users/") && !encoded.includes("/private/") && !encoded.includes("timestamp"), "receipt contains a machine path or timestamp", errors);
  if (errors.length > 0) throw new Error(errors.join("\n"));
}

function readSnapshot(root: string, relativePath: string, label: string, errors: string[]): SnapshotFile | undefined {
  const filePath = repositoryRelativeArtifactPath(root, relativePath);
  if (filePath === undefined || !existsSync(filePath)) {
    errors.push(`${label} does not exist: ${relativePath}`);
    return undefined;
  }
  try {
    return JSON.parse(readFileSync(filePath, "utf8")) as SnapshotFile;
  } catch {
    errors.push(`${label} does not parse: ${relativePath}`);
    return undefined;
  }
}

/// g16.111: an A1 receipt is evidence only when the scenario it names is the
/// committed one (hash), both snapshots are the committed artifacts (hash),
/// both carry a real run record for their runtime, both ran against that
/// scenario hash, and the recomputed diff is empty.
function validateAccessibilityBlock(receipt: NucleusReceipt, root: string, errors: string[]): void {
  const block = receipt.accessibility;
  if (block === undefined) return;
  const row = block.scenario_path.match(/^test\/nucleus-a11y\/scenarios\/([a-z0-9-]+)\.json$/)?.[1];
  assert(row !== undefined, `receipt accessibility scenario_path is not a shared scenario file: ${block.scenario_path}`, errors);
  if (row === undefined) return;
  assert(block.gpui_snapshot_path === `test/nucleus-a11y/snapshots/${row}.gpui.json`, "receipt accessibility gpui_snapshot_path does not belong to the scenario row", errors);
  assert(block.svelte_snapshot_path === `test/nucleus-a11y/snapshots/${row}.svelte.json`, "receipt accessibility svelte_snapshot_path does not belong to the scenario row", errors);

  let loaded: ReturnType<typeof readScenario> | undefined;
  try {
    loaded = readScenario(root, row);
  } catch (error) {
    errors.push(`receipt accessibility scenario cannot be read: ${error instanceof Error ? error.message : String(error)}`);
    return;
  }
  assert(loaded.sha256 === block.scenario_sha256, "receipt accessibility scenario SHA-256 does not match the committed scenario file", errors);
  assert(loaded.scenario.component === receipt.component, "receipt accessibility scenario component does not match the receipt", errors);
  assert(loaded.scenario.scenario_id === receipt.scenario_id, "receipt accessibility scenario id does not match the receipt", errors);
  assert(
    JSON.stringify(loaded.scenario.web_only_exclusions) === JSON.stringify(block.web_only_exclusions),
    "receipt accessibility web_only_exclusions do not match the scenario file",
    errors,
  );

  for (const [label, relativePath, expectedHash] of [
    ["receipt accessibility gpui snapshot", block.gpui_snapshot_path, block.gpui_snapshot_sha256],
    ["receipt accessibility svelte snapshot", block.svelte_snapshot_path, block.svelte_snapshot_sha256],
  ] as const) {
    const filePath = repositoryRelativeArtifactPath(root, relativePath);
    if (filePath !== undefined && existsSync(filePath)) {
      assert(sha256File(filePath) === expectedHash, `${label} SHA-256 does not match: ${relativePath}`, errors);
    }
    assert(
      receipt.artifact_paths.some((artifact) => artifact.path === relativePath && artifact.sha256 === expectedHash),
      `${label} is not listed in artifact_paths with the same SHA-256`,
      errors,
    );
  }

  const gpui = readSnapshot(root, block.gpui_snapshot_path, "receipt accessibility gpui snapshot", errors);
  const svelte = readSnapshot(root, block.svelte_snapshot_path, "receipt accessibility svelte snapshot", errors);
  if (gpui === undefined || svelte === undefined) return;
  for (const [label, snapshot, runtime, run] of [
    ["gpui snapshot", gpui, A1_GPUI_RUNTIME, GPUI_RUN_RECORD],
    ["svelte snapshot", svelte, A1_SVELTE_RUNTIME, SVELTE_RUN_RECORD],
  ] as const) {
    assert(snapshot.schema === A1_SNAPSHOT_SCHEMA, `receipt accessibility ${label} schema is not ${A1_SNAPSHOT_SCHEMA}`, errors);
    assert(snapshot.component === receipt.component, `receipt accessibility ${label} component does not match the receipt`, errors);
    assert(snapshot.scenario_id === receipt.scenario_id, `receipt accessibility ${label} scenario id does not match the receipt`, errors);
    assert(snapshot.scenario_path === block.scenario_path, `receipt accessibility ${label} scenario path does not match the receipt`, errors);
    assert(snapshot.scenario_sha256 === block.scenario_sha256, `receipt accessibility ${label} ran against a different scenario hash`, errors);
    assert(snapshot.runtime === runtime, `receipt accessibility ${label} runtime is not ${runtime}`, errors);
    assert(JSON.stringify(snapshot.run) === JSON.stringify(run), `receipt accessibility ${label} lacks the executed run record`, errors);
    assert(
      Array.isArray(snapshot.nodes) && (snapshot.nodes.length > 0 || receipt.component === "StatusIndicator"),
      `receipt accessibility ${label} has no nodes`,
      errors,
    );
  }
  if (!Array.isArray(gpui.nodes) || !Array.isArray(svelte.nodes)) return;
  const diff = diffSnapshotNodes(gpui.nodes, svelte.nodes);
  assert(diff.length === 0, `receipt accessibility snapshots diverge: ${JSON.stringify(diff)}`, errors);
  assert(block.diff.length === 0, "receipt accessibility diff is not empty", errors);
}

export function receiptFileStem(receipt: { component: string; scenario_id: string; proof_level: NucleusProofLevel }): string {
  const stem = `${receipt.component.toLowerCase().replaceAll(" ", "-")}--${receipt.scenario_id.replaceAll(".", "-")}`;
  if (receipt.proof_level === "A1") return `${stem}--a1`;
  if (receipt.proof_level === "V1") return `${stem}--v1`;
  return stem;
}

function currentSourceMatchesReceipt(manifest: NucleusManifest, root: string): boolean {
  try {
    execFileSync("git", ["diff", "--quiet", manifest.resolution.source_commit, "HEAD", "--", ...SOURCE_PATHS], { cwd: root, stdio: "ignore" });
    execFileSync("git", ["diff", "--quiet", "HEAD", "--", ...SOURCE_PATHS], { cwd: root, stdio: "ignore" });
    return true;
  } catch {
    return false;
  }
}

function canonicalReceiptFiles(root: string): string[] {
  const directory = rootPath(root, NUCLEUS_RECEIPT_DIR);
  if (!existsSync(directory)) return [];
  return readdirSync(directory).filter((file) => file.endsWith(".json")).sort();
}

export function loadValidatedNucleusReceipts(root = ROOT): Array<{ path: string; receipt: AnyNucleusReceipt }> {
  const manifest = loadNucleusManifest(root);
  const receipts: Array<{ path: string; receipt: AnyNucleusReceipt }> = [];
  const errors: string[] = [];
  const seenComponents = new Set<string>();
  for (const file of canonicalReceiptFiles(root)) {
    const relativePath = `${NUCLEUS_RECEIPT_DIR}/${file}`;
    try {
      const receipt = readJson<AnyNucleusReceipt>(root, relativePath);
      validateNucleusReceipt(receipt, manifest, root);
      const key = `${receipt.component}/${receipt.proof_level}`;
      if (seenComponents.has(key)) throw new Error(`duplicate ${receipt.proof_level} receipt component ${receipt.component}`);
      seenComponents.add(key);
      if (file !== `${receiptFileStem(receipt)}.json`) throw new Error(`receipt file name must be ${receiptFileStem(receipt)}.json`);
      receipts.push({ path: relativePath, receipt });
    } catch (error) {
      errors.push(`${relativePath}: ${error instanceof Error ? error.message : String(error)}`);
    }
  }
  if (receipts.length > 0 && !currentSourceMatchesReceipt(manifest, root)) {
    errors.push(`receipt source commit ${manifest.resolution.source_commit} no longer matches the mounted runtime source`);
  }
  if (errors.length > 0) throw new Error(errors.join("\n"));
  return receipts;
}

export function deriveNucleusReceiptRows(root = ROOT): NucleusReceiptRow[] {
  const manifest = loadNucleusManifest(root);
  const validated = loadValidatedNucleusReceipts(root);
  const m1 = new Map(validated.filter((item) => item.receipt.proof_level === "M1").map((item) => [item.receipt.component, item]));
  const a1 = new Map(validated.filter((item) => item.receipt.proof_level === "A1").map((item) => [item.receipt.component, item]));
  const v1 = new Map(validated.filter((item) => item.receipt.proof_level === "V1").map((item) => [item.receipt.component, item]));
  return manifest.components.map((entry) => {
    const row: NucleusReceiptRow = { entry };
    const mounted = m1.get(entry.name);
    if (mounted !== undefined && mounted.receipt.proof_level !== "V1") {
      row.receiptPath = mounted.path;
      row.receipt = mounted.receipt;
    }
    const accessible = a1.get(entry.name);
    if (accessible !== undefined && accessible.receipt.proof_level !== "V1") {
      row.a1ReceiptPath = accessible.path;
      row.a1Receipt = accessible.receipt;
    }
    const visual = v1.get(entry.name);
    if (visual !== undefined && visual.receipt.proof_level === "V1") {
      row.v1ReceiptPath = visual.path;
      row.v1Receipt = visual.receipt;
    }
    return row;
  });
}

/// g17.001 V1: deterministic component comparison evidence imported from the
/// validated Poodle Lab cohort bundle. The bundle is evidence, not
/// implementation input: it is copied byte-for-byte, re-validated by directory
/// hash and validator version, and never edited. Reported findings are
/// retained verbatim in every receipt; they adjudicate nothing.
export const NUCLEUS_V1_BUNDLE_DIR = "docs/logs/2026-09/08-140648-g01-006-cohort-batch-bundle";
export const NUCLEUS_V1_BUNDLE_SCHEMA = "poodle-lab.cohort-run.v1";
export const NUCLEUS_V1_RUN_ID = "2026-09-08T14-06-48";
export const NUCLEUS_V1_VALIDATOR_VERSION = "1.0.0";
export const NUCLEUS_V1_DIRECTORY_SHA256 = "0512b830e94bc30a7f1d2c623c6e081c85d4aa0086a60f9adfd905badc612a99";
export const NUCLEUS_V1_SUMMARY_SHA256 = "f1f32bb05d88d60d2d2e6cecb83251e024826b05c678dfd2ddd19826c80ce7e6";
export const NUCLEUS_V1_REPORT_SHA256 = "3475132657b355a1c6b3246c3a21db2c660f5fb719c0986a6d791f3e9447d087";
export const NUCLEUS_V1_LAB_MERGE_COMMIT = "f99465f048d7c5c58603b99ae51f3209e581848e";
export const NUCLEUS_V1_LAB_CLOSEOUT_COMMIT = "13ddc2fcbc0897a9f2ec78ee0dd061ce74c7f46d";
export const NUCLEUS_V1_POODLE_PIN = "8bd95d3a2cdf8c86edacb450cc33a0a4d02b9983";
export const NUCLEUS_V1_SUMMARY_FILE = "summary.json";
export const NUCLEUS_V1_REPORT_FILE = "report.md";
export const NUCLEUS_V1_STATES = ["initial", "after-actions"] as const;
export const NUCLEUS_V1_PAIRS = ["svelte-react", "svelte-gpui"] as const;
export const NUCLEUS_V1_CHANNELS = ["dimensions", "geometry", "roles", "pixels"] as const;
export const NUCLEUS_V1_RUNTIMES = ["svelte", "react", "gpui"] as const;

export type NucleusVisualChannelVerdict = {
  status: "pass" | "fail";
  findings: NucleusVisualFinding[];
  metrics?: {
    differingPixels: number;
    totalPixels: number;
    ratio: number;
  };
};

export type NucleusVisualPairVerdict = {
  fixture: string;
  pair: (typeof NUCLEUS_V1_PAIRS)[number];
  ok: boolean;
  channels: Record<(typeof NUCLEUS_V1_CHANNELS)[number], NucleusVisualChannelVerdict>;
};

export type NucleusVisualBundle = {
  dir: typeof NUCLEUS_V1_BUNDLE_DIR;
  directory_sha256: string;
  summary_sha256: string;
  lab_merge_commit: string;
  lab_closeout_commit: string;
  run_id: typeof NUCLEUS_V1_RUN_ID;
  validator_version: typeof NUCLEUS_V1_VALIDATOR_VERSION;
  poodle_pin: string;
};

export type NucleusV1Receipt = {
  schema: typeof NUCLEUS_RECEIPT_SCHEMA;
  component: string;
  scenario_id: string;
  proof_level: "V1";
  outcome: "compared";
  lab_bundle: NucleusVisualBundle;
  fixtures: string[];
  pairs: NucleusVisualPairVerdict[];
  finding_count: number;
};

export type AnyNucleusReceipt = NucleusReceipt | NucleusV1Receipt;

const V1_RECEIPT_REQUIRED_KEYS = [
  "schema",
  "component",
  "scenario_id",
  "proof_level",
  "outcome",
  "lab_bundle",
  "fixtures",
  "pairs",
  "finding_count",
];
const V1_BUNDLE_REQUIRED_KEYS = [
  "dir",
  "directory_sha256",
  "summary_sha256",
  "lab_merge_commit",
  "lab_closeout_commit",
  "run_id",
  "validator_version",
  "poodle_pin",
];
const V1_PAIR_REQUIRED_KEYS = ["fixture", "pair", "ok", "channels"];
const V1_CHANNEL_REQUIRED_KEYS = ["status", "findings"];
const V1_FINDING_REQUIRED_KEYS = ["channel", "subject", "detail"];
const V1_METRICS_REQUIRED_KEYS = ["differingPixels", "totalPixels", "ratio"];

function assertV1Metrics(value: unknown, label: string, required: boolean, errors: string[]): void {
  if (value === undefined) {
    assert(required === false, `${label} metrics is missing`, errors);
    return;
  }
  if (!assertExactObject(value, `${label} metrics`, V1_METRICS_REQUIRED_KEYS, [], errors)) return;
  assert(typeof value.differingPixels === "number" && Number.isInteger(value.differingPixels) && value.differingPixels >= 0, `${label} metrics differingPixels must be a non-negative integer`, errors);
  assert(typeof value.totalPixels === "number" && Number.isInteger(value.totalPixels) && value.totalPixels > 0, `${label} metrics totalPixels must be a positive integer`, errors);
  assert(typeof value.ratio === "number" && value.ratio >= 0 && value.ratio <= 1, `${label} metrics ratio must be between 0 and 1`, errors);
}

function v1ReceiptShapeErrors(receipt: JsonObject): string[] {
  const errors: string[] = [];
  if (Object.hasOwn(receipt, "production_path_observation") || Object.hasOwn(receipt, "runtime")) {
    errors.push("a mounted M1/A1 body is not a visual receipt; V1 proof level requires the Lab visual block");
  }
  if (!assertExactObject(receipt, "receipt", V1_RECEIPT_REQUIRED_KEYS, [], errors)) return errors;
  if (assertExactObject(receipt.lab_bundle, "receipt lab_bundle", V1_BUNDLE_REQUIRED_KEYS, [], errors)) {
    const bundle = receipt.lab_bundle;
    for (const key of V1_BUNDLE_REQUIRED_KEYS) {
      assert(typeof bundle[key] === "string" && (bundle[key] as string).length > 0, `receipt lab_bundle ${key} must be a non-empty string`, errors);
    }
  }
  if (
    !assertArray(receipt.fixtures, "receipt fixtures", errors) ||
    receipt.fixtures.length !== 2 ||
    !receipt.fixtures.every((fixture) => typeof fixture === "string" && fixture.length > 0)
  ) {
    errors.push("receipt fixtures must be exactly two cohort fixture ids");
  }
  if (assertArray(receipt.pairs, "receipt pairs", errors)) {
    if (receipt.pairs.length !== 4) errors.push("receipt pairs must be exactly four pair verdicts");
    receipt.pairs.forEach((pair, index) => {
      const label = `receipt pairs[${index}]`;
      if (!assertExactObject(pair, label, V1_PAIR_REQUIRED_KEYS, [], errors)) return;
      assert(typeof pair.fixture === "string" && pair.fixture.length > 0, `${label} fixture must be a non-empty string`, errors);
      assert(pair.pair === "svelte-react" || pair.pair === "svelte-gpui", `${label} pair must be svelte-react or svelte-gpui`, errors);
      assert(typeof pair.ok === "boolean", `${label} ok must be a boolean`, errors);
      if (!isJsonObject(pair.channels)) {
        errors.push(`${label} channels must be an object`);
        return;
      }
      for (const channel of NUCLEUS_V1_CHANNELS) {
        const verdict = (pair.channels as Record<string, unknown>)[channel];
        const channelLabel = `${label} channels ${channel}`;
        if (!assertExactObject(verdict, channelLabel, V1_CHANNEL_REQUIRED_KEYS, ["metrics"], errors)) continue;
        assert(verdict.status === "pass" || verdict.status === "fail", `${channelLabel} status must be pass or fail`, errors);
        assertV1Metrics(verdict.metrics, channelLabel, channel === "pixels", errors);
        if (assertArray(verdict.findings, `${channelLabel} findings`, errors)) {
          verdict.findings.forEach((finding, findingIndex) => {
            if (!assertExactObject(finding, `${channelLabel} findings[${findingIndex}]`, V1_FINDING_REQUIRED_KEYS, [], errors)) return;
            for (const key of V1_FINDING_REQUIRED_KEYS) {
              assert(
                typeof (finding as Record<string, unknown>)[key] === "string" && ((finding as Record<string, unknown>)[key] as string).length > 0,
                `${channelLabel} findings[${findingIndex}] ${key} must be a non-empty string`,
                errors,
              );
            }
          });
        }
      }
      const channelKeys = Object.keys(pair.channels);
      assert(
        channelKeys.length === NUCLEUS_V1_CHANNELS.length && NUCLEUS_V1_CHANNELS.every((channel) => channelKeys.includes(channel)),
        `${label} channels must be exactly dimensions, geometry, roles, and pixels`,
        errors,
      );
    });
  }
  assert(typeof receipt.finding_count === "number" && Number.isInteger(receipt.finding_count) && receipt.finding_count >= 0, "receipt finding_count must be a non-negative integer", errors);
  return errors;
}

/// g17.001: the live Poodle scenario behind each Lab cohort slug. The mapping
/// is derived, never assumed: a manifest row claims exactly one scenario file
/// whose committed component and scenario id match the row.
export type V1ScenarioMap = Map<string, { component: string; scenario_id: string }>;

export function nucleusV1ScenarioMap(root = ROOT): V1ScenarioMap {
  const map: V1ScenarioMap = new Map();
  const directory = rootPath(root, "test/nucleus-a11y/scenarios");
  for (const file of readdirSync(directory).filter((entry) => entry.endsWith(".json")).sort()) {
    const stem = file.slice(0, -".json".length);
    const loaded = readScenario(root, stem);
    map.set(stem, { component: loaded.scenario.component, scenario_id: loaded.scenario.scenario_id });
  }
  return map;
}

export type ValidatedV1Bundle = {
  slugs: string[];
  fixtures: string[];
  pairs: NucleusVisualPairVerdict[];
  pairByKey: Map<string, NucleusVisualPairVerdict>;
  slugByComponent: Map<string, string>;
  findingCount: number;
  pairsOk: number;
};

function v1SlugOf(fixture: unknown): string | undefined {
  if (typeof fixture !== "string") return undefined;
  const match = fixture.match(/^cohort\/([a-z0-9-]+)\/(initial|after-actions)$/);
  return match?.[1];
}

function assertV1Policy(policy: unknown, errors: string[]): void {
  if (!isJsonObject(policy)) {
    errors.push("bundle policy must be an object");
    return;
  }
  for (const [table, expected] of [["GEOMETRY", GEOMETRY], ["ROLES", ROLES], ["PIXELS", PIXELS]] as const) {
    const actual = policy[table];
    if (!isJsonObject(actual)) {
      errors.push(`bundle policy ${table} must be an object`);
      continue;
    }
    for (const [key, value] of Object.entries(expected)) {
      assert(actual[key] === value, `bundle policy ${table}.${key} differs from the fixed g15.047 table`, errors);
    }
    assert(
      Object.keys(actual).length === Object.keys(expected).length,
      `bundle policy ${table} carries unexpected keys beside the fixed g15.047 table`,
      errors,
    );
  }
  assert(Object.keys(policy).length === 3, "bundle policy must carry exactly GEOMETRY, ROLES, and PIXELS", errors);
}

/// g17.001: fail-closed validation of the imported Lab bundle document. The
/// document is trusted only after: pinned Lab and Poodle identity, the fixed
/// g15.047 tolerance table, 174 captures with two agreeing repeats and proved
/// foreground, a bijective slug/scenario/manifest mapping, 116 verdicts that
/// cover every implied pair with ok agreeing with every channel, and verdict
/// counts that recompute exactly.
export function validateV1BundleDocument(document: unknown, scenarios: V1ScenarioMap, manifest: NucleusManifest): ValidatedV1Bundle {
  const errors: string[] = [];
  if (!isJsonObject(document)) throw new Error("bundle summary must be an object");
  assert(document.schema === NUCLEUS_V1_BUNDLE_SCHEMA, `bundle schema is not ${NUCLEUS_V1_BUNDLE_SCHEMA}`, errors);
  assert(document.lane === "cohort", "bundle lane is not cohort", errors);
  assert(document.runId === NUCLEUS_V1_RUN_ID, `bundle run id is not ${NUCLEUS_V1_RUN_ID}`, errors);
  assert(document.closedBatch === true, "bundle is not a closed batch", errors);
  const manifestBlock = isJsonObject(document.manifest) ? document.manifest : undefined;
  assert(manifestBlock?.fixtures === 58, "bundle manifest does not cover 58 fixtures", errors);
  const poodle = isJsonObject(document.sources) && isJsonObject(document.sources.poodle) ? (document.sources.poodle as JsonObject) : undefined;
  assert(poodle?.commit === NUCLEUS_V1_POODLE_PIN, `bundle Poodle pin is not ${NUCLEUS_V1_POODLE_PIN}`, errors);
  assertV1Policy(document.policy, errors);

  const slugToEntry = new Map<string, NucleusEntry>();
  for (const entry of manifest.components) {
    const matches = [...scenarios.entries()].filter(([, identity]) => identity.component === entry.name && identity.scenario_id === entry.scenario_id);
    assert(matches.length === 1, `${entry.name} maps to ${matches.length} scenario files, expected exactly one`, errors);
    if (matches.length === 1 && matches[0] !== undefined) {
      const stem = matches[0][0];
      assert(!slugToEntry.has(stem), `scenario ${stem} is claimed by more than one manifest row`, errors);
      slugToEntry.set(stem, entry);
    }
  }

  const slugs = new Set<string>();
  const seenCaptures = new Set<string>();
  if (assertArray(document.captures, "bundle captures", errors)) {
    assert(document.captures.length === 174, `bundle carries ${document.captures.length} captures, expected 174`, errors);
    document.captures.forEach((capture, index) => {
      const label = `bundle captures[${index}]`;
      if (!isJsonObject(capture)) {
        errors.push(`${label} must be an object`);
        return;
      }
      const slug = v1SlugOf(capture.fixture);
      assert(slug !== undefined, `${label} fixture is not a cohort initial/after-actions id: ${String(capture.fixture)}`, errors);
      if (slug !== undefined) slugs.add(slug);
      assert(typeof capture.runtime === "string" && (NUCLEUS_V1_RUNTIMES as readonly string[]).includes(capture.runtime), `${label} runtime must be svelte, react, or gpui`, errors);
      assert(typeof capture.pngSha256 === "string" && /^[0-9a-f]{64}$/.test(capture.pngSha256), `${label} pngSha256 must be 64 lowercase hex characters`, errors);
      const repeats = isJsonObject(capture.repeats) ? (capture.repeats as JsonObject) : undefined;
      const hashes = Array.isArray(repeats?.sha256) ? (repeats?.sha256 as unknown[]) : undefined;
      assert(repeats?.count === 2, `${label} must carry exactly two repeats`, errors);
      assert(hashes !== undefined && hashes.length === 2 && hashes.every((hash) => hash === capture.pngSha256), `${label} repeats do not agree exactly with the capture hash`, errors);
      assert(repeats?.agreedExactly === true, `${label} repeats did not agree exactly`, errors);
      const foreground = isJsonObject(capture.foreground) ? (capture.foreground as JsonObject) : undefined;
      assert(foreground?.verdict === "proved", `${label} lacks proved foreground evidence`, errors);
      const key = `${String(capture.fixture)}|${String(capture.runtime)}`;
      assert(!seenCaptures.has(key), `${label} duplicates capture ${key}`, errors);
      seenCaptures.add(key);
    });
  }
  assert(slugs.size === 29, `bundle covers ${slugs.size} cohort slugs, expected 29`, errors);
  for (const slug of slugs) {
    assert(scenarios.has(slug), `bundle slug ${slug} has no Poodle scenario file`, errors);
    assert(slugToEntry.has(slug), `bundle slug ${slug} maps to no manifest row`, errors);
    for (const runtime of NUCLEUS_V1_RUNTIMES) {
      assert(seenCaptures.has(`cohort/${slug}/initial|${runtime}`) && seenCaptures.has(`cohort/${slug}/after-actions|${runtime}`), `bundle lacks a ${runtime} capture for slug ${slug}`, errors);
    }
  }
  for (const [stem] of slugToEntry) {
    assert(slugs.has(stem), `manifest scenario ${stem} has no bundle coverage`, errors);
  }

  const pairs: NucleusVisualPairVerdict[] = [];
  const pairByKey = new Map<string, NucleusVisualPairVerdict>();
  let findingCount = 0;
  if (assertArray(document.comparisons, "bundle comparisons", errors)) {
    document.comparisons.forEach((comparison, index) => {
      const label = `bundle comparisons[${index}]`;
      if (!isJsonObject(comparison)) {
        errors.push(`${label} must be an object`);
        return;
      }
      const slug = v1SlugOf(comparison.fixture);
      assert(slug !== undefined && slugs.has(slug), `${label} names a fixture absent from the bundle captures: ${String(comparison.fixture)}`, errors);
      assert(comparison.pair === "svelte-react" || comparison.pair === "svelte-gpui", `${label} pair must be svelte-react or svelte-gpui`, errors);
      assert(typeof comparison.ok === "boolean", `${label} ok must be a boolean`, errors);
      const key = `${String(comparison.fixture)}|${String(comparison.pair)}`;
      assert(!pairByKey.has(key), `${label} duplicates pair verdict ${key}`, errors);
      if (!isJsonObject(comparison.channels)) {
        errors.push(`${label} channels must be an object`);
        return;
      }
      const channels = comparison.channels as Record<string, unknown>;
      assert(
        Object.keys(channels).length === NUCLEUS_V1_CHANNELS.length && NUCLEUS_V1_CHANNELS.every((channel) => Object.hasOwn(channels, channel)),
        `${label} channels must be exactly dimensions, geometry, roles, and pixels`,
        errors,
      );
      let pairOk = true;
      for (const channel of NUCLEUS_V1_CHANNELS) {
        const verdict = channels[channel];
        const channelLabel = `${label} channels ${channel}`;
        if (!isJsonObject(verdict)) {
          errors.push(`${channelLabel} must be an object`);
          pairOk = false;
          continue;
        }
        assertV1Metrics(verdict.metrics, channelLabel, channel === "pixels", errors);
        if (!assertArray(verdict.findings, `${channelLabel} findings`, errors)) {
          pairOk = false;
          continue;
        }
        for (const [findingIndex, finding] of verdict.findings.entries()) {
          if (!isJsonObject(finding)) {
            errors.push(`${channelLabel} findings[${findingIndex}] must be an object`);
            pairOk = false;
            continue;
          }
          assert(finding.channel === channel, `${channelLabel} findings[${findingIndex}] belongs to a different channel`, errors);
          assert(typeof finding.subject === "string" && finding.subject.length > 0, `${channelLabel} findings[${findingIndex}] subject must be a non-empty string`, errors);
          assert(typeof finding.detail === "string" && finding.detail.length > 0, `${channelLabel} findings[${findingIndex}] detail must be a non-empty string`, errors);
          findingCount += 1;
        }
        if (verdict.status === "pass" && (verdict.findings as unknown[]).length > 0) {
          errors.push(`${channelLabel} passes but carries findings`);
          pairOk = false;
        }
        if (verdict.status === "fail" && (verdict.findings as unknown[]).length === 0) {
          errors.push(`${channelLabel} fails but carries no finding`);
          pairOk = false;
        }
        if (verdict.status !== "pass") pairOk = false;
      }
      assert(comparison.ok === pairOk, `${label} ok does not agree with its channels`, errors);
      const verdict: NucleusVisualPairVerdict = {
        fixture: comparison.fixture as string,
        pair: comparison.pair as (typeof NUCLEUS_V1_PAIRS)[number],
        ok: comparison.ok as boolean,
        channels: comparison.channels as NucleusVisualPairVerdict["channels"],
      };
      pairs.push(verdict);
      pairByKey.set(key, verdict);
    });
  }
  for (const slug of slugs) {
    for (const state of NUCLEUS_V1_STATES) {
      for (const pair of NUCLEUS_V1_PAIRS) {
        assert(pairByKey.has(`cohort/${slug}/${state}|${pair}`), `bundle lacks pair verdict cohort/${slug}/${state} ${pair}`, errors);
      }
    }
  }
  assert(pairs.length === 116, `bundle carries ${pairs.length} pair verdicts, expected 116`, errors);
  const pairsOk = pairs.filter((pair) => pair.ok).length;
  const verdictBlock = isJsonObject(document.verdict) ? (document.verdict as JsonObject) : undefined;
  assert(verdictBlock?.capturesAdmitted === 174, "bundle verdict capturesAdmitted does not recompute to 174", errors);
  assert(verdictBlock?.pairsCompared === 116, "bundle verdict pairsCompared does not recompute to 116", errors);
  assert(verdictBlock?.pairsOk === pairsOk, "bundle verdict pairsOk does not match the verdicts", errors);
  assert(verdictBlock?.findings === findingCount, "bundle verdict findings do not match the retained findings", errors);
  const encoded = JSON.stringify(document);
  assert(!encoded.includes("/Users/") && !encoded.includes("/private/"), "bundle summary names a machine path", errors);
  if (errors.length > 0) throw new Error(errors.join("\n"));
  pairs.sort((left, right) => (left.fixture < right.fixture ? -1 : left.fixture > right.fixture ? 1 : left.pair < right.pair ? -1 : 1));
  const fixtures = [...slugs].sort().flatMap((slug) => NUCLEUS_V1_STATES.map((state) => `cohort/${slug}/${state}`));
  const slugByComponent = new Map<string, string>();
  for (const [stem, entry] of slugToEntry) slugByComponent.set(entry.name, stem);
  return { slugs: [...slugs].sort(), fixtures, pairs, pairByKey, slugByComponent, findingCount, pairsOk };
}

/// g17.001: the Lab directory hash the citing card records. SHA-256 over each
/// file's relative path and content hash in sorted path order, mirroring the
/// Lab validator: any byte changed, added, or removed changes the digest.
export function v1DirectoryHash(root: string, dir = NUCLEUS_V1_BUNDLE_DIR): { sha256: string; files: Array<{ path: string; sha256: string }> } {
  const absolute = rootPath(root, dir);
  const out: string[] = [];
  const walk = (current: string): void => {
    for (const entry of readdirSync(current).sort()) {
      const full = path.join(current, entry);
      if (lstatSync(full).isDirectory()) walk(full);
      else out.push(path.relative(absolute, full).split(path.sep).join("/"));
    }
  };
  walk(absolute);
  out.sort();
  const files = out.map((relativePath) => ({
    path: relativePath,
    sha256: createHash("sha256").update(readFileSync(path.join(absolute, relativePath))).digest("hex"),
  }));
  const digest = createHash("sha256");
  for (const file of files) digest.update(`${file.path}\0${file.sha256}\n`);
  return { sha256: digest.digest("hex"), files };
}

function readV1BundleFile(root: string, file: string, expectedSha256: string): string {
  const relativePath = `${NUCLEUS_V1_BUNDLE_DIR}/${file}`;
  const filePath = rootPath(root, relativePath);
  if (!existsSync(filePath)) throw new Error(`imported bundle file is missing: ${relativePath}`);
  const content = readFileSync(filePath, "utf8");
  const actual = createHash("sha256").update(content).digest("hex");
  if (actual !== expectedSha256) throw new Error(`imported bundle file does not match the Lab bytes: ${relativePath}`);
  return content;
}

/// g17.001: import validation for the copied Lab bundle. Re-hashes every
/// imported byte and refuses the import unless the directory hash, validator
/// version, and document contents still identify the named Lab run.
export function loadValidatedV1Bundle(root = ROOT, manifest = loadNucleusManifest(root)): { bundle: ValidatedV1Bundle; summarySha256: string } {
  readV1BundleFile(root, NUCLEUS_V1_REPORT_FILE, NUCLEUS_V1_REPORT_SHA256);
  const summary = readV1BundleFile(root, NUCLEUS_V1_SUMMARY_FILE, NUCLEUS_V1_SUMMARY_SHA256);
  const hashed = v1DirectoryHash(root);
  if (hashed.sha256 !== NUCLEUS_V1_DIRECTORY_SHA256) {
    throw new Error(`imported bundle directory hash ${hashed.sha256} is not the cited Lab bundle ${NUCLEUS_V1_DIRECTORY_SHA256}`);
  }
  let document: unknown;
  try {
    document = JSON.parse(summary) as unknown;
  } catch {
    throw new Error(`imported bundle summary does not parse: ${NUCLEUS_V1_BUNDLE_DIR}/${NUCLEUS_V1_SUMMARY_FILE}`);
  }
  return { bundle: validateV1BundleDocument(document, nucleusV1ScenarioMap(root), manifest), summarySha256: NUCLEUS_V1_SUMMARY_SHA256 };
}

function expectedV1BundleBlock(summarySha256: string): NucleusVisualBundle {
  return {
    dir: NUCLEUS_V1_BUNDLE_DIR,
    directory_sha256: NUCLEUS_V1_DIRECTORY_SHA256,
    summary_sha256: summarySha256,
    lab_merge_commit: NUCLEUS_V1_LAB_MERGE_COMMIT,
    lab_closeout_commit: NUCLEUS_V1_LAB_CLOSEOUT_COMMIT,
    run_id: NUCLEUS_V1_RUN_ID,
    validator_version: NUCLEUS_V1_VALIDATOR_VERSION,
    poodle_pin: NUCLEUS_V1_POODLE_PIN,
  };
}

/// g17.001: derive one V1 receipt per covered manifest row. Only rows the
/// validated bundle covers are emitted; unknown, duplicate, missing, or
/// mismatched scenario/fixture identities refuse the whole batch.
export function deriveV1Receipts(bundle: ValidatedV1Bundle, manifest: NucleusManifest, summarySha256: string): NucleusV1Receipt[] {
  const lab_bundle = expectedV1BundleBlock(summarySha256);
  return manifest.components.map((entry) => {
    const slug = bundle.slugByComponent.get(entry.name);
    if (slug === undefined) throw new Error(`${entry.name} has no validated bundle coverage`);
    const fixtures = NUCLEUS_V1_STATES.map((state) => `cohort/${slug}/${state}`);
    const pairs = fixtures.flatMap((fixture) =>
      NUCLEUS_V1_PAIRS.map((pair) => {
        const verdict = bundle.pairByKey.get(`${fixture}|${pair}`);
        if (verdict === undefined) throw new Error(`bundle lacks pair verdict ${fixture} ${pair}`);
        return verdict;
      }),
    );
    const finding_count = pairs.reduce(
      (count, pair) => count + NUCLEUS_V1_CHANNELS.reduce((channelCount, channel) => channelCount + pair.channels[channel].findings.length, 0),
      0,
    );
    return {
      schema: NUCLEUS_RECEIPT_SCHEMA,
      component: entry.name,
      scenario_id: entry.scenario_id,
      proof_level: "V1",
      outcome: "compared",
      lab_bundle: { ...lab_bundle },
      fixtures,
      pairs,
      finding_count,
    };
  });
}

/// g17.001: emit the validated V1 receipts. Validation runs first: a tampered
/// bundle, an unknown fixture, or a broken mapping throws before any receipt
/// is written.
export function emitNucleusV1Receipts(root = ROOT): string[] {
  const manifest = loadNucleusManifest(root);
  const { bundle, summarySha256 } = loadValidatedV1Bundle(root, manifest);
  const receipts = deriveV1Receipts(bundle, manifest, summarySha256);
  mkdirSync(rootPath(root, NUCLEUS_RECEIPT_DIR), { recursive: true });
  return receipts.map((receipt) => {
    const relativePath = `${NUCLEUS_RECEIPT_DIR}/${receiptFileStem(receipt)}.json`;
    writeFileSync(rootPath(root, relativePath), JSON.stringify(receipt, null, 2));
    return relativePath;
  });
}

function validateV1Receipt(receipt: NucleusV1Receipt, manifest: NucleusManifest, root: string, errors: string[]): void {
  assert(receipt.schema === NUCLEUS_RECEIPT_SCHEMA, "receipt schema is not current", errors);
  assert(receipt.outcome === "compared", "a V1 receipt outcome is compared, never passed", errors);
  assert(
    Object.hasOwn(receipt, "production_path_observation") === false,
    "a V1 receipt carries no mounted production-path observation; V1 proof level requires the Lab visual block",
    errors,
  );
  const expectedBundle = expectedV1BundleBlock(NUCLEUS_V1_SUMMARY_SHA256);
  assert(JSON.stringify(receipt.lab_bundle) === JSON.stringify(expectedBundle), "receipt Lab bundle identity does not match the cited run", errors);
  let expected: NucleusV1Receipt[] | undefined;
  try {
    const loaded = loadValidatedV1Bundle(root, manifest);
    expected = deriveV1Receipts(loaded.bundle, manifest, loaded.summarySha256);
  } catch (error) {
    errors.push(`V1 bundle revalidation failed: ${error instanceof Error ? error.message.split("\n")[0] : String(error)}`);
  }
  if (expected !== undefined) {
    const match = expected.find((candidate) => candidate.component === receipt.component);
    assert(match !== undefined, `receipt component has no validated V1 coverage: ${receipt.component}`, errors);
    if (match !== undefined) {
      assert(receipt.scenario_id === match.scenario_id, `${receipt.component} receipt scenario does not match the manifest`, errors);
      assert(JSON.stringify(receipt) === JSON.stringify(match), `${receipt.component} V1 receipt does not match the validated Lab bundle derivation`, errors);
    }
  }
  const encoded = JSON.stringify(receipt);
  assert(!encoded.includes("/Users/") && !encoded.includes("/private/") && !encoded.includes("timestamp"), "receipt contains a machine path or timestamp", errors);
}

function v1Main(): void {
  if (process.argv.includes("--write-v1")) {
    for (const file of emitNucleusV1Receipts(ROOT)) console.log(`Wrote ${file}.`);
    return;
  }
  throw new Error("usage: bun scripts/nucleus-parity-receipts.ts --write-v1");
}

if (import.meta.main) v1Main();
