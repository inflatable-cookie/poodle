/**
 * Fetch dependencies for every Rust crate in the repo.
 *
 * There is deliberately no root Cargo workspace. Each crate stands alone, so
 * this fetches each manifest in turn.
 */
import { readdirSync, statSync } from "node:fs";
import { join, resolve } from "node:path";

const ROOT = resolve(new URL("..", import.meta.url).pathname);

function manifests(dir: string): string[] {
  return readdirSync(dir).flatMap((entry) => {
    const path = join(dir, entry);
    if (entry === "node_modules" || entry === "target") return [];
    if (statSync(path).isDirectory()) return manifests(path);
    return entry === "Cargo.toml" ? [path] : [];
  });
}

let fetched = 0;

for (const manifest of manifests(join(ROOT, "packages")).sort()) {
  console.log(`fetch ${manifest}`);
  const result = Bun.spawnSync(["sh", join(ROOT, "scripts/rust-driver.sh"), "fetch", "--manifest-path", manifest], {
    stdout: "inherit",
    stderr: "inherit",
  });
  if (result.exitCode !== 0) {
    console.error(`rust fetch failed for ${manifest}`);
    process.exit(1);
  }
  fetched += 1;
}

console.log(`fetched ${fetched} crate manifest(s).`);
