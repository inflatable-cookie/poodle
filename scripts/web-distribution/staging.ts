import { spawnSync } from "node:child_process";
import { existsSync, mkdtempSync, readdirSync, renameSync, rmSync, mkdirSync } from "node:fs";
import { basename, join } from "node:path";

export function distDir(packageRoot: string): string {
  return join(packageRoot, "dist");
}

export function cleanStaging(packageRoot: string): string {
  const outDir = distDir(packageRoot);
  if (existsSync(outDir)) rmSync(outDir, { recursive: true, force: true });
  mkdirSync(outDir, { recursive: true });
  return outDir;
}

const STAGING_PREFIX = ".dist-staging-";
const PREVIOUS_SUFFIX = ".__previous-";

function previousBackups(packageRoot: string): string[] {
  let entries: string[];
  try {
    entries = readdirSync(packageRoot);
  } catch {
    return [];
  }
  return entries
    .filter((entry) => entry.startsWith(`dist${PREVIOUS_SUFFIX}`))
    .map((entry) => join(packageRoot, entry));
}

/**
 * Create a fresh sibling staging directory on the same filesystem as `dist`.
 * The package build writes its full tree here; `publishStaging` exchanges it
 * with `dist` so readers never see a half-built tree.
 */
export function createStagingDir(packageRoot: string): string {
  for (const stale of previousBackups(packageRoot)) {
    rmSync(stale, { recursive: true, force: true });
  }
  return mkdtempSync(join(packageRoot, STAGING_PREFIX));
}

export function discardStaging(stagedDir: string): void {
  rmSync(stagedDir, { recursive: true, force: true });
}

function isStagingDir(packageRoot: string, dir: string): boolean {
  return basename(dir).startsWith(STAGING_PREFIX) && join(packageRoot, basename(dir)) === dir;
}

// One directory-exchange rename: Linux renameat2(RENAME_EXCHANGE), macOS
// renamex_np(RENAME_SWAP). Exits 0 with OK on success, 2 with ERRNO=<n> on
// failure, 3 with UNSUPPORTED when the platform has no exchange call.
const EXCHANGE_SCRIPT = `
import ctypes, ctypes.util, sys
src, dst = sys.argv[1], sys.argv[2]
plat = sys.platform
try:
    if plat == "darwin":
        libc = ctypes.CDLL("/usr/lib/libSystem.B.dylib", use_errno=True)
        fn = libc.renamex_np
        fn.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.c_uint]
        fn.restype = ctypes.c_int
        rc = fn(src.encode(), dst.encode(), 0x2)
    elif plat.startswith("linux"):
        lib = ctypes.util.find_library("c") or "libc.so.6"
        libc = ctypes.CDLL(lib, use_errno=True)
        fn = libc.renameat2
        fn.argtypes = [ctypes.c_int, ctypes.c_char_p, ctypes.c_int, ctypes.c_char_p, ctypes.c_uint]
        fn.restype = ctypes.c_long
        rc = fn(-100, src.encode(), -100, dst.encode(), 2)
    else:
        print("UNSUPPORTED")
        sys.exit(3)
except AttributeError:
    print("UNSUPPORTED")
    sys.exit(3)
if rc != 0:
    print("ERRNO=%d" % ctypes.get_errno())
    sys.exit(2)
print("OK")
`;

const EXDEV_ERRNO = 18;
// The exchange call itself is missing or the filesystem does not implement
// it (Linux ENOSYS/EINVAL/EOPNOTSUPP, macOS ENOSYS/EINVAL/EOPNOTSUPP).
const UNSUPPORTED_ERRNOS = new Set([22, 38, 78, 95, 102]);

function unsupported(reason: string): Error {
  const error = new Error(`directory exchange unsupported: ${reason}`);
  (error as { code?: string }).code = "EXCHANGE_UNSUPPORTED";
  return error;
}

function exdevError(): Error {
  const error = new Error(
    "atomic dist swap needs staged output on the same filesystem as dist (exchange returned EXDEV)",
  );
  (error as { code?: string }).code = "EXDEV";
  return error;
}

