import { createHash } from "node:crypto";
import { execFileSync } from "node:child_process";
import { existsSync, lstatSync, readdirSync, readFileSync } from "node:fs";
import path from "node:path";

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

export type NucleusReceipt = {
  schema: typeof NUCLEUS_RECEIPT_SCHEMA;
  component: string;
  scenario_id: string;
  proof_level: "M1";
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
};

export type NucleusReceiptRow = {
  entry: NucleusEntry;
  receiptPath?: string;
  receipt?: NucleusReceipt;
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
const OBSERVATION_REQUIRED_KEYS = ["observed", "mount", "render_path", "input_dispatch"];
const ARTIFACT_REQUIRED_KEYS = ["path", "sha256"];

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
  if (!assertExactObject(receipt, "receipt", RECEIPT_REQUIRED_KEYS, [], errors)) return errors;
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
  assert(receipt.proof_level === "M1", "receipt proof level must be M1; A1 and V1 require separate evidence", errors);
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
      if (seenComponents.has(receipt.component)) throw new Error(`duplicate receipt component ${receipt.component}`);
      seenComponents.add(receipt.component);
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
  const receipts = new Map(loadValidatedNucleusReceipts(root).map((item) => [item.receipt.component, item]));
  return manifest.components.map((entry) => {
    const match = receipts.get(entry.name);
    return match === undefined ? { entry } : { entry, receiptPath: match.path, receipt: match.receipt };
  });
}
