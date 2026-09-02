import { readdirSync, readFileSync, statSync } from "node:fs";
import { join, posix, relative } from "node:path";

const DECLARATION_SUFFIX = /\.d\.[cm]?ts$/;
const TIMESTAMP_KEY = /timestamp|builtAt|generatedAt|createdAt/i;
const ISO_DATE = /\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}/;
const ABSOLUTE_PATH = /(?:^|["'\s])(?:\/Users\/|\/home\/|\/tmp\/|[A-Za-z]:\\)/;
const FORBIDDEN_IMPORT =
  /from\s+["'](marked|svelte|svelte\/|react|react-dom)["']|require\(\s*["'](marked|svelte|react|react-dom)["']\s*\)/;

export type DistAuditOptions = {
  distDir: string;
  publicFiles: readonly string[];
  forbiddenModules: readonly string[];
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

export function auditStagedDist(options: DistAuditOptions): void {
  const files = walkFiles(options.distDir);
  const publicSet = new Set(options.publicFiles);
  const seenPublic = new Set<string>();

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

    if (distPath.endsWith(".js") || distPath.endsWith(".mjs")) {
      if (FORBIDDEN_IMPORT.test(bytes)) {
        throw new Error(`forbidden parser or shell module entered ${distPath}`);
      }
      for (const name of options.forbiddenModules) {
        if (bytes.includes(`from "${name}"`) || bytes.includes(`from '${name}'`)) {
          throw new Error(`${name} entered ${distPath}`);
        }
      }
      if (ABSOLUTE_PATH.test(bytes) && bytes.includes("/Users/")) {
        throw new Error(`workspace path leaked into ${distPath}`);
      }
    }
  }

  const missing = options.publicFiles.filter((path) => !seenPublic.has(path));
  if (missing.length > 0) {
    throw new Error(`missing staged public file(s): ${missing.join(", ")}`);
  }
}
