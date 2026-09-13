// g18.032 / spec 071: version-independent npm web-candidate admission.
//
// The release lane must not encode one generation or one source/target version.
// This module derives the candidate identity from the compared commits and
// applies one generic law:
//
//   1. the target is a greater pre-1.0 semantic version of the base;
//   2. root, core, Svelte and React move in exact lockstep;
//   3. internal web dependency requirements and `bun.lock` match the target;
//   4. the changelog and one matching release note describe the target;
//   5. exactly one commit changes the release-input set, and every later
//      change is a generated/evidence execution record bound to that commit;
//   6. every changed path is a release input or a declared evidence surface.
//
// A violation names the specific rule. Historical `g16.054`/`g18.006` modes in
// `scope.ts` are retained as evidence and are not consulted here.

import { readdirSync, statSync } from "node:fs";
import { join } from "node:path";

import {
  cargoIntraRepoRequirements,
  cargoSectionForLine,
  changedJsonLeafPaths,
  changedPathsForCommitRange,
  changelogInventory,
  commitChangedPaths,
  gitShowFile,
  internalJsDependencies,
  isJsonRecord,
  parseCargoDiffLines,
  recordedSourceCommits,
  requireExactCommit,
  runCapture,
  sortedUnique,
} from "./scope";

export const WEB_CANDIDATE_MODE = "web-candidate" as const;
export const WEB_CANDIDATE_MODE_ENV = "POODLE_WEB_CANDIDATE_MODE";

const ROOT_JS_MANIFEST_PATH = "package.json";
/** Root, core, Svelte and the private paired React build move in lockstep. */
export const WEB_CANDIDATE_JS_MANIFEST_PATHS = [
  ROOT_JS_MANIFEST_PATH,
  "packages/core/package.json",
  "packages/svelte/components/package.json",
  "packages/react/components/package.json",
] as const;
const RELEASED_JS_MANIFEST_PATHS = WEB_CANDIDATE_JS_MANIFEST_PATHS.filter(
  (path) => path !== ROOT_JS_MANIFEST_PATH,
);

const CHANGELOG_PATH = "CHANGELOG.md";
const RELEASE_NOTES_INDEX_PATH = "docs/release-notes/README.md";
const BUN_LOCK_PATH = "bun.lock";

const WEB_SEMVER = /^0\.(\d+)\.(\d+)$/;
const INTERNAL_JS_DEPENDENCY_PREFIX = "@inflatable-cookie/poodle-";

/**
 * Generated/evidence surfaces that may follow the frozen release-input commit.
 * They are protocol families, not one version's file list: regenerated codegen
 * stamps, the committed evidence trees and a bounded execution record.
 */
export const WEB_CANDIDATE_EVIDENCE_PATTERNS = [
  /^packages\/codegen\/generated\//,
  /^packages\/contracts\/[^/]+\/src\/generated\//,
  /^packages\/[^/]+\/src\/generated\//,
  /^packages\/[^/]+\/(?:preview|components)\/src\/generated\//,
  /^docs\/evidence\//,
  /^docs\/logs\/\d{4}-\d{2}\/\d{8}-g\d{2}-\d{3}-[a-z0-9-]+\.md$/,
] as const;

export type WebCandidateVersions = {
  sourceVersion: string;
  targetVersion: string;
};

export type WebCandidateProof = {
  mode: typeof WEB_CANDIDATE_MODE;
  sourceVersion: string;
  targetVersion: string;
  releaseNotePath: string;
  frozenReleaseInputCommit: string;
  changedPaths: readonly string[];
  releaseInputPaths: readonly string[];
  evidencePaths: readonly string[];
};

function isWebSemver(value: string): boolean {
  return WEB_SEMVER.test(value);
}

function compareWebVersions(left: string, right: string): number {
  const leftMatch = WEB_SEMVER.exec(left)!;
  const rightMatch = WEB_SEMVER.exec(right)!;
  const minor = Number(leftMatch[1]) - Number(rightMatch[1]);
  if (minor !== 0) return minor;
  return Number(leftMatch[2]) - Number(rightMatch[2]);
}

async function readManifestVersion(
  checkoutRoot: string,
  commit: string,
  path: string,
): Promise<string> {
  const text = await gitShowFile(checkoutRoot, commit, path);
  if (text === null) throw new Error(`web candidate requires ${path} at ${commit}`);
  const manifest = JSON.parse(text) as Record<string, unknown>;
  if (typeof manifest.version !== "string") {
    throw new Error(`web candidate requires a version field in ${path}`);
  }
  return manifest.version;
}

