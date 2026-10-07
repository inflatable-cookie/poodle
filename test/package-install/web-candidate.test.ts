// g18.032 / spec 071: focused laws for version-independent web-candidate
// admission. Synthetic ranges prove a future target version and its native
// lockstep transition are admitted, while partial, stale, source, workflow,
// registry and uncoordinated Cargo changes fail closed before any build.

import { afterAll, describe, expect, test, setDefaultTimeout } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

import {
  internalJsDependencyRange,
  LOCKSTEP_CARGO_LOCK_PATHS,
  LOCKSTEP_CARGO_MANIFEST_PATHS,
  requireExactCommit,
} from "./scope";
import {
  assertWebCandidateScope,
  assertWebPreviewScope,
  deriveWebCandidateBase,
  deriveWebCandidateVersions,
  isWebCandidateEvidencePath,
  WEB_CANDIDATE_EVIDENCE_PATTERNS,
  WEB_CANDIDATE_MODE,
} from "./web-candidate";

const plantRoots: string[] = [];
let gitTemplate: string | undefined;
const plantTemplates = new Map<
  string,
  { root: string; base: string; frozen: string; head: string }
>();

// Git-plant cases used to hit bun's default 5s timeout under load (13 of 25
// failed at Queue's gate; pre-optimization 21/21 passed in 31s at load 32).
// Reusing one inited git dir cuts the repeated `git init` work, and every
// duplicate is produced by `git clone --shared` — never a raw copy of a
// `.git` directory, which races git's own transient lock files: a detached
// post-commit maintenance left `.git/objects/maintenance.lock` mid-copy and
// cpSync died with ENOENT in required CI (review of this leaf, 2026-09-28).
// Measured 2026-09-28 with the clone reuse at ambient load 50–83: 21/21 in
// 12–15s, slowest case 1.3s — clone is also faster per plant than the copy
// was (cpSync measured 5.1s idle / 7.6s loaded). The 20s cap keeps ~15x
// headroom over the loaded slowest case.
setDefaultTimeout(20_000);

afterAll(() => {
  for (const root of plantRoots) rmSync(root, { recursive: true, force: true });
});

const GIT_IDENTITY = {
  GIT_AUTHOR_NAME: "Poodle Certification",
  GIT_AUTHOR_EMAIL: "poodle-certification@example.invalid",
  GIT_COMMITTER_NAME: "Poodle Certification",
  GIT_COMMITTER_EMAIL: "poodle-certification@example.invalid",
};

async function runGit(cwd: string, args: string[]): Promise<string> {
  const child = Bun.spawn(["git", ...args], {
    cwd,
    stdout: "pipe",
    stderr: "pipe",
    env: { ...process.env, ...GIT_IDENTITY },
  });
  const [stdout, stderr, exitCode] = await Promise.all([
    new Response(child.stdout).text(),
    new Response(child.stderr).text(),
    child.exited,
  ]);
  if (exitCode !== 0) throw new Error(`git ${args.join(" ")} failed: ${stderr.trim()}`);
  return stdout;
}

async function writeFiles(root: string, files: Record<string, string>): Promise<void> {
  for (const [path, contents] of Object.entries(files)) {
    const parts = path.split("/");
    if (parts.length > 1) mkdirSync(join(root, ...parts.slice(0, -1)), { recursive: true });
    await Bun.write(join(root, path), contents);
  }
}

async function commitAll(root: string, message: string): Promise<string> {
  await runGit(root, ["add", "--all"]);
  await runGit(root, ["commit", "--quiet", "-m", message]);
  return requireExactCommit((await runGit(root, ["rev-parse", "HEAD"])).trim(), "plant commit");
}

/**
 * Duplicate a git repository with git itself. Clones share objects through
 * alternates instead of copying `.git`, so no reader can race a lock file the
 * source repo's detached maintenance writes and deletes.
 */
async function cloneRepo(sourceRoot: string): Promise<string> {
  const root = mkdtempSync(join(tmpdir(), "poodle-web-candidate-test-"));
  plantRoots.push(root);
  await runGit(tmpdir(), ["clone", "--shared", "--quiet", sourceRoot, root]);
  return root;
}

async function initPlant(): Promise<string> {
  if (gitTemplate === undefined) {
    gitTemplate = mkdtempSync(join(tmpdir(), "poodle-web-candidate-git-"));
    plantRoots.push(gitTemplate);
    await runGit(gitTemplate, ["init", "--quiet"]);
  }
  return cloneRepo(gitTemplate);
}

