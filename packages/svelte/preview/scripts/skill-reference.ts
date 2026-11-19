/**
 * The installable Svelte implementer skill's generated reference.
 *
 * The skill is hand-written where judgment lives (routing, finish checks) and
 * generated where drift is the risk: the component/helper/rule inventory comes
 * from contracts, public package exports and the preview manifest. `docs:lint`
 * compares the committed reference with this builder so a changed contract or
 * export cannot silently leave the skill stale.
 */

import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

import { componentDocsMap } from "../src/component-docs";
import { docsNavigationSections, packageSurfaceCoverage } from "../src/parity";

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const previewDir = path.resolve(scriptDir, "..");
const repoRoot = path.resolve(previewDir, "../../..");
const contractsDir = path.join(repoRoot, "docs", "contracts");
const componentContractsDir = path.join(contractsDir, "components");
const skillDir = path.join(repoRoot, "skills", "poodle");
const skillPath = path.join(skillDir, "SKILL.md");
const referencePath = path.join(skillDir, "references", "reference.md");

const LAYER_ORDER = ["foundation", "primitives", "composites", "composite", "display"];

type ContractEntry = {
  slug: string;
  title: string;
  status: string;
  layer: string;
  names: string[];
  summary: string;
  note: string;
};

function readContract(slug: string): ContractEntry {
  const filePath = path.join(componentContractsDir, `${slug}.md`);
  const markdown = fs.readFileSync(filePath, "utf8");
  const lines = markdown.split(/\r?\n/);
  const title = (lines.find((line) => line.startsWith("# ")) ?? `# ${slug}`).slice(2).trim();
  const status = (markdown.match(/^Status:\s*(.+)$/m)?.[1] ?? "").trim();

  const layer = (markdown.match(/^- Layer:\s*`?([^`\n]+)`?\s*$/m)?.[1] ?? "").trim();

  const componentNameLine = markdown.match(/^- Component name:\s*(.+)$/m)?.[1] ?? "";
  const backtickedNames = Array.from(componentNameLine.matchAll(/`([^`]+)`/g), (match) => match[1]);
  const names = backtickedNames.length > 0
    ? backtickedNames
    : componentNameLine
        .split(",")
        .map((name) => name.trim())
        .filter(Boolean);

  let summary = "";
  const summaryStart = lines.findIndex((line) => /^- Summary:/.test(line));
  if (summaryStart >= 0) {
    const parts = [lines[summaryStart].replace(/^- Summary:\s*/, "")];
    for (let index = summaryStart + 1; index < lines.length; index += 1) {
      const line = lines[index];
      if (line.trim().length === 0 || /^-\s+/.test(line) || line.startsWith("#")) break;
      parts.push(line);
    }
    summary = parts.join(" ").replace(/\s+/g, " ").trim();
  }

  const noteLines: string[] = [];
  let inNote = false;
  for (const line of lines) {
    if (!line.startsWith(">")) {
      if (inNote) break;
      continue;
    }
    inNote = true;
    noteLines.push(line.replace(/^>\s?/, ""));
  }
  const note = noteLines
    .join(" ")
    .replace(/^\*\*Implementation note\*\*:\s*/, "")
    .replace(/\s+/g, " ")
    .trim();

  return { slug, title, status, layer, names, summary, note };
}

function contractSlugs(): string[] {
  return fs
    .readdirSync(componentContractsDir)
    .filter((entry) => entry.endsWith(".md") && entry !== "README.md")
    .map((entry) => entry.slice(0, -3))
    .sort();
}

function sectionTitle(sectionId: string): string {
  return docsNavigationSections.find((section) => section.id === sectionId)?.title ?? sectionId;
}

function previewLabel(exportNames: string[]): string {
  const entries = exportNames
    .map((name) => packageSurfaceCoverage.find((entry) => entry.exportName === name))
    .filter((entry): entry is (typeof packageSurfaceCoverage)[number] => Boolean(entry));

  if (entries.length === 0) return "none (shared contract)";
  if (entries.some((entry) => entry.status === "previewed")) {
    const sections = Array.from(
      new Set(entries.flatMap((entry) => entry.sectionIds)),
    ).map(sectionTitle);
    return sections.length > 0 ? sections.join(", ") : "previewed";
  }
  return "none (contract-only)";
}