/** Discover every tracked Cargo manifest and lock below `packages/`. */
function discoverCargoPaths(checkoutRoot: string): {
  manifests: string[];
  locks: string[];
} {
  const manifests: string[] = [];
  const locks: string[] = [];
  const walk = (relative: string): void => {
    for (const entry of readdirSync(join(checkoutRoot, relative), { withFileTypes: true })) {
      if (entry.name === "target" || entry.name === "node_modules" || entry.name.startsWith(".")) {
        continue;
      }
      const child = relative === "" ? entry.name : `${relative}/${entry.name}`;
      if (entry.isDirectory()) {
        walk(child);
        continue;
      }
      if (entry.name === "Cargo.toml") manifests.push(child);
      if (entry.name === "Cargo.lock") locks.push(child);
    }
  };
  if (statSync(join(checkoutRoot, "packages")).isDirectory()) walk("packages");
  return { manifests: sortedUnique(manifests), locks: sortedUnique(locks) };
}

/** The version-carrying surface shared by every lockstep participant. */
export function webCandidateVersionPaths(checkoutRoot: string): string[] {
  const { manifests, locks } = discoverCargoPaths(checkoutRoot);
  return sortedUnique([...manifests, ...locks, ...WEB_CANDIDATE_JS_MANIFEST_PATHS]);
}

/** The complete release-input set derives from the target version, nothing else. */
export function webCandidateReleaseInputPaths(
  checkoutRoot: string,
  targetVersion: string,
): string[] {
  const { manifests, locks } = discoverCargoPaths(checkoutRoot);
  return sortedUnique([
    CHANGELOG_PATH,
    BUN_LOCK_PATH,
    RELEASE_NOTES_INDEX_PATH,
    `docs/release-notes/${targetVersion}.md`,
    ...manifests,
    ...locks,
    ...WEB_CANDIDATE_JS_MANIFEST_PATHS,
  ]);
}

export async function deriveWebCandidateVersions(
  checkoutRoot: string,
  requiredBaseCommit: string,
  sourceCommit: string,
): Promise<WebCandidateVersions> {
  requireExactCommit(requiredBaseCommit, "required base commit");
  requireExactCommit(sourceCommit, "certification source commit");
  const sourceVersion = await readManifestVersion(
    checkoutRoot,
    requiredBaseCommit,
    ROOT_JS_MANIFEST_PATH,
  );
  const targetVersion = await readManifestVersion(
    checkoutRoot,
    sourceCommit,
    ROOT_JS_MANIFEST_PATH,
  );
  if (!isWebSemver(sourceVersion)) {
    throw new Error(
      `web candidate rejected base version ${sourceVersion}: expected a pre-1.0 semantic version`,
    );
  }
  if (!isWebSemver(targetVersion)) {
    throw new Error(
      `web candidate rejected target version ${targetVersion}: expected a pre-1.0 semantic version`,
    );
  }
  if (compareWebVersions(targetVersion, sourceVersion) <= 0) {
    throw new Error(
      `web candidate requires the target version to exceed the base version: ${sourceVersion} -> ${targetVersion}`,
    );
  }
  return { sourceVersion, targetVersion };
}

export function isWebCandidateEvidencePath(path: string): boolean {
  return WEB_CANDIDATE_EVIDENCE_PATTERNS.some((pattern) => pattern.test(path));
}

export function isWebCandidateReleaseInputPath(
  path: string,
  releaseInputPaths: readonly string[],
): boolean {
  return releaseInputPaths.includes(path);
}

const MANIFEST_LEAF_ALLOWLIST: Record<string, readonly string[]> = {
  "package.json": ["version"],
  "packages/core/package.json": ["version"],
  "packages/svelte/components/package.json": [
    "version",
    "dependencies.@inflatable-cookie/poodle-core",
  ],
  "packages/react/components/package.json": [
    "version",
    "dependencies.@inflatable-cookie/poodle-core",
  ],
};

/**
 * Root, core, Svelte and React must carry the exact lockstep transition and its
 * intra-repository dependency requirements. Nothing else in those manifests
 * may move, and the private packages must stay private.
 */