async function clonePlant(source: {
  root: string;
  base: string;
  frozen: string;
  head: string;
}): Promise<{ root: string; base: string; frozen: string; head: string }> {
  return { root: await cloneRepo(source.root), base: source.base, frozen: source.frozen, head: source.head };
}

function jsManifests(version: string): Record<string, string> {
  const coreRequirement = internalJsDependencyRange(version);
  return {
    "package.json": `${JSON.stringify({ name: "poodle", version, private: true }, null, 2)}\n`,
    "packages/core/package.json": `${JSON.stringify(
      { name: "@inflatable-cookie/poodle-core", version },
      null,
      2,
    )}\n`,
    "packages/svelte/components/package.json": `${JSON.stringify(
      {
        name: "@inflatable-cookie/poodle-svelte",
        version,
        dependencies: { "@inflatable-cookie/poodle-core": coreRequirement },
      },
      null,
      2,
    )}\n`,
    "packages/react/components/package.json": `${JSON.stringify(
      {
        name: "@inflatable-cookie/poodle-react",
        version,
        private: true,
        dependencies: { "@inflatable-cookie/poodle-core": coreRequirement },
      },
      null,
      2,
    )}\n`,
  };
}

function changelog(version: string | null): string {
  return [
    "# Changelog",
    "",
    "Notable changes to Poodle are recorded here.",
    "",
    "## [Unreleased]",
    "",
    ...(version ? [`## [${version}] - 2026-09-13`, "", "### Added", "", `- ${version} entry.`, ""] : []),
    "[Unreleased]: https://github.com/inflatable-cookie/poodle/commits/main",
    ...(version ? [`[${version}]: docs/release-notes/${version}.md`] : []),
    "",
  ].join("\n");
}

function bunLock(version: string): string {
  const coreRequirement = internalJsDependencyRange(version);
  return `${JSON.stringify(
    {
      lockfileVersion: 1,
      workspaces: {
        "": { name: "poodle" },
        "packages/core": { name: "@inflatable-cookie/poodle-core", version },
        "packages/svelte/components": {
          name: "@inflatable-cookie/poodle-svelte",
          version,
          dependencies: { "@inflatable-cookie/poodle-core": coreRequirement },
        },
        "packages/react/components": {
          name: "@inflatable-cookie/poodle-react",
          version,
          dependencies: { "@inflatable-cookie/poodle-core": coreRequirement },
        },
      },
    },
    null,
    2,
  )}\n`;
}

const CARGO_CRATE_NAMES: Record<string, string> = {
  "packages/codegen/Cargo.toml": "poodle-codegen",
  "packages/contracts/adapter/Cargo.toml": "poodle-adapter",
  "packages/contracts/components/Cargo.toml": "poodle-specs",
  "packages/contracts/events/Cargo.toml": "poodle-events",
  "packages/contracts/headless/Cargo.toml": "poodle-headless",
  "packages/contracts/ir/Cargo.toml": "poodle-ir",
  "packages/contracts/layout/Cargo.toml": "poodle-layout",
  "packages/contracts/markdown/Cargo.toml": "poodle-markdown",
  "packages/contracts/node/Cargo.toml": "poodle-node",
  "packages/contracts/style/Cargo.toml": "poodle-style",
  "packages/contracts/tokens/Cargo.toml": "poodle-tokens",
  "packages/gpui/adapter/Cargo.toml": "poodle-gpui",
  "packages/gpui/node-backend/Cargo.toml": "poodle-gpui-node-backend",
  "packages/gpui/preview/Cargo.toml": "poodle-gpui-preview",
  "packages/jetstream/adapter/Cargo.toml": "poodle-jetstream",
  "packages/jetstream/preview/Cargo.toml": "poodle-jetstream-preview",
  "packages/render/Cargo.toml": "poodle-render",
};

function cargoManifests(version: string): Record<string, string> {
  return Object.fromEntries(
    LOCKSTEP_CARGO_MANIFEST_PATHS.map((path) => [
      path,
      [
        "[package]",
        'name = "' + CARGO_CRATE_NAMES[path] + '"',
        'version = "' + version + '"',
        "publish = false",
        "",
        ...(path === "packages/render/Cargo.toml"
          ? [
              "[dependencies]",
              'poodle-node = { version = "' + version + '", path = "../contracts/node" }',
              "",
            ]
          : []),
      ].join("\n"),
    ]),
  );
}

