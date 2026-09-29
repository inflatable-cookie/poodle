import { afterAll, describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

import {
  LOCK_PATH,
  packagesSectionUnchanged,
  refreshWorkspaceLock,
  refreshWorkspaceLockText,
  workspaceLockDrift,
} from "./workspace-lock.ts";

const repoRoot = join(dirname(fileURLToPath(import.meta.url)), "..");
const plantRoots: string[] = [];

afterAll(() => {
  for (const root of plantRoots) {
    rmSync(root, { recursive: true, force: true });
  }
});

function plant(lock: string, coreVersion: string, range: string): string {
  const root = mkdtempSync(join(tmpdir(), "poodle-workspace-lock-"));
  plantRoots.push(root);
  writeFileSync(
    join(root, "package.json"),
    JSON.stringify({
      name: "poodle",
      workspaces: ["packages/core", "packages/lib"],
    }) + "\n",
  );
  mkdirSync(join(root, "packages/core"), { recursive: true });
  mkdirSync(join(root, "packages/lib"), { recursive: true });
  writeFileSync(
    join(root, "packages/core/package.json"),
    JSON.stringify({
      name: "@inflatable-cookie/poodle-core",
      version: coreVersion,
    }) + "\n",
  );
  writeFileSync(
    join(root, "packages/lib/package.json"),
    JSON.stringify({
      name: "@inflatable-cookie/poodle-lib",
      version: coreVersion,
      dependencies: { "@inflatable-cookie/poodle-core": range },
    }) + "\n",
  );
  writeFileSync(join(root, LOCK_PATH), lock);
  return root;
}

function lockAt(version: string, range: string): string {
  return `{
  "lockfileVersion": 2,
  "workspaces": {
    "packages/core": {
      "name": "@inflatable-cookie/poodle-core",
      "version": "${version}",
    },
    "packages/lib": {
      "name": "@inflatable-cookie/poodle-lib",
      "version": "${version}",
      "dependencies": {
        "@inflatable-cookie/poodle-core": "${range}",
      },
    },
  },
  "packages": {
    "left-pad": ["left-pad@1.3.0", "", {}, "sha512-unchanged=="],
  },
}
`;
}

describe("workspace lock check and refresh", () => {
  test("the committed lock matches package.json", () => {
    expect(workspaceLockDrift(repoRoot)).toEqual([]);
  });

  test("a stale lock after a version bump fails the check", () => {
    const root = plant(lockAt("0.4.6", ">=0.4.6 <0.5"), "0.4.7", ">=0.4.7 <0.5");
    const findings = workspaceLockDrift(root);
    expect(findings.some((finding) => finding.includes("0.4.6") && finding.includes("0.4.7"))).toBe(
      true,
    );
    expect(findings.some((finding) => finding.includes(">=0.4.6 <0.5"))).toBe(true);
  });

  test("refresh updates only workspaces and the check then passes", () => {
    const before = lockAt("0.4.6", ">=0.4.6 <0.5");
    const root = plant(before, "0.4.7", ">=0.4.7 <0.5");
    expect(workspaceLockDrift(root).length).toBeGreaterThan(0);

    const after = refreshWorkspaceLockText(root, before);
    expect(packagesSectionUnchanged(before, after)).toBe(true);
    expect(after).toContain('"version": "0.4.7"');
    expect(after).toContain('">=0.4.7 <0.5"');
    expect(after).not.toContain('"version": "0.4.6"');
    expect(after).toContain("left-pad@1.3.0");

    const written = refreshWorkspaceLock(root);
    expect(written.changed).toBe(true);
    expect(workspaceLockDrift(root)).toEqual([]);
    expect(readFileSync(join(root, LOCK_PATH), "utf8")).toContain("left-pad@1.3.0");
  });
});