async function assertJsLockstep(
  checkoutRoot: string,
  requiredBaseCommit: string,
  sourceCommit: string,
  changedPaths: string[],
  { sourceVersion, targetVersion }: WebCandidateVersions,
): Promise<void> {
  for (const path of WEB_CANDIDATE_JS_MANIFEST_PATHS) {
    const before = await gitShowFile(checkoutRoot, requiredBaseCommit, path);
    const after = await gitShowFile(checkoutRoot, sourceCommit, path);
    if (before === null || after === null) {
      throw new Error(`web candidate rejected added or removed lockstep manifest ${path}`);
    }
    const beforeManifest = JSON.parse(before) as Record<string, unknown>;
    const afterManifest = JSON.parse(after) as Record<string, unknown>;
    if (beforeManifest.version !== sourceVersion) {
      throw new Error(`web candidate requires ${path} to begin at version ${sourceVersion}`);
    }
    if (afterManifest.version !== targetVersion) {
      throw new Error(`web candidate requires ${path} to carry version ${targetVersion}`);
    }
    if (changedPaths.includes(path)) {
      const allowed = MANIFEST_LEAF_ALLOWLIST[path];
      if (!allowed) {
        throw new Error(`web candidate has no manifest honesty rule for ${path}`);
      }
      const unauthorized = changedJsonLeafPaths(beforeManifest, afterManifest)
        .filter((change) => !allowed.includes(change))
        .sort();
      if (unauthorized.length > 0) {
        throw new Error(
          `web candidate rejected unauthorized ${path} changes: ${unauthorized.join(", ")}`,
        );
      }
    }
    if (
      (path === ROOT_JS_MANIFEST_PATH || path === "packages/react/components/package.json") &&
      afterManifest.private !== true
    ) {
      throw new Error(`web candidate rejected ${path} admission: package must remain private`);
    }
    const beforeDependencies = internalJsDependencies(beforeManifest);
    const afterDependencies = internalJsDependencies(afterManifest);
    for (const dependency of sortedUnique([
      ...beforeDependencies.keys(),
      ...afterDependencies.keys(),
    ])) {
      const beforeSpecifier = beforeDependencies.get(dependency);
      const afterSpecifier = afterDependencies.get(dependency);
      if (beforeSpecifier === undefined || afterSpecifier === undefined) {
        throw new Error(
          `web candidate rejected added or removed internal JS dependency ${dependency} in ${path}`,
        );
      }
      if (beforeSpecifier !== sourceVersion || afterSpecifier !== targetVersion) {
        throw new Error(
          `web candidate requires internal JS dependency ${dependency} in ${path} to move ${sourceVersion} -> ${targetVersion}, found ${beforeSpecifier} -> ${afterSpecifier}`,
        );
      }
    }
  }
}

/** Cargo manifests and locks carry the same lockstep transition, nothing else. */
async function assertCargoLockstep(
  checkoutRoot: string,
  requiredBaseCommit: string,
  sourceCommit: string,
  changedPaths: string[],
  { sourceVersion, targetVersion }: WebCandidateVersions,
): Promise<void> {
  const { manifests, locks } = discoverCargoPaths(checkoutRoot);
  for (const path of [...manifests, ...locks]) {
    if (!changedPaths.includes(path)) continue;
    const diff = await runCapture(
      ["git", "diff", "--no-ext-diff", "--unified=0", requiredBaseCommit, sourceCommit, "--", path],
      checkoutRoot,
    );
    const { added, removed } = parseCargoDiffLines(diff);
    const transport = [...removed, ...added.map(({ line }) => line)].filter(
      (line) =>
        /^\s*(?:publish|registry|source)\s*=/.test(line) ||
        /^\s*\[(?:patch|replace)(?:\.|\])/.test(line),
    );
    if (transport.length > 0) {
      throw new Error(
        `web candidate rejected Cargo publication/registry/source content in ${path}: ${transport.join(", ")}`,
      );
    }
    if (removed.length !== added.length) {
      throw new Error(
        `web candidate rejected unpaired Cargo lockstep content in ${path}; only version and exact intra-repository Poodle requirement versions may change`,
      );
    }
    const sourceText = (await gitShowFile(checkoutRoot, sourceCommit, path))!;
    for (let index = 0; index < removed.length; index += 1) {
      const oldLine = removed[index];
      const newLine = added[index].line;
      const section = cargoSectionForLine(sourceText, added[index].lineNumber);
      const packageVersion =
        section === "package" &&
        oldLine === `version = "${sourceVersion}"` &&
        newLine === `version = "${targetVersion}"`;
      const requirement = new RegExp(
        `^(poodle-[A-Za-z0-9_-]+)\\s*=\\s*\\{\\s*version\\s*=\\s*"${sourceVersion.replaceAll(".", "\\.")}",\\s*path\\s*=\\s*"([^"]+)"\\s*\\}$`,
      ).exec(oldLine);
      const requirementAfter = requirement
        ? new RegExp(
            `^${requirement[1]}\\s*=\\s*\\{\\s*version\\s*=\\s*"${targetVersion.replaceAll(".", "\\.")}",\\s*path\\s*=\\s*"${requirement[2]}"\\s*\\}$`,
          ).test(newLine)
        : false;
      const dependencyVersion =
        (section === "dependencies" || section === "dev-dependencies") && requirementAfter;
      if (!packageVersion && !dependencyVersion) {
        throw new Error(
          `web candidate rejected unauthorized Cargo change in ${path}: ${oldLine} -> ${newLine}`,
        );
      }
    }
    const beforeText = (await gitShowFile(checkoutRoot, requiredBaseCommit, path))!;
    const beforeRequirements = cargoIntraRepoRequirements(beforeText);
    const afterRequirements = cargoIntraRepoRequirements(sourceText);
    for (const key of sortedUnique([...beforeRequirements.keys(), ...afterRequirements.keys()])) {
      const beforeRequirement = beforeRequirements.get(key);
      const afterRequirement = afterRequirements.get(key);
      if (!beforeRequirement || !afterRequirement) {
        throw new Error(`web candidate rejected added or removed intra-repository Cargo requirement in ${path}`);
      }
      if (beforeRequirement.path !== afterRequirement.path) {
        throw new Error(`web candidate rejected retargeted intra-repository Cargo requirement in ${path}`);
      }
      if (afterRequirement.version === null) {
        if (beforeRequirement.version !== null) {
          throw new Error(`web candidate rejected version removal from an intra-repository Cargo requirement in ${path}`);
        }
        continue;
      }
      if (
        beforeRequirement.version !== sourceVersion ||
        afterRequirement.version !== targetVersion
      ) {
        throw new Error(
          `web candidate requires intra-repository Cargo requirement in ${path} to move ${sourceVersion} -> ${targetVersion}`,
        );
      }
    }
  }
}

