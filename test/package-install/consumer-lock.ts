import { parseJsonc } from "../../scripts/workspace-lock";

export type ConsumerLockInput = {
  lockText: string;
  declaredNames: Iterable<string>;
};

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

function packageContextNames(packages: Record<string, unknown>): Set<string> {
  const names = new Set<string>();
  for (const [key, value] of Object.entries(packages)) {
    if (isWorkspaceResolution(value) || !Array.isArray(value) || typeof value[0] !== "string") continue;
    const spec = specNameAndVersion(value[0]);
    if (spec !== undefined && spec.name !== key) names.add(spec.name);
  }
  return names;
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
 * the top-level package; a global override would collapse them, so those
 * names are omitted.
 */
export function lockedTransitiveOverrides(input: ConsumerLockInput): Record<string, string> {
  const parsed = parseJsonc(input.lockText) as BunLockFile;
  const packages = parsed.packages ?? {};
  const declared = new Set(input.declaredNames);
  const contextual = packageContextNames(packages);
  const overrides: Record<string, string> = {};
  for (const [key, value] of Object.entries(packages)) {
    if (isWorkspaceResolution(value) || !Array.isArray(value) || typeof value[0] !== "string") continue;
    const spec = specNameAndVersion(value[0]);
    if (spec === undefined || spec.name !== key) continue;
    if (skipForDeclared(key, declared) || contextual.has(key)) continue;
    overrides[key] = spec.version;
  }
  return overrides;
}

export function declaredPackageNames(
  dependencies: Record<string, string> = {},
  devDependencies: Record<string, string> = {},
): string[] {
  return [...Object.keys(dependencies), ...Object.keys(devDependencies)];
}
