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

if (json) {
  console.log(
    JSON.stringify(
      {
        totalFiles,
        modernFiles,
        summary,
        topFiles,
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
