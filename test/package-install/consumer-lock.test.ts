import { afterAll, describe, expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

import { declaredPackageNames, lockedTransitiveOverrides } from "./consumer-lock";

const roots: string[] = [];
const servers: Array<{ stop: () => void }> = [];

afterAll(() => {
  for (const server of servers) server.stop();
  for (const dir of roots) rmSync(dir, { recursive: true, force: true });
});

function packNpmTarball(name: string, version: string, extra: Record<string, unknown> = {}): Uint8Array {
  const staging = mkdtempSync(join(tmpdir(), "poodle-consumer-lock-pack-"));
  roots.push(staging);
  mkdirSync(join(staging, "package"), { recursive: true });
  writeFileSync(
    join(staging, "package", "package.json"),
    `${JSON.stringify({ name, version, ...extra })}\n`,
  );
  const tarball = join(staging, `${name}-${version}.tgz`);
  const child = Bun.spawnSync(["tar", "-czf", tarball, "-C", staging, "package"]);
  if (child.exitCode !== 0) throw new Error(child.stderr.toString());
  return new Uint8Array(readFileSync(tarball));
}

function integrity(bytes: Uint8Array): string {
  return `sha512-${createHash("sha512").update(bytes).digest("base64")}`;
}

type PackumentVersion = {
  name: string;
  version: string;
  dependencies?: Record<string, string>;
  dist: { tarball: string; integrity: string };
};

function startRegistry(
  packages: Record<
    string,
    {
      latest: string;
      versions: Record<string, Uint8Array | "missing">;
      dependencies?: Record<string, Record<string, string>>;
    }
  >,
): string {
  const tarballs = new Map<string, Uint8Array>();
  const packuments = new Map<string, unknown>();

  const server = Bun.serve({
    hostname: "127.0.0.1",
    port: 0,
    fetch(request) {
      const url = new URL(request.url);
      const path = decodeURIComponent(url.pathname);
      const tarball = tarballs.get(path);
      if (tarball) {
        return new Response(tarball, { headers: { "content-type": "application/octet-stream" } });
      }
      const name = path.replace(/^\//, "");
      const packument = packuments.get(name);
      if (packument) {
        return new Response(JSON.stringify(packument), {
          headers: { "content-type": "application/json" },
        });
      }
      return new Response("Not found", { status: 404 });
    },
  });
  servers.push(server);
  const origin = `http://127.0.0.1:${server.port}`;

  for (const [name, spec] of Object.entries(packages)) {
    const versions: Record<string, PackumentVersion> = {};
    for (const [version, bytes] of Object.entries(spec.versions)) {
      const tarballPath = `/${name}/-/${name}-${version}.tgz`;
      if (bytes !== "missing") tarballs.set(tarballPath, bytes);
      versions[version] = {
        name,
        version,
        dependencies: spec.dependencies?.[version],
        dist: {
          tarball: `${origin}${tarballPath}`,
          integrity: bytes === "missing" ? integrity(new Uint8Array([1, 2, 3])) : integrity(bytes),
        },
      };
    }
    packuments.set(name, { name, "dist-tags": { latest: spec.latest }, versions });
  }

  return origin;
}

async function bunInstall(cwd: string, registry: string): Promise<{ exitCode: number; output: string }> {
  const child = Bun.spawn(["bun", "install", "--ignore-scripts"], {
    cwd,
    stdout: "pipe",
    stderr: "pipe",
    env: {
      ...process.env,
      npm_config_registry: registry,
      BUN_INSTALL_CACHE_DIR: join(cwd, ".bun-cache"),
    },
  });
  const [stdout, stderr, exitCode] = await Promise.all([
    new Response(child.stdout).text(),
    new Response(child.stderr).text(),
    child.exited,
  ]);
  return { exitCode, output: `${stdout}\n${stderr}` };
}

const REPO_LOCK = `{
  "lockfileVersion": 1,
  "workspaces": {
    "": { "name": "poodle" }
  },
  "packages": {
    "@inflatable-cookie/poodle-core": ["@inflatable-cookie/poodle-core@workspace:packages/core"],
    "react": ["react@19.3.0", "", {}, "sha512-react=="],
    "std-env": ["std-env@4.2.0", "", {}, "sha512-stdenv=="],
    "vitest": ["vitest@5.0.2", "", { "dependencies": { "std-env": "^4.2.0" } }, "sha512-vitest=="],
    "@vitest/mocker": ["@vitest/mocker@5.0.2", "", {}, "sha512-mocker=="],
    "vite": ["vite@8.3.1", "", {}, "sha512-vite=="]
  }
}
`;

describe("packed consumer lock from bun.lock", () => {
  test("keeps locked transitives and drops declared names, workspace packages and mismatched tool families", () => {
    const overrides = lockedTransitiveOverrides({
      lockText: REPO_LOCK,
      declaredNames: declaredPackageNames(
        { "@inflatable-cookie/poodle-core": "file:./core.tgz", react: "18.0.0" },
        { vitest: "4.1.10", vite: "7.3.1" },
      ),
    });
    expect(overrides["std-env"]).toBe("4.2.0");
    expect(overrides.react).toBeUndefined();
    expect(overrides.vitest).toBeUndefined();
    expect(overrides["@vitest/mocker"]).toBeUndefined();
    expect(overrides.vite).toBeUndefined();
    expect(overrides["@inflatable-cookie/poodle-core"]).toBeUndefined();
  });
});

describe("packed consumer install against a planted registry", () => {
  test("a transitive whose newest version 404s still installs the locked version", async () => {
    const leafV1 = packNpmTarball("poodle-plant-leaf", "1.0.0");
    const host = packNpmTarball("poodle-plant-host", "1.0.0", {
      dependencies: { "poodle-plant-leaf": "^1.0.0" },
    });
    const registry = startRegistry({
      "poodle-plant-host": {
        latest: "1.0.0",
        versions: { "1.0.0": host },
        dependencies: { "1.0.0": { "poodle-plant-leaf": "^1.0.0" } },
      },
      "poodle-plant-leaf": {
        latest: "1.1.0",
        versions: { "1.0.0": leafV1, "1.1.0": "missing" },
      },
    });
    const root = mkdtempSync(join(tmpdir(), "poodle-consumer-lock-ok-"));
    roots.push(root);
    const dependencies = { "poodle-plant-host": "1.0.0" };
    const lockText = `{
      "packages": {
        "poodle-plant-leaf": ["poodle-plant-leaf@1.0.0", "", {}, "sha512-leaf=="]
      }
    }`;
    writeFileSync(
      join(root, "package.json"),
      `${JSON.stringify(
        {
          name: "poodle-plant-consumer",
          private: true,
          dependencies,
          overrides: lockedTransitiveOverrides({
            lockText,
            declaredNames: declaredPackageNames(dependencies),
          }),
        },
        null,
        2,
      )}\n`,
    );
    writeFileSync(join(root, ".npmrc"), `registry=${registry}\n`);
    const result = await bunInstall(root, registry);
    if (result.exitCode !== 0) throw new Error(result.output);
    const installed = JSON.parse(
      readFileSync(join(root, "node_modules", "poodle-plant-leaf", "package.json"), "utf8"),
    ) as { version: string };
    expect(installed.version).toBe("1.0.0");
  });

  test("a dependency with no resolvable version still fails", async () => {
    const host = packNpmTarball("poodle-plant-ghost-host", "1.0.0", {
      dependencies: { "poodle-plant-ghost": "1.0.0" },
    });
    const registry = startRegistry({
      "poodle-plant-ghost-host": {
        latest: "1.0.0",
        versions: { "1.0.0": host },
        dependencies: { "1.0.0": { "poodle-plant-ghost": "1.0.0" } },
      },
    });
    const root = mkdtempSync(join(tmpdir(), "poodle-consumer-lock-fail-"));
    roots.push(root);
    const dependencies = { "poodle-plant-ghost-host": "1.0.0" };
    writeFileSync(
      join(root, "package.json"),
      `${JSON.stringify(
        {
          name: "poodle-plant-consumer",
          private: true,
          dependencies,
          overrides: lockedTransitiveOverrides({
            lockText: `{ "packages": {} }`,
            declaredNames: declaredPackageNames(dependencies),
          }),
        },
        null,
        2,
      )}\n`,
    );
    writeFileSync(join(root, ".npmrc"), `registry=${registry}\n`);
    const result = await bunInstall(root, registry);
    expect(result.exitCode).not.toBe(0);
    expect(result.output).toMatch(/poodle-plant-ghost|404|not found|unable to resolve/i);
  });
});
