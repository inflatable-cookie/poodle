#!/usr/bin/env bun
/** Apply the one lockstep web and native version transition. */

import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";

import {
  LOCKSTEP_CARGO_LOCK_PATHS,
  LOCKSTEP_CARGO_MANIFEST_PATHS,
  LOCKSTEP_JS_MANIFEST_PATHS,
  cargoPackageIdentity,
  comparePreOneVersions,
  updateCargoLockText,
  updateCargoManifestText,
} from "./release-versioning";
import { packagesSectionUnchanged, refreshWorkspaceLockText, workspaceLockDrift } from "./workspace-lock";

const BUN_LOCK_PATH = "bun.lock";
const INTERNAL_JS_PREFIX = "@inflatable-cookie/poodle-";
const JS_DEPENDENCY_FIELDS = [
  "dependencies",
  "devDependencies",
  "peerDependencies",
  "optionalDependencies",
] as const;

type JsonRecord = Record<string, unknown>;
type PlannedChange = { path: string; before: string; after: string };

function record(value: unknown, label: string): JsonRecord {
  if (value === null || typeof value !== "object" || Array.isArray(value)) {
    throw new Error(`${label} must contain a JSON object`);
  }
  return value as JsonRecord;
}

function updateJsManifest(
  text: string,
  path: string,
  sourceVersion: string,
  targetVersion: string,
): string {
  const manifest = record(JSON.parse(text), path);
  if (manifest.version !== sourceVersion) {
    throw new Error(`${path} must be at ${sourceVersion}, found ${String(manifest.version)}`);
  }
  manifest.version = targetVersion;
  for (const field of JS_DEPENDENCY_FIELDS) {
    const dependencies = manifest[field];
    if (dependencies === undefined) continue;
    const dependencyMap = record(dependencies, `${path} ${field}`);
    for (const [name, specifier] of Object.entries(dependencyMap)) {
      if (!name.startsWith(INTERNAL_JS_PREFIX)) continue;
      const expected = `>=${sourceVersion} <0.${Number(sourceVersion.split(".")[1]) + 1}`;
      if (specifier !== expected) {
        throw new Error(`${path} ${field}.${name} must be ${expected}, found ${String(specifier)}`);
      }
      dependencyMap[name] = `>=${targetVersion} <0.${Number(targetVersion.split(".")[1]) + 1}`;
    }
  }
  return `${JSON.stringify(manifest, null, 2)}\n`;
}

function makeLockPlanningRoot(
  sourceRoot: string,
  manifests: ReadonlyMap<string, string>,
  lockText: string,
): string {
  const root = mkdtempSync(join(tmpdir(), "poodle-release-version-plan-"));
  const rootManifestText = manifests.get("package.json");
  if (rootManifestText === undefined) throw new Error("cannot plan release bump: missing package.json");
  const rootManifest = record(JSON.parse(rootManifestText), "package.json");
  if (!Array.isArray(rootManifest.workspaces) || !rootManifest.workspaces.every((path) => typeof path === "string")) {
    throw new Error("package.json workspaces must be a list of workspace paths");
  }
  const paths = [
    "package.json",
    ...rootManifest.workspaces.map((workspace) => `${workspace}/package.json`),
    BUN_LOCK_PATH,
  ];
  for (const path of paths) {
    const contents = path === BUN_LOCK_PATH
      ? lockText
      : manifests.get(path) ?? readFileSync(join(sourceRoot, path), "utf8");
    if (contents === undefined) throw new Error(`cannot plan ${BUN_LOCK_PATH}: missing ${path}`);
    const absolute = join(root, path);
    mkdirSync(dirname(absolute), { recursive: true });
    writeFileSync(absolute, contents);
  }
  return root;
}

