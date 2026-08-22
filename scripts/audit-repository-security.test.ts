import { expect, test } from "bun:test";
import {
  approvedGitRevisions,
  validateCargoLockSources,
  validateCargoManifestSources,
} from "./repository-security-policy.ts";
import { missingNoticeMarkers } from "./license-compliance-policy.ts";

const zedUrl = "https://github.com/inflatable-cookie/zed";
const zedRevision = approvedGitRevisions.get(zedUrl)!;

test("unknown manifest Git repositories fail closed", () => {
  const errors = validateCargoManifestSources(
    "fixture/Cargo.toml",
    `gpui = { git = "https://example.com/unreviewed", rev = "${zedRevision}" }`,
  );

  expect(errors.join("\n")).toContain("is not approved");
});

test("mutable manifest Git references fail closed", () => {
  const errors = validateCargoManifestSources(
    "fixture/Cargo.toml",
    `gpui = { git = "${zedUrl}", branch = "main" }`,
  );

  expect(errors.join("\n")).toContain("must use an immutable full rev");
});

test("changed manifest revisions fail closed", () => {
  const errors = validateCargoManifestSources(
    "fixture/Cargo.toml",
    `gpui = { git = "${zedUrl}", rev = "0000000000000000000000000000000000000000" }`,
  );

  expect(errors.join("\n")).toContain("expected reviewed revision");
});

test("lockfile URL and revision pairs fail closed", () => {
  const errors = validateCargoLockSources(
    "fixture/Cargo.lock",
    'source = "git+https://github.com/zed-industries/zed?rev=0000000000000000000000000000000000000000#0000000000000000000000000000000000000000"',
  );

  expect(errors.join("\n")).toContain("is not approved");
});

test("missing notice markers remain a failure", () => {
  expect(missingNoticeMarkers("present", ["present", "missing"])).toEqual([
    "missing",
  ]);
});
