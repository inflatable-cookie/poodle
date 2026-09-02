import { readdirSync, readFileSync, statSync } from "node:fs";
import { join, posix, relative } from "node:path";

const DECLARATION_SUFFIX = /\.d\.[cm]?ts$/;
const TIMESTAMP_KEY = /timestamp|builtAt|generatedAt|createdAt/i;
const ISO_DATE = /\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}/;
const MANIFEST_DEP_SECTIONS = [
  "dependencies",
  "devDependencies",
  "peerDependencies",
  "optionalDependencies",
] as const;

export type DistAuditOptions = {
  distDir: string;
  publicFiles: readonly string[];
  forbiddenModules: readonly string[];
  moduleIds?: readonly string[];
  specifiers?: readonly string[];
};

function walkFiles(root: string): string[] {
  const files: string[] = [];
  const visit = (directory: string) => {
    for (const entry of readdirSync(directory).sort()) {
      const full = join(directory, entry);
      if (statSync(full).isDirectory()) visit(full);
      else files.push(full);
    }
  };
  visit(root);
  return files;
}

function toDistPath(distDir: string, abs: string): string {
  return posix.join("dist", relative(distDir, abs).split("\\").join("/"));
}

function isAbsolutePathValue(value: string): boolean {
  if (value === "/" || value === "/*") return false;
  if (/[<>]/.test(value)) return false;
  if (/^\/\/[^/\r\n]+/.test(value)) return false;
  if (/^\//.test(value)) return true;
  return (
    /^[A-Za-z]:[\\/]/.test(value) ||
    /^\\\\[^\\\r\n]+\\[^\\\r\n]+/.test(value) ||
    /^file:(?:\/\/)?\//i.test(value)
  );
}

function decodeEscape(source: string, index: number): { value: string; next: number } {
  const escaped = source[index + 1];
  if (escaped === undefined) return { value: "\\", next: index + 1 };
  const simple: Record<string, string> = {
    "0": "\0",
    b: "\b",
    f: "\f",
    n: "\n",
    r: "\r",
    t: "\t",
    v: "\v",
  };
  if (escaped in simple) return { value: simple[escaped] ?? "", next: index + 2 };
  if (escaped === "\n") return { value: "", next: index + 2 };
  if (escaped === "\r") {
    return { value: "", next: source[index + 2] === "\n" ? index + 3 : index + 2 };
  }
  if (escaped === "x") {
    const digits = source.slice(index + 2, index + 4);
    if (/^[0-9A-Fa-f]{2}$/.test(digits)) {
      return { value: String.fromCodePoint(Number.parseInt(digits, 16)), next: index + 4 };
    }
  }
  if (escaped === "u") {
    if (source[index + 2] === "{") {
      const close = source.indexOf("}", index + 3);
      const digits = close === -1 ? "" : source.slice(index + 3, close);
      if (/^[0-9A-Fa-f]{1,6}$/.test(digits)) {
        const codePoint = Number.parseInt(digits, 16);
        if (codePoint <= 0x10ffff) {
          return { value: String.fromCodePoint(codePoint), next: close + 1 };
        }
      }
    }
    const digits = source.slice(index + 2, index + 6);
    if (/^[0-9A-Fa-f]{4}$/.test(digits)) {
      return { value: String.fromCodePoint(Number.parseInt(digits, 16)), next: index + 6 };
    }
  }
  return { value: escaped, next: index + 2 };
}

function quotedValues(source: string): string[] {
  const values: string[] = [];
  let index = 0;
  while (index < source.length) {
    const current = source[index];
    const next = source[index + 1];
    if (current === "/" && next === "/") {
      index = source.indexOf("\n", index + 2);
      if (index === -1) break;
      continue;
    }
    if (current === "/" && next === "*") {
      const end = source.indexOf("*/", index + 2);
      index = end === -1 ? source.length : end + 2;
      continue;
    }
    if (current !== '"' && current !== "'" && current !== "`") {
      index += 1;
      continue;
    }
    const quote = current;
    let value = "";
    index += 1;
    while (index < source.length) {
      const character = source[index];
      if (character === "\\" && index + 1 < source.length) {
        const decoded = decodeEscape(source, index);
        value += decoded.value;
        index = decoded.next;
        continue;
      }
      if (character === quote) {
        index += 1;
        break;
      }
      value += character;
      index += 1;
    }
    values.push(value);
  }
  return values;
}

function absolutePathInSource(source: string): string | null {
  return quotedValues(source).find(isAbsolutePathValue) ?? null;
}

function absolutePathInJson(value: unknown): string | null {
  if (typeof value === "string") return isAbsolutePathValue(value) ? value : null;
  if (Array.isArray(value)) {
    for (const member of value) {
      const hit = absolutePathInJson(member);
      if (hit) return hit;
    }
    return null;
  }
  if (value && typeof value === "object") {
    for (const member of Object.values(value)) {
      const hit = absolutePathInJson(member);
      if (hit) return hit;
    }
  }
  return null;
}

function npmAliasTarget(specifier: unknown): string | null {
  if (typeof specifier !== "string" || !specifier.startsWith("npm:")) return null;
  const target = specifier.slice(4);
  if (target.startsWith("@")) {
    const slash = target.indexOf("/");
    if (slash === -1) return target;
    const version = target.indexOf("@", slash + 1);
    return version === -1 ? target : target.slice(0, version);
  }
  const version = target.indexOf("@");
  return version === -1 ? target : target.slice(0, version);
}

function escapeRegExp(value: string): string {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

export function specifierMentionsModule(bytes: string, name: string): boolean {
  const body = `${escapeRegExp(name)}(?:/[^"'\\s]*)?`;
  return new RegExp(
    [
      `from\\s+["']${body}["']`,
      `import\\s*\\(\\s*["']${body}["']`,
      `require\\s*\\(\\s*["']${body}["']`,
      `import\\s+["']${body}["']`,
    ].join("|"),
  ).test(bytes);
}

export function forbiddenGraphHit(
  forbiddenModules: readonly string[],
  moduleIds: readonly string[],
  specifiers: readonly string[],
): string | null {
  for (const name of forbiddenModules) {
    for (const specifier of specifiers) {
      if (specifier === name || specifier.startsWith(`${name}/`)) return specifier;
    }
    const needle = `/node_modules/${name}`;
    for (const id of moduleIds) {
      const normalized = id.replace(/\\/g, "/");
      if (
        normalized.includes(`${needle}/`) ||
        normalized.endsWith(needle) ||
        normalized.includes(`/node_modules/${name}@`)
      ) {
        return name;
      }
    }
  }
  return null;
}

export function auditPackageDependencies(
  manifest: Record<string, unknown>,
  forbiddenModules: readonly string[],
): void {
  for (const section of MANIFEST_DEP_SECTIONS) {
    const deps = manifest[section];
    if (!deps || typeof deps !== "object" || Array.isArray(deps)) continue;
    for (const [dependencyName, dependencySpecifier] of Object.entries(deps)) {
      const aliasTarget = npmAliasTarget(dependencySpecifier);
      for (const name of forbiddenModules) {
        if (dependencyName === name || aliasTarget === name) {
          const detail = aliasTarget ? `${dependencyName} aliases ${aliasTarget}` : name;
          throw new Error(`package.json ${section} lists forbidden module ${detail}`);
        }
      }
    }
  }
}

export function auditStagedDist(options: DistAuditOptions): void {
  const files = walkFiles(options.distDir);
  const publicSet = new Set(options.publicFiles);
  const seenPublic = new Set<string>();
  const graphHit = forbiddenGraphHit(
    options.forbiddenModules,
    options.moduleIds ?? [],
    options.specifiers ?? [],
  );
  if (graphHit) {
    throw new Error(`forbidden parser or shell module entered the module graph: ${graphHit}`);
  }

  for (const abs of files) {
    const distPath = toDistPath(options.distDir, abs);
    const basename = distPath.split("/").at(-1) ?? distPath;

    if (distPath.endsWith(".map")) {
      throw new Error(`source map is forbidden in staging: ${distPath}`);
    }
    if (/\.(tsx?|svelte)$/.test(distPath) && !DECLARATION_SUFFIX.test(distPath)) {
      throw new Error(`raw source is forbidden in staging: ${distPath}`);
    }
    const hashed = basename.match(/-([A-Za-z0-9]{8})\.[A-Za-z0-9]+$/);
    if (hashed && /[0-9]/.test(hashed[1])) {
      throw new Error(`hashed filename is forbidden: ${distPath}`);
    }

    const bytes = readFileSync(abs, "utf8");
    if (distPath === "dist/.poodle-build.json") {
      if (TIMESTAMP_KEY.test(bytes) || ISO_DATE.test(bytes)) {
        throw new Error("receipt contains a timestamp");
      }
      let receipt: unknown;
      try {
        receipt = JSON.parse(bytes);
      } catch {
        throw new Error("receipt is not valid JSON");
      }
      if (absolutePathInJson(receipt)) {
        throw new Error("receipt contains an absolute path");
      }
      continue;
    }

    if (publicSet.has(distPath)) {
      seenPublic.add(distPath);
    } else if (distPath.startsWith("dist/chunks/") && distPath.endsWith(".js")) {
      // shared chunks with stable [name] templates
    } else if (DECLARATION_SUFFIX.test(distPath)) {
      // implementation declarations backing public .d.ts re-exports
    } else {
      throw new Error(`unexpected staged file ${distPath}`);
    }

    const inspectText =
      distPath.endsWith(".js") ||
      distPath.endsWith(".mjs") ||
      DECLARATION_SUFFIX.test(distPath);
    if (inspectText) {
      for (const name of options.forbiddenModules) {
        if (specifierMentionsModule(bytes, name)) {
          throw new Error(`forbidden parser or shell module entered ${distPath}: ${name}`);
        }
      }
      const pathHit = absolutePathInSource(bytes);
      if (pathHit) {
        throw new Error(`workspace path leaked into ${distPath}: ${pathHit}`);
      }
    }
  }

  const missing = options.publicFiles.filter((path) => !seenPublic.has(path));
  if (missing.length > 0) {
    throw new Error(`missing staged public file(s): ${missing.join(", ")}`);
  }
}
