import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";

import { pinnedBunVersion } from "./fresh";

describe("ci:fresh pinned Bun", () => {
  test("reads an exact bun@x.y.z packageManager pin", () => {
    expect(pinnedBunVersion(JSON.stringify({ packageManager: "bun@1.4.2" }))).toBe("1.4.2");
  });

  test("refuses a missing, ranged or non-Bun pin", () => {
    for (const manager of [undefined, "bun@^1.4.2", "bun@1.4", "npm@10.0.0"]) {
      expect(() => pinnedBunVersion(JSON.stringify({ packageManager: manager }))).toThrow(/exact bun@x\.y\.z/);
    }
  });

  test("the repository pins an exact Bun", () => {
    const root = join(import.meta.dir, "..", "..");
    expect(pinnedBunVersion(readFileSync(join(root, "package.json"), "utf8"))).toMatch(/^\d+\.\d+\.\d+$/);
  });
});