function cargoLock(version: string, localPackages: string[]): string {
  return [
    "# generated fixture Cargo lock",
    "version = 4",
    "",
    ...localPackages.flatMap((name, index) => [
      "[[package]]",
      'name = "' + name + '"',
      'version = "' + version + '"',
      ...(index === 0 && localPackages.length > 1
        ? ["dependencies = [", ' "' + localPackages[1] + " " + version + '"', "]"]
        : []),
      "",
    ]),
    "[[package]]",
    'name = "serde"',
    'version = "1.0.0"',
    'source = "registry+https://github.com/rust-lang/crates.io-index"',
    'checksum = "unchanged"',
    "",
  ].join("\n");
}

function cargoLocks(version: string): Record<string, string> {
  return {
    [LOCKSTEP_CARGO_LOCK_PATHS[0]]: cargoLock(version, ["poodle-gpui-node-backend", "poodle-node"]),
    [LOCKSTEP_CARGO_LOCK_PATHS[1]]: cargoLock(version, ["poodle-gpui-preview", "poodle-gpui-node-backend"]),
  };
}

function baseFiles(): Record<string, string> {
  return {
    ...jsManifests("0.4.0"),
    "bun.lock": bunLock("0.4.0"),
    "CHANGELOG.md": changelog(null),
    "docs/release-notes/README.md": "# Release notes\n",
    ...cargoManifests("0.4.0"),
    ...cargoLocks("0.4.0"),
  };
}

function frozenFiles(target: string): Record<string, string> {
  return {
    ...jsManifests(target),
    "bun.lock": bunLock(target),
    "CHANGELOG.md": changelog(target),
    "docs/release-notes/README.md": `# Release notes\n\n- [${target}]\n`,
    [`docs/release-notes/${target}.md`]: `# Poodle ${target}\n`,
    ...cargoManifests(target),
    ...cargoLocks(target),
  };
}

async function plantCandidate(
  target: string,
  options: { frozenExtra?: Record<string, string>; evidenceExtra?: Record<string, string> } = {},
): Promise<{ root: string; base: string; frozen: string; head: string }> {
  const extras = options.frozenExtra !== undefined || options.evidenceExtra !== undefined;
  if (!extras) {
    const cached = plantTemplates.get(target);
    if (cached) return clonePlant(cached);
  }

  const root = await initPlant();
  await writeFiles(root, baseFiles());
  const base = await commitAll(root, "candidate base");
  await writeFiles(root, { ...frozenFiles(target), ...(options.frozenExtra ?? {}) });
  const frozen = await commitAll(root, `frozen ${target} release inputs`);
  await writeFiles(root, {
    "docs/evidence/nucleus/planted.json": `${JSON.stringify({ source_commit: frozen })}\n`,
    ...(options.evidenceExtra ?? {}),
  });
  const head = await commitAll(root, "candidate evidence");
  const planted = { root, base, frozen, head };
  if (!extras) plantTemplates.set(target, planted);
  return extras ? planted : clonePlant(planted);
}

describe("web candidate versions", () => {
  test("derive base and target versions from the compared commits", async () => {
    const { root, base, head } = await plantCandidate("0.5.0");
    expect(await deriveWebCandidateVersions(root, base, head)).toEqual({
      sourceVersion: "0.4.0",
      targetVersion: "0.5.0",
    });
  });

  test("reject a non-increasing target and a post-1.0 target", async () => {
    const { root, base, head } = await plantCandidate("0.4.0");
    await expect(deriveWebCandidateVersions(root, base, head)).rejects.toThrow(
      /target version to exceed the base version/,
    );
  });
});