/** `bun.lock` must resolve every released workspace package at the target. */
async function assertBunLock(
  checkoutRoot: string,
  requiredBaseCommit: string,
  sourceCommit: string,
  { sourceVersion, targetVersion }: WebCandidateVersions,
): Promise<void> {
  const before = await gitShowFile(checkoutRoot, requiredBaseCommit, BUN_LOCK_PATH);
  const after = await gitShowFile(checkoutRoot, sourceCommit, BUN_LOCK_PATH);
  if (before === null || after === null) {
    throw new Error(`web candidate requires ${BUN_LOCK_PATH} at both commits`);
  }
  for (const path of RELEASED_JS_MANIFEST_PATHS) {
    const workspace = path.replace(/\/package\.json$/, "");
    const name = JSON.parse((await gitShowFile(checkoutRoot, sourceCommit, path))!)
      .name as string;
    const block = new RegExp(
      `"${workspace.replaceAll(".", "\\.")}":\\s*\\{[^}]*"name":\\s*"${name.replaceAll(".", "\\.")}"[^}]*"version":\\s*"${targetVersion.replaceAll(".", "\\.")}"`,
    );
    if (!block.test(after)) {
      throw new Error(
        `web candidate requires ${BUN_LOCK_PATH} to resolve ${name} at ${targetVersion}`,
      );
    }
    if (after.includes(`"${name}": "${sourceVersion}"`)) {
      throw new Error(
        `web candidate rejected stale ${name} dependency specifier ${sourceVersion} in ${BUN_LOCK_PATH}`,
      );
    }
  }
}

/**
 * The changelog and one matching release note must describe the target. The
 * changelog link is the machine-readable binding between the two.
 */
async function assertReleaseNotes(
  checkoutRoot: string,
  sourceCommit: string,
  { targetVersion }: WebCandidateVersions,
): Promise<void> {
  const changelog = await gitShowFile(checkoutRoot, sourceCommit, CHANGELOG_PATH);
  if (changelog === null) throw new Error(`web candidate requires ${CHANGELOG_PATH}`);
  let inventory;
  try {
    inventory = changelogInventory(changelog);
  } catch (error) {
    throw new Error(`web candidate rejected unparsable ${CHANGELOG_PATH}: ${String(error)}`);
  }
  const releaseNotePath = `docs/release-notes/${targetVersion}.md`;
  if (!inventory.releases.some(({ version }) => version === targetVersion)) {
    throw new Error(`web candidate requires a ${targetVersion} changelog release`);
  }
  const link = inventory.links.find(({ label }) => label === targetVersion);
  if (!link || link.target !== releaseNotePath) {
    throw new Error(
      `web candidate requires the ${targetVersion} changelog link to ${releaseNotePath}`,
    );
  }
  const notes = await gitShowFile(checkoutRoot, sourceCommit, releaseNotePath);
  if (notes === null || notes.trim() === "") {
    throw new Error(`web candidate requires staged release notes at ${releaseNotePath}`);
  }
}

