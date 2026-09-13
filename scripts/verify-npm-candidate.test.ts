// g18.032 / spec 071: focused laws for publish-time identity verification.
// A mismatched hash, version, commit, package set or stray tarball must fail
// before any npm mutation.

import { afterAll, describe, expect, test } from "bun:test";
import { cpSync, mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";

import { sha256File, verifyNpmCandidate } from "./verify-npm-candidate";

const root = resolve(import.meta.dir, "..");
const roots: string[] = [];

afterAll(() => {
  for (const dir of roots) rmSync(dir, { recursive: true, force: true });
});

const SOURCE_COMMIT = "a".repeat(40);

function packFake(work: string, name: string, version: string, tarballName: string): string {
  const staging = join(work, `staging-${tarballName}`);
  mkdirSync(join(staging, "package"), { recursive: true });
  writeFileSync(
    join(staging, "package", "package.json"),
    `${JSON.stringify({ name, version })}\n`,
  );
  const tarball = join(work, tarballName);
  const child = Bun.spawnSync(["tar", "-czf", tarball, "-C", staging, "package"]);
  if (child.exitCode !== 0) throw new Error(child.stderr.toString());
  return tarball;
}

function makeCandidate(options: { version?: string; stray?: boolean } = {}): {
  dir: string;
  manifestPath: string;
} {
  const version = options.version ?? "0.5.0";
  const dir = mkdtempSync(join(tmpdir(), "poodle-npm-candidate-"));
  roots.push(dir);
  const packages = [
    { name: "@inflatable-cookie/poodle-core", path: "packages/core", tarball: "core.tgz" },
    { name: "@inflatable-cookie/poodle-svelte", path: "packages/svelte/components", tarball: "svelte.tgz" },
  ];
  const entries = packages.map((entry) => {
    packFake(dir, entry.name, version, entry.tarball);
    return { ...entry, version, sha256: sha256File(join(dir, entry.tarball)) };
  });
  if (options.stray) cpSync(join(dir, entries[0].tarball), join(dir, "stray.tgz"));
  const manifestPath = join(dir, "poodle-npm-candidate.json");
  writeFileSync(
    manifestPath,
    `${JSON.stringify(
      { schema: "poodle.npm-candidate.v1", sourceCommit: SOURCE_COMMIT, version, packages: entries },
      null,
      2,
    )}\n`,
  );
  return { dir, manifestPath };
}

describe("npm candidate identity", () => {
  test("accepts a candidate that binds the tag, commit, version and hashes", () => {
    const { dir, manifestPath } = makeCandidate();
    const manifest = verifyNpmCandidate({
      root,
      manifestPath,
      archiveDir: dir,
      tagVersion: "0.5.0",
      sourceCommit: SOURCE_COMMIT,
    });
    expect(manifest.version).toBe("0.5.0");
    expect(manifest.packages.map((entry) => entry.name)).toEqual([
      "@inflatable-cookie/poodle-core",
      "@inflatable-cookie/poodle-svelte",
    ]);
  });

  test("rejects a floating tag version and a mismatched source commit", () => {
    const { dir, manifestPath } = makeCandidate();
    expect(() =>
      verifyNpmCandidate({ root, manifestPath, archiveDir: dir, tagVersion: "0.5.1" }),
    ).toThrow(/does not match tag/);
    expect(() =>
      verifyNpmCandidate({ root, manifestPath, archiveDir: dir, sourceCommit: "b".repeat(40) }),
    ).toThrow(/does not match the tag commit/);
  });

  test("rejects drifted bytes and a stray tarball", () => {
    const drifted = makeCandidate();
    writeFileSync(join(drifted.dir, "core.tgz"), "not the proved bytes");
    expect(() =>
      verifyNpmCandidate({ root, manifestPath: drifted.manifestPath, archiveDir: drifted.dir }),
    ).toThrow(/does not match manifest/);

    const stray = makeCandidate({ stray: true });
    expect(() =>
      verifyNpmCandidate({ root, manifestPath: stray.manifestPath, archiveDir: stray.dir }),
    ).toThrow(/unexpected tarballs/);
  });

  test("rejects a package set that drifts from the release authority", () => {
    const { dir, manifestPath } = makeCandidate();
    writeFileSync(
      manifestPath,
      `${JSON.stringify(
        {
          schema: "poodle.npm-candidate.v1",
          sourceCommit: SOURCE_COMMIT,
          version: "0.5.0",
          packages: [
            {
              name: "@inflatable-cookie/poodle-react",
              path: "packages/react/components",
              tarball: "core.tgz",
              version: "0.5.0",
              sha256: sha256File(join(dir, "core.tgz")),
            },
          ],
        },
        null,
        2,
      )}\n`,
    );
    expect(() => verifyNpmCandidate({ root, manifestPath, archiveDir: dir })).toThrow(
      /publishes 1 packages|not in the release authority/,
    );
  });
});
