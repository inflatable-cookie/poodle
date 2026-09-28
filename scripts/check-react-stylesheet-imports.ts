/**
 * Fail when a React source file renders a shared Poodle class without
 * importing the `@inflatable-cookie/poodle-core/styles/*.css` sheet that
 * defines it. The UiPresentationProvider case is the prototype: without
 * `ui-presentation-provider.css` the wrapper is a plain block.
 */

import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const STYLES_DIR = "packages/core/src/styles";
const REACT_SRC = "packages/react/components/src";
const CSS_IMPORT = /@inflatable-cookie\/poodle-core\/styles\/([A-Za-z0-9_-]+)\.css/g;
const CLASS_IN_CSS = /\.(poodle-[A-Za-z0-9_-]+)/g;
const CLASS_IN_SOURCE = /\bpoodle-[A-Za-z0-9_-]+/g;
const CSS_RELATIVE_IMPORT = /@import\s+["']\.\/([A-Za-z0-9_-]+)\.css["']/g;

export type StylesheetImportFailure = {
  file: string;
  className: string;
  stylesheets: string[];
};

function listFiles(dir: string, extension: string): string[] {
  const full = path.join(ROOT, dir);
  if (!fs.existsSync(full)) return [];
  const files: string[] = [];
  for (const entry of fs.readdirSync(full, { withFileTypes: true })) {
    const relative = `${dir}/${entry.name}`;
    if (entry.isDirectory()) files.push(...listFiles(relative, extension));
    else if (entry.isFile() && entry.name.endsWith(extension)) files.push(relative);
  }
  return files.sort();
}

function cssGraph(): { owners: Map<string, Set<string>>; imports: Map<string, string[]> } {
  const owners = new Map<string, Set<string>>();
  const imports = new Map<string, string[]>();
  for (const relative of listFiles(STYLES_DIR, ".css")) {
    const sheet = path.basename(relative, ".css");
    const source = fs.readFileSync(path.join(ROOT, relative), "utf8");
    imports.set(
      sheet,
      [...source.matchAll(CSS_RELATIVE_IMPORT)].map((match) => match[1]),
    );
    for (const match of source.matchAll(CLASS_IN_CSS)) {
      const className = match[1];
      let sheets = owners.get(className);
      if (!sheets) {
        sheets = new Set();
        owners.set(className, sheets);
      }
      sheets.add(sheet);
    }
  }
  return { owners, imports };
}

function importedSheetClosure(source: string, cssImports: Map<string, string[]>): Set<string> {
  const seen = new Set<string>();
  const stack = [...source.matchAll(CSS_IMPORT)].map((match) => match[1]);
  while (stack.length > 0) {
    const sheet = stack.pop();
    if (sheet === undefined || seen.has(sheet)) continue;
    seen.add(sheet);
    for (const child of cssImports.get(sheet) ?? []) stack.push(child);
  }
  return seen;
}

function takeBalanced(source: string, start: number, open: string, close: string): string {
  let depth = 1;
  let index = start;
  while (index < source.length && depth > 0) {
    const ch = source[index];
    if (ch === open) depth += 1;
    else if (ch === close) depth -= 1;
    index += 1;
  }
  return source.slice(start, index);
}

function renderedClasses(source: string): string[] {
  const classes = new Set<string>();
  const quoted = /className\s*=\s*(?:"([^"]*)"|'([^']*)')/g;
  for (const match of source.matchAll(quoted)) {
    for (const classMatch of (match[1] ?? match[2] ?? "").matchAll(CLASS_IN_SOURCE)) {
      classes.add(classMatch[0]);
    }
  }
  const expr = /className\s*=\s*\{/g;
  let match: RegExpExecArray | null;
  while ((match = expr.exec(source))) {
    const region = takeBalanced(source, expr.lastIndex, "{", "}");
    for (const classMatch of region.matchAll(CLASS_IN_SOURCE)) classes.add(classMatch[0]);
  }
  const assigned = /\.className\s*=\s*(?:"([^"]*)"|'([^']*)'|`([^`]*)`)/g;
  for (const assign of source.matchAll(assigned)) {
    for (const classMatch of (assign[1] ?? assign[2] ?? assign[3] ?? "").matchAll(CLASS_IN_SOURCE)) {
      classes.add(classMatch[0]);
    }
  }
  return [...classes];
}

export function collectReactStylesheetImportFailures(): StylesheetImportFailure[] {
  const { owners, imports } = cssGraph();
  const failures: StylesheetImportFailure[] = [];
  for (const relative of listFiles(REACT_SRC, ".tsx").concat(listFiles(REACT_SRC, ".ts"))) {
    const source = fs.readFileSync(path.join(ROOT, relative), "utf8");
    const imported = importedSheetClosure(source, imports);
    for (const className of renderedClasses(source)) {
      const sheets = owners.get(className);
      if (sheets === undefined) continue;
      if ([...sheets].some((sheet) => imported.has(sheet))) continue;
      failures.push({
        file: relative,
        className,
        stylesheets: [...sheets].sort(),
      });
    }
  }
  return failures;
}

function main(): void {
  const failures = collectReactStylesheetImportFailures();
  if (failures.length > 0) {
    const lines = failures.map(
      (failure) =>
        `${failure.file}: renders .${failure.className} without importing ${failure.stylesheets
          .map((sheet) => `@inflatable-cookie/poodle-core/styles/${sheet}.css`)
          .join(" or ")}`,
    );
    console.error(`React stylesheet import drift:\n${lines.join("\n")}`);
    process.exit(1);
  }
  console.log("React stylesheet imports: every rendered Poodle class imports its core sheet.");
}

if (import.meta.main) main();