/**
 * Exactly one commit changes the derived release-input set; every later change
 * is a declared evidence surface, and the changed evidence binds its
 * `source_commit` to that frozen commit.
 */
async function assertFrozenReleaseInputRange(
  checkoutRoot: string,
  requiredBaseCommit: string,
  sourceCommit: string,
  releaseInputPaths: readonly string[],
): Promise<string> {
  const commits = (
    await runCapture(
      ["git", "rev-list", "--first-parent", "--reverse", `${requiredBaseCommit}..${sourceCommit}`],
      checkoutRoot,
    )
  )
    .split("\n")
    .map((line) => line.trim())
    .filter(Boolean);
  const frozen: string[] = [];
  const rangePaths = new Set<string>();
  for (const commit of commits) {
    const paths = await commitChangedPaths(checkoutRoot, commit);
    for (const path of paths) rangePaths.add(path);
    if (paths.some((path) => releaseInputPaths.includes(path))) frozen.push(commit);
  }
  if (frozen.length !== 1) {
    throw new Error(
      `web candidate requires exactly one frozen release-input commit; found ${frozen.length}`,
    );
  }
  const frozenCommit = frozen[0];
  let bound = false;
  for (const path of sortedUnique([...rangePaths].filter(isWebCandidateEvidencePath))) {
    const text = await gitShowFile(checkoutRoot, sourceCommit, path);
    if (text === null) throw new Error(`web candidate rejected removed evidence ${path}`);
    for (const recorded of recordedSourceCommits(text)) {
      if (recorded !== frozenCommit) {
        throw new Error(
          `web candidate rejected evidence ${path} bound to ${recorded} instead of the frozen release-input commit ${frozenCommit}`,
        );
      }
      bound = true;
    }
  }
  if (!bound) {
    throw new Error(
      `web candidate requires generated evidence whose source_commit names the frozen release-input commit ${frozenCommit}`,
    );
  }
  return frozenCommit;
}

/**
 * The single generic web-candidate admission. It derives the version pair,
 * proves the lockstep release inputs, then admits only the bounded evidence
 * suffix. Any violation names the specific rule.
 */
export async function assertWebCandidateScope(
  checkoutRoot: string,
  requiredBaseCommit: string,
  sourceCommit: string,
): Promise<WebCandidateProof> {
  const versions = await deriveWebCandidateVersions(
    checkoutRoot,
    requiredBaseCommit,
    sourceCommit,
  );
  const releaseInputPaths = webCandidateReleaseInputPaths(checkoutRoot, versions.targetVersion);
  const changedPaths = await changedPathsForCommitRange(
    checkoutRoot,
    requiredBaseCommit,
    sourceCommit,
  );
  if (changedPaths.length === 0) {
    throw new Error("web candidate found no changed paths");
  }
  const outside = changedPaths.filter(
    (path) => !releaseInputPaths.includes(path) && !isWebCandidateEvidencePath(path),
  );
  if (outside.length > 0) {
    throw new Error(
      `web candidate rejected paths outside the release-input and evidence surfaces: ${outside.join(", ")}`,
    );
  }
  const missing = releaseInputPaths.filter((path) => !changedPaths.includes(path));
  if (missing.length > 0) {
    throw new Error(
      `web candidate requires the complete ${versions.targetVersion} release-input set; missing: ${missing.join(", ")}`,
    );
  }
  await assertJsLockstep(checkoutRoot, requiredBaseCommit, sourceCommit, changedPaths, versions);
  await assertCargoLockstep(checkoutRoot, requiredBaseCommit, sourceCommit, changedPaths, versions);
  await assertBunLock(checkoutRoot, requiredBaseCommit, sourceCommit, versions);
  await assertReleaseNotes(checkoutRoot, sourceCommit, versions);
  const frozenReleaseInputCommit = await assertFrozenReleaseInputRange(
    checkoutRoot,
    requiredBaseCommit,
    sourceCommit,
    releaseInputPaths,
  );
  return {
    mode: WEB_CANDIDATE_MODE,
    sourceVersion: versions.sourceVersion,
    targetVersion: versions.targetVersion,
    releaseNotePath: `docs/release-notes/${versions.targetVersion}.md`,
    frozenReleaseInputCommit,
    changedPaths,
    releaseInputPaths,
    evidencePaths: changedPaths.filter(isWebCandidateEvidencePath),
  };
}