function svelteLabel(exportNames: string[]): string {
  if (exportNames.length === 0) return "none";
  const kinds = Array.from(
    new Set(
      exportNames.map((name) => {
        const entry = packageSurfaceCoverage.find((candidate) => candidate.exportName === name);
        return entry?.kind === "helper" ? "helper" : "component";
      }),
    ),
  );
  const prefix = kinds.length === 1 && kinds[0] === "helper" ? "helper " : "";
  return `${prefix}${exportNames.map((name) => `\`${name}\``).join(", ")}`;
}

function contractEntryLine(contract: ContractEntry): string {
  const matches = packageSurfaceCoverage
    .filter((entry) => contract.names.includes(entry.exportName) && !entry.exportName.startsWith("type "))
    .map((entry) => entry.exportName);
  const summary = contract.summary || firstProseParagraph(fs.readFileSync(
    path.join(componentContractsDir, `${contract.slug}.md`),
    "utf8",
  )) || contract.note || contract.title;
  const details = [
    `contract \`docs/contracts/components/${contract.slug}.md\``,
    `Svelte ${svelteLabel(matches)}`,
    `specimen ${previewLabel(matches)}`,
  ];
  const status = contract.status.split(" — ")[0].trim();
  if (status) details.push(`status ${status}`);
  if (contract.note && /no standalone|utility function|compose this shape/i.test(contract.note)) {
    const note = contract.note.length > 180
      ? `${contract.note.slice(0, contract.note.lastIndexOf(" ", 180))}…`
      : contract.note;
    details.push(`note ${note}`);
  }
  return `- **${contract.names.join(" / ") || contract.title}** — ${summary} — ${details.join(" · ")}`;
}

function firstProseParagraph(markdown: string): string {
  const lines = markdown.split(/\r?\n/);
  const paragraph: string[] = [];
  const metadata = /^(?:Status|Updated|Owner|Applies to|Depends on):/;
  for (const rawLine of lines) {
    const line = rawLine.trim();
    if (line.length === 0) {
      if (paragraph.length > 0) break;
      continue;
    }
    if (
      line.startsWith("#") ||
      line.startsWith(">") ||
      line.startsWith("|") ||
      /^[-*]\s+/.test(line) ||
      /^\d+\.\s+/.test(line) ||
      metadata.test(line)
    ) {
      if (paragraph.length > 0) break;
      continue;
    }
    paragraph.push(line);
  }
  const text = paragraph.join(" ").replace(/\s+/g, " ").trim();
  if (text.length <= 220) return text;
  const sentence = text.slice(0, 220);
  const boundary = Math.max(sentence.lastIndexOf(". "), sentence.lastIndexOf("; "));
  return `${(boundary > 80 ? sentence.slice(0, boundary + 1) : sentence.slice(0, sentence.lastIndexOf(" "))).trim()}…`;
}

