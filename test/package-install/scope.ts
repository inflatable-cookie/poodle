import { mkdtempSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

export const CERTIFICATION_WRITABLE_PATHS = [
  "PAPERCUTS.md",
  "scripts/web-distribution/**",
  "tasks/effigy.tasks.toml",
  "test/package-install/**",
  "docs/roadmaps/g16/059-installed-web-distribution-certification.md",
  "docs/logs/2026-09/20260902-g16-059-installed-web-distribution-certification.md",
] as const;

export const G16_054_CANDIDATE_SCOPE_MODE = "g16.054-candidate" as const;
export const G18_006_CANDIDATE_SCOPE_MODE = "g18.006-candidate" as const;
export const CANDIDATE_SCOPE_MODES = [
  G16_054_CANDIDATE_SCOPE_MODE,
  G18_006_CANDIDATE_SCOPE_MODE,
] as const;
export type CandidateScopeMode = (typeof CANDIDATE_SCOPE_MODES)[number];
export const CERTIFICATION_SCOPE_MODE_ENV = "POODLE_WEB_PACK_INSTALL_SCOPE_MODE";

export const LOCKSTEP_CARGO_MANIFEST_PATHS = [
  "packages/codegen/Cargo.toml",
  "packages/contracts/adapter/Cargo.toml",
  "packages/contracts/components/Cargo.toml",
  "packages/contracts/events/Cargo.toml",
  "packages/contracts/headless/Cargo.toml",
  "packages/contracts/ir/Cargo.toml",
  "packages/contracts/layout/Cargo.toml",
  "packages/contracts/markdown/Cargo.toml",
  "packages/contracts/node/Cargo.toml",
  "packages/contracts/style/Cargo.toml",
  "packages/contracts/tokens/Cargo.toml",
  "packages/gpui/adapter/Cargo.toml",
  "packages/gpui/node-backend/Cargo.toml",
  "packages/gpui/preview/Cargo.toml",
  "packages/jetstream/adapter/Cargo.toml",
  "packages/jetstream/preview/Cargo.toml",
  "packages/render/Cargo.toml",
] as const;

export const LOCKSTEP_CARGO_LOCK_PATHS = [
  "packages/gpui/node-backend/Cargo.lock",
  "packages/gpui/preview/Cargo.lock",
] as const;

export const LOCKSTEP_JS_MANIFEST_PATHS = [
  "packages/core/package.json",
  "packages/svelte/components/package.json",
  "packages/react/components/package.json",
] as const;

export const CANDIDATE_VERSION_PATHS = [
  ...LOCKSTEP_CARGO_MANIFEST_PATHS,
  ...LOCKSTEP_CARGO_LOCK_PATHS,
  ...LOCKSTEP_JS_MANIFEST_PATHS,
] as const;

export const CANDIDATE_GENERATED_STAMP_PATHS = [
  "packages/codegen/generated/conformance/vectors.json",
  "packages/codegen/generated/docs/badge.md",
  "packages/codegen/generated/docs/gauge.md",
  "packages/codegen/generated/docs/search-field.md",
  "packages/codegen/generated/json/badge.json",
  "packages/codegen/generated/json/gauge.json",
  "packages/codegen/generated/json/index.json",
  "packages/codegen/generated/json/search-field.json",
  "packages/codegen/generated/registry/registry.json",
  "packages/codegen/generated/schema/schema.json",
  "packages/codegen/generated/ts/badge.ts",
  "packages/codegen/generated/ts/gauge.ts",
  "packages/codegen/generated/ts/index.ts",
  "packages/codegen/generated/ts/search-field.ts",
  "packages/codegen/generated/ts/shared-types.ts",
  "packages/contracts/headless/src/generated/machines/hover.rs",
  "packages/contracts/headless/src/generated/machines/menu.rs",
  "packages/contracts/headless/src/generated/machines/modal.rs",
  "packages/contracts/headless/src/generated/machines/popover.rs",
  "packages/core/src/generated/machines/hover.ts",
  "packages/core/src/generated/machines/menu.ts",
  "packages/core/src/generated/machines/modal.ts",
  "packages/core/src/generated/machines/popover.ts",
  "packages/gpui/preview/src/generated/catalogue/catalogue.rs",
  "packages/gpui/preview/src/generated/preview-shell.rs",
  "packages/gpui/preview/src/generated/specimens/specimens.rs",
  "packages/jetstream/preview/src/generated/catalogue/catalogue.rs",
  "packages/jetstream/preview/src/generated/preview-shell.rs",
  "packages/jetstream/preview/src/generated/specimens/specimens.rs",
  "packages/react/preview/src/generated/catalogue/catalogue.ts",
  "packages/react/preview/src/generated/preview-shell.ts",
  "packages/react/preview/src/generated/specimens/avatar-specimen.ts",
  "packages/react/preview/src/generated/specimens/callout-specimen.ts",
  "packages/react/preview/src/generated/specimens/empty-state-specimen.ts",
  "packages/react/preview/src/generated/specimens/pill-specimen.ts",
  "packages/react/preview/src/generated/specimens/specimen-scenes.ts",
  "packages/react/preview/src/generated/specimens/spinner-specimen.ts",
  "packages/svelte/preview/src/generated/catalogue/catalogue.ts",
  "packages/svelte/preview/src/generated/preview-shell.ts",
  "packages/svelte/preview/src/generated/specimens/avatar-specimen.ts",
  "packages/svelte/preview/src/generated/specimens/callout-specimen.ts",
  "packages/svelte/preview/src/generated/specimens/empty-state-specimen.ts",
  "packages/svelte/preview/src/generated/specimens/pill-specimen.ts",
  "packages/svelte/preview/src/generated/specimens/specimen-scenes.ts",
  "packages/svelte/preview/src/generated/specimens/spinner-specimen.ts",
] as const;

export const CANDIDATE_WRITABLE_PATHS = [
  ...CERTIFICATION_WRITABLE_PATHS,
  "CHANGELOG.md",
  "bun.lock",
  "docs/release-notes/0.2.3.md",
  "docs/release-notes/0.3.0.md",
  "docs/release-notes/README.md",
  "docs/roadmaps/g16/054-historycenter-v030-release-candidate.md",
  "docs/logs/2026-09/20260902-g16-054-v030-release-candidate.md",
  "packages/core/README.md",
  "packages/react/components/README.md",
  "packages/svelte/components/README.md",
  ...CANDIDATE_VERSION_PATHS,
  ...CANDIDATE_GENERATED_STAMP_PATHS,
] as const;

/**
 * g18.006 release inputs. These are the lockstep version manifests, the
 * intra-repository requirements they carry, the tracked locks, the changelog
 * and the staged `0.4.0` release notes. Nothing else is a candidate input.
 */
export const G18_006_RELEASE_INPUT_PATHS = [
  "CHANGELOG.md",
  "bun.lock",
  "docs/release-notes/README.md",
  "docs/release-notes/0.4.0.md",
  ...LOCKSTEP_CARGO_MANIFEST_PATHS,
  ...LOCKSTEP_CARGO_LOCK_PATHS,
  ...LOCKSTEP_JS_MANIFEST_PATHS,
] as const;

/**
 * Generated and evidence surfaces that may follow the frozen release-input
 * commit: the codegen version stamps, the complete Nucleus cohort, the GPUI
 * census cohort and its mounted receipts. The fixed g18.006 execution record
 * is matched by pattern in the policy below.
 */
export const G18_006_EVIDENCE_PATHS = [
  ...CANDIDATE_GENERATED_STAMP_PATHS,
  "docs/evidence/nucleus/nucleus-parity-manifest.json",
  "docs/evidence/nucleus/nucleus-parity-receipts/**",
  "docs/evidence/nucleus/parity-evidence-ledger.md",
  "docs/evidence/gpui/**",
] as const;

/** The single fixed g18.006 execution record family. */
export const G18_006_EXECUTION_RECORD_PATTERN =
  /^docs\/logs\/\d{4}-\d{2}\/\d{8}-g18-006-[a-z0-9-]+\.md$/;

export const G18_006_WRITABLE_PATHS = [
  ...G18_006_RELEASE_INPUT_PATHS,
  ...G18_006_EVIDENCE_PATHS,
] as const;

/**
 * One closed admission policy per candidate release. Policy data stays
 * together: the source/target versions, the exact release inputs, the
 * generated/evidence families, the content rules and whether the candidate
 * may carry an evidence-only suffix after its frozen release-input commit.
 */
export type CandidatePolicy = {
  mode: CandidateScopeMode;
  sourceVersion: string;
  targetVersion: string;
  cargoManifestPaths: readonly string[];
  jsManifestPaths: readonly string[];
  versionPaths: readonly string[];
  writablePaths: readonly string[];
  writablePathPatterns: readonly RegExp[];
  evidencePaths: readonly string[];
  releaseInputPaths: readonly string[];
  releaseNotePath: string;
  /** g16.054 certifies a single candidate-tree child of the branch base. */
  requiresDirectSource: boolean;
  /** g18.006 certifies one frozen release-input commit plus evidence. */
  requiresFrozenReleaseInputs: boolean;
};

const G16_054_CANDIDATE_POLICY: CandidatePolicy = {
  mode: G16_054_CANDIDATE_SCOPE_MODE,
  sourceVersion: "0.2.3",
  targetVersion: "0.3.0",
  cargoManifestPaths: LOCKSTEP_CARGO_MANIFEST_PATHS,
  jsManifestPaths: LOCKSTEP_JS_MANIFEST_PATHS,
  versionPaths: CANDIDATE_VERSION_PATHS,
  writablePaths: CANDIDATE_WRITABLE_PATHS,
  writablePathPatterns: [],
  evidencePaths: CANDIDATE_GENERATED_STAMP_PATHS,
  releaseInputPaths: [],
  releaseNotePath: "docs/release-notes/0.3.0.md",
  requiresDirectSource: true,
  requiresFrozenReleaseInputs: false,
};

const G18_006_CANDIDATE_POLICY: CandidatePolicy = {
  mode: G18_006_CANDIDATE_SCOPE_MODE,
  sourceVersion: "0.3.0",
  targetVersion: "0.4.0",
  cargoManifestPaths: LOCKSTEP_CARGO_MANIFEST_PATHS,
  jsManifestPaths: LOCKSTEP_JS_MANIFEST_PATHS,
  versionPaths: CANDIDATE_VERSION_PATHS,
  writablePaths: G18_006_WRITABLE_PATHS,
  writablePathPatterns: [G18_006_EXECUTION_RECORD_PATTERN],
  evidencePaths: G18_006_EVIDENCE_PATHS,
  releaseInputPaths: G18_006_RELEASE_INPUT_PATHS,
  releaseNotePath: "docs/release-notes/0.4.0.md",
  requiresDirectSource: false,
  requiresFrozenReleaseInputs: true,
};

export function candidatePolicy(mode: CandidateScopeMode): CandidatePolicy {
  return mode === G16_054_CANDIDATE_SCOPE_MODE
    ? G16_054_CANDIDATE_POLICY
    : G18_006_CANDIDATE_POLICY;
}

const PRIVATE_DECLARATION_TOOLS_MANIFEST =
  "scripts/web-distribution/declaration-tools/package.json";

export const CERTIFICATION_FORBIDDEN_SURFACES = [
  {
    label: "workflow",
    patterns: [".github/workflows/**", ".github/actions/**"],
  },
  {
    label: "release",
    patterns: [
      "CHANGELOG.md",
      "CHANGELOG.*",
      "RELEASE_NOTES.md",
      "release/**",
      "releases/**",
      "docs/release/**",
      "docs/releases/**",
      ".changeset/**",
      "scripts/release/**",
      "scripts/publish/**",
      "tasks/release/**",
    ],
  },
  {
    label: "registry",
    patterns: [
      ".npmrc",
      ".npmignore",
      ".yarnrc",
      ".yarnrc.yml",
      ".pnpmfile.cjs",
      "registry/**",
      "scripts/registry/**",
      "scripts/publish/**",
    ],
  },
] as const;

export type CertificationScopeMode = "strict" | CandidateScopeMode;
export type InstalledScopeMode = "ordinary" | CertificationScopeMode;

export type InstalledScopeProof = {
  mode: InstalledScopeMode;
  requiredBaseCommit: string;
  sourceCommit: string;
  changedPaths: string[];
};

export type CertificationScopeProof = InstalledScopeProof & {
  mode: CertificationScopeMode;
};

function isCandidateScopeMode(mode: string): mode is CandidateScopeMode {
  return (CANDIDATE_SCOPE_MODES as readonly string[]).includes(mode);
}

function isCertificationScopeMode(mode: InstalledScopeMode): mode is CertificationScopeMode {
  return mode !== "ordinary";
}

export function emitsCertificationReceipt(mode: InstalledScopeMode): boolean {
  return isCertificationScopeMode(mode);
}

/** The policy allowlist for a certification mode, or null for ordinary. */
export function candidateWritablePaths(mode: InstalledScopeMode): readonly string[] | null {
  if (!isCertificationScopeMode(mode)) return null;
  return mode === "strict" ? CERTIFICATION_WRITABLE_PATHS : candidatePolicy(mode).writablePaths;
}

export function readInstalledScopeMode(value: string | undefined): InstalledScopeMode {
  if (!value || value === "ordinary") return "ordinary";
  if (value === "strict") return "strict";
  if (isCandidateScopeMode(value)) return value;
  throw new Error(
    `${CERTIFICATION_SCOPE_MODE_ENV} must be ordinary, strict, or ${CANDIDATE_SCOPE_MODES.join(
      ", ",
    )}: ${value}`,
  );
}

export function formatInstalledRunOutput(args: {
  mode: InstalledScopeMode;
  sourceCommit: string;
  falsificationReceipts: unknown;
  receiptSha256?: string;
  receipt?: unknown;
}): string {
  if (emitsCertificationReceipt(args.mode)) {
    return JSON.stringify(
      {
        receiptSha256: args.receiptSha256,
        receipt: args.receipt,
        falsificationReceipts: args.falsificationReceipts,
      },
      null,
      2,
    );
  }
  return JSON.stringify(
    {
      mode: args.mode,
      sourceCommit: args.sourceCommit,
      falsificationReceipts: args.falsificationReceipts,
    },
    null,
    2,
  );
}

function matchesScopePattern(path: string, pattern: string): boolean {
  if (pattern.endsWith("/**")) {
    return path.startsWith(pattern.slice(0, -2));
  }
  if (pattern.endsWith(".*")) {
    return path.startsWith(pattern.slice(0, -1));
  }
  return path === pattern;
}

function isCargoManifestOrLockPath(path: string): boolean {
  return (
    path === "Cargo.toml" ||
    path === "Cargo.lock" ||
    /^(?:packages|scripts)\/[^/]+(?:\/[^/]+)*\/Cargo\.(?:toml|lock)$/.test(path)
  );
}

function isCargoTomlPath(path: string): boolean {
  return isCargoManifestOrLockPath(path) && path.endsWith(".toml");
}

function isJsManifestPath(path: string): boolean {
  return (
    path === "package.json" ||
    /^(?:packages|scripts)\/[^/]+(?:\/[^/]+)*\/package\.json$/.test(path)
  );
}

const CHANGELOG_PATH = "CHANGELOG.md";
const EXECUTION_LOG_PATH = /^docs\/logs\/\d{4}-\d{2}\/\d{8}-g\d{2}-\d{3}-[a-z0-9-]+\.md$/;
const RELEASE_NOTE_PATH = /^docs\/release-notes\/[^/]+\.md$/;

type ChangelogInventory = {
  preamble: string;
  releases: { version: string; date: string; entries: string[] }[];
  links: { label: string; target: string }[];
};

function normalizedChangelogEntry(lines: string[]): string {
  return lines
    .join(" ")
    .replace(/^\s*-\s+/, "")
    .replace(/^\*\*(?:Release status|Release posture|Downstream checks)\.\*\*\s+/, "")
    .replace(/\s+/g, " ")
    .trim();
}

/**
 * Parse the release-semantic inventory from the repository's Keep a Changelog
 * shape. Structural headings, wrapping, and the three prose-to-entry labels
 * used by changelog cleanup are syntax; versions, dates, links, and entry text
 * are not. Unsupported shapes throw so ordinary scope fails closed.
 */
function changelogInventory(text: string): ChangelogInventory {
  const lines = text.replaceAll("\r\n", "\n").split("\n");
  if (lines[0] !== "# Changelog") throw new Error("missing changelog title");

  const preambleLines: string[] = [];
  const links: { label: string; target: string }[] = [];
  const releases: { version: string; date: string; entries: string[] }[] = [];
  let release: { version: string; date: string; entries: string[] } | null = null;
  let entryLines: string[] = [];
  let sawUnreleased = false;
  let sawReferenceLinks = false;
  const preparedUnpublishedVersions = new Set<string>();
  const changelogSections = new Set([
    "Added",
    "Added and changed",
    "Breaking",
    "Changed",
    "Deprecated",
    "Downstream checks",
    "Fixed",
    "Removed",
    "Security",
  ]);

  const flushEntry = () => {
    const entry = normalizedChangelogEntry(entryLines);
    entryLines = [];
    if (!entry) return;
    if (!release) throw new Error("changelog entry outside a release");
    if (release.version === "Unreleased" && entry === "Nothing yet.") return;
    release.entries.push(entry);
  };

  for (const line of lines.slice(1)) {
    const releaseHeading = /^## \[([^\]]+)\](?: - (\d{4}-\d{2}-\d{2})(?: \((.+)\))?)?$/.exec(line);
    if (releaseHeading) {
      if (sawReferenceLinks) throw new Error("release found after changelog links");
      flushEntry();
      const version = releaseHeading[1];
      const date = releaseHeading[2] ?? "";
      const annotation = releaseHeading[3];
      if ((version === "Unreleased") !== (date === "")) {
        throw new Error("invalid changelog release heading");
      }
      if (annotation !== undefined) {
        if (annotation !== "prepared — unpublished" || version === "Unreleased") {
          throw new Error("unsupported changelog release annotation");
        }
        preparedUnpublishedVersions.add(version);
      }
      if (releases.some((candidate) => candidate.version === version)) {
        throw new Error("duplicate changelog release");
      }
      release = { version, date, entries: [] };
      releases.push(release);
      if (version === "Unreleased") sawUnreleased = true;
      continue;
    }
    const link = /^\[([^\]]+)\]:\s+(\S+)\s*$/.exec(line);
    if (link) {
      flushEntry();
      sawReferenceLinks = true;
      links.push({ label: link[1], target: link[2] });
      continue;
    }
    const sectionHeading = /^###\s+(.+)$/.exec(line);
    if (sectionHeading) {
      if (sawReferenceLinks || !changelogSections.has(sectionHeading[1])) {
        throw new Error("unsupported changelog section");
      }
      flushEntry();
      if (!release) throw new Error("changelog section outside a release");
      continue;
    }
    if (/^#{1,6}\s/.test(line)) throw new Error("unsupported changelog heading");
    if (!release) {
      preambleLines.push(line);
      continue;
    }
    if (sawReferenceLinks && line.trim() !== "") {
      throw new Error("content found after changelog links");
    }
    if (line.trim() === "") {
      flushEntry();
      continue;
    }
    if (/^-\s+/.test(line)) flushEntry();
    entryLines.push(line);
  }
  flushEntry();

  if (!sawUnreleased || releases[0]?.version !== "Unreleased") {
    throw new Error("missing leading Unreleased section");
  }
  const unreleased = releases[0];
  if (unreleased.entries.length > 0) throw new Error("Unreleased is not empty");
  for (const version of preparedUnpublishedVersions) {
    const annotatedRelease = releases.find((candidate) => candidate.version === version);
    if (
      !annotatedRelease?.entries.some(
        (entry) =>
          entry.includes("was prepared") &&
          (entry.includes("unpublished") || entry.includes("never tagged or published")),
      )
    ) {
      throw new Error("release annotation lacks matching entry payload");
    }
  }
  if (links.length !== releases.length) throw new Error("release links are incomplete");
  const linkLabels = new Set(links.map(({ label }) => label));
  if (linkLabels.size !== links.length || releases.some(({ version }) => !linkLabels.has(version))) {
    throw new Error("release links do not match releases");
  }

  return {
    preamble: preambleLines.join(" ").replace(/\s+/g, " ").trim(),
    releases: releases.map(({ version, date, entries }) => ({
      version,
      date,
      entries: [...entries].sort(),
    })),
    links: [...links].sort((left, right) => left.label.localeCompare(right.label)),
  };
}

function isChangelogMaintenanceRange(changedPaths: string[]): boolean {
  if (!changedPaths.includes(CHANGELOG_PATH)) return false;
  const logs = changedPaths.filter((path) => EXECUTION_LOG_PATH.test(path));
  return (
    logs.length === 1 &&
    changedPaths.every((path) => path === CHANGELOG_PATH || EXECUTION_LOG_PATH.test(path))
  );
}

async function ordinaryChangelogMaintenancePermitted(
  checkoutRoot: string,
  requiredBaseCommit: string,
  sourceCommit: string,
  changedPaths: string[],
): Promise<boolean> {
  if (!isChangelogMaintenanceRange(changedPaths)) return false;
  const before = await gitShowFile(checkoutRoot, requiredBaseCommit, CHANGELOG_PATH);
  const after = await gitShowFile(checkoutRoot, sourceCommit, CHANGELOG_PATH);
  if (before === null || after === null) return false;
  try {
    return JSON.stringify(changelogInventory(before)) === JSON.stringify(changelogInventory(after));
  } catch {
    return false;
  }
}

export function isWritableCertificationPath(
  path: string,
  mode: CertificationScopeMode = "strict",
): boolean {
  const writablePaths = mode === "strict"
    ? CERTIFICATION_WRITABLE_PATHS
    : candidatePolicy(mode).writablePaths;
  if (writablePaths.some((pattern) => matchesScopePattern(path, pattern))) return true;
  if (mode === "strict") return false;
  return candidatePolicy(mode).writablePathPatterns.some((pattern) => pattern.test(path));
}

function isCandidatePath(path: string, paths: readonly string[]): boolean {
  return paths.some((candidatePath) => candidatePath === path);
}

export function forbiddenCertificationSurfaceLabels(
  path: string,
  mode: InstalledScopeMode = "strict",
): string[] {
  const labels = CERTIFICATION_FORBIDDEN_SURFACES.filter((surface) =>
    surface.patterns.some((pattern) => matchesScopePattern(path, pattern)),
  ).map((surface) => surface.label);
  const candidateMode = isCandidateScopeMode(mode);
  const candidateReleaseHonestyPath = candidateMode && path === "CHANGELOG.md";
  const filteredLabels = candidateReleaseHonestyPath
    ? labels.filter((label) => label !== "release")
    : labels;
  const isPackageManifest = isJsManifestPath(path);
  const candidateVersionPath =
    candidateMode && isCandidatePath(path, candidatePolicy(mode).versionPaths);
  if (
    isPackageManifest &&
    path !== PRIVATE_DECLARATION_TOOLS_MANIFEST &&
    !candidateVersionPath &&
    mode !== "ordinary"
  ) {
    filteredLabels.push("version");
  }
  // Ordinary Cargo version/publication labels are content-derived later;
  // path membership alone is not a package release version.
  if (isCargoManifestOrLockPath(path) && mode !== "ordinary" && !candidateVersionPath) {
    filteredLabels.push("version");
  }
  return [...new Set(filteredLabels)];
}

function looksLikeCargoManifest(text: string): boolean {
  return text.split(/\r?\n/).some((line) => /^\s*\[[^\]]+\]\s*(?:#.*)?$/.test(line));
}

function cargoTableName(line: string): string | null {
  const match = /^\s*\[([^\]]+)\]\s*(?:#.*)?$/.exec(line);
  return match ? match[1].trim() : null;
}

function cargoBareKeyValue(line: string): { key: string; value: string } | null {
  const match = /^\s*([A-Za-z0-9_.-]+)\s*=\s*(.*?)\s*(?:#.*)?$/.exec(line);
  if (!match) return null;
  return { key: match[1], value: match[2].trim() };
}

function isPackageVersionSection(section: string): boolean {
  return section === "package" || section === "workspace.package";
}

function isPatchOrReplaceSection(section: string): boolean {
  return (
    section === "patch" ||
    section === "replace" ||
    section.startsWith("patch.") ||
    section.startsWith("replace.")
  );
}

function cargoSectionKey(text: string, section: string, key: string): string | undefined {
  let current = "";
  for (const line of text.split(/\r?\n/)) {
    const name = cargoTableName(line);
    if (name !== null) {
      current = name;
      continue;
    }
    if (current !== section) continue;
    const kv = cargoBareKeyValue(line);
    if (kv?.key === key) return kv.value;
  }
  return undefined;
}

function cargoPackageVersionSignal(text: string): string {
  return JSON.stringify({
    packageVersion:
      cargoSectionKey(text, "package", "version") ??
      cargoSectionKey(text, "package", "version.workspace"),
    workspacePackageVersion:
      cargoSectionKey(text, "workspace.package", "version") ??
      cargoSectionKey(text, "workspace.package", "version.workspace"),
  });
}

function unquoteTomlValue(value: string): string {
  return value.replace(/^"(.*)"$/, "$1");
}

function hasInlineTransportKey(line: string): boolean {
  return /(?:^|\{|,)\s*(?:source|registry)\s*=/.test(line);
}

function cargoTransportFingerprint(text: string): string {
  const parts: string[] = [];
  let section = "";
  for (const raw of text.split(/\r?\n/)) {
    const line = raw.trim();
    if (!line || line.startsWith("#")) continue;
    const name = cargoTableName(line);
    if (name !== null) {
      section = name;
      if (isPatchOrReplaceSection(section)) parts.push(`[${section}]`);
      continue;
    }
    if (isPatchOrReplaceSection(section)) {
      parts.push(`${section}|${line}`);
      continue;
    }
    const kv = cargoBareKeyValue(line);
    if (
      isPackageVersionSection(section) &&
      kv &&
      (kv.key === "publish" || kv.key === "registry" || kv.key === "source")
    ) {
      parts.push(`${section}|${kv.key}=${kv.value}`);
      continue;
    }
    if (!isPackageVersionSection(section) && hasInlineTransportKey(line)) {
      parts.push(`${section}|${line}`);
    }
  }
  return parts.sort().join("\n");
}

function isEffectivelyPublishable(text: string): boolean {
  const publish =
    cargoSectionKey(text, "package", "publish") ??
    cargoSectionKey(text, "workspace.package", "publish");
  return publish === undefined || unquoteTomlValue(publish) !== "false";
}

function newManifestHasDisallowedTransport(text: string): boolean {
  const allowed = new Set(["package|publish=false", "workspace.package|publish=false"]);
  const fingerprint = cargoTransportFingerprint(text);
  if (fingerprint.length === 0) return false;
  return fingerprint.split("\n").some((part) => !allowed.has(part));
}

function ordinaryCargoForbiddenLabels(
  before: string | null,
  after: string | null,
): string[] {
  const labels: string[] = [];
  if (before !== null && after !== null) {
    if (cargoPackageVersionSignal(before) !== cargoPackageVersionSignal(after)) {
      labels.push("version");
    }
    if (cargoTransportFingerprint(before) !== cargoTransportFingerprint(after)) {
      labels.push("registry");
    }
    return [...new Set(labels)];
  }
  if (after !== null && (isEffectivelyPublishable(after) || newManifestHasDisallowedTransport(after))) {
    labels.push("registry");
  }
  return labels;
}

/**
 * Ordinary JS package-manifest labels are content-derived. Dependency and
 * export changes are accepted only when the release-sensitive posture is
 * unchanged: package version, package name, the private publication flag,
 * and the publication/registry transport fields (`publishConfig`,
 * top-level `registry`). Anything else about the manifest may move.
 * Unparsable, added, or deleted manifests fail closed as version surfaces.
 */
function ordinaryJsForbiddenLabels(
  before: string | null,
  after: string | null,
): string[] {
  if (before === null || after === null) return ["version"];
  let beforeJson: unknown;
  let afterJson: unknown;
  try {
    beforeJson = JSON.parse(before);
  } catch {
    return ["version"];
  }
  try {
    afterJson = JSON.parse(after);
  } catch {
    return ["version"];
  }
  if (!isJsonRecord(beforeJson) || !isJsonRecord(afterJson)) return ["version"];
  const labels: string[] = [];
  for (const key of ["version", "name"]) {
    if (JSON.stringify(beforeJson[key]) !== JSON.stringify(afterJson[key])) {
      labels.push("version");
    }
  }
  if ((beforeJson["private"] === true) !== (afterJson["private"] === true)) {
    labels.push("registry");
  }
  const transportLeaves = [
    ...changedJsonLeafPaths(
      beforeJson["publishConfig"] ?? {},
      afterJson["publishConfig"] ?? {},
      "publishConfig",
    ),
    ...changedJsonLeafPaths(
      beforeJson["registry"] ?? null,
      afterJson["registry"] ?? null,
      "registry",
    ),
  ];
  if (transportLeaves.length > 0) labels.push("registry");
  return [...new Set(labels)];
}

function isJsonRecord(value: unknown): value is Record<string, unknown> {
  return Boolean(value) && typeof value === "object" && !Array.isArray(value);
}

function changedJsonLeafPaths(
  before: unknown,
  after: unknown,
  prefix = "",
): string[] {
  if (isJsonRecord(before) && isJsonRecord(after)) {
    const keys = new Set([...Object.keys(before), ...Object.keys(after)]);
    return [...keys].flatMap((key) =>
      changedJsonLeafPaths(before[key], after[key], prefix ? `${prefix}.${key}` : key),
    );
  }
  return JSON.stringify(before) === JSON.stringify(after) ? [] : [prefix];
}

async function runCapture(
  command: string[],
  cwd: string,
): Promise<string> {
  const child = Bun.spawn(command, {
    cwd,
    stdout: "pipe",
    stderr: "pipe",
  });
  const [stdout, stderr, exitCode] = await Promise.all([
    new Response(child.stdout).text(),
    new Response(child.stderr).text(),
    child.exited,
  ]);
  if (exitCode !== 0) {
    throw new Error(
      `Command failed (${exitCode}): ${command.join(" ")}\n${stderr.trim()}`,
    );
  }
  return stdout;
}

async function runResult(
  command: string[],
  cwd: string,
): Promise<{ exitCode: number }> {
  const child = Bun.spawn(command, {
    cwd,
    stdout: "pipe",
    stderr: "pipe",
  });
  const exitCode = await child.exited;
  return { exitCode };
}

async function gitShowFile(
  checkoutRoot: string,
  commit: string,
  path: string,
): Promise<string | null> {
  const exists = await runResult(
    ["git", "cat-file", "-e", `${commit}:${path}`],
    checkoutRoot,
  );
  if (exists.exitCode !== 0) return null;
  return runCapture(["git", "show", `${commit}:${path}`], checkoutRoot);
}

async function ordinaryJsForbiddenSurfaces(
  checkoutRoot: string,
  requiredBaseCommit: string,
  sourceCommit: string,
  changedPaths: string[],
): Promise<{ path: string; surface: string }[]> {
  const forbidden: { path: string; surface: string }[] = [];
  for (const path of changedPaths) {
    if (!isJsManifestPath(path) || path === PRIVATE_DECLARATION_TOOLS_MANIFEST) continue;
    const before = await gitShowFile(checkoutRoot, requiredBaseCommit, path);
    const after = await gitShowFile(checkoutRoot, sourceCommit, path);
    for (const surface of ordinaryJsForbiddenLabels(before, after)) {
      forbidden.push({ path, surface });
    }
  }
  return forbidden;
}

async function ordinaryCargoForbiddenSurfaces(
  checkoutRoot: string,
  requiredBaseCommit: string,
  sourceCommit: string,
  changedPaths: string[],
): Promise<{ path: string; surface: string }[]> {
  const forbidden: { path: string; surface: string }[] = [];
  for (const path of changedPaths) {
    if (!isCargoTomlPath(path)) continue;
    const before = await gitShowFile(checkoutRoot, requiredBaseCommit, path);
    const after = await gitShowFile(checkoutRoot, sourceCommit, path);
    if (
      (before !== null && !looksLikeCargoManifest(before)) ||
      (after !== null && !looksLikeCargoManifest(after))
    ) {
      throw new Error(`certification scope rejected unparsable Cargo manifest: ${path}`);
    }
    for (const surface of ordinaryCargoForbiddenLabels(before, after)) {
      forbidden.push({ path, surface });
    }
  }
  return forbidden;
}

function sortedUnique(values: Iterable<string>): string[] {
  return [...new Set(values)].sort();
}

export function requireExactCommit(value: string, label: string): string {
  if (!/^[0-9a-f]{40}$/.test(value)) {
    throw new Error(`${label} is not an exact Git SHA: ${value}`);
  }
  return value;
}

const CANDIDATE_MANIFEST_LEAF_ALLOWLIST: Record<string, readonly string[]> = {
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

const INTERNAL_JS_DEPENDENCY_SECTIONS = [
  "dependencies",
  "devDependencies",
  "peerDependencies",
  "optionalDependencies",
] as const;

const INTERNAL_JS_DEPENDENCY_PREFIX = "@inflatable-cookie/poodle-";

/**
 * Every internal Poodle dependency an installed JS manifest declares. The
 * closed candidate policy requires the same dependency identity at the base
 * and the head with an exact `sourceVersion` -> `targetVersion` transition, so
 * a stale or arbitrary specifier cannot ride a lockstep version bump.
 */
function internalJsDependencies(manifest: Record<string, unknown>): Map<string, string> {
  const dependencies = new Map<string, string>();
  for (const section of INTERNAL_JS_DEPENDENCY_SECTIONS) {
    const entries = manifest[section];
    if (entries === undefined) continue;
    if (!isJsonRecord(entries)) {
      throw new Error(`candidate scope rejected non-record JS dependency section ${section}`);
    }
    for (const [name, specifier] of Object.entries(entries)) {
      if (!name.startsWith(INTERNAL_JS_DEPENDENCY_PREFIX)) continue;
      if (typeof specifier !== "string") {
        throw new Error(`candidate scope rejected non-string internal JS dependency ${name}`);
      }
      dependencies.set(`${section}:${name}`, specifier);
    }
  }
  return dependencies;
}

async function assertCandidateManifestHonesty(
  checkoutRoot: string,
  requiredBaseCommit: string,
  sourceCommit: string,
  changedPaths: string[],
  policy: CandidatePolicy,
): Promise<void> {
  for (const path of policy.jsManifestPaths) {
    if (!changedPaths.includes(path)) continue;
    const allowedChanges = CANDIDATE_MANIFEST_LEAF_ALLOWLIST[path];
    if (!allowedChanges) {
      throw new Error(`candidate scope has no manifest honesty rule for ${path}`);
    }
    const before = JSON.parse(
      await runCapture(["git", "show", `${requiredBaseCommit}:${path}`], checkoutRoot),
    ) as Record<string, unknown>;
    const after = JSON.parse(
      await runCapture(["git", "show", `${sourceCommit}:${path}`], checkoutRoot),
    ) as Record<string, unknown>;
    const changes = changedJsonLeafPaths(before, after).sort();
    const unauthorizedChanges = changes.filter((change) => !allowedChanges.includes(change));
    if (unauthorizedChanges.length > 0) {
      throw new Error(
        `candidate scope rejected unauthorized ${path} changes: ${unauthorizedChanges.join(", ")}`,
      );
    }
    if (before.version !== policy.sourceVersion) {
      throw new Error(
        `candidate scope requires ${path} to begin at version ${policy.sourceVersion}`,
      );
    }
    if (after.version !== policy.targetVersion) {
      throw new Error(`candidate scope requires ${path} version ${policy.targetVersion}`);
    }
    if (path === "packages/react/components/package.json" && after.private !== true) {
      throw new Error("candidate scope rejected React admission: package must remain private");
    }
    const beforeInternal = internalJsDependencies(before);
    const afterInternal = internalJsDependencies(after);
    for (const dependency of sortedUnique([
      ...beforeInternal.keys(),
      ...afterInternal.keys(),
    ])) {
      const beforeSpecifier = beforeInternal.get(dependency);
      const afterSpecifier = afterInternal.get(dependency);
      if (beforeSpecifier === undefined || afterSpecifier === undefined) {
        throw new Error(
          `candidate scope rejected added or removed internal JS dependency ${dependency} in ${path}`,
        );
      }
      if (
        beforeSpecifier !== policy.sourceVersion ||
        afterSpecifier !== policy.targetVersion
      ) {
        throw new Error(
          `candidate scope requires internal JS dependency ${dependency} in ${path} to move ${policy.sourceVersion} -> ${policy.targetVersion}, found ${beforeSpecifier} -> ${afterSpecifier}`,
        );
      }
    }
  }
}

function cargoSectionForLine(text: string, lineNumber: number): string {
  let section = "";
  for (const [index, line] of text.split(/\r?\n/).entries()) {
    const match = /^\s*\[([^\]]+)\]\s*$/.exec(line);
    if (match) section = match[1];
    if (index + 1 === lineNumber) return section;
  }
  return section;
}

type CargoDiffLine = { line: string; lineNumber: number };

function parseCargoDiffLines(diff: string): {
  added: CargoDiffLine[];
  removed: string[];
} {
  let nextLineNumber = 0;
  const added: CargoDiffLine[] = [];
  const removed: string[] = [];
  for (const line of diff.split("\n")) {
    const hunk = /^@@ -\d+(?:,\d+)? \+(\d+)/.exec(line);
    if (hunk) {
      nextLineNumber = Number(hunk[1]);
      continue;
    }
    if (line.startsWith("+++") || line.startsWith("---")) continue;
    if (line.startsWith("+")) {
      added.push({ line: line.slice(1), lineNumber: nextLineNumber });
      nextLineNumber += 1;
      continue;
    }
    if (line.startsWith("-")) {
      removed.push(line.slice(1));
      continue;
    }
    if (line.startsWith(" ")) nextLineNumber += 1;
  }
  return { added, removed };
}

type CandidateCargoRequirement = { name: string; path: string };

function parseCandidateCargoRequirement(
  line: string,
  version: string,
): CandidateCargoRequirement | null {
  const match = new RegExp(
    `^(poodle-[A-Za-z0-9_-]+)\\s*=\\s*\\{\\s*version\\s*=\\s*"${version.replaceAll(".", "\\.")}",\\s*path\\s*=\\s*"([^"]+)"\\s*\\}$`,
  ).exec(line);
  if (!match) return null;
  return { name: match[1], path: match[2] };
}

const INLINE_CARGO_REQUIREMENT_SECTIONS = new Set([
  "dependencies",
  "dev-dependencies",
  "build-dependencies",
]);

type InlineCargoRequirement = { name: string; version: string | null; path: string | null };

function parseInlineCargoRequirement(line: string): InlineCargoRequirement | null {
  const match = /^\s*(poodle-[A-Za-z0-9_-]+)\s*=\s*\{(.*)\}\s*(?:#.*)?$/.exec(line);
  if (!match) return null;
  const entries = new Map<string, string>();
  for (const part of match[2].split(",")) {
    const keyValue = /^\s*([A-Za-z0-9_.-]+)\s*=\s*"([^"]*)"\s*$/.exec(part);
    if (keyValue) entries.set(keyValue[1], keyValue[2]);
  }
  return {
    name: match[1],
    version: entries.get("version") ?? null,
    path: entries.get("path") ?? null,
  };
}

/**
 * Every intra-repository Poodle requirement a Cargo manifest carries, keyed by
 * crate name. Version-carrying requirements must move `sourceVersion` ->
 * `targetVersion`; path-only requirements must stay byte-identical in
 * identity, so a manifest cannot leave a stale or arbitrary requirement behind
 * a lockstep `[package]` version bump.
 */
function cargoIntraRepoRequirements(text: string): Map<string, InlineCargoRequirement> {
  const requirements = new Map<string, InlineCargoRequirement>();
  let section = "";
  for (const raw of text.split(/\r?\n/)) {
    const table = cargoTableName(raw);
    if (table !== null) {
      section = table;
      continue;
    }
    if (!INLINE_CARGO_REQUIREMENT_SECTIONS.has(section)) continue;
    const requirement = parseInlineCargoRequirement(raw);
    if (requirement) requirements.set(requirement.name, requirement);
  }
  return requirements;
}

async function assertCandidateCargoManifestHonesty(
  checkoutRoot: string,
  requiredBaseCommit: string,
  sourceCommit: string,
  changedPaths: string[],
  policy: CandidatePolicy,
): Promise<void> {
  const cargoPaths = policy.cargoManifestPaths;
  for (const path of cargoPaths) {
    if (!changedPaths.includes(path)) continue;
    const diff = await runCapture(
      [
        "git",
        "diff",
        "--no-ext-diff",
        "--unified=0",
        requiredBaseCommit,
        sourceCommit,
        "--",
        path,
      ],
      checkoutRoot,
    );
    const { added, removed } = parseCargoDiffLines(diff);
    const changedLineText = [
      ...removed,
      ...added.map(({ line }) => line),
    ];
    const transportLines = changedLineText.filter((line) =>
      /^\s*(?:publish|registry|source)\s*=/.test(line) ||
      /^\s*\[(?:patch|replace)(?:\.|\])/.test(line),
    );
    if (transportLines.length > 0) {
      throw new Error(
        `candidate scope rejected Cargo publication/registry/source content in ${path}: ${transportLines.join(", ")}`,
      );
    }
    if (removed.length !== added.length) {
      throw new Error(
        `candidate scope rejected unpaired Cargo manifest content in ${path}; only version and exact intra-repository Poodle requirements may change`,
      );
    }
    const sourceText = await runCapture(
      ["git", "show", `${sourceCommit}:${path}`],
      checkoutRoot,
    );
    for (let index = 0; index < removed.length; index += 1) {
      const oldLine = removed[index];
      const newLine = added[index].line;
      const section = cargoSectionForLine(sourceText, added[index].lineNumber);
      const packageVersionChange =
        section === "package" &&
        oldLine === `version = "${policy.sourceVersion}"` &&
        newLine === `version = "${policy.targetVersion}"`;
      const oldRequirement = parseCandidateCargoRequirement(oldLine, policy.sourceVersion);
      const newRequirement = parseCandidateCargoRequirement(newLine, policy.targetVersion);
      const dependencyVersionChange =
        (section === "dependencies" || section === "dev-dependencies") &&
        oldRequirement !== null &&
        newRequirement !== null &&
        oldRequirement.name === newRequirement.name &&
        oldRequirement.path === newRequirement.path;
      if (!packageVersionChange && !dependencyVersionChange) {
        throw new Error(
          `candidate scope rejected unauthorized Cargo manifest change in ${path}: ${oldLine} -> ${newLine}; only [package] version and same-identity intra-repository Poodle requirement version changes may appear`,
        );
      }
    }
    const baseText = await runCapture(
      ["git", "show", `${requiredBaseCommit}:${path}`],
      checkoutRoot,
    );
    const beforeRequirements = cargoIntraRepoRequirements(baseText);
    const afterRequirements = cargoIntraRepoRequirements(sourceText);
    for (const name of sortedUnique([
      ...beforeRequirements.keys(),
      ...afterRequirements.keys(),
    ])) {
      const beforeRequirement = beforeRequirements.get(name);
      const afterRequirement = afterRequirements.get(name);
      if (!beforeRequirement || !afterRequirement) {
        throw new Error(
          `candidate scope rejected added or removed intra-repository Cargo requirement ${name} in ${path}`,
        );
      }
      if (beforeRequirement.path !== afterRequirement.path) {
        throw new Error(
          `candidate scope rejected retargeted intra-repository Cargo requirement ${name} in ${path}`,
        );
      }
      if (afterRequirement.version === null) {
        if (beforeRequirement.version !== null) {
          throw new Error(
            `candidate scope rejected version removal from intra-repository Cargo requirement ${name} in ${path}`,
          );
        }
        continue;
      }
      if (
        beforeRequirement.version !== policy.sourceVersion ||
        afterRequirement.version !== policy.targetVersion
      ) {
        throw new Error(
          `candidate scope requires intra-repository Cargo requirement ${name} in ${path} to move ${policy.sourceVersion} -> ${policy.targetVersion}, found ${beforeRequirement.version} -> ${afterRequirement.version}`,
        );
      }
    }
  }
}

async function assertDirectCandidateSource(
  checkoutRoot: string,
  requiredBaseCommit: string,
  sourceCommit: string,
): Promise<void> {
  const firstParent = requireExactCommit(
    (
      await runCapture(["git", "rev-parse", `${sourceCommit}^`], checkoutRoot)
    ).trim(),
    "candidate source parent",
  );
  const commitDistance = Number(
    (
      await runCapture(
        ["git", "rev-list", "--count", `${requiredBaseCommit}..${sourceCommit}`],
        checkoutRoot,
      )
    ).trim(),
  );
  if (firstParent !== requiredBaseCommit || commitDistance !== 1) {
    throw new Error(
      `candidate scope requires the certified source to be the direct one-commit child of ${requiredBaseCommit}; evidence heads or hidden prior candidate commits are rejected (source ${sourceCommit}, first parent ${firstParent}, distance ${commitDistance})`,
    );
  }
}

async function changedPathsForCommitRange(
  checkoutRoot: string,
  requiredBaseCommit: string,
  sourceCommit: string,
): Promise<string[]> {
  const ancestor = await runResult(
    ["git", "merge-base", "--is-ancestor", requiredBaseCommit, sourceCommit],
    checkoutRoot,
  );
  if (ancestor.exitCode !== 0) {
    throw new Error(
      `certification source ${sourceCommit} is not descended from required base ${requiredBaseCommit}`,
    );
  }
  const changedPaths = await runCapture(
    [
      "git",
      "diff",
      "--name-only",
      "--no-renames",
      "--diff-filter=ACDMRTUXB",
      "-z",
      requiredBaseCommit,
      sourceCommit,
    ],
    checkoutRoot,
  );
  return sortedUnique(changedPaths.split("\0").filter(Boolean));
}

async function commitChangedPaths(checkoutRoot: string, commit: string): Promise<string[]> {
  const output = await runCapture(
    [
      "git",
      "diff-tree",
      "-r",
      "--name-only",
      "--no-commit-id",
      "--first-parent",
      "-m",
      "--no-renames",
      "-z",
      commit,
    ],
    checkoutRoot,
  );
  return sortedUnique(output.split("\0").filter(Boolean));
}

function recordedSourceCommits(text: string): string[] {
  return [...text.matchAll(/"source_commit"\s*:\s*"([0-9a-f]{40})"/g)].map(
    (match) => match[1],
  );
}

function isClosedCandidateEvidencePath(path: string, policy: CandidatePolicy): boolean {
  return policy.evidencePaths.some((pattern) => matchesScopePattern(path, pattern));
}

async function assertCandidateReleaseHonesty(
  checkoutRoot: string,
  sourceCommit: string,
  policy: CandidatePolicy,
): Promise<void> {
  const changelog = await gitShowFile(checkoutRoot, sourceCommit, CHANGELOG_PATH);
  if (changelog === null) {
    throw new Error(`closed candidate scope requires ${CHANGELOG_PATH}`);
  }
  let inventory: ChangelogInventory;
  try {
    inventory = changelogInventory(changelog);
  } catch (error) {
    throw new Error(
      `closed candidate scope rejected unparsable ${CHANGELOG_PATH}: ${String(error)}`,
    );
  }
  const release = inventory.releases.find(
    ({ version }) => version === policy.targetVersion,
  );
  if (!release) {
    throw new Error(
      `closed candidate scope requires a ${policy.targetVersion} changelog release`,
    );
  }
  const link = inventory.links.find(({ label }) => label === policy.targetVersion);
  if (!link || link.target !== policy.releaseNotePath) {
    throw new Error(
      `closed candidate scope requires the ${policy.targetVersion} changelog link to ${policy.releaseNotePath}`,
    );
  }
  const notes = await gitShowFile(checkoutRoot, sourceCommit, policy.releaseNotePath);
  if (notes === null || notes.trim() === "") {
    throw new Error(
      `closed candidate scope requires staged release notes at ${policy.releaseNotePath}`,
    );
  }
}

/**
 * Validate the frozen candidate identity: exactly one commit changes the
 * complete release-input set, every later change is generated evidence or the
 * fixed execution record, and the changed evidence binds its `source_commit`
 * to that frozen release-input commit. Because the release-input commit is
 * unique, the final inputs are byte-identical to it by construction.
 */
async function assertFrozenCandidateRange(
  checkoutRoot: string,
  requiredBaseCommit: string,
  sourceCommit: string,
  changedPaths: string[],
  policy: CandidatePolicy,
): Promise<void> {
  const missingInputs = policy.releaseInputPaths.filter(
    (path) => !changedPaths.includes(path),
  );
  if (missingInputs.length > 0) {
    throw new Error(
      `closed candidate scope requires the complete ${policy.targetVersion} release-input set; missing: ${missingInputs.join(", ")}`,
    );
  }
  const commits = (
    await runCapture(
      ["git", "rev-list", "--first-parent", "--reverse", `${requiredBaseCommit}..${sourceCommit}`],
      checkoutRoot,
    )
  )
    .split("\n")
    .map((line) => line.trim())
    .filter(Boolean);
  const frozenCommits: string[] = [];
  const rangePaths = new Set<string>();
  for (const commit of commits) {
    const paths = await commitChangedPaths(checkoutRoot, commit);
    for (const path of paths) rangePaths.add(path);
    if (paths.some((path) => policy.releaseInputPaths.includes(path))) {
      frozenCommits.push(commit);
    }
  }
  if (frozenCommits.length !== 1) {
    throw new Error(
      `closed candidate scope requires exactly one frozen ${policy.targetVersion} release-input commit; found ${frozenCommits.length}: ${frozenCommits.join(", ")}`,
    );
  }
  const frozenCommit = frozenCommits[0];
  let bound = false;
  for (const path of sortedUnique(
    [...rangePaths].filter((candidate) => isClosedCandidateEvidencePath(candidate, policy)),
  )) {
    const text = await gitShowFile(checkoutRoot, sourceCommit, path);
    if (text === null) {
      throw new Error(
        `closed candidate scope rejected removed candidate evidence: ${path}`,
      );
    }
    for (const recorded of recordedSourceCommits(text)) {
      if (recorded !== frozenCommit) {
        throw new Error(
          `closed candidate scope rejected evidence ${path} bound to ${recorded} instead of the frozen release-input commit ${frozenCommit}`,
        );
      }
      bound = true;
    }
  }
  if (!bound) {
    throw new Error(
      `closed candidate scope requires generated evidence whose source_commit names the frozen release-input commit ${frozenCommit}`,
    );
  }
}

/**
 * The single closed candidate validator. It is used both by the explicit
 * `g18.006-candidate` certification mode and by ordinary CI recognition, so a
 * PR lane and a local certification run cannot disagree about admission.
 */
async function assertClosedCandidateRange(
  checkoutRoot: string,
  requiredBaseCommit: string,
  sourceCommit: string,
  changedPaths: string[],
  policy: CandidatePolicy,
): Promise<void> {
  const outsideAllowlist = changedPaths.filter(
    (path) => !isWritableCertificationPath(path, policy.mode),
  );
  if (outsideAllowlist.length > 0) {
    throw new Error(
      `certification scope rejected paths outside writable allowlist: ${outsideAllowlist.join(", ")}`,
    );
  }
  const forbidden = changedPaths.flatMap((path) =>
    forbiddenCertificationSurfaceLabels(path, policy.mode).map((surface) => ({
      path,
      surface,
    })),
  );
  if (forbidden.length > 0) {
    throw new Error(
      `certification scope rejected forbidden ${forbidden
        .map(({ surface, path }) => `${surface} surface: ${path}`)
        .join(", ")}`,
    );
  }
  await assertFrozenCandidateRange(
    checkoutRoot,
    requiredBaseCommit,
    sourceCommit,
    changedPaths,
    policy,
  );
  await assertCandidateManifestHonesty(
    checkoutRoot,
    requiredBaseCommit,
    sourceCommit,
    changedPaths,
    policy,
  );
  await assertCandidateCargoManifestHonesty(
    checkoutRoot,
    requiredBaseCommit,
    sourceCommit,
    changedPaths,
    policy,
  );
  await assertCandidateReleaseHonesty(checkoutRoot, sourceCommit, policy);
}

/**
 * Ordinary CI carries no candidate environment variable. It may admit a
 * release-bearing range only when the complete base-to-head diff satisfies the
 * closed g18.006 policy; any failure stays an ordinary rejection.
 */
async function ordinaryAdmitsClosedCandidate(
  checkoutRoot: string,
  requiredBaseCommit: string,
  sourceCommit: string,
  changedPaths: string[],
): Promise<boolean> {
  try {
    await assertClosedCandidateRange(
      checkoutRoot,
      requiredBaseCommit,
      sourceCommit,
      changedPaths,
      candidatePolicy(G18_006_CANDIDATE_SCOPE_MODE),
    );
    return true;
  } catch {
    return false;
  }
}

export async function assertInstalledScope(
  checkoutRoot: string,
  requiredBaseCommit: string,
  sourceCommit: string,
  mode: InstalledScopeMode,
): Promise<InstalledScopeProof> {
  requireExactCommit(requiredBaseCommit, "required base commit");
  requireExactCommit(sourceCommit, "certification source commit");
  const changedPaths = await changedPathsForCommitRange(
    checkoutRoot,
    requiredBaseCommit,
    sourceCommit,
  );
  if (isCertificationScopeMode(mode) && changedPaths.length === 0) {
    throw new Error("certification scope found no changed paths");
  }
  const policy = isCandidateScopeMode(mode) ? candidatePolicy(mode) : null;
  if (policy?.requiresDirectSource) {
    await assertDirectCandidateSource(checkoutRoot, requiredBaseCommit, sourceCommit);
  }
  let forbidden = changedPaths.flatMap((path) =>
    forbiddenCertificationSurfaceLabels(path, mode).map((surface) => ({ path, surface })),
  );
  if (mode === "ordinary") {
    if (
      await ordinaryChangelogMaintenancePermitted(
        checkoutRoot,
        requiredBaseCommit,
        sourceCommit,
        changedPaths,
      )
    ) {
      forbidden = forbidden.filter(
        ({ path, surface }) => path !== CHANGELOG_PATH || surface !== "release",
      );
    }
    forbidden.push(
      ...(await ordinaryCargoForbiddenSurfaces(
        checkoutRoot,
        requiredBaseCommit,
        sourceCommit,
        changedPaths,
      )),
    );
    forbidden.push(
      ...(await ordinaryJsForbiddenSurfaces(
        checkoutRoot,
        requiredBaseCommit,
        sourceCommit,
        changedPaths,
      )),
    );
    // Ordinary CI has no candidate environment variable. A release-bearing
    // range (a forbidden version/release surface or any staged release note)
    // is admitted only when the complete diff satisfies the closed g18.006
    // candidate policy; anything else stays an ordinary rejection.
    const releaseNoteChanges = changedPaths.filter((path) => RELEASE_NOTE_PATH.test(path));
    if (
      (forbidden.length > 0 || releaseNoteChanges.length > 0) &&
      (await ordinaryAdmitsClosedCandidate(
        checkoutRoot,
        requiredBaseCommit,
        sourceCommit,
        changedPaths,
      ))
    ) {
      forbidden = [];
    } else if (forbidden.length === 0 && releaseNoteChanges.length > 0) {
      throw new Error(
        `certification scope rejected a partial release-note change without the closed ${G18_006_CANDIDATE_SCOPE_MODE} candidate: ${releaseNoteChanges.join(", ")}`,
      );
    }
  }
  if (forbidden.length > 0) {
    throw new Error(
      `certification scope rejected forbidden ${forbidden
        .map(({ surface, path }) => `${surface} surface: ${path}`)
        .join(", ")}`,
    );
  }
  if (policy?.requiresFrozenReleaseInputs) {
    await assertClosedCandidateRange(
      checkoutRoot,
      requiredBaseCommit,
      sourceCommit,
      changedPaths,
      policy,
    );
  } else if (isCertificationScopeMode(mode)) {
    const outsideAllowlist = changedPaths.filter(
      (path) => !isWritableCertificationPath(path, mode),
    );
    if (outsideAllowlist.length > 0) {
      throw new Error(
        `certification scope rejected paths outside writable allowlist: ${outsideAllowlist.join(", ")}`,
      );
    }
    if (policy) {
      await assertCandidateManifestHonesty(
        checkoutRoot,
        requiredBaseCommit,
        sourceCommit,
        changedPaths,
        policy,
      );
      await assertCandidateCargoManifestHonesty(
        checkoutRoot,
        requiredBaseCommit,
        sourceCommit,
        changedPaths,
        policy,
      );
    }
  }
  return { mode, requiredBaseCommit, sourceCommit, changedPaths };
}

export async function assertCertificationScope(
  checkoutRoot: string,
  requiredBaseCommit: string,
  sourceCommit: string,
  mode: CertificationScopeMode = "strict",
): Promise<CertificationScopeProof> {
  const proof = await assertInstalledScope(
    checkoutRoot,
    requiredBaseCommit,
    sourceCommit,
    mode,
  );
  if (!isCertificationScopeMode(proof.mode)) {
    throw new Error(`expected certification scope mode, found ${proof.mode}`);
  }
  return proof;
}

export async function withDisposableGitPlant<T>(
  work: (root: string) => Promise<T>,
): Promise<T> {
  const plantRoot = mkdtempSync(join(tmpdir(), "poodle-installed-scope-plant-"));
  try {
    return await work(plantRoot);
  } finally {
    rmSync(plantRoot, { recursive: true, force: true });
  }
}