/**
 * Atomically exchange two directories with a single rename. Both directories
 * must exist. Throws EXDEV when they span filesystems.
 */
export function exchangeDirectories(first: string, second: string): void {
  let result;
  try {
    result = spawnSync("python3", ["-c", EXCHANGE_SCRIPT, first, second], {
      encoding: "utf8",
    });
  } catch (error) {
    throw unsupported(
      `python3 unavailable: ${error instanceof Error ? error.message : String(error)}`,
    );
  }
  if (result.error) {
    throw unsupported(
      `python3 unavailable: ${result.error.message ?? String(result.error)}`,
    );
  }
  const output = `${result.stdout ?? ""}${result.stderr ?? ""}`;
  if (result.status === 0) return;
  if (result.status === 3 || output.includes("UNSUPPORTED")) {
    throw unsupported(output.trim() || `python3 exit ${result.status}`);
  }
  const errno = /ERRNO=(\d+)/.exec(output)?.[1];
  if (errno !== undefined) {
    const code = Number(errno);
    if (code === EXDEV_ERRNO) throw exdevError();
    if (UNSUPPORTED_ERRNOS.has(code)) throw unsupported(`exchange errno ${code}`);
    throw new Error(`directory exchange failed with errno ${code}`);
  }
  throw new Error(`directory exchange failed (python3 exit ${result.status}): ${output.trim()}`);
}

/** Probe whether the exchange call works inside `dir`. Cleans up after itself. */
export function directoryExchangeAvailable(dir: string): boolean {
  const first = mkdtempSync(join(dir, ".exchange-probe-a-"));
  const second = mkdtempSync(join(dir, ".exchange-probe-b-"));
  try {
    exchangeDirectories(first, second);
    return true;
  } catch (error) {
    if ((error as { code?: string } | null)?.code === "EXCHANGE_UNSUPPORTED") return false;
    throw error;
  } finally {
    rmSync(first, { recursive: true, force: true });
    rmSync(second, { recursive: true, force: true });
  }
}

// Pre-exchange fallback for filesystems without an exchange call: the old
// tree is kept as a sibling backup until the staged rename succeeds. A
// concurrent cross-process reader can briefly observe dist missing between
// the two renames.
function legacyTwoRenamePublish(packageRoot: string, stagedDir: string, outDir: string): void {
  const backup = `${outDir}${PREVIOUS_SUFFIX}${process.pid}-${Date.now()}`;
  renameSync(outDir, backup);
  try {
    renameSync(stagedDir, outDir);
  } catch (error) {
    try {
      renameSync(backup, outDir);
    } catch {
      // The backup still holds the previous tree; leave it for the next build
      // to clean up rather than masking the original failure.
    }
    throw error;
  }
  rmSync(backup, { recursive: true, force: true });
}

/**
 * Replace `dist` with the fully built staging tree via one directory
 * exchange: a linked consumer sees either the complete previous tree or the
 * complete new tree, never a missing or half-built `dist`. The previous tree
 * ends up at the staging path and is removed. A failed build must call
 * `discardStaging` instead so the previous `dist` is untouched.
 */
export function publishStaging(packageRoot: string, stagedDir: string): string {
  if (!isStagingDir(packageRoot, stagedDir)) {
    throw new Error(`refusing to publish non-staging directory ${stagedDir}`);
  }
  const outDir = distDir(packageRoot);
  if (!existsSync(outDir)) {
    // First build: nothing for a consumer to observe yet.
    renameSync(stagedDir, outDir);
    return outDir;
  }
  try {
    exchangeDirectories(stagedDir, outDir);
  } catch (error) {
    if ((error as { code?: string } | null)?.code === "EXCHANGE_UNSUPPORTED") {
      legacyTwoRenamePublish(packageRoot, stagedDir, outDir);
      return outDir;
    }
    throw error;
  }
  discardStaging(stagedDir);
  return outDir;
}
