import { parseJsonc } from "../../scripts/workspace-lock";

export type ConsumerLockInput = {
  lockText: string;
  declaredNames: Iterable<string>;
};

export type LockOverrideValue = string | Record<string, string>;
export type LockOverrideMap = Record<string, LockOverrideValue>;

type BunLockFile = {
  packages?: Record<string, unknown>;
};

function isWorkspaceResolution(value: unknown): boolean {
  return Array.isArray(value) && typeof value[0] === "string" && value[0].includes("@workspace:");
}

function specNameAndVersion(spec: string): { name: string; version: string } | undefined {
  const at = spec.lastIndexOf("@");
  if (at <= 0) return undefined;
  const name = spec.slice(0, at);
  const version = spec.slice(at + 1);
  if (!name || !version || version.startsWith("workspace:")) return undefined;
  return { name, version };
}

function skipForDeclared(name: string, declared: Set<string>): boolean {
  if (declared.has(name)) return true;
  if (declared.has("vitest") && (name === "vitest" || name.startsWith("@vitest/"))) return true;
  if (
    declared.has("vite") &&
    (name === "vite" || name === "rolldown" || name.startsWith("@vitejs/") || name.startsWith("@rolldown/"))
  ) {
    return true;
  }
  return false;
}

function parentOfContextKey(key: string, name: string): string | undefined {
  const suffix = `/${name}`;
  if (key.endsWith(suffix) && key.length > suffix.length) return key.slice(0, -suffix.length);
  return undefined;
}

function setTopLevel(overrides: LockOverrideMap, name: string, version: string): void {
  const current = overrides[name];
  if (current === undefined) {
    overrides[name] = version;
    return;
  }
  if (typeof current === "object" && current["."] === undefined) current["."] = version;
}

function setNested(overrides: LockOverrideMap, parent: string, name: string, version: string): void {
  const current = overrides[parent];
  if (current === undefined) {
    overrides[parent] = { [name]: version };
    return;
  }
  if (typeof current === "string") {
    overrides[parent] = { ".": current, [name]: version };
    return;
  }
  current[name] = version;
}

/**
 * Transitive pins for a source-free consumer, taken from the repository
 * `bun.lock`. Direct consumer specs stay as declared (React 18, Vitest 4,
 * file tarballs). A just-published latest that 404s cannot float the install.
 * A dependency absent from the lock still has to resolve from the registry
 * and fails closed.
 *
 * bun 1.4.2 ignores an incomplete rewritten lock ("Failed to resolve root
 * prod dependency"), so this is an override map rather than a lock file.
 * Package-context keys (`svelte/magic-string`) keep a different version than
 * the top-level package; those become nested overrides under the parent so
 * both resolutions stay locked.
 */
export function lockedTransitiveOverrides(input: ConsumerLockInput): LockOverrideMap {
  const parsed = parseJsonc(input.lockText) as BunLockFile;
  const packages = parsed.packages ?? {};
  const declared = new Set(input.declaredNames);
  const overrides: LockOverrideMap = {};
  for (const [key, value] of Object.entries(packages)) {
    if (isWorkspaceResolution(value) || !Array.isArray(value) || typeof value[0] !== "string") continue;
    const spec = specNameAndVersion(value[0]);
    if (spec === undefined) continue;
    if (skipForDeclared(spec.name, declared)) continue;
    if (spec.name === key) {
      setTopLevel(overrides, key, spec.version);
      continue;
    }
    const parent = parentOfContextKey(key, spec.name);
    if (parent === undefined) continue;
    setNested(overrides, parent, spec.name, spec.version);
  }
  return overrides;
}

export function declaredPackageNames(
  dependencies: Record<string, string> = {},
  devDependencies: Record<string, string> = {},
): string[] {
  return [...Object.keys(dependencies), ...Object.keys(devDependencies)];
}
