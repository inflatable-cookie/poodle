/** Shared lockstep release surfaces and Cargo lockfile version handling. */

export const LOCKSTEP_JS_MANIFEST_PATHS = [
  "package.json",
  "packages/core/package.json",
  "packages/svelte/components/package.json",
  "packages/react/components/package.json",
] as const;

export const LOCKSTEP_CARGO_MANIFEST_PATHS = [
  "packages/codegen/Cargo.toml",
  "packages/contracts/adapter/Cargo.toml",
  "packages/contracts/components/Cargo.toml",
  "packages/contracts/events/Cargo.toml",
  "packages/contracts/headless/Cargo.toml",
  "packages/contracts/ir/Cargo.toml",
  "packages/contracts/layout/Cargo.toml",
  "packages/contracts/markdown/Cargo.toml",
  "packages/contracts/node/Cargo.toml",
  "packages/contracts/style/Cargo.toml",
  "packages/contracts/tokens/Cargo.toml",
  "packages/gpui/adapter/Cargo.toml",
  "packages/gpui/node-backend/Cargo.toml",
  "packages/gpui/preview/Cargo.toml",
  "packages/jetstream/adapter/Cargo.toml",
  "packages/jetstream/preview/Cargo.toml",
  "packages/render/Cargo.toml",
] as const;

export const LOCKSTEP_CARGO_LOCK_PATHS = [
  "packages/gpui/node-backend/Cargo.lock",
  "packages/gpui/preview/Cargo.lock",
] as const;

export type CargoPackageIdentity = { name: string; version: string };

export function cargoPackageIdentity(text: string, label: string): CargoPackageIdentity {
  let section = "";
  let name: string | undefined;
  let version: string | undefined;
  for (const line of text.split(/\r?\n/)) {
    if (/^\s*\[\[/.test(line)) {
      section = "";
      continue;
    }
    const table = /^\s*\[([^\]]+)\]\s*(?:#.*)?$/.exec(line);
    if (table) {
      section = table[1];
      continue;
    }
    if (section !== "package") continue;
    const field = /^\s*(name|version)\s*=\s*"([^"]+)"\s*(?:#.*)?$/.exec(line);
    if (field?.[1] === "name") name = field[2];
    if (field?.[1] === "version") version = field[2];
  }
  if (!name || !version) throw new Error(`${label} must declare [package] name and version`);
  return { name, version };
}

export function updateCargoManifestText(
  text: string,
  path: string,
  sourceVersion: string,
  targetVersion: string,
): { text: string; name: string } {
  const identity = cargoPackageIdentity(text, path);
  if (identity.version !== sourceVersion) {
    throw new Error(`${path} must be at ${sourceVersion}, found ${identity.version}`);
  }
  const lines = text.split("\n");
  let section = "";
  let packageVersions = 0;
  for (let index = 0; index < lines.length; index += 1) {
    const line = lines[index];
    if (/^\s*\[\[/.test(line)) {
      section = "";
      continue;
    }
    const table = /^\s*\[([^\]]+)\]\s*(?:#.*)?$/.exec(line);
    if (table) {
      section = table[1];
      continue;
    }
    if (section === "package") {
      const version = /^(\s*version\s*=\s*)"([^"]+)"(\s*(?:#.*)?)$/.exec(line);
      if (version) {
        if (version[2] !== sourceVersion) {
          throw new Error(
            `${path} package version must be ${sourceVersion}, found ${version[2]}`,
          );
        }
        lines[index] = `${version[1]}"${targetVersion}"${version[3]}`;
        packageVersions += 1;
      }
    }
    const dependency = /^(\s*poodle-[A-Za-z0-9_-]+\s*=\s*\{)(.*)(\}\s*(?:#.*)?)$/.exec(line);
    if (!dependency || !/\bpath\s*=\s*"[^"]+"/.test(dependency[2])) continue;
    const version = /\bversion\s*=\s*"([^"]+)"/.exec(dependency[2]);
    if (!version) continue;
    if (version[1] !== sourceVersion) {
      throw new Error(
        `${path} internal path dependency must be ${sourceVersion}, found ${version[1]}`,
      );
    }
    const attrs = dependency[2].replace(
      /\bversion\s*=\s*"[^"]+"/,
      `version = "${targetVersion}"`,
    );
    lines[index] = `${dependency[1]}${attrs}${dependency[3]}`;
  }
  if (packageVersions !== 1) {
    throw new Error(`${path} must have exactly one [package] version`);
  }
  return { text: lines.join("\n"), name: identity.name };
}

type CargoLockBlock = {
  name: string;
  version: string;
  source: string | null;
  start: number;
  end: number;
};

function cargoLockBlocks(text: string, label: string): CargoLockBlock[] {
  const starts = [...text.matchAll(/^\[\[package\]\]\s*$/gm)].map((match) => match.index!);
  if (starts.length === 0) throw new Error(`${label} has no Cargo package entries`);
  return starts.map((start, index) => {
    const end = starts[index + 1] ?? text.length;
    const block = text.slice(start, end);
    const name = /^name\s*=\s*"([^"]+)"\s*$/m.exec(block)?.[1];
    const version = /^version\s*=\s*"([^"]+)"\s*$/m.exec(block)?.[1];
    const source = /^source\s*=\s*"([^"]+)"\s*$/m.exec(block)?.[1] ?? null;
    if (!name || !version) throw new Error(`${label} has an unparsable package entry`);
    return { name, version, source, start, end };
  });
}