function sharedRuleLines(): string[] {
  const files = fs
    .readdirSync(contractsDir)
    .filter((entry) => /^\d{3}-.+\.md$/.test(entry))
    .sort();

  return files.map((file) => {
    const markdown = fs.readFileSync(path.join(contractsDir, file), "utf8");
    const title = (markdown.match(/^#\s+(.+)$/m)?.[1] ?? file).trim();
    const summary = firstProseParagraph(markdown);
    return `- **${title}** — \`docs/contracts/${file}\`${summary ? ` — ${summary}` : ""}`;
  });
}

function packageSubpathLines(): string[] {
  const packages = [
    ["packages/svelte/components/package.json", "@inflatable-cookie/poodle-svelte"],
    ["packages/core/package.json", "@inflatable-cookie/poodle-core"],
  ] as const;

  const lines: string[] = [];
  for (const [manifestPath, packageName] of packages) {
    const manifest = JSON.parse(fs.readFileSync(path.join(repoRoot, manifestPath), "utf8")) as {
      exports?: Record<string, unknown>;
    };
    const keys = Object.keys(manifest.exports ?? {}).sort();
    for (const key of keys) {
      const specifier = key === "." ? packageName : `${packageName}/${key.replace(/^\.\//, "")}`;
      lines.push(`- \`${specifier}\``);
    }
  }
  return lines;
}

function helperGroups(): string[] {
  const groups = new Map<
    string,
    { note: string; status: string; sectionIds: string[]; exportNames: string[] }
  >();

  for (const entry of packageSurfaceCoverage) {
    if (entry.kind !== "helper") continue;
    const key = `${entry.status}|${entry.sectionIds.join(",")}|${entry.note}`;
    const group = groups.get(key) ?? {
      note: entry.note,
      status: entry.status,
      sectionIds: [...entry.sectionIds],
      exportNames: [],
    };
    if (entry.exportName.startsWith("type ")) continue;
    group.exportNames.push(entry.exportName);
    groups.set(key, group);
  }

  return Array.from(groups.values())
    .filter((group) => group.exportNames.length > 0)
    .sort((left, right) => left.exportNames[0].localeCompare(right.exportNames[0]))
    .map((group) => {
      const scope = group.status === "previewed"
        ? `specimen ${group.sectionIds.map(sectionTitle).join(", ") || "previewed"}`
        : "contract-only";
      return `- ${group.exportNames.map((name) => `\`${name}\``).join(", ")} — ${group.note} — ${scope}`;
    });
}

export function buildSkillReference(): string {
  const contracts = contractSlugs().map(readContract);
  const byLayer = new Map<string, ContractEntry[]>();
  for (const contract of contracts) {
    const layer = contract.layer || "unlayered";
    const list = byLayer.get(layer) ?? [];
    list.push(contract);
    byLayer.set(layer, list);
  }

  const layers = Array.from(byLayer.keys()).sort((left, right) => {
    const leftIndex = LAYER_ORDER.indexOf(left);
    const rightIndex = LAYER_ORDER.indexOf(right);
    if (leftIndex >= 0 && rightIndex >= 0) return leftIndex - rightIndex;
    if (leftIndex >= 0) return -1;
    if (rightIndex >= 0) return 1;
    return left.localeCompare(right);
  });

  const componentExports = packageSurfaceCoverage.filter((entry) => entry.kind === "component");
  const helperExports = packageSurfaceCoverage.filter((entry) => entry.kind === "helper");
  const helperCount = new Set(
    helperExports
      .map((entry) => entry.exportName)
      .filter((name) => !name.startsWith("type ")),
  ).size;
  const previewed = componentExports.filter((entry) => entry.status === "previewed").length;
  const contractOnly = componentExports.length - previewed;
  const previewManifestCount = Object.keys(componentDocsMap).length;
  const contractsWithoutPreviewEntry = contracts.filter(
    (contract) => !(contract.slug in componentDocsMap),
  ).length;
  const contractsWithExport = contracts.filter((contract) =>
    contract.names.some((name) => packageSurfaceCoverage.some((entry) => entry.exportName === name)),
  ).length;

  const lines: string[] = [
    "# Poodle Svelte reference",
    "",
    "Generated by `effigy skill:reference` from `docs/contracts`, the public",
    "package exports and the preview manifest. Do not edit by hand.",
    "",
    `Contracts: ${contracts.length} · preview manifest entries: ${previewManifestCount}` +
      ` (${contractsWithoutPreviewEntry} contracts have none) · Svelte components: ${componentExports.length}` +
      ` (${previewed} previewed, ${contractOnly} contract-only) · helper exports: ${helperCount}` +
      ` · contracts with a Svelte export: ${contractsWithExport}`,
    "",
    "Read the consumer's pinned `@inflatable-cookie/poodle-svelte` version first.",
    "This file describes the checkout that generated it; the installed package is",
    "the authority for which exports exist at the pinned version.",
    "",
    "Resolve each repository path below with the consumer's pinned release tag:",
    "`https://github.com/inflatable-cookie/poodle/blob/v<VERSION>/<path>`.",
    "",
    "## Components",
    "",
    "Every entry is one contract. `Svelte none` means the contract is shared with",
    "native renderers or is composed by the caller; `specimen none (contract-only)`",
    "means the export exists but the preview harness does not exercise it.",
    "",
  ];

  for (const layer of layers) {
    lines.push(`### ${layer}`);
    lines.push("");
    for (const contract of byLayer.get(layer) ?? []) {
      lines.push(contractEntryLine(contract));
    }
    lines.push("");
  }

  lines.push("## Helpers");
  lines.push("");
  lines.push("Public helper exports from `@inflatable-cookie/poodle-svelte`, grouped");
  lines.push("as the preview manifest groups them.");
  lines.push("");
  lines.push(...helperGroups());
  lines.push("");
  lines.push("### Package subpaths");
  lines.push("");
  lines.push("Import subpaths present in the published package manifests.");
  lines.push("");
  lines.push(...packageSubpathLines());
  lines.push("");
  lines.push("## Shared rules");
  lines.push("");
  lines.push("Cross-cutting contracts that bind more than one component. Read the");
  lines.push("rule once instead of restating it in application code.");
  lines.push("");
  lines.push(...sharedRuleLines());
  lines.push("");
  lines.push("Component index: `docs/contracts/components/README.md`.");
  lines.push("");

  return lines.join("\n");
}

function expect(condition: boolean, message: string, errors: string[]): void {
  if (!condition) errors.push(message);
}

function validSubpath(packageName: string, subpath: string): boolean {
  const manifestPath = packageName === "@inflatable-cookie/poodle-svelte"
    ? "packages/svelte/components/package.json"
    : "packages/core/package.json";
  const manifest = JSON.parse(fs.readFileSync(path.join(repoRoot, manifestPath), "utf8")) as {
    exports?: Record<string, unknown>;
  };
  const keys = Object.keys(manifest.exports ?? {});
  if (subpath.length === 0) return keys.includes(".");
  const target = `./${subpath}`;
  return keys.some((key) => {
    if (key === target) return true;
    if (!key.includes("*")) return false;
    const pattern = new RegExp(`^${key.replace(/[.+?^${}()|[\]\\]/g, "\\$&").replace(/\*/g, ".*")}$`);
    return pattern.test(target);
  });
}

function localLinkTargets(markdown: string): string[] {
  const targets: string[] = [];
  for (const match of markdown.matchAll(/!?\[[^\]]*\]\(([^)]+)\)/g)) {
    const raw = match[1].trim().replace(/^<|>$/g, "");
    if (/^(?:https?:|mailto:|#)/.test(raw)) continue;
    targets.push(decodeURIComponent(raw.split("#", 1)[0]));
  }
  return targets;
}

function pathCandidates(markdown: string): string[] {
  const candidates = new Set<string>(localLinkTargets(markdown));
  for (const match of markdown.matchAll(/`(docs\/[^`\s]+\.md)`/g)) candidates.add(match[1]);
  for (const match of markdown.matchAll(/\(?(docs\/[a-z0-9./-]+\.md)\)?/g)) candidates.add(match[1]);
  return Array.from(candidates);
}

export function skillReferenceDriftErrors(): string[] {
  const errors: string[] = [];

  expect(fs.existsSync(skillPath), "skills/poodle/SKILL.md is missing.", errors);
  expect(fs.existsSync(referencePath), "skills/poodle/references/reference.md is missing.", errors);
  if (errors.length > 0) return errors;

  const skill = fs.readFileSync(skillPath, "utf8");
  const reference = fs.readFileSync(referencePath, "utf8");
  const generated = buildSkillReference();

  expect(
    reference === generated,
    "skills/poodle/references/reference.md is stale; run `effigy skill:reference`.",
    errors,
  );

  expect(
    /^---\nname: poodle\n/m.test(skill),
    "skills/poodle/SKILL.md must declare `name: poodle` in its frontmatter.",
    errors,
  );
  expect(
    /^description: .+/m.test(skill),
    "skills/poodle/SKILL.md must declare a description.",
    errors,
  );
  expect(
    skill.includes("node_modules/@inflatable-cookie/poodle-svelte/package.json"),
    "skills/poodle/SKILL.md must read the consumer's pinned package version.",
    errors,
  );
  expect(
    skill.includes("blob/v${VERSION}/docs/contracts/components/"),
    "skills/poodle/SKILL.md must build contract links from the pinned release tag.",
    errors,
  );

  for (const [label, markdown] of [
    ["skills/poodle/SKILL.md", skill],
    ["skills/poodle/references/reference.md", reference],
  ] as const) {
    expect(
      !/github\.com\/inflatable-cookie\/poodle\/blob\/(?:main|master)\//.test(markdown),
      `${label} must use a release-tagged contract link, not main or master.`,
      errors,
    );

    for (const target of pathCandidates(markdown)) {
      if (target.length === 0) continue;
      expect(
        fs.existsSync(path.resolve(path.dirname(skillDir), target)) ||
          fs.existsSync(path.join(repoRoot, target)),
        `${label} points at missing repository path ${target}.`,
        errors,
      );
    }

    for (const match of markdown.matchAll(
      /@inflatable-cookie\/poodle-(svelte|core)((?:\/[A-Za-z0-9._*-]+)*)/g,
    )) {
      const packageName = `@inflatable-cookie/poodle-${match[1]}`;
      const subpath = (match[2] ?? "").replace(/^\//, "");
      // A filesystem read of the installed manifest is not an import.
      if (subpath.endsWith("package.json")) continue;
      expect(
        validSubpath(packageName, subpath),
        `${label} imports ${packageName}${subpath ? `/${subpath}` : ""}, which the package manifest does not export.`,
        errors,
      );
    }

  }

  // The decision table is the skill's routing claim. Every recipe and
  // contract slug it names must exist, so a rename cannot leave a dead route.
  const knownContracts = new Set(contractSlugs());
  const knownRecipes = new Set(
    fs
      .readdirSync(path.join(repoRoot, "docs", "guides"))
      .filter((file) => /^\d{3}-.*-recipes\.md$/.test(file)),
  );
  for (const line of skill.split(/\r?\n/)) {
    if (!/^\|.*\|$/.test(line)) continue;
    const cells = line.split("|").slice(1, -1).map((cell) => cell.trim());
    if (cells.length < 3 || cells[0] === "View requirement" || cells[0].startsWith("---")) continue;
    for (const match of cells[1].matchAll(/`([^`]+)`/g)) {
      expect(
        knownRecipes.has(match[1]),
        `skills/poodle/SKILL.md decision table names unknown recipe \`${match[1]}\`.`,
        errors,
      );
    }
    for (const match of cells[2].matchAll(/`([^`]+)`/g)) {
      expect(
        knownContracts.has(match[1]),
        `skills/poodle/SKILL.md decision table names unknown contract \`${match[1]}\`.`,
        errors,
      );
    }
  }

  return errors;
}
