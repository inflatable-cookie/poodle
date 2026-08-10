import { readdirSync, readFileSync, statSync } from "node:fs";
import { join, relative, resolve } from "node:path";

type PatternCheck = {
  id: string;
  label: string;
  regex: RegExp;
};

type FileFinding = {
  file: string;
  matches: string[];
};

const componentRoot = resolve(import.meta.dir, "..", "src");
const repoRoot = resolve(import.meta.dir, "../../../..");
const contractRoot = join(repoRoot, "docs/contracts/components");
const previewRoot = join(repoRoot, "packages/svelte/preview/src");
const json = process.argv.includes("--json");

const patternChecks: PatternCheck[] = [
  {
    id: "create-event-dispatcher",
    label: "uses createEventDispatcher",
    regex: /\bcreateEventDispatcher\b/,
  },
  {
    id: "export-let",
    label: "uses export let props",
    regex: /\bexport let\b/,
  },
  {
    id: "reactive-label",
    label: "uses $: reactive labels",
    regex: /(^|\n)\s*\$:/,
  },
  {
    id: "legacy-slots",
    label: "uses legacy slots",
    regex: /<slot\b|\$\$slots/,
  },
  {
    id: "legacy-events",
    label: "uses on: event directives",
    regex: /\bon:/,
  },
  {
    id: "module-context",
    label: "uses <script context=\"module\">",
    regex: /<script\s+context="module"/,
  },
];

const findings = walk(componentRoot)
  .filter((file) => file.endsWith(".svelte"))
  .map((file) => inspectFile(file))
  .filter((finding): finding is FileFinding => finding !== null);

const summary = patternChecks.map((check) => ({
  id: check.id,
  label: check.label,
  files: findings.filter((finding) => finding.matches.includes(check.id)).length,
}));

const totalFiles = walk(componentRoot).filter((file) => file.endsWith(".svelte")).length;
const modernFiles = totalFiles - findings.length;
const topFiles = findings
  .slice()
  .sort((a, b) => b.matches.length - a.matches.length || a.file.localeCompare(b.file))
  .slice(0, 12);
const coverage = inspectCoverage();

if (json) {
  console.log(
    JSON.stringify(
      {
        totalFiles,
        modernFiles,
        summary,
        topFiles,
        coverage,
      },
      null,
      2,
    ),
  );
  process.exit(0);
}

console.log("# Svelte Surface Audit");
console.log("");
console.log(`- total component files: ${totalFiles}`);
console.log(`- files with no tracked legacy markers: ${modernFiles}`);
console.log(`- files with one or more tracked legacy markers: ${findings.length}`);
console.log("");
console.log("## Pattern Summary");
console.log("");
for (const item of summary) {
  console.log(`- ${item.label}: ${item.files} files`);
}
console.log("");
console.log("## Highest Legacy Density");
console.log("");
for (const finding of topFiles) {
  const labels = finding.matches
    .map((match) => patternChecks.find((check) => check.id === match)?.label ?? match)
    .join("; ");
  console.log(`- ${finding.file}: ${finding.matches.length} markers (${labels})`);
}
console.log("");
console.log("## Public Surface Coverage");
console.log("");
console.log(`- public component exports: ${coverage.totalExports}`);
console.log(`- components with full coverage: ${coverage.coveredExports}`);
console.log(`- components with coverage gaps: ${coverage.gaps.length}`);
if (coverage.gaps.length > 0) {
  console.log("");
  for (const gap of coverage.gaps) {
    console.log(`- ${gap.name} (${gap.slug}): ${gap.missing.join(", ")}`);
  }
  process.exitCode = 1;
}

function inspectFile(file: string): FileFinding | null {
  const source = readFileSync(file, "utf8");
  const matches = patternChecks
    .filter((check) => check.regex.test(source))
    .map((check) => check.id);

  if (matches.length === 0) {
    return null;
  }

  return {
    file: relative(componentRoot, file),
    matches,
  };
}

function walk(dir: string): string[] {
  const entries = readdirSync(dir).sort((a, b) => a.localeCompare(b));
  const files: string[] = [];

  for (const entry of entries) {
    const path = join(dir, entry);
    const stat = statSync(path);
    if (stat.isDirectory()) {
      files.push(...walk(path));
      continue;
    }
    files.push(path);
  }

  return files;
}

function inspectCoverage() {
  const indexSource = readFileSync(join(componentRoot, "index.ts"), "utf8");
  const exports = Array.from(
    indexSource.matchAll(/export\s+\{\s+default\s+as\s+(\w+)\s+\}\s+from\s+"\.\/(\w+)\.svelte";/g),
  ).map((match) => ({
    name: match[1],
    slug: toSlug(match[1]),
  }));

  const contractSlugs = new Set(
    readdirSync(contractRoot)
      .filter((file) => file.endsWith(".md"))
      .map((file) => file.replace(/\.md$/, "")),
  );
  const specimenRegistrySource = readFileSync(join(previewRoot, "specimens/registry.ts"), "utf8");
  const specimenSlugs = new Set(
    Array.from(
      specimenRegistrySource.matchAll(/^[ \t]*(?:"([a-z0-9-]+)"|([a-z][a-z0-9]*)):\s*\w+Specimen,/gm),
    ).map((match) => match[1] ?? match[2]),
  );
  const componentRegistrySource = readFileSync(join(previewRoot, "component-registry.ts"), "utf8");
  const componentRegistrySlugs = new Set(
    Array.from(componentRegistrySource.matchAll(/entry\(\s*"([^"]+)"/g)).map((match) => toSlug(match[1])),
  );
  const docsSource = readFileSync(join(previewRoot, "component-docs.ts"), "utf8");
  const usageDocSlugs = new Set(
    Array.from(docsSource.matchAll(/^[ \t]{2}(?:"([a-z0-9-]+)"|([a-z][a-z0-9]*)):\s*\{/gm)).map(
      (match) => match[1] ?? match[2],
    ),
  );

  const gaps = exports
    .map((component) => {
      const missing: string[] = [];
      if (!contractSlugs.has(component.slug)) missing.push("contract");
      if (!componentRegistrySlugs.has(component.slug)) missing.push("component registry");
      if (!specimenSlugs.has(component.slug)) missing.push("specimen registry");
      if (!usageDocSlugs.has(component.slug)) missing.push("usage docs");
      return {
        ...component,
        missing,
      };
    })
    .filter((component) => component.missing.length > 0);

  return {
    totalExports: exports.length,
    coveredExports: exports.length - gaps.length,
    gaps,
  };
}

function toSlug(name: string): string {
  return name
    .replace(/([A-Z]+)([A-Z][a-z])/g, "$1-$2")
    .replace(/([a-z])([A-Z])/g, "$1-$2")
    .toLowerCase();
}