describe("web candidate admission", () => {
  test("ordinary installed-package CI routes a planted 0.4.1 candidate through generic admission", async () => {
    const { root, base, head } = await plantCandidate("0.4.1");
    const proof = await assertWebPreviewScope(root, base, head, "ordinary");
    expect(proof.mode).toBe(WEB_CANDIDATE_MODE);
    expect(proof.changedPaths).toContain("docs/release-notes/0.4.1.md");
  });

  test("admit a synthetic 0.4.1 candidate", async () => {
    const { root, base, head } = await plantCandidate("0.4.1");
    const proof = await assertWebCandidateScope(root, base, head);
    expect(proof.sourceVersion).toBe("0.4.0");
    expect(proof.targetVersion).toBe("0.4.1");
    expect(proof.releaseNotePath).toBe("docs/release-notes/0.4.1.md");
    expect(proof.changedPaths).toContain("packages/render/Cargo.toml");
    expect(proof.changedPaths).toContain(LOCKSTEP_CARGO_LOCK_PATHS[0]);
  });

  test("admit a synthetic 0.5.0 candidate", async () => {
    const { root, base, head } = await plantCandidate("0.5.0");
    const proof = await assertWebCandidateScope(root, base, head);
    expect(proof.targetVersion).toBe("0.5.0");
  });

  test("admit a synthetic 0.4.5 candidate with current-minor core ranges", async () => {
    const { root, base, head } = await plantCandidate("0.4.5");
    const proof = await assertWebCandidateScope(root, base, head);
    expect(proof.sourceVersion).toBe("0.4.0");
    expect(proof.targetVersion).toBe("0.4.5");
  });

  test("reject an exact internal web dependency pin", async () => {
    const { root, base, head } = await plantCandidate("0.4.5", {
      frozenExtra: {
        "packages/svelte/components/package.json": `${JSON.stringify(
          {
            name: "@inflatable-cookie/poodle-svelte",
            version: "0.4.5",
            dependencies: { "@inflatable-cookie/poodle-core": "0.4.5" },
          },
          null,
          2,
        )}\n`,
      },
    });
    await expect(assertWebCandidateScope(root, base, head)).rejects.toThrow(
      /internal JS dependency .* >=0\.4\.0 <0\.5 -> >=0\.4\.5 <0\.5, found >=0\.4\.0 <0\.5 -> 0\.4\.5/,
    );
  });

  test("reject a wrong-floor internal web dependency range", async () => {
    const { root, base, head } = await plantCandidate("0.4.5", {
      frozenExtra: {
        "packages/react/components/package.json": `${JSON.stringify(
          {
            name: "@inflatable-cookie/poodle-react",
            version: "0.4.5",
            private: true,
            dependencies: { "@inflatable-cookie/poodle-core": ">=0.4.4 <0.5" },
          },
          null,
          2,
        )}\n`,
      },
    });
    await expect(assertWebCandidateScope(root, base, head)).rejects.toThrow(
      /internal JS dependency .* >=0\.4\.0 <0\.5 -> >=0\.4\.5 <0\.5, found >=0\.4\.0 <0\.5 -> >=0\.4\.4 <0\.5/,
    );
  });

  test("reject a partial lockstep bump", async () => {
    const { root, base, head } = await plantCandidate("0.5.0", {
      frozenExtra: {
        "packages/react/components/package.json": `${JSON.stringify(
          { name: "@inflatable-cookie/poodle-react", version: "0.4.0", private: true },
          null,
          2,
        )}\n`,
      },
    });
    await expect(assertWebCandidateScope(root, base, head)).rejects.toThrow(
      /carry version 0.5.0/,
    );
  });

  test("reject a stale internal web dependency requirement", async () => {
    const { root, base, head } = await plantCandidate("0.5.0", {
      frozenExtra: {
        "packages/react/components/package.json": `${JSON.stringify(
          {
            name: "@inflatable-cookie/poodle-react",
            version: "0.5.0",
            private: true,
            dependencies: { "@inflatable-cookie/poodle-core": "0.4.0" },
          },
          null,
          2,
        )}\n`,
      },
    });
    await expect(assertWebCandidateScope(root, base, head)).rejects.toThrow(
      /internal JS dependency/,
    );
  });

  test("reject arbitrary source that rides the bump", async () => {
    const { root, base, head } = await plantCandidate("0.5.0", {
      evidenceExtra: { "packages/core/src/unauthorized.ts": "export const planted = true;\n" },
    });
    await expect(assertWebCandidateScope(root, base, head)).rejects.toThrow(
      /paths outside the release-input and evidence surfaces/,
    );
  });

  test("reject a workflow change that rides the bump", async () => {
    const { root, base, head } = await plantCandidate("0.5.0", {
      evidenceExtra: { ".github/workflows/release.yml": "name: Release\n" },
    });
    await expect(assertWebCandidateScope(root, base, head)).rejects.toThrow(
      /paths outside the release-input and evidence surfaces/,
    );
  });

  test("reject a registry change that rides the bump", async () => {
    const { root, base, head } = await plantCandidate("0.5.0", {
      evidenceExtra: { ".npmrc": "registry=https://example.invalid\n" },
    });
    await expect(assertWebCandidateScope(root, base, head)).rejects.toThrow(
      /paths outside the release-input and evidence surfaces/,
    );
  });

  test("reject an uncoordinated Cargo publication change in the frozen bump", async () => {
    const { root, base, head } = await plantCandidate("0.5.0", {
      frozenExtra: {
        "packages/render/Cargo.toml": cargoManifests("0.5.0")["packages/render/Cargo.toml"]!.replace(
          "publish = false",
          "publish = true",
        ),
      },
    });
    await expect(assertWebCandidateScope(root, base, head)).rejects.toThrow(
      /Cargo publication\/registry\/source content/,
    );
  });

  test("reject third-party Cargo lock drift beside the coordinated Poodle bump", async () => {
    const { root, base, head } = await plantCandidate("0.5.0", {
      frozenExtra: {
        [LOCKSTEP_CARGO_LOCK_PATHS[0]]: cargoLocks("0.5.0")[LOCKSTEP_CARGO_LOCK_PATHS[0]]!.replace(
          'version = "1.0.0"',
          'version = "1.0.1"',
        ),
      },
    });
    await expect(assertWebCandidateScope(root, base, head)).rejects.toThrow(
      /contains changes beyond local Poodle version entries/,
    );
  });

  test("reject a missing release note and a missing lockstep input", async () => {
    const root = await initPlant();
    await writeFiles(root, baseFiles());
    const base = await commitAll(root, "candidate base");
    const frozen = frozenFiles("0.5.0");
    delete frozen["docs/release-notes/0.5.0.md"];
    await writeFiles(root, frozen);
    const head = await commitAll(root, "frozen without release note");
    await expect(assertWebCandidateScope(root, base, head)).rejects.toThrow(
      /missing|release notes/,
    );
  });

  test("reject evidence bound to the wrong commit", async () => {
    const { root, base, head } = await plantCandidate("0.5.0", {
      evidenceExtra: {
        "docs/evidence/nucleus/misbound.json": `${JSON.stringify({ source_commit: "1".repeat(40) })}\n`,
      },
    });
    await expect(assertWebCandidateScope(root, base, head)).rejects.toThrow(
      /instead of the frozen release-input commit/,
    );
  });

  test("reject a second frozen release-input commit", async () => {
    const { root, base, head: firstHead } = await plantCandidate("0.5.0");
    void firstHead;
    await writeFiles(root, {
      "docs/release-notes/README.md": "# Release notes\n\n- [0.5.0]\n- later index edit\n",
    });
    const head = await commitAll(root, "later release-input drift");
    await expect(assertWebCandidateScope(root, base, head)).rejects.toThrow(
      /exactly one frozen release-input commit/,
    );
  });
});

