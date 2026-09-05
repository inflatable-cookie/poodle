/**
 * g16.005 — downstream dual-dependency proof.
 *
 * Builds `consumer/`, a crate written the way an ordinary crates.io user
 * would write it: it declares `gpui-unofficial = "1.19.0-pre"` for itself AND
 * depends on `poodle-gpui-node-backend`, then passes GPUI values across the
 * boundary in both directions. If Poodle resolves GPUI from anywhere but
 * crates.io `gpui-unofficial`,
 * Cargo gives the two crates different identities and this stops compiling.
 * That is the v0.2.1 defect, reproduced as a gate.
 *
 * The crate is copied to a temporary directory before building, so no
 * lockfile and no target directory ever land in the repository, and the
 * `path` dependencies are rewritten to absolute paths into this checkout.
 *
 * A negative control runs after the real proof: the same crate with one
 * deliberately wrong type annotation must FAIL. A proof that cannot fail is
 * not a proof.
 *
 * Headless: no window, no window server, no permission. `cargo check` is the
 * compile — type resolution is exactly what is being proved.
 */
import { spawnSync } from "node:child_process";
import { cpSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";

const HERE = new URL(".", import.meta.url).pathname;
const REPO = resolve(HERE, "..", "..");
const SOURCE = join(HERE, "consumer");

let failures = 0;
function check(label: string, ok: boolean, detail = ""): void {
  if (ok) {
    console.log(`  PASS  ${label}`);
  } else {
    console.log(`  FAIL  ${label}${detail ? `:\n${detail}` : ""}`);
    failures += 1;
  }
}

/** Materialise the consumer in `dir`, optionally corrupting one annotation. */
function stage(dir: string, corrupt: boolean): void {
  cpSync(SOURCE, dir, { recursive: true });
  const manifest = readFileSync(join(SOURCE, "Cargo.toml.template"), "utf8").replaceAll(
    "__POODLE_ROOT__",
    REPO,
  );
  writeFileSync(join(dir, "Cargo.toml"), manifest);
  rmSync(join(dir, "Cargo.toml.template"));

  if (corrupt) {
    const main = join(dir, "src", "main.rs");
    const source = readFileSync(main, "utf8").replace(
      "fn poodle_colour(value: ColorValue) -> Hsla {",
      "fn poodle_colour(value: ColorValue) -> AnyElement {",
    );
    writeFileSync(main, source);
  }
}

// One stable target directory outside the repository. The consumer crate is
// staged fresh every run, but its dependency graph is the same GPUI build
// every time, so a shared cache turns a multi-minute cold compile into a
// seconds-long recheck without ever writing into the checkout.
const TARGET_DIR = join(tmpdir(), "poodle-dual-dependency-target");

function compile(dir: string): { status: number | null; stderr: string } {
  const result = spawnSync("cargo", ["check", "--quiet", "--manifest-path", join(dir, "Cargo.toml")], {
    encoding: "utf8",
    env: { ...process.env, CARGO_TARGET_DIR: TARGET_DIR },
  });
  return { status: result.status, stderr: result.stderr ?? "" };
}

console.log("## downstream consumer: crates.io gpui-unofficial 1.19.0-pre + poodle-gpui-node-backend");
const work = mkdtempSync(join(tmpdir(), "poodle-dual-dependency-"));
try {
  const proof = join(work, "proof");
  stage(proof, false);

  const manifest = readFileSync(join(proof, "Cargo.toml"), "utf8");
  // Comment lines are stripped: the manifest explains in prose that it uses
  // no override, and that sentence must not read as one.
  const active = manifest
    .split("\n")
    .filter((line) => !line.trimStart().startsWith("#"))
    .join("\n");
  check(
    'the consumer declares gpui-unofficial = "1.19.0-pre" itself',
    active.includes('gpui-unofficial = "1.19.0-pre"'),
  );
  check(
    "the consumer uses no patch, replace, or override section",
    !/\[patch|\[replace|\bpaths\s*=/.test(active),
    "an override would make the proof meaningless",
  );

  const built = compile(proof);
  check(
    "the consumer compiles against Poodle and its own crates.io gpui-unofficial",
    built.status === 0,
    built.stderr.trim(),
  );

  // The lockfile Cargo just produced is the resolution evidence: exactly one
  // gpui, from the registry.
  const lock = readFileSync(join(proof, "Cargo.lock"), "utf8");
  const gpuiEntries = [...lock.matchAll(/\[\[package\]\]\nname = "(gpui[^"]*)"\nversion = "([^"]+)"\nsource = "([^"]+)"/g)];
  const gpuiCore = gpuiEntries.filter(([, name]) => name === "gpui-unofficial");
  check(
    "the resolved graph contains exactly one gpui-unofficial",
    gpuiCore.length === 1,
    gpuiCore.map(([, , version, source]) => `${version} ${source}`).join(", "),
  );
  check(
    "the graph does not also resolve crates.io gpui 0.2.x",
    !gpuiEntries.some(([, name]) => name === "gpui"),
    gpuiEntries.map(([, name, version]) => `${name} ${version}`).join(", "),
  );
  check(
    "every gpui* crate resolves from the crates.io registry",
    gpuiEntries.length > 0 && gpuiEntries.every(([, , , source]) => source.startsWith("registry+")),
    gpuiEntries.map(([, name, , source]) => `${name}: ${source}`).join("\n"),
  );
  if (gpuiCore.length === 1) console.log(`  resolved: gpui-unofficial ${gpuiCore[0][2]} from ${gpuiCore[0][3]}`);

  console.log("## transitive shape — the graph must compile tinyvec with std (g16.092)");
  // tinyvec 1.13.0 broke its alloc-only build: the new `with_initial_len` calls
  // `vec!` where only `alloc::vec::{self, Vec}` is imported (the module, not the
  // macro), and without the `std` feature the crate is `no_std`, so nothing puts
  // `vec!` in scope. A fresh consumer resolution has no lockfile to fall back
  // on, so the proof compile then dies inside the dependency and the negative
  // control never reaches its intended type mismatch. The repaired graph shape —
  // asserted here so it cannot silently regress — is that something below this
  // consumer enables tinyvec's `std` feature.
  const tinyvecShape = spawnSync(
    "cargo",
    ["tree", "--manifest-path", join(proof, "Cargo.toml"), "--invert", "tinyvec", "--edges", "features"],
    { encoding: "utf8", env: { ...process.env, CARGO_TARGET_DIR: TARGET_DIR } },
  );
  const tinyvecTree = tinyvecShape.stdout ?? "";
  const tinyvecVersion = tinyvecTree.match(/^tinyvec v(\S+)/m);
  if (tinyvecVersion) console.log(`  resolved: tinyvec ${tinyvecVersion[1]} from fresh resolution`);
  check(
    "the consumer graph enables tinyvec's std feature (alloc-only cannot compile)",
    tinyvecShape.status === 0 && /tinyvec feature "std"/.test(tinyvecTree),
    tinyvecTree || tinyvecShape.stderr || "",
  );

  console.log("## negative control — the proof must be able to fail");
  const negative = join(work, "negative");
  stage(negative, true);
  const broken = compile(negative);
  check(
    "a wrong GPUI type annotation fails the compile",
    broken.status !== 0,
    "a corrupted consumer compiled clean, so this gate proves nothing",
  );
  check(
    "the failure is a type mismatch, the same class a divergent crate identity produces",
    /mismatched types|expected .*AnyElement/.test(broken.stderr),
    broken.stderr.trim().slice(0, 600),
  );
} finally {
  rmSync(work, { recursive: true, force: true });
}

console.log(
  failures === 0
    ? "\ndual-dependency proof: all checks pass"
    : `\ndual-dependency proof: ${failures} failed`,
);
process.exit(failures === 0 ? 0 : 1);
