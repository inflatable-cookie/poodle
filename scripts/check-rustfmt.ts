/**
 * poodle#072 — the rustfmt gate over every Poodle Rust crate.
 *
 * `cargo fmt` cannot be the gate. There is deliberately no root Cargo
 * workspace, and stable rustfmt has no per-path exclusion (`ignore` in
 * rustfmt.toml is nightly-only), so the only stable way to leave generated
 * Rust alone is to enumerate the authored `.rs` files and drive rustfmt over
 * them directly with `--config skip_children=true`. That flag matters:
 * rustfmt recurses into child modules from every explicitly passed file
 * (`#[path = "generated/…"]` mod declarations resolve and are formatted
 * too), which would reformat generated Rust that must stay byte-identical to
 * its generator — an identity owned by `ir:check`, `catalogue:check`,
 * `audit:tokens` and `audit:icons`, not here. Nothing authored is missed by
 * skipping children: the enumeration below passes every authored `.rs` file
 * under each crate explicitly.
 *
 * Excluded, and the generator that owns each:
 *   - any `generated/` directory segment:
 *       packages/contracts/tokens/src/generated/    build-tokens.ts
 *       packages/contracts/headless/src/generated/  poodle-codegen `machine-rust`
 *       packages/gpui/preview/src/generated/        poodle-codegen `shell-rust`, `specimen-rust`, `catalogue-rust`
 *       packages/jetstream/preview/src/generated/   same targets
 *   - `*.generated.rs`:
 *       packages/contracts/components/src/icon_geometry.generated.rs  build-default-icons.ts
 *
 *   bun scripts/check-rustfmt.ts           # check mode (the gate)
 *   bun scripts/check-rustfmt.ts --write   # format in place, before committing
 *   bun scripts/check-rustfmt.ts --list    # print the per-crate file census
 *
 * Run with `effigy check:rustfmt` (a `ci:rust` member).
 */
import { spawnSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";

const ROOT = path.resolve(import.meta.dir, "..");

/** Walk `dir` depth-first, returning every entry path below it. */
function walk(dir: string, skip: (name: string) => boolean): string[] {
  const found: string[] = [];
  for (const entry of fs.readdirSync(dir, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    if (skip(entry.name)) continue;
    const entryPath = path.join(dir, entry.name);
    if (entry.isDirectory()) found.push(...walk(entryPath, skip));
    else found.push(entryPath);
  }
  return found;
}

/** Every crate manifest under packages/, in stable order. */
export function crateManifests(): string[] {
  const packages = path.join(ROOT, "packages");
  return walk(packages, (name) => name === "node_modules" || name === "target")
    .filter((file) => path.basename(file) === "Cargo.toml");
}

/**
 * Generated Rust stays byte-identical to its generator and is none of this
 * gate's business. `generated/` directories and `*.generated.rs` cover every
 * generator in the repository (see the header for the exact mapping).
 */
export function isGenerated(repoRelativePath: string): boolean {
  return repoRelativePath.includes(`${path.sep}generated${path.sep}`) || /\.generated\.rs$/.test(repoRelativePath);
}

/** The crate's authored `.rs` files, relative to the repository root. */
export function authoredRustFiles(crateDir: string): string[] {
  return walk(crateDir, (name) => name === "target" || name === "node_modules")
    .filter((file) => file.endsWith(".rs"))
    .map((file) => path.relative(ROOT, file))
    .filter((file) => !isGenerated(file));
}

/** The crate's Rust edition, read from its manifest. A missing edition fails. */
export function editionFor(manifest: string): string {
  const text = fs.readFileSync(manifest, "utf8");
  const match = /^\s*edition\s*=\s*"([^"]+)"/m.exec(text);
  if (!match) {
    console.error(`check-rustfmt: no edition in ${path.relative(ROOT, manifest)}; refusing to guess`);
    process.exit(1);
  }
  return match[1];
}

/**
 * The rustfmt argv for one crate. `--config skip_children=true` is the
 * recursion guard described in the header; it is pinned by
 * `check-rustfmt.test.ts` because dropping it would silently put generated
 * files back in rustfmt's reach.
 */
export function rustfmtArgs(edition: string, files: string[], check: boolean): string[] {
  return ["--edition", edition, "--config", "skip_children=true", ...(check ? ["--check"] : []), ...files];
}

function runRustfmt(args: string[], crateLabel: string): { status: number; stdout: string; stderr: string } {
  const result = spawnSync("rustfmt", args, { encoding: "utf8", maxBuffer: 64 * 1024 * 1024 });
  if (result.error) {
    console.error(`check-rustfmt: could not run rustfmt (${result.error.message}). Is the Rust toolchain installed?`);
    process.exit(1);
  }
  if (result.status === null || (result.status !== 0 && result.status !== 1)) {
    console.error(`check-rustfmt: rustfmt failed on ${crateLabel}\n${result.stderr}`);
    process.exit(1);
  }
  return { status: result.status, stdout: result.stdout, stderr: result.stderr };
}

function main(): void {
  const mode = process.argv[2] ?? "--check";
  if (mode !== "--check" && mode !== "--write" && mode !== "--list") {
    console.error("usage: bun scripts/check-rustfmt.ts [--check|--write|--list]");
    process.exit(1);
  }

  const manifests = crateManifests();
  if (manifests.length === 0) {
    console.error("check-rustfmt: no crate manifests found under packages/; refusing to pass on an empty census");
    process.exit(1);
  }

  let crates = 0;
  let files = 0;
  let unformatted: string[] = [];

  for (const manifest of manifests) {
    const crateDir = path.dirname(manifest);
    const crateFiles = authoredRustFiles(crateDir);
    if (crateFiles.length === 0) continue;
    const label = path.relative(ROOT, crateDir);
    crates += 1;
    files += crateFiles.length;

    if (mode === "--list") {
      console.log(`${label}: ${crateFiles.length} file(s)`);
      for (const file of crateFiles) console.log(`  ${file}`);
      continue;
    }

    const check = mode === "--check";
    const result = runRustfmt(rustfmtArgs(editionFor(manifest), crateFiles, check), label);
    if (check && result.status === 1) {
      unformatted.push(label);
      console.log(`check-rustfmt: ${label} is not rustfmt-clean:\n${result.stdout}`);
    } else if (check) {
      console.log(`check-rustfmt: ${label} clean (${crateFiles.length} file(s))`);
    } else {
      console.log(`check-rustfmt: ${label} formatted (${crateFiles.length} file(s))`);
    }
  }

  if (mode === "--list") {
    console.log(`check-rustfmt: ${crates} crate(s), ${files} authored file(s)`);
    return;
  }
  if (mode === "--write") {
    console.log(`check-rustfmt: formatted ${files} file(s) across ${crates} crate(s)`);
    return;
  }
  if (unformatted.length > 0) {
    console.error(
      `check-rustfmt: ${unformatted.length} crate(s) not rustfmt-clean: ${unformatted.join(", ")}\n` +
        `run \`bun scripts/check-rustfmt.ts --write\` and commit the result`,
    );
    process.exit(1);
  }
  console.log(`check-rustfmt: ${crates} crate(s), ${files} file(s), all rustfmt-clean`);
}

if (import.meta.main) main();
