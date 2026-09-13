// g18.032 / spec 071: focused laws for version-independent web-candidate
// admission. Synthetic ranges prove a future target version is admitted and
// that partial, stale, source, workflow, registry and native changes fail
// closed before any build.

import { afterAll, describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

import { requireExactCommit, withDisposableGitPlant } from "./scope";
import {
  assertWebCandidateScope,
  deriveWebCandidateVersions,
  isWebCandidateEvidencePath,
  WEB_CANDIDATE_EVIDENCE_PATTERNS,
} from "./web-candidate";

const plantRoots: string[] = [];

afterAll(() => {
  for (const root of plantRoots) rmSync(root, { recursive: true, force: true });
});

async function runGit(root: string, args: string[]): Promise<string> {
  const child = Bun.spawn(["git", "-C", root, ...args], { stdout: "pipe", stderr: "pipe" });
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

async function initPlant(): Promise<string> {
  const root = mkdtempSync(join(tmpdir(), "poodle-web-candidate-test-"));
  plantRoots.push(root);
  await runGit(root, ["init", "--quiet"]);
  await runGit(root, ["config", "user.email", "poodle-certification@example.invalid"]);
  await runGit(root, ["config", "user.name", "Poodle Certification"]);
  return root;
}

function jsManifests(version: string): Record<string, string> {
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
        dependencies: { "@inflatable-cookie/poodle-core": version },
      },
      null,
      2,
    )}\n`,
    "packages/react/components/package.json": `${JSON.stringify(
      {
        name: "@inflatable-cookie/poodle-react",
        version,
        private: true,
        dependencies: { "@inflatable-cookie/poodle-core": version },
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
  return `${JSON.stringify(
    {
      lockfileVersion: 1,
      workspaces: {
        "": { name: "poodle" },
        "packages/core": { name: "@inflatable-cookie/poodle-core", version },
        "packages/svelte/components": {
          name: "@inflatable-cookie/poodle-svelte",
          version,
          dependencies: { "@inflatable-cookie/poodle-core": version },
        },
        "packages/react/components": {
          name: "@inflatable-cookie/poodle-react",
          version,
          dependencies: { "@inflatable-cookie/poodle-core": version },
        },
      },
    },
    null,
    2,
  )}\n`;
}

function baseFiles(): Record<string, string> {
  return {
    ...jsManifests("0.4.0"),
    "bun.lock": bunLock("0.4.0"),
    "CHANGELOG.md": changelog(null),
    "docs/release-notes/README.md": "# Release notes\n",
    "packages/render/Cargo.toml": [
      "[package]",
      'name = "poodle-render"',
      'version = "0.4.0"',
      "publish = false",
      "",
    ].join("\n"),
    "packages/render/Cargo.lock": [
      "version = 4",
      "",
      "[[package]]",
      'name = "poodle-render"',
      'version = "0.4.0"',
      "",
    ].join("\n"),
  };
}

function frozenFiles(target: string): Record<string, string> {
  return {
    ...jsManifests(target),
    "bun.lock": bunLock(target),
    "CHANGELOG.md": changelog(target),
    "docs/release-notes/README.md": `# Release notes\n\n- [${target}]\n`,
    [`docs/release-notes/${target}.md`]: `# Poodle ${target}\n`,
  };
}

async function plantCandidate(
  target: string,
  options: { frozenExtra?: Record<string, string>; evidenceExtra?: Record<string, string> } = {},
): Promise<{ root: string; base: string; frozen: string; head: string }> {
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
  return { root, base, frozen, head };
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
  test("admit a synthetic 0.4.1 candidate", async () => {
    const { root, base, head } = await plantCandidate("0.4.1");
    const proof = await assertWebCandidateScope(root, base, head);
    expect(proof.sourceVersion).toBe("0.4.0");
    expect(proof.targetVersion).toBe("0.4.1");
    expect(proof.releaseNotePath).toBe("docs/release-notes/0.4.1.md");
    expect(proof.changedPaths).not.toContain("packages/render/Cargo.toml");
    expect(proof.changedPaths).not.toContain("packages/render/Cargo.lock");
  });

  test("admit a synthetic 0.5.0 candidate", async () => {
    const { root, base, head } = await plantCandidate("0.5.0");
    const proof = await assertWebCandidateScope(root, base, head);
    expect(proof.targetVersion).toBe("0.5.0");
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

  test("reject a native Cargo change that rides the bump", async () => {
    const { root, base, head } = await plantCandidate("0.5.0", {
      evidenceExtra: {
        "packages/render/Cargo.toml": [
          "[package]",
          'name = "poodle-render"',
          'version = "0.5.0"',
          "publish = false",
          "",
        ].join("\n"),
      },
    });
    await expect(assertWebCandidateScope(root, base, head)).rejects.toThrow(
      /native|release-input|Cargo/,
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
  test("generated, evidence and execution-record families are admitted", () => {
    expect(isWebCandidateEvidencePath("packages/core/src/generated/machines/hover.ts")).toBe(true);
    expect(isWebCandidateEvidencePath("packages/svelte/preview/src/generated/catalogue/catalogue.ts")).toBe(true);
    expect(isWebCandidateEvidencePath("docs/evidence/gpui/census.json")).toBe(true);
    expect(isWebCandidateEvidencePath("docs/logs/2026-09/20260913-g19-001-release.md")).toBe(true);
    expect(isWebCandidateEvidencePath("packages/core/src/index.ts")).toBe(false);
    expect(WEB_CANDIDATE_EVIDENCE_PATTERNS.length).toBeGreaterThanOrEqual(5);
  });
});