/** Plan and apply a strictly increasing lockstep release version. */
export function bumpLockstepVersion(root: string, targetVersion: string): string[] {
  if (!/^0\.\d+\.\d+$/.test(targetVersion)) {
    throw new Error(`target must be a pre-1.0 X.Y.Z version, found ${targetVersion}`);
  }
  const jsManifests = new Map<string, string>();
  const cargoManifests = new Map<string, string>();
  const cargoNames = new Set<string>();
  const sourceVersions = new Set<string>();

  for (const path of LOCKSTEP_JS_MANIFEST_PATHS) {
    const text = readFileSync(join(root, path), "utf8");
    const manifest = record(JSON.parse(text), path);
    if (typeof manifest.version !== "string") throw new Error(`${path} must declare a version`);
    sourceVersions.add(manifest.version);
    jsManifests.set(path, text);
  }
  if (sourceVersions.size !== 1) {
    throw new Error(`JavaScript release manifests are not in lockstep: ${[...sourceVersions].join(", ")}`);
  }
  const sourceVersion = [...sourceVersions][0];
  if (comparePreOneVersions(targetVersion, sourceVersion) <= 0) {
    throw new Error(`target version must exceed ${sourceVersion}; refusing a non-increasing bump`);
  }

  for (const path of LOCKSTEP_CARGO_MANIFEST_PATHS) {
    const text = readFileSync(join(root, path), "utf8");
    const identity = cargoPackageIdentity(text, path);
    sourceVersions.add(identity.version);
    cargoNames.add(identity.name);
    cargoManifests.set(path, text);
  }
  for (const version of sourceVersions) {
    if (version !== sourceVersion) {
      throw new Error(
        `all Rust and JavaScript release manifests must be at ${sourceVersion}; found ${version}`,
      );
    }
  }

  const changes: PlannedChange[] = [];
  for (const [path, before] of jsManifests) {
    changes.push({
      path,
      before,
      after: updateJsManifest(before, path, sourceVersion, targetVersion),
    });
  }
  for (const [path, before] of cargoManifests) {
    const updated = updateCargoManifestText(before, path, sourceVersion, targetVersion);
    changes.push({ path, before, after: updated.text });
  }
  for (const path of LOCKSTEP_CARGO_LOCK_PATHS) {
    const before = readFileSync(join(root, path), "utf8");
    const after = updateCargoLockText(before, cargoNames, sourceVersion, targetVersion, path);
    changes.push({ path, before, after });
  }

  const bunLockBefore = readFileSync(join(root, BUN_LOCK_PATH), "utf8");
  const plannedJsManifests = new Map<string, string>();
  for (const change of changes.slice(0, LOCKSTEP_JS_MANIFEST_PATHS.length)) {
    plannedJsManifests.set(change.path, change.after);
  }
  const planningRoot = makeLockPlanningRoot(
    root,
    plannedJsManifests,
    bunLockBefore,
  );
  let bunLockAfter: string;
  try {
    bunLockAfter = refreshWorkspaceLockText(planningRoot, bunLockBefore);
    if (!packagesSectionUnchanged(bunLockBefore, bunLockAfter)) {
      throw new Error(`${BUN_LOCK_PATH} refresh changed dependency package resolutions`);
    }
    const drift = workspaceLockDrift(planningRoot, bunLockAfter);
    if (drift.length > 0) {
      throw new Error(`${BUN_LOCK_PATH} refresh is incomplete: ${drift.join("; ")}`);
    }
  } finally {
    rmSync(planningRoot, { recursive: true, force: true });
  }
  changes.push({ path: BUN_LOCK_PATH, before: bunLockBefore, after: bunLockAfter });

  for (const { path, after } of changes) writeFileSync(join(root, path), after);
  return changes
    .filter(({ before, after }) => before !== after)
    .map(({ path }) => path)
    .sort();
}

if (import.meta.main) {
  const args = process.argv.slice(2);
  const targetVersion = args[0];
  if (!targetVersion || args.length !== 1) {
    console.error("usage: effigy release:bump X.Y.Z");
    process.exit(2);
  }
  try {
    const changed = bumpLockstepVersion(process.cwd(), targetVersion);
    console.log(`Lockstep release versions updated to ${targetVersion}:`);
    for (const path of changed) console.log(`  ${path}`);
  } catch (error) {
    console.error(error instanceof Error ? error.message : String(error));
    process.exit(1);
  }
}
