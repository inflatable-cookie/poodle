import { expect, test } from "bun:test";
import {
  approvedGitRevisions,
  registryOnlyCrates,
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