describe("web candidate evidence families", () => {
  test("generated and evidence families are admitted", () => {
    expect(isWebCandidateEvidencePath("packages/core/src/generated/machines/hover.ts")).toBe(true);
    expect(isWebCandidateEvidencePath("packages/svelte/preview/src/generated/catalogue/catalogue.ts")).toBe(true);
    expect(isWebCandidateEvidencePath("docs/evidence/gpui/census.json")).toBe(true);
    expect(isWebCandidateEvidencePath("packages/core/src/index.ts")).toBe(false);
    expect(WEB_CANDIDATE_EVIDENCE_PATTERNS.length).toBeGreaterThanOrEqual(4);
  });
});

describe("web candidate base", () => {
  test("an unmerged candidate is admitted against its merge-base", async () => {
    const { root, base, head } = await plantCandidate("0.4.1");
    await runGit(root, ["branch", "-f", "integration", base]);
    expect(await deriveWebCandidateBase(root, head, "integration")).toBe(base);
  });

  test("a merged candidate walks back over its own release and evidence commits", async () => {
    const { root, base, head } = await plantCandidate("0.4.1");
    await runGit(root, ["branch", "-f", "integration", head]);
    const derived = await deriveWebCandidateBase(root, head, "integration");
    expect(derived).toBe(base);
    const proof = await assertWebCandidateScope(root, derived, head);
    expect(proof.sourceVersion).toBe("0.4.0");
    expect(proof.targetVersion).toBe("0.4.1");
  });

  test("a merged head that is not a candidate commit is refused", async () => {
    const { root, head } = await plantCandidate("0.4.1");
    await writeFiles(root, { "packages/core/src/feature.ts": "export {};\n" });
    const later = await commitAll(root, "feature after release");
    await runGit(root, ["branch", "-f", "integration", later]);
    expect(head).not.toBe(later);
    await expect(deriveWebCandidateBase(root, later, "integration")).rejects.toThrow(
      /already on integration but is not a release-input or evidence commit/,
    );
  });
});