function rewriteLockDependencyVersions(
  block: string,
  crateNames: ReadonlySet<string>,
  sourceVersion: string,
  targetVersion: string,
): string {
  return block.replace(
    /"(poodle-[A-Za-z0-9_-]+)( [^"]+)?"/g,
    (whole, name: string, suffix = "") => {
      if (!crateNames.has(name) || !suffix.startsWith(" ")) return whole;
      const tail = suffix.slice(1);
      const versionPrefix = `${sourceVersion} `;
      const targetPrefix = `${targetVersion} `;
      if (tail === sourceVersion) return `"${name} ${targetVersion}"`;
      if (tail === targetVersion) return whole;
      if (tail.startsWith(versionPrefix)) {
        return `"${name} ${targetVersion}${tail.slice(sourceVersion.length)}"`;
      }
      if (tail.startsWith(targetPrefix)) return whole;
      throw new Error(`Cargo.lock dependency ${name} carries unexpected version ${tail}`);
    },
  );
}

/** Update only local Poodle package versions and their explicitly versioned lock references. */
export function updateCargoLockText(
  text: string,
  crateNames: ReadonlySet<string>,
  sourceVersion: string,
  targetVersion: string,
  label: string,
): string {
  let next = "";
  let cursor = 0;
  let changedPackages = 0;
  for (const entry of cargoLockBlocks(text, label)) {
    next += text.slice(cursor, entry.start);
    let block = text.slice(entry.start, entry.end);
    if (crateNames.has(entry.name) && entry.source === null) {
      if (entry.version !== sourceVersion) {
        throw new Error(
          `${label} requires local ${entry.name} at ${sourceVersion}, found ${entry.version}`,
        );
      }
      const versionLine = new RegExp(`^(version\\s*=\\s*)"${sourceVersion.replaceAll(".", "\\.")}"(\\s*)$`, "m");
      if (!versionLine.test(block)) {
        throw new Error(`${label} has no version line for ${entry.name}`);
      }
      block = block.replace(versionLine, `$1"${targetVersion}"$2`);
      block = rewriteLockDependencyVersions(block, crateNames, sourceVersion, targetVersion);
      changedPackages += 1;
    } else if (entry.name.startsWith("poodle-") && entry.source === null) {
      throw new Error(
        `${label} contains unrecognized local Poodle crate ${entry.name}`,
      );
    }
    next += block;
    cursor = entry.end;
  }
  next += text.slice(cursor);
  if (changedPackages === 0) throw new Error(`${label} contains no local Poodle crates to bump`);
  return next;
}

/** Prove two Cargo locks differ only by the coordinated local Poodle version transition. */
export function assertCargoLockVersionTransition(
  before: string,
  after: string,
  crateNames: ReadonlySet<string>,
  sourceVersion: string,
  targetVersion: string,
  label: string,
): void {
  const beforeBlocks = cargoLockBlocks(before, `${label} base`);
  const afterBlocks = cargoLockBlocks(after, `${label} candidate`);
  const beforeLocal = beforeBlocks.filter((entry) => crateNames.has(entry.name) && entry.source === null);
  const afterLocal = afterBlocks.filter((entry) => crateNames.has(entry.name) && entry.source === null);
  if (beforeLocal.length === 0 || beforeLocal.length !== afterLocal.length) {
    throw new Error(`${label} must retain its local Poodle package set`);
  }
  const beforeNames = beforeLocal.map(({ name }) => name).sort();
  const afterNames = afterLocal.map(({ name }) => name).sort();
  if (beforeNames.some((name, index) => name !== afterNames[index])) {
    throw new Error(`${label} must retain its local Poodle package identities`);
  }
  for (const entry of beforeLocal) {
    if (entry.version !== sourceVersion) {
      throw new Error(
        `${label} base requires local ${entry.name} at ${sourceVersion}, found ${entry.version}`,
      );
    }
  }
  for (const entry of afterLocal) {
    if (entry.version !== targetVersion) {
      throw new Error(
        `${label} candidate requires local ${entry.name} at ${targetVersion}, found ${entry.version}`,
      );
    }
  }

  const canonical = (
    text: string,
    entries: CargoLockBlock[],
    expectedVersion: string,
  ): string => {
    let result = "";
    let cursor = 0;
    for (const entry of entries) {
      result += text.slice(cursor, entry.start);
      let block = text.slice(entry.start, entry.end);
      if (crateNames.has(entry.name) && entry.source === null) {
        block = block.replace(
          /^version\s*=\s*"[^"]+"\s*$/m,
          'version = "<lockstep>"',
        );
        block = block.replace(
          /"(poodle-[A-Za-z0-9_-]+) ([^"]+)"/g,
          (whole, name: string, suffix: string) => {
            if (!crateNames.has(name)) return whole;
            const version = suffix.split(" ", 1)[0];
            if (version !== expectedVersion) {
              throw new Error(
                `${label} dependency ${name} must be ${expectedVersion}, found ${suffix}`,
              );
            }
            return `"${name} <lockstep>${suffix.slice(version.length)}"`;
          },
        );
      }
      result += block;
      cursor = entry.end;
    }
    return result + text.slice(cursor);
  };
  if (
    canonical(before, beforeBlocks, sourceVersion) !==
    canonical(after, afterBlocks, targetVersion)
  ) {
    throw new Error(`${label} contains changes beyond local Poodle version entries`);
  }
}

export function semverTuple(version: string): [number, number, number] | null {
  const match = /^0\.(0|[1-9]\d*)\.(0|[1-9]\d*)$/.exec(version);
  return match ? [0, Number(match[1]), Number(match[2])] : null;
}

export function comparePreOneVersions(left: string, right: string): number {
  const a = semverTuple(left);
  const b = semverTuple(right);
  if (!a || !b) {
    throw new Error(`expected pre-1.0 X.Y.Z versions, received ${left} and ${right}`);
  }
  return a[1] - b[1] || a[2] - b[2];
}
