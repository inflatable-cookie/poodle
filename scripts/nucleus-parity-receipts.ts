import { createHash } from "node:crypto";
import { execFileSync } from "node:child_process";
import { existsSync, lstatSync, readdirSync, readFileSync } from "node:fs";
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

export const NUCLEUS_MANIFEST_PATH = "docs/roadmaps/g16/nucleus-parity-manifest.json";
export const NUCLEUS_MANIFEST_SCHEMA_PATH = "docs/roadmaps/g16/nucleus-parity-manifest.schema.json";
export const NUCLEUS_SCHEMA_PATH = "docs/roadmaps/g16/nucleus-parity-receipt.schema.json";
export const NUCLEUS_RECEIPT_DIR = "docs/roadmaps/g16/nucleus-parity-receipts";
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

export type NucleusProofLevel = "M1" | "A1";

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
  proof_level: NucleusProofLevel;
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

export function validateNucleusReceipt(receipt: NucleusReceipt, manifest = loadNucleusManifest(), root = ROOT): void {
  const shapeErrors = receiptShapeErrors(receipt);
  if (shapeErrors.length > 0) throw new Error(shapeErrors.join("\n"));
  const errors: string[] = [];
  const entry = [...manifest.prerequisites, ...manifest.components].find((candidate) => candidate.name === receipt.component);
  assert(receipt.schema === NUCLEUS_RECEIPT_SCHEMA, "receipt schema is not current", errors);
  assert(entry !== undefined && entry.rendered !== false, `receipt component is not a rendered manifest entry: ${receipt.component}`, errors);
  if (entry !== undefined) assert(receipt.scenario_id === entry.scenario_id, `${receipt.component} receipt scenario does not match the manifest`, errors);
  assert(receipt.proof_level === "M1" || receipt.proof_level === "A1", "receipt proof level must be M1 or A1; V1 requires separate evidence", errors);
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
    assert(Array.isArray(snapshot.nodes) && snapshot.nodes.length > 0, `receipt accessibility ${label} has no nodes`, errors);
  }
  if (!Array.isArray(gpui.nodes) || !Array.isArray(svelte.nodes)) return;
  const diff = diffSnapshotNodes(gpui.nodes, svelte.nodes);
  assert(diff.length === 0, `receipt accessibility snapshots diverge: ${JSON.stringify(diff)}`, errors);
  assert(block.diff.length === 0, "receipt accessibility diff is not empty", errors);
}

export function receiptFileStem(receipt: Pick<NucleusReceipt, "component" | "scenario_id" | "proof_level">): string {
  const stem = `${receipt.component.toLowerCase().replaceAll(" ", "-")}--${receipt.scenario_id.replaceAll(".", "-")}`;
  return receipt.proof_level === "A1" ? `${stem}--a1` : stem;
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

export function loadValidatedNucleusReceipts(root = ROOT): Array<{ path: string; receipt: NucleusReceipt }> {
  const manifest = loadNucleusManifest(root);
  const receipts: Array<{ path: string; receipt: NucleusReceipt }> = [];
  const errors: string[] = [];
  const seenComponents = new Set<string>();
  for (const file of canonicalReceiptFiles(root)) {
    const relativePath = `${NUCLEUS_RECEIPT_DIR}/${file}`;
    try {
      const receipt = readJson<NucleusReceipt>(root, relativePath);
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
  return manifest.components.map((entry) => {
    const row: NucleusReceiptRow = { entry };
    const mounted = m1.get(entry.name);
    if (mounted !== undefined) {
      row.receiptPath = mounted.path;
      row.receipt = mounted.receipt;
    }
    const accessible = a1.get(entry.name);
    if (accessible !== undefined) {
      row.a1ReceiptPath = accessible.path;
      row.a1Receipt = accessible.receipt;
    }
    return row;
  });
}
