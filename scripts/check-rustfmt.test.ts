/**
 * The exclusion boundary is the subtle part of the rustfmt gate: a generated
 * file that slipped into the enumerated set would be reformatted in write
 * mode and break the byte-identity its own gate (`ir:check`, `audit:tokens`,
 * `audit:icons`) pins. These tests pin the boundary against the real tree.
 *
 * Run with `bun test scripts/check-rustfmt.test.ts`.
 */
import { describe, expect, test } from "bun:test";
import path from "node:path";
import {
  authoredRustFiles,
  crateManifests,
  editionFor,
  isGenerated,
  rustfmtArgs,
} from "./check-rustfmt.ts";

describe("isGenerated", () => {
  test("excludes generated directories and *.generated.rs", () => {
    expect(isGenerated(path.join("packages/contracts/tokens/src/generated", "primitives.rs"))).toBeTrue();
    expect(isGenerated("packages/contracts/headless/src/generated/machines/modal.rs")).toBeTrue();
    expect(isGenerated("packages/gpui/preview/src/generated/preview-shell.rs")).toBeTrue();
    expect(isGenerated("packages/jetstream/preview/src/generated/specimens/specimens.rs")).toBeTrue();
    expect(isGenerated("packages/contracts/components/src/icon_geometry.generated.rs")).toBeTrue();
  });

  test("keeps authored files, including generated-adjacent names", () => {
    expect(isGenerated("packages/render/src/action_discovery_panel.rs")).toBeFalse();
    expect(isGenerated("packages/contracts/tokens/src/lib.rs")).toBeFalse();
    // A `generated` prefix or suffix in a file name is not a generated directory.
    expect(isGenerated("packages/x/src/generated_tokens.rs")).toBeFalse();
    expect(isGenerated("packages/x/src/generated_tests/mod.rs")).toBeFalse();
  });
});

describe("crateManifests", () => {
  test("finds every Rust crate in the repository", () => {
    const manifests = crateManifests();
    const relative = manifests.map((manifest) => path.relative(path.resolve(import.meta.dir, ".."), manifest));
    for (const expected of [
      "packages/codegen/Cargo.toml",
      "packages/render/Cargo.toml",
      "packages/contracts/headless/Cargo.toml",
      "packages/contracts/tokens/Cargo.toml",
      "packages/gpui/preview/Cargo.toml",
      "packages/jetstream/preview/Cargo.toml",
    ]) {
      expect(relative).toContain(expected);
    }
    expect(manifests.length).toBe(17);
  });
});

describe("authoredRustFiles", () => {
  test("tokens crate keeps only its authored sources", () => {
    const files = authoredRustFiles(path.resolve(import.meta.dir, "..", "packages/contracts/tokens"));
    expect(files).toEqual(["packages/contracts/tokens/src/lib.rs"]);
  });

  test("render crate keeps the path-included authored icon_geometry module", () => {
    const files = authoredRustFiles(path.resolve(import.meta.dir, "..", "packages/render"));
    expect(files).toContain("packages/render/src/icon_geometry.rs");
    expect(files.some((file) => file.includes("generated"))).toBeFalse();
  });
});

describe("rustfmtArgs", () => {
  test("always disables child-module recursion", () => {
    // Dropping skip_children puts generated `#[path]` modules back in
    // rustfmt's reach from every explicitly passed file.
    expect(rustfmtArgs("2021", ["a.rs"], true)).toContain("skip_children=true");
    expect(rustfmtArgs("2024", ["a.rs"], false)).toContain("skip_children=true");
  });

  test("asks for a diff only in check mode", () => {
    expect(rustfmtArgs("2021", ["a.rs", "b.rs"], true)).toContain("--check");
    expect(rustfmtArgs("2021", ["a.rs", "b.rs"], false)).not.toContain("--check");
  });
});

describe("editionFor", () => {
  test("reads each crate's declared edition", () => {
    expect(editionFor(path.resolve(import.meta.dir, "..", "packages/codegen/Cargo.toml"))).toBe("2021");
    expect(editionFor(path.resolve(import.meta.dir, "..", "packages/jetstream/preview/Cargo.toml"))).toBe("2024");
  });
});
