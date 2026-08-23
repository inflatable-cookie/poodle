/**
 * g16.005 — downstream dual-dependency proof.
 *
 * Builds `consumer/`, a crate written the way an ordinary crates.io user
 * would write it: it declares `gpui = "0.2.2"` for itself AND depends on
 * `poodle-gpui-node-backend`, then passes GPUI values across the boundary in
 * both directions. If Poodle resolves `gpui` from anywhere but crates.io,
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

console.log("## downstream consumer: crates.io gpui 0.2.2 + poodle-gpui-node-backend");
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
  check('the consumer declares gpui = "0.2.2" itself', active.includes('gpui = "0.2.2"'));
  check(
    "the consumer uses no patch, replace, or override section",
    !/\[patch|\[replace|\bpaths\s*=/.test(active),
    "an override would make the proof meaningless",
  );

  const built = compile(proof);
  check(
    "the consumer compiles against Poodle and its own crates.io gpui",
    built.status === 0,
    built.stderr.trim(),
  );

  // The lockfile Cargo just produced is the resolution evidence: exactly one
  // gpui, from the registry.
  const lock = readFileSync(join(proof, "Cargo.lock"), "utf8");
  const gpuiEntries = [...lock.matchAll(/\[\[package\]\]\nname = "(gpui[^"]*)"\nversion = "([^"]+)"\nsource = "([^"]+)"/g)];
  const gpuiCore = gpuiEntries.filter(([, name]) => name === "gpui");
  check(
    "the resolved graph contains exactly one gpui",
    gpuiCore.length === 1,
    gpuiCore.map(([, , version, source]) => `${version} ${source}`).join(", "),
  );
  check(
    "every gpui* crate resolves from the crates.io registry",
    gpuiEntries.length > 0 && gpuiEntries.every(([, , , source]) => source.startsWith("registry+")),
    gpuiEntries.map(([, name, , source]) => `${name}: ${source}`).join("\n"),
  );
  if (gpuiCore.length === 1) console.log(`  resolved: gpui ${gpuiCore[0][2]} from ${gpuiCore[0][3]}`);

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
