/**
 * `effigy ci:fresh`: validate a fresh disposable checkout with the repository's
 * pinned Bun, whatever Bun the host PATH provides.
 *
 * Queue runs the repository's validation command with the host PATH. A host Bun
 * that differs from `packageManager` installs a different node_modules layout,
 * which changes TypeScript declaration emit and fails `ci:web` for reasons the
 * candidate never introduced. So this script resolves the pinned Bun, puts it
 * first on PATH through a temporary directory outside the checkout, then runs
 * the frozen install and the required PR lanes (`effigy ci`) with it.
 */
import { spawnSync } from "node:child_process";
import { mkdtempSync, readFileSync, rmSync, symlinkSync } from "node:fs";
import { tmpdir } from "node:os";
import { delimiter, join } from "node:path";

export function pinnedBunVersion(packageJson: string): string {
  const manager = (JSON.parse(packageJson) as { packageManager?: unknown }).packageManager;
  const match = typeof manager === "string" ? /^bun@(\d+\.\d+\.\d+)$/.exec(manager) : null;
  if (!match) {
    throw new Error(`package.json packageManager must pin an exact bun@x.y.z (got ${JSON.stringify(manager)})`);
  }
  return match[1];
}

function capture(command: string, args: string[], env: NodeJS.ProcessEnv = process.env): string {
  const result = spawnSync(command, args, { encoding: "utf8", env });
  if (result.status !== 0) {
    throw new Error(`${command} ${args.join(" ")} failed (${result.status}): ${result.stderr.trim()}`);
  }
  return result.stdout.trim();
}

/** Returns a PATH whose first entry provides `bun` and `bunx` at `version`. */
export function pathWithPinnedBun(version: string, scratch: string): string {
  const hostVersion = capture("bun", ["--version"]);
  if (hostVersion === version) return process.env.PATH ?? "";
  // `bunx bun@<version>` fetches the pinned binary into Bun's own cache (never
  // the checkout); its execPath is the binary to put first on PATH.
  const binary = capture("bunx", [`bun@${version}`, "-e", "console.log(process.execPath)"]).split("\n").pop()!;
  symlinkSync(binary, join(scratch, "bun"));
  symlinkSync(binary, join(scratch, "bunx"));
  const path = `${scratch}${delimiter}${process.env.PATH ?? ""}`;
  const resolved = capture("bun", ["--version"], { ...process.env, PATH: path });
  if (resolved !== version) {
    throw new Error(`pinned bun ${version} resolved to ${resolved} (${binary})`);
  }
  return path;
}

function run(command: string, args: string[], path: string): number {
  const result = spawnSync(command, args, { stdio: "inherit", env: { ...process.env, PATH: path } });
  return result.status ?? 1;
}

if (import.meta.main) {
  const version = pinnedBunVersion(readFileSync(join(process.cwd(), "package.json"), "utf8"));
  const scratch = mkdtempSync(join(tmpdir(), "poodle-ci-fresh-bun-"));
  let status = 1;
  try {
    const path = pathWithPinnedBun(version, scratch);
    console.log(`ci:fresh: bun ${version}`);
    status = run("bun", ["install", "--frozen-lockfile"], path);
    if (status === 0) status = run("effigy", ["ci"], path);
  } finally {
    rmSync(scratch, { recursive: true, force: true });
  }
  process.exit(status);
}
