import { readdirSync, readFileSync, statSync } from "node:fs";
import { join, posix, relative } from "node:path";

const DECLARATION_SUFFIX = /\.d\.[cm]?ts$/;
const TIMESTAMP_KEY = /timestamp|builtAt|generatedAt|createdAt/i;
const ISO_DATE = /\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}/;
const ABSOLUTE_PATH = /(?:\/(?:Users|home|tmp)\/|[A-Za-z]:(?:\\+|\/(?!\/)))/;
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

function sourceWithoutComments(bytes: string): string {
  return bytes.replace(/\/\*[\s\S]*?\*\//g, "").replace(/^\s*\/\/.*$/gm, "");
}

function toDistPath(distDir: string, abs: string): string {
  return posix.join("dist", relative(distDir, abs).split("\\").join("/"));
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
    for (const name of forbiddenModules) {
      if (Object.hasOwn(deps, name)) {
        throw new Error(`package.json ${section} lists forbidden module ${name}`);
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
      if (ABSOLUTE_PATH.test(bytes)) {
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
      if (ABSOLUTE_PATH.test(sourceWithoutComments(bytes))) {
        throw new Error(`workspace path leaked into ${distPath}`);
      }
    }
  }

  const missing = options.publicFiles.filter((path) => !seenPublic.has(path));
  if (missing.length > 0) {
    throw new Error(`missing staged public file(s): ${missing.join(", ")}`);
  }
}
