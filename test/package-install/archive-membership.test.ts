import { describe, expect, test } from "bun:test";

import { packedMemberMissing } from "./archive-membership";

describe("packed archive directory membership", () => {
  test("a packed src tree without a bare package/src entry is present", () => {
    expect(
      packedMemberMissing(["package/package.json", "package/src/index.ts"], "src"),
    ).toBe(false);
  });

  test("a packed dist tree without a bare package/dist entry is present", () => {
    expect(
      packedMemberMissing(
        ["package/package.json", "package/dist/index.js", "package/dist/.poodle-build.json"],
        "dist",
      ),
    ).toBe(false);
  });

  test("an extensionless LICENSE file is a file, not a tree", () => {
    expect(packedMemberMissing(["package/LICENSE"], "LICENSE")).toBe(false);
    expect(packedMemberMissing(["package/README.md"], "README.md")).toBe(false);
  });

  test("files listing dist still fails when the dist tree is omitted", () => {
    expect(
      packedMemberMissing(["package/package.json", "package/README.md", "package/LICENSE"], "dist"),
    ).toBe(true);
  });

  test("a sibling dist-extra path does not satisfy files: dist", () => {
    expect(packedMemberMissing(["package/dist-extra/index.js"], "dist")).toBe(true);
  });
});
