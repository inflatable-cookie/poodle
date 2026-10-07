import { afterAll, describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";

import {
  LOCKSTEP_CARGO_LOCK_PATHS,
  LOCKSTEP_CARGO_MANIFEST_PATHS,
  LOCKSTEP_JS_MANIFEST_PATHS,
} from "./release-versioning";
import { bumpLockstepVersion } from "./release-version";
import { packagesSectionUnchanged, workspaceLockDrift } from "./workspace-lock";

const roots: string[] = [];
const SOURCE_VERSION = "0.4.11";
const TARGET_VERSION = "0.4.12";

afterAll(() => {
  for (const root of roots) rmSync(root, { recursive: true, force: true });
});

function crateName(path: string): string {
  const directory = path.replace(/\/Cargo\.toml$/, "").split("/").at(-1)!;
  const names: Record<string, string> = {
    codegen: "poodle-codegen",
    adapter: path.includes("contracts")
      ? "poodle-adapter"
      : path.includes("jetstream")
        ? "poodle-jetstream"
        : "poodle-gpui",
    components: "poodle-specs",
    events: "poodle-events",
    headless: "poodle-headless",
    ir: "poodle-ir",
    layout: "poodle-layout",
    markdown: "poodle-markdown",
    node: "poodle-node",
    style: "poodle-style",
    tokens: "poodle-tokens",
    "node-backend": "poodle-gpui-node-backend",
    preview: path.includes("gpui") ? "poodle-gpui-preview" : "poodle-jetstream-preview",
    jetstream: "poodle-jetstream",
    render: "poodle-render",
  };
  return names[directory]!;
}

function manifest(path: string): string {
  const name = crateName(path);
  return [
    "[package]",
    'name = "' + name + '"',
    'version = "' + SOURCE_VERSION + '"',
    "publish = false",
    "",
    ...(path === "packages/render/Cargo.toml"
      ? [
          "[dependencies]",
          'poodle-node = { version = "' + SOURCE_VERSION + '", path = "../contracts/node" }',
          "",
        ]
      : []),
  ].join("\n");
}

function cargoLock(crateNames: string[]): string {
  return [
    "# generated fixture Cargo lock",
    "version = 4",
    "",
    ...crateNames.flatMap((name, index) => [
      "[[package]]",
      'name = "' + name + '"',
      'version = "' + SOURCE_VERSION + '"',
      ...(index === 0 && crateNames.length > 1
        ? ["dependencies = [", ' "' + crateNames[1] + " " + SOURCE_VERSION + '"', "]"]
        : []),
      "",
    ]),
    "[[package]]",
    'name = "serde"',
    'version = "1.0.0"',
    'source = "registry+https://github.com/rust-lang/crates.io-index"',
    'checksum = "external-resolution-stays-fixed"',
    "",
  ].join("\n");
}

function fixture(): string {
  const root = mkdtempSync(join(tmpdir(), "poodle-release-version-test-"));
  roots.push(root);
  const workspaces = [
    "packages/core",
    "packages/tokens",
    "packages/svelte/components",
    "packages/svelte/preview",
    "packages/react/components",
    "packages/react/preview",
  ];
  const jsNames: Record<string, string> = {
    "packages/core": "@inflatable-cookie/poodle-core",
    "packages/svelte/components": "@inflatable-cookie/poodle-svelte",
    "packages/react/components": "@inflatable-cookie/poodle-react",
  };
  for (const path of LOCKSTEP_JS_MANIFEST_PATHS) {
    const fullPath = join(root, path);
    mkdirSync(dirname(fullPath), { recursive: true });
    const isRoot = path === "package.json";
    const isCore = path === "packages/core/package.json";
    let manifest: Record<string, unknown>;
    if (isRoot) {
      manifest = { name: "poodle", version: SOURCE_VERSION, private: true, workspaces };
    } else {
      manifest = {
        name: jsNames[path.replace(/\/package\.json$/, "")]!,
        version: SOURCE_VERSION,
        ...(path.includes("react") ? { private: true } : {}),
      };
      if (!isCore) {
        manifest.dependencies = {
          "@inflatable-cookie/poodle-core": ">=" + SOURCE_VERSION + " <0.5",
        };
      }
    }
    writeFileSync(fullPath, JSON.stringify(manifest, null, 2) + "\n");
  }
  const unchangedWorkspaces = {
    "packages/tokens": "@inflatable-cookie/poodle-tokens",
    "packages/svelte/preview": "@inflatable-cookie/poodle-svelte-preview",
    "packages/react/preview": "@inflatable-cookie/poodle-react-preview",
  };
  for (const [path, name] of Object.entries(unchangedWorkspaces)) {
    const fullPath = join(root, path, "package.json");
    mkdirSync(dirname(fullPath), { recursive: true });
    writeFileSync(
      fullPath,
      JSON.stringify({ name, version: "0.0.0", private: true }, null, 2) + "\n",
    );
  }
  for (const path of LOCKSTEP_CARGO_MANIFEST_PATHS) {
    const fullPath = join(root, path);
    mkdirSync(dirname(fullPath), { recursive: true });
    writeFileSync(fullPath, manifest(path));
  }
  writeFileSync(
    join(root, "bun.lock"),
    JSON.stringify(
      {
        lockfileVersion: 1,
        workspaces: {
          "": { name: "poodle" },
          "packages/core": { name: jsNames["packages/core"], version: SOURCE_VERSION },
          "packages/tokens": { name: "@inflatable-cookie/poodle-tokens", version: "0.0.0" },
          "packages/svelte/components": {
            name: jsNames["packages/svelte/components"],
            version: SOURCE_VERSION,
            dependencies: { "@inflatable-cookie/poodle-core": ">=" + SOURCE_VERSION + " <0.5" },
          },
          "packages/react/components": {
            name: jsNames["packages/react/components"],
            version: SOURCE_VERSION,
            dependencies: { "@inflatable-cookie/poodle-core": ">=" + SOURCE_VERSION + " <0.5" },
          },
          "packages/svelte/preview": { name: "@inflatable-cookie/poodle-svelte-preview", version: "0.0.0" },
          "packages/react/preview": { name: "@inflatable-cookie/poodle-react-preview", version: "0.0.0" },
        },
        packages: { serde: ["serde@1.0.0", "", {}, "unchanged"] },
      },
      null,
      2,
    ) + "\n",
  );
  writeFileSync(
    join(root, LOCKSTEP_CARGO_LOCK_PATHS[0]),
    cargoLock(["poodle-gpui-node-backend", "poodle-node"]),
  );
  writeFileSync(
    join(root, LOCKSTEP_CARGO_LOCK_PATHS[1]),
    cargoLock(["poodle-gpui-preview", "poodle-gpui-node-backend"]),
  );
  return root;
}

describe("lockstep release version bump", () => {
  test("release:bump restamps codegen outputs and census updates accept evidence arguments", () => {
    const tasks = readFileSync(new URL("../tasks/effigy.tasks.toml", import.meta.url), "utf8");
    const bump = /"release:bump"\s*=\s*\[([\s\S]*?)\n\]/.exec(tasks)?.[1] ?? "";
    expect(bump).toContain("bun scripts/release-version.ts {args}");
    expect(bump).toContain('{ task = "ir:build" }');
    expect(bump).toContain('{ task = "catalogue:build" }');
    expect(tasks).toContain('"update:gpui-census" = "bun scripts/gpui-functionality-census.ts {args}"');
  });

  test("updates each JS and Rust version source and only the lockfile version slices", () => {
    const root = fixture();
    const lockBefore = readFileSync(join(root, "bun.lock"), "utf8");
    const rustLockBefore = readFileSync(join(root, LOCKSTEP_CARGO_LOCK_PATHS[0]), "utf8");
    const changed = bumpLockstepVersion(root, TARGET_VERSION);

    expect(changed).toContain("package.json");
    expect(changed).toContain("bun.lock");
    for (const path of LOCKSTEP_JS_MANIFEST_PATHS) {
      expect(readFileSync(join(root, path), "utf8")).toContain(TARGET_VERSION);
    }
    for (const path of LOCKSTEP_CARGO_MANIFEST_PATHS) {
      const contents = readFileSync(join(root, path), "utf8");
      expect(contents).toContain('version = "' + TARGET_VERSION + '"');
      if (path === "packages/render/Cargo.toml") {
        expect(contents).toContain('poodle-node = { version = "' + TARGET_VERSION + '"');
      }
    }
    for (const path of LOCKSTEP_CARGO_LOCK_PATHS) {
      expect(readFileSync(join(root, path), "utf8")).toContain(
        'version = "' + TARGET_VERSION + '"',
      );
    }
    const lockAfter = readFileSync(join(root, "bun.lock"), "utf8");
    expect(packagesSectionUnchanged(lockBefore, lockAfter)).toBe(true);
    expect(workspaceLockDrift(root)).toEqual([]);
    expect(readFileSync(join(root, LOCKSTEP_CARGO_LOCK_PATHS[0]), "utf8")).toContain(
      '"poodle-node ' + TARGET_VERSION + '"',
    );
    expect(readFileSync(join(root, LOCKSTEP_CARGO_LOCK_PATHS[0]), "utf8")).toContain(
      "external-resolution-stays-fixed",
    );
    expect(rustLockBefore).toContain('version = "' + SOURCE_VERSION + '"');
  });

  test("refuses equal and lower versions without writing any files", () => {
    const root = fixture();
    const before = readFileSync(join(root, LOCKSTEP_CARGO_MANIFEST_PATHS[0]), "utf8");
    expect(() => bumpLockstepVersion(root, SOURCE_VERSION)).toThrow(/non-increasing/);
    expect(() => bumpLockstepVersion(root, "0.4.10")).toThrow(/non-increasing/);
    expect(readFileSync(join(root, LOCKSTEP_CARGO_MANIFEST_PATHS[0]), "utf8")).toBe(before);
  });

  test("refuses a manifest that has drifted from the shared release version", () => {
    const root = fixture();
    const path = join(root, LOCKSTEP_CARGO_MANIFEST_PATHS[0]);
    writeFileSync(path, readFileSync(path, "utf8").replace(SOURCE_VERSION, "0.4.10"));
    expect(() => bumpLockstepVersion(root, TARGET_VERSION)).toThrow(/all Rust and JavaScript release manifests/);
  });
});
