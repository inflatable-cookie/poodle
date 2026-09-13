import { execFileSync } from "node:child_process";
import { mkdirSync, mkdtempSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { afterAll, expect, test } from "bun:test";
import {
  approvedGitRevisions,
  registryOnlyCrates,
  secretPatternHits,
  validateCargoLockSources,
  validateCargoManifestSources,
} from "./repository-security-policy.ts";
import { missingNoticeMarkers } from "./license-compliance-policy.ts";

const reviewedUrl = "https://github.com/inflatable-cookie/example";
const reviewedRevision = "87d9afbe71ef06ea0634499dc35d104bb29dc020";
// The production allowlist is empty (g16.005). These fixtures keep the
// revision and mutable-reference rules under test without re-admitting a
// repository nothing in Poodle depends on.
const reviewed = new Map([[reviewedUrl, reviewedRevision]]);

test("the production Git allowlist is empty, so every Git source fails closed", () => {
  expect([...approvedGitRevisions.keys()]).toEqual([]);

  const errors = validateCargoManifestSources(
    "fixture/Cargo.toml",
    `serde = { git = "${reviewedUrl}", rev = "${reviewedRevision}" }`,
  );

  expect(errors.join("\n")).toContain("is not approved");
});

test("unknown manifest Git repositories fail closed", () => {
  const errors = validateCargoManifestSources(
    "fixture/Cargo.toml",
    `serde = { git = "https://example.com/unreviewed", rev = "${reviewedRevision}" }`,
    reviewed,
  );

  expect(errors.join("\n")).toContain("is not approved");
});

test("mutable manifest Git references fail closed", () => {
  const errors = validateCargoManifestSources(
    "fixture/Cargo.toml",
    `serde = { git = "${reviewedUrl}", branch = "main" }`,
    reviewed,
  );

  expect(errors.join("\n")).toContain("must use an immutable full rev");
});

test("changed manifest revisions fail closed", () => {
  const errors = validateCargoManifestSources(
    "fixture/Cargo.toml",
    `serde = { git = "${reviewedUrl}", rev = "0000000000000000000000000000000000000000" }`,
    reviewed,
  );

  expect(errors.join("\n")).toContain("expected reviewed revision");
});

test("lockfile URL and revision pairs fail closed", () => {
  const errors = validateCargoLockSources(
    "fixture/Cargo.lock",
    'source = "git+https://github.com/zed-industries/zed?rev=0000000000000000000000000000000000000000#0000000000000000000000000000000000000000"',
    reviewed,
  );

  expect(errors.join("\n")).toContain("is not approved");
});

// ── g16.005: GPUI is registry-only in the public graph ────────────────
//
// Published v0.2.1 resolved `gpui` from a Git fork, so a consumer on
// crates.io gpui received a different crate identity and could not pass GPUI
// types through Poodle. These are the checks that make that regression
// impossible to land again quietly.

test("gpui and gpui_platform are named as registry-only", () => {
  expect(registryOnlyCrates).toEqual(["gpui", "gpui_platform"]);
});

test("a Git-sourced gpui manifest dependency is rejected by name", () => {
  const errors = validateCargoManifestSources(
    "packages/gpui/node-backend/Cargo.toml",
    `gpui = { git = "${reviewedUrl}", rev = "${reviewedRevision}" }`,
    // Even from an APPROVED repository at its reviewed revision.
    reviewed,
  );

  expect(errors.join("\n")).toContain("gpui must resolve from crates.io");
});

test("a Git-sourced gpui_platform manifest dependency is rejected by name", () => {
  const errors = validateCargoManifestSources(
    "packages/gpui/preview/Cargo.toml",
    `gpui_platform = { git = "${reviewedUrl}", rev = "${reviewedRevision}", features = ["font-kit"] }`,
    reviewed,
  );

  expect(errors.join("\n")).toContain("gpui_platform must resolve from crates.io");
});

test("a Git-sourced gpui lockfile entry is rejected by name", () => {
  const errors = validateCargoLockSources(
    "packages/gpui/preview/Cargo.lock",
    [
      "[[package]]",
      'name = "gpui"',
      'version = "0.2.2"',
      `source = "git+${reviewedUrl}?rev=${reviewedRevision}#${reviewedRevision}"`,
    ].join("\n"),
    reviewed,
  );

  expect(errors.join("\n")).toContain(
    "gpui resolves from a Git source",
  );
});

test("a registry gpui entry passes, and an unrelated Git crate is not blamed on gpui", () => {
  const clean = validateCargoLockSources(
    "packages/gpui/preview/Cargo.lock",
    [
      "[[package]]",
      'name = "gpui"',
      'version = "0.2.2"',
      'source = "registry+https://github.com/rust-lang/crates.io-index"',
    ].join("\n"),
  );
  expect(clean).toEqual([]);

  const unrelated = validateCargoLockSources(
    "fixture/Cargo.lock",
    [
      "[[package]]",
      'name = "gpui"',
      'version = "0.2.2"',
      'source = "registry+https://github.com/rust-lang/crates.io-index"',
      "",
      "[[package]]",
      'name = "some-other-crate"',
      'version = "1.0.0"',
      `source = "git+${reviewedUrl}?rev=${reviewedRevision}#${reviewedRevision}"`,
    ].join("\n"),
    reviewed,
  );
  expect(unrelated.join("\n")).not.toContain("gpui resolves from a Git source");
});

test("historical documentation is not a Cargo manifest and is never scanned", () => {
  // The policy runs on Cargo.toml / Cargo.lock only. A log or research note
  // that quotes the old fork pin must stay readable.
  const errors = validateCargoManifestSources(
    "fixture/Cargo.toml",
    `# gpui = { git = "${reviewedUrl}", rev = "${reviewedRevision}" }`,
    reviewed,
  );

  expect(errors).toEqual([]);
});

test("missing notice markers remain a failure", () => {
  expect(missingNoticeMarkers("present", ["present", "missing"])).toEqual([
    "missing",
  ]);
});

const tokenBody = "A".repeat(20);

function glued(...parts: string[]): string {
  return parts.join("");
}

function openAiUserToken(): string {
  return glued("sk-", tokenBody);
}

function openAiProjectToken(): string {
  return glued("sk-", "proj-", tokenBody);
}

test("OpenAI user tokens remain detected at whitespace, quote, =, and : boundaries", () => {
  const token = openAiUserToken();
  for (const prefix of [" ", "\n", "\t", '"', "'", "=", ":"]) {
    expect(secretPatternHits(`${prefix}${token}`)).toContain("OpenAI token");
  }
  expect(secretPatternHits(token)).toContain("OpenAI token");
});

test("quoted OpenAI project tokens remain detected", () => {
  const token = openAiProjectToken();
  expect(secretPatternHits(`"${token}"`)).toContain("OpenAI token");
  expect(secretPatternHits(`'${token}'`)).toContain("OpenAI token");
  expect(secretPatternHits(`=${token}`)).toContain("OpenAI token");
  expect(secretPatternHits(`:${token}`)).toContain("OpenAI token");
  expect(secretPatternHits(` ${token}`)).toContain("OpenAI token");
});

test("embedded English compounds are not OpenAI tokens", () => {
  expect(secretPatternHits("mask-plus-translated-highlight")).not.toContain(
    "OpenAI token",
  );
  expect(
    secretPatternHits("023-task-backed-agent-workflow-contract.md"),
  ).not.toContain("OpenAI token");
});

test("OpenAI near misses and letter-prefixed embeddings are not tokens", () => {
  expect(secretPatternHits(glued("sk-", "A".repeat(19)))).not.toContain(
    "OpenAI token",
  );
  expect(secretPatternHits(`a${openAiUserToken()}`)).not.toContain(
    "OpenAI token",
  );
  expect(secretPatternHits(`a${openAiProjectToken()}`)).not.toContain(
    "OpenAI token",
  );
});

test("every other secret class still matches its production shape", () => {
  expect(
    secretPatternHits(glued("-----BEGIN ", "PRIVATE KEY-----")),
  ).toContain("private key");
  expect(secretPatternHits(glued("AKIA", "B".repeat(16)))).toContain(
    "AWS access key",
  );
  expect(secretPatternHits(glued("ghp_", "c".repeat(30)))).toContain(
    "GitHub token",
  );
  expect(secretPatternHits(glued("xoxb-", "d".repeat(10)))).toContain(
    "Slack token",
  );
  expect(secretPatternHits(glued("rk_live_", "e".repeat(16)))).toContain(
    "Stripe live key",
  );
  expect(
    secretPatternHits(
      glued("eyJ", "f".repeat(10), ".", "g".repeat(10), ".", "h".repeat(10)),
    ),
  ).toContain("JWT");
  expect(
    secretPatternHits(glued("https://", "alice", ":", "secret", "@", "host")),
  ).toContain("credential URL");
});

// ── g18.033: tracked symlinks are audited as links, never followed ────
//
// The repository-security walker enumerates `git ls-files` and reads every
// entry. `readFileSync` follows a symlink, so the intentional tracked link
// `.claude/skills/impeccable` -> `.agents/skills/impeccable` aborted the walk
// with `EISDIR`. These fixtures plant the same shape in a throwaway repo: the
// link's own text must be audited, and a directory target must not be read.

const securityScriptDir = path.dirname(fileURLToPath(import.meta.url));
const securityRepoRoot = path.resolve(securityScriptDir, "..");
const securityAuditScript = path.join(
  securityRepoRoot,
  "scripts",
  "audit-repository-security.ts",
);

const securityFixtureRoots: string[] = [];

function securityFixtureRepo(): string {
  const root = mkdtempSync(path.join(tmpdir(), "poodle-security-link-"));
  securityFixtureRoots.push(root);
  mkdirSync(path.join(root, ".agents", "skills", "impeccable"), {
    recursive: true,
  });
  writeFileSync(
    path.join(root, ".agents", "skills", "impeccable", "SKILL.md"),
    "# fixture skill\n",
  );
  mkdirSync(path.join(root, ".claude", "skills"), { recursive: true });
  symlinkSync(
    "../../.agents/skills/impeccable",
    path.join(root, ".claude", "skills", "impeccable"),
  );
  execFileSync("git", ["init", "-q"], { cwd: root });
  trackSecurityFixture(root);
  return root;
}

function trackSecurityFixture(root: string): void {
  execFileSync("git", ["add", "-A"], { cwd: root });
}

function runSecurityAudit(root: string): { status: number; output: string } {
  try {
    const output = execFileSync("bun", [securityAuditScript], {
      cwd: root,
      encoding: "utf8",
      stdio: ["ignore", "pipe", "pipe"],
    });
    return { status: 0, output };
  } catch (error) {
    const execError = error as {
      status?: number;
      stdout?: string | Buffer;
      stderr?: string | Buffer;
    };
    return {
      status: execError.status ?? 1,
      output: `${execError.stdout?.toString() ?? ""}${execError.stderr?.toString() ?? ""}`,
    };
  }
}

afterAll(() => {
  for (const root of securityFixtureRoots) {
    rmSync(root, { recursive: true, force: true });
  }
});

test("a tracked symlink to a tracked directory is audited without following it", () => {
  const root = securityFixtureRepo();
  const { status, output } = runSecurityAudit(root);
  expect(status).toBe(0);
  expect(output).toContain("Security hygiene clean");
});

test("a tracked symlink's own link text is audited, not skipped", () => {
  const root = securityFixtureRepo();
  symlinkSync(
    glued("ghp_", "c".repeat(30)),
    path.join(root, ".claude", "skills", "release-notes"),
  );
  trackSecurityFixture(root);
  const { status, output } = runSecurityAudit(root);
  expect(status).toBe(1);
  expect(output).toContain("symlink target contains a GitHub token");
});
