/**
 * Fetch dependencies for every Rust crate in the repo.
 *
 * There is deliberately no root Cargo workspace: the jetstream crates
 * path-depend on the sibling `jetstream` repo (`../jetstream/crates/*`),
 * and a workspace would make every cargo invocation — including
 * contracts-only and web checkouts — fail whenever that repo is absent.
 * Each crate stands alone, so this fetches each manifest in turn instead,
 * and skips the jetstream tree with a note when the engine is not checked
 * out rather than failing the bootstrap.
 */
import { existsSync, readdirSync, statSync } from "node:fs";
import { join, resolve } from "node:path";

const ROOT = resolve(new URL("..", import.meta.url).pathname);
const ENGINE = resolve(ROOT, "../jetstream");

function manifests(dir: string): string[] {
  return readdirSync(dir).flatMap((entry) => {
    const path = join(dir, entry);
    if (entry === "node_modules" || entry === "target") return [];
    if (statSync(path).isDirectory()) return manifests(path);
    return entry === "Cargo.toml" ? [path] : [];
  });
}

const engineAvailable = existsSync(join(ENGINE, "Cargo.toml"));
let fetched = 0;
let skipped = 0;

for (const manifest of manifests(join(ROOT, "packages")).sort()) {
  if (manifest.includes("/jetstream/") && !engineAvailable) {
    console.log(`skip  ${manifest} (sibling jetstream repo not checked out)`);
    skipped += 1;
    continue;
  }
  console.log(`fetch ${manifest}`);
  const result = Bun.spawnSync(["cargo", "fetch", "--manifest-path", manifest], {
    stdout: "inherit",
    stderr: "inherit",
  });
  if (result.exitCode !== 0) {
    console.error(`cargo fetch failed for ${manifest}`);
    process.exit(1);
  }
  fetched += 1;
}

console.log(
  `fetched ${fetched} crate manifest(s)${skipped ? `, skipped ${skipped} (jetstream engine absent)` : ""}.`,
);
