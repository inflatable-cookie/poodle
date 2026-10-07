import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { docsFamilies, docsSections } from "../src/catalog";
import {
  buildPreviewUrl,
  docsNavigationSections,
  packageSurfaceCoverage,
  parityTargets,
} from "../src/parity";
import { accessibilityAuditTargets } from "../src/accessibility";
import { containerQueryDriftErrors } from "./container-query-drift";
import { contractCallbackDrift } from "./contract-callback-drift";
import { contractDriftErrors, contractPropDrift } from "./contract-prop-drift";
import { contractSpecDrift } from "./contract-spec-drift";
import { focusRingDriftErrors } from "./focus-ring-drift";
import { PORTABLE_ROUTE_COUNT, ROSTER_WEB_ONLY_NAMES } from "../../../../scripts/component-denominator";

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const previewDir = path.resolve(scriptDir, "..");
const repoRoot = path.resolve(previewDir, "../../..");
const contractsDir = path.join(repoRoot, "docs", "contracts");
const releaseManifestPath = path.join(repoRoot, "packages", "release-manifest.json");
const releaseOperationsPath = path.join(repoRoot, "packages", "release-operations.json");
const ecosystemAcceptancePath = path.join(repoRoot, "packages", "ecosystem-acceptance.json");
const referenceAppsPath = path.join(repoRoot, "packages", "reference-apps.json");
const gpuiPreviewBaselinePath = path.join(repoRoot, "packages", "gpui", "preview-app-baseline.json");
const gpuiNativeAccessibilityProofPath = path.join(
  repoRoot,
  "packages",
  "gpui",
  "native-accessibility-proof.json",
);
const gpuiCrossRuntimeParityReportPath = path.join(
  repoRoot,
  "packages",
  "gpui",
  "cross-runtime-parity-report.json",
);
const sharedDemoAppAuditPath = path.join(repoRoot, "packages", "shared-demo-app-audit.json");
const sharedDemoAppContractPath = path.join(repoRoot, "packages", "shared-demo-app-contract.json");
const gpuiAdapterCrateName = "poodle-gpui";
const gpuiAdapterCratePath = "packages/gpui/adapter";
const gpuiTokenSource = "poodle-tokens";

function collectMarkdownFiles(directory: string): string[] {
  return fs
    .readdirSync(directory, { withFileTypes: true })
    .flatMap((entry) => {
      const entryPath = path.join(directory, entry.name);
      if (entry.isDirectory()) {
        return collectMarkdownFiles(entryPath);
      }

      return entry.isFile() && entry.name.endsWith(".md") ? [entryPath] : [];
    })
    .sort();
}

function parseBulletList(markdown: string, heading: string): string[] {
  const lines = markdown.split(/\r?\n/);
  const items: string[] = [];
  let inSection = false;

  for (const line of lines) {
    const trimmed = line.trim();

    if (!inSection) {
      if (trimmed === heading) {
        inSection = true;
      }
      continue;
    }

    if (trimmed.startsWith("## ")) {
      break;
    }

    const match = trimmed.match(/^- `([^`]+)`$/);
    if (match) {
      items.push(match[1]);
    }
  }

  return items;
}

function parseLooseBulletList(markdown: string, heading: string): string[] {
  const lines = markdown.split(/\r?\n/);
  const items: string[] = [];
  let inSection = false;

  for (const line of lines) {
    const trimmed = line.trim();

    if (!inSection) {
      if (trimmed === heading) {
        inSection = true;
      }
      continue;
    }

    if (trimmed.startsWith("## ")) {
      break;
    }

    const match = trimmed.match(/^- (.+)$/);
    if (match) {
      items.push(match[1]);
    }
  }

  return items;
}

function pascalToKebabCase(value: string): string {
  return value
    .replace(/([a-z0-9])([A-Z])/g, "$1-$2")
    .replace(/([A-Z])([A-Z][a-z])/g, "$1-$2")
    .toLowerCase();
}

function parseDefaultComponentExports(source: string): string[] {
  return Array.from(
    source.matchAll(/^export \{ default as (\w+) \} from "\.\/.+\.svelte";$/gm),
    (match) => match[1],
  ).sort();
}

function parseNamedRootExports(source: string): string[] {
  return Array.from(source.matchAll(/^export \{([^}]+)\} from ".+";$/gm))
    .flatMap((match) =>
      match[1]
        .split(",")
        .map((entry) => entry.trim())
        .filter((entry) => entry.length > 0 && !entry.includes("default as")),
    )
    .sort();
}

function expect(condition: boolean, message: string, errors: string[]): void {
  if (!condition) {
    errors.push(message);
  }
}

function compareLists(
  label: string,
  actual: string[],
  expected: string[],
  errors: string[],
): void {
  const actualSet = new Set(actual);
  const expectedSet = new Set(expected);
  const missing = expected.filter((item) => !actualSet.has(item));
  const unexpected = actual.filter((item) => !expectedSet.has(item));

  if (missing.length > 0) {
    errors.push(`${label} is missing: ${missing.join(", ")}`);
  }

  if (unexpected.length > 0) {
    errors.push(`${label} has unexpected entries: ${unexpected.join(", ")}`);
  }
}

function resolveContractRoots(contractRoot: string): string[] {
  return contractRoot
    .split(/\s+\+\s+/)
    .map((value) => value.trim())
    .filter(Boolean);
}

function parseNumberedHeadingSequence(markdown: string): number[] {
  return Array.from(markdown.matchAll(/^##\s+(\d+)\.\s+.+$/gm), (match) => Number(match[1]));
}

function parseCargoPoodleMetadata(source: string): {
  name: string | null;
  publicIntent: boolean | null;
  channel: string | null;
  stability: string | null;
} {
  const nameMatch = source.match(/^name = "([^"]+)"$/m);
  const publicIntentMatch = source.match(/^public-intent = (true|false)$/m);
  const channelMatch = source.match(/^channel = "([^"]+)"$/m);
  const stabilityMatch = source.match(/^stability = "([^"]+)"$/m);

  return {
    name: nameMatch?.[1] ?? null,
    publicIntent:
      publicIntentMatch?.[1] === "true"
        ? true
        : publicIntentMatch?.[1] === "false"
          ? false
          : null,
    channel: channelMatch?.[1] ?? null,
    stability: stabilityMatch?.[1] ?? null,
  };
}

function validateComponentContracts(errors: string[]): number {
  const componentContractFiles = [
    ...collectMarkdownFiles(path.join(contractsDir, "components")).filter((file) => !file.endsWith("README.md")),
  ];

  for (const filePath of componentContractFiles) {
    const markdown = fs.readFileSync(filePath, "utf8");
    const relativePath = path.relative(repoRoot, filePath);
    const headingNumbers = parseNumberedHeadingSequence(markdown);

    expect(/^# .+/m.test(markdown), `${relativePath} is missing a title heading.`, errors);
    expect(/^Status: .+/m.test(markdown), `${relativePath} is missing Status metadata.`, errors);
    expect(/^Updated: .+/m.test(markdown), `${relativePath} is missing Updated metadata.`, errors);
    expect(/^- Component name: /m.test(markdown), `${relativePath} is missing component-name metadata.`, errors);
    expect(/^- Layer: /m.test(markdown), `${relativePath} is missing layer metadata.`, errors);
    expect(
      !/^\s*(?:[-*]\s+)?Not yet implemented\.?\s*$/im.test(markdown),
      `${relativePath} contains execution status; record it in a roadmap or log instead.`,
      errors,
    );
    expect(
      !/\bjs_[a-z_]+\b/.test(markdown),
      `${relativePath} references a retired Jetstream component entry point.`,
      errors,
    );
    expect(markdown.includes("## 1. Purpose"), `${relativePath} is missing a purpose section.`, errors);
    expect(
      /^##\s+\d+\.\s+Accessibility$/m.test(markdown),
      `${relativePath} is missing an accessibility section.`,
      errors,
    );
    if (headingNumbers.length > 0) {
      expect(headingNumbers[0] === 1, `${relativePath} must start numbered headings at 1.`, errors);

      for (let index = 1; index < headingNumbers.length; index += 1) {
        expect(
          headingNumbers[index] === headingNumbers[index - 1] + 1,
          `${relativePath} has a broken numbered-heading sequence.`,
          errors,
        );
      }
    }
  }

  return componentContractFiles.length;
}

function validateCurrentArchitectureReferences(errors: string[]): void {
  const currentFiles = [
    path.join(repoRoot, "README.md"),
    path.join(repoRoot, "AGENTS.md"),
    path.join(repoRoot, "CLAUDE.md"),
    path.join(repoRoot, "CONTRIBUTING.md"),
    ...collectMarkdownFiles(path.join(repoRoot, "docs", "knowledge")),
    ...collectMarkdownFiles(path.join(repoRoot, "docs", "contracts")),
    ...collectMarkdownFiles(path.join(repoRoot, "docs", "guides")),
  ];

  for (const filePath of currentFiles) {
    const markdown = fs.readFileSync(filePath, "utf8");
    const relativePath = path.relative(repoRoot, filePath);
    expect(
      !/packages\/(?:gpui|jetstream)\/components/.test(markdown),
      `${relativePath} references a retired native component tier.`,
      errors,
    );
  }
}

function validateContractIndexes(errors: string[]): void {
  const componentContracts = collectMarkdownFiles(path.join(contractsDir, "components"))
    .map((file) => path.basename(file));

  compareLists(
    "docs/contracts/components/README.md current contracts",
    parseBulletList(
      fs.readFileSync(path.join(contractsDir, "components", "README.md"), "utf8"),
      "## Current Contracts",
    ),
    componentContracts.filter((file) => file !== "README.md"),
    errors,
  );

  compareLists(
    "docs/contracts/README.md current contracts",
    parseBulletList(
      fs.readFileSync(path.join(contractsDir, "README.md"), "utf8"),
      "## Current Contracts",
    ),
    [
      "template/component-contract-template.md",
      ...componentContracts.map((file) => `components/${file}`),
    ],
    errors,
  );
}

function validateOperatorGuides(errors: string[]): number {
  const guideFiles = collectMarkdownFiles(path.join(repoRoot, "docs", "guides"));
  const packageReadmes = collectMarkdownFiles(path.join(repoRoot, "packages")).filter(
    (filePath) => path.basename(filePath) === "README.md" && !filePath.includes(`${path.sep}node_modules${path.sep}`),
  );
  const files = [
    path.join(repoRoot, "README.md"),
    path.join(repoRoot, "CONTRIBUTING.md"),
    path.join(repoRoot, "SECURITY.md"),
    path.join(repoRoot, "docs", "README.md"),
    ...guideFiles,
    ...packageReadmes,
  ];
  const forbiddenPatterns = [
    { pattern: /<svelte:fragment\b/, description: "legacy <svelte:fragment> composition" },
    { pattern: /<slot\b/, description: "legacy <slot> composition" },
    { pattern: /\bslot\s*=/, description: "legacy slot attributes" },
    { pattern: /\bon:[a-z]/, description: "legacy Svelte event directives" },
    { pattern: /^\s*export let\b/m, description: "legacy Svelte component props" },
    { pattern: /^\s*\$:/m, description: "legacy Svelte reactive labels" },
    { pattern: /ButtonVariant::Solid\b/, description: "the removed Rust ButtonVariant::Solid variant" },
    { pattern: /\bMediaBrowseItem\b/, description: "the nonexistent MediaBrowseItem type" },
    { pattern: /\bbun run tokens:build\b/, description: "the deprecated bun token-build command" },
  ];

  for (const filePath of files) {
    const markdown = fs.readFileSync(filePath, "utf8");
    const relativePath = path.relative(repoRoot, filePath);

    for (const { pattern, description } of forbiddenPatterns) {
      expect(!pattern.test(markdown), `${relativePath} uses ${description}.`, errors);
    }

    for (const match of markdown.matchAll(/\[[^\]]*\]\(([^)]+)\)/g)) {
      const rawTarget = match[1].trim().replace(/^<|>$/g, "");
      if (/^(?:https?:|mailto:|#)/.test(rawTarget)) continue;

      const localTarget = decodeURIComponent(rawTarget.split("#", 1)[0]);
      expect(
        localTarget.length === 0 || fs.existsSync(path.resolve(path.dirname(filePath), localTarget)),
        `${relativePath} links to missing local target ${rawTarget}.`,
        errors,
      );
    }
  }

  const iconManifest = JSON.parse(
    fs.readFileSync(path.join(repoRoot, "packages", "core", "src", "icons", "default-icons.json"), "utf8"),
  ) as { lucideVersion: string };
  const rootPackage = JSON.parse(fs.readFileSync(path.join(repoRoot, "package.json"), "utf8")) as {
    devDependencies: Record<string, string>;
  };
  const svelteGuide = fs.readFileSync(
    path.join(repoRoot, "docs", "guides", "svelte-developer-guide.md"),
    "utf8",
  );
  const lucideVersion = iconManifest.lucideVersion;

  expect(
    rootPackage.devDependencies["lucide-static"] === lucideVersion,
    "package.json lucide-static version must match the default icon manifest.",
    errors,
  );
  expect(
    svelteGuide.includes(`"lucide-static": "${lucideVersion}"`),
    "docs/guides/svelte-developer-guide.md must show the canonical exact lucide-static version.",
    errors,
  );

  return files.length;
}

function validateSveltePackageSurface(
  packagePath: string,
  packageName: string,
  errors: string[],
): void {
  const packageDir = path.join(repoRoot, packagePath);
  const indexPath = path.join(packageDir, "src", "index.ts");
  const readmePath = path.join(packageDir, "README.md");
  const indexSource = fs.readFileSync(indexPath, "utf8");
  const readme = fs.readFileSync(readmePath, "utf8");
  const readmePublicSurface = parseLooseBulletList(readme, "## Public Surface");
  const documentedExports = readmePublicSurface
    .filter((entry) => entry.startsWith("`") && entry.endsWith("`"))
    .map((entry) => entry.slice(1, -1))
    .sort();
  const componentExports = parseDefaultComponentExports(indexSource);
  const helperExports = parseNamedRootExports(indexSource);
  const contractFiles = new Set(
    collectMarkdownFiles(path.join(contractsDir, "components"))
      .map((file) => path.basename(file))
      .filter((file) => file !== "README.md"),
  );

  compareLists(
    `${packagePath} README public surface`,
    documentedExports,
    [...componentExports, ...helperExports].sort(),
    errors,
  );

  const missingContracts = componentExports
    .map((componentName) => `${pascalToKebabCase(componentName)}.md`)
    .filter((contractFile) => !contractFiles.has(contractFile));

  if (missingContracts.length > 0) {
    errors.push(`${packageName} exports missing contracts: ${missingContracts.join(", ")}`);
  }

  expect(
    readme.includes(`- root import: \`${packageName}\``),
    `${packagePath}/README.md is missing "root import: ${packageName}".`,
    errors,
  );
  expect(
    readme.includes(`- type-only import: \`${packageName}/types\``),
    `${packagePath}/README.md is missing "type-only import: ${packageName}/types".`,
    errors,
  );
}

function validatePackageSurfaceCoverage(
  packagePath: string,
  packageName: "@inflatable-cookie/poodle-svelte" | "@inflatable-cookie/poodle-svelte" | "@inflatable-cookie/poodle-svelte",
  errors: string[],
): void {
  const packageDir = path.join(repoRoot, packagePath);
  const indexSource = fs.readFileSync(path.join(packageDir, "src", "index.ts"), "utf8");
  const markdownPath = path.join(packageDir, "src", "markdown.ts");
  const markdownSource = fs.existsSync(markdownPath)
    ? fs.readFileSync(markdownPath, "utf8")
    : "";
  const componentExports = [
    ...parseDefaultComponentExports(indexSource),
    ...parseDefaultComponentExports(markdownSource),
  ].sort();
  const helperExports = parseNamedRootExports(indexSource);
  const actualExports = [...componentExports, ...helperExports].sort();
  const coverageEntries = packageSurfaceCoverage.filter((entry) => entry.packageName === packageName);
  const documentedExports = coverageEntries.map((entry) => entry.exportName).sort();
  const previewSections = docsNavigationSections;
  const previewSectionIds = new Set(previewSections.map((entry) => entry.id));
  const componentExportSet = new Set(componentExports);
  const helperExportSet = new Set(helperExports);
  const seenExports = new Set<string>();

  compareLists(`${packagePath} preview coverage`, documentedExports, actualExports, errors);

  for (const entry of coverageEntries) {
    expect(
      !seenExports.has(entry.exportName),
      `${packagePath} preview coverage duplicates "${entry.exportName}".`,
      errors,
    );
    seenExports.add(entry.exportName);

    expect(
      entry.note.trim().length > 0,
      `${packagePath} preview coverage is missing review guidance for "${entry.exportName}".`,
      errors,
    );

    const expectedKind = componentExportSet.has(entry.exportName)
      ? "component"
      : helperExportSet.has(entry.exportName)
        ? "helper"
        : null;

    expect(
      expectedKind === entry.kind,
      `${packagePath} preview coverage marks "${entry.exportName}" as "${entry.kind}" but the package exports it as "${expectedKind ?? "missing"}".`,
      errors,
    );

    if (entry.status === "previewed") {
      expect(
        entry.sectionIds.length > 0,
        `${packagePath} preview coverage marks "${entry.exportName}" as previewed without section coverage.`,
        errors,
      );

      const sectionIdSet = new Set(entry.sectionIds);
      expect(
        sectionIdSet.size === entry.sectionIds.length,
        `${packagePath} preview coverage repeats section ids for "${entry.exportName}".`,
        errors,
      );

      for (const sectionId of entry.sectionIds) {
        expect(
          previewSectionIds.has(sectionId),
          `${packagePath} preview coverage points "${entry.exportName}" at unknown section "${sectionId}".`,
          errors,
        );

        const section = previewSections.find((candidate) => candidate.id === sectionId);
        expect(
          Boolean(section) && (section.id === "catalog-hub" || section.packageName.includes(packageName)),
          `${packagePath} preview coverage points "${entry.exportName}" at unrelated section "${sectionId}".`,
          errors,
        );
      }
    } else {
      expect(
        entry.sectionIds.length === 0,
        `${packagePath} preview coverage marks "${entry.exportName}" as contract-only but still references preview sections.`,
        errors,
      );
    }
  }
}

function validateDocsCatalog(errors: string[]): void {
  const sectionIds = new Set<string>();
  const familyIds = new Set<string>();

  for (const section of docsSections) {
    expect(!sectionIds.has(section.id), `Duplicate docs section id "${section.id}".`, errors);
    sectionIds.add(section.id);

    expect(section.title.trim().length > 0, `Docs section "${section.id}" is missing a title.`, errors);
    expect(section.packageName.trim().length > 0, `Docs section "${section.id}" is missing package ownership.`, errors);
    expect(section.summary.trim().length > 0, `Docs section "${section.id}" is missing summary copy.`, errors);
    expect(section.exampleTypes.length > 0, `Docs section "${section.id}" is missing example coverage tags.`, errors);

    for (const contractRoot of resolveContractRoots(section.contractRoot)) {
      expect(
        fs.existsSync(path.join(repoRoot, contractRoot)),
        `Docs section "${section.id}" references missing contract root "${contractRoot}".`,
        errors,
      );
    }
  }

  for (const family of docsFamilies) {
    expect(!familyIds.has(family.id), `Duplicate docs family id "${family.id}".`, errors);
    familyIds.add(family.id);

    expect(family.summary.trim().length > 0, `Docs family "${family.id}" is missing summary copy.`, errors);
    expect(
      family.adoptionBar.trim().length > 0,
      `Docs family "${family.id}" is missing adoption-bar guidance.`,
      errors,
    );

    for (const contractRoot of resolveContractRoots(family.contractRoot)) {
      expect(
        fs.existsSync(path.join(repoRoot, contractRoot)),
        `Docs family "${family.id}" references missing contract root "${contractRoot}".`,
        errors,
      );
    }

    for (const sectionId of family.sectionIds) {
      expect(sectionIds.has(sectionId), `Docs family "${family.id}" references unknown section "${sectionId}".`, errors);
    }
  }
}

function validateParityCoverage(errors: string[]): void {
  const docsSectionIds = new Set(docsNavigationSections.map((entry) => entry.id));
  const parityTargetIds = new Set(parityTargets.map((entry) => entry.sectionId));

  for (const sectionId of docsSectionIds) {
    expect(parityTargetIds.has(sectionId), `Missing parity target for docs section "${sectionId}".`, errors);
  }

  for (const target of parityTargets) {
    expect(docsSectionIds.has(target.sectionId), `Unknown parity target "${target.sectionId}".`, errors);
    expect(target.automatedChecks.length > 0, `Parity target "${target.sectionId}" has no automated checks.`, errors);
    expect(target.reviewRoutes.length > 0, `Parity target "${target.sectionId}" has no review routes.`, errors);

    if (target.harnessCoverage.visual !== "not-applicable") {
      expect(target.manualChecks.length > 0, `Parity target "${target.sectionId}" has no manual review notes.`, errors);
    }

    const seenUrls = new Set<string>();
    for (const route of target.reviewRoutes) {
      expect(
        route.state.sectionId === target.sectionId,
        `Parity target "${target.sectionId}" includes mismatched route section "${route.state.sectionId}".`,
        errors,
      );

      const url = buildPreviewUrl(route.state);
      expect(!seenUrls.has(url), `Parity target "${target.sectionId}" has duplicate review URL "${url}".`, errors);
      seenUrls.add(url);
    }
  }
}

function validateAccessibilityAudit(errors: string[]): void {
  const docsSectionIds = new Set(docsNavigationSections.map((entry) => entry.id));
  const accessibilityTargetIds = new Set(accessibilityAuditTargets.map((entry) => entry.sectionId));

  for (const sectionId of docsSectionIds) {
    expect(
      accessibilityTargetIds.has(sectionId),
      `Missing accessibility audit target for docs section "${sectionId}".`,
      errors,
    );
  }

  for (const target of accessibilityAuditTargets) {
    expect(
      docsSectionIds.has(target.sectionId),
      `Unknown accessibility audit target "${target.sectionId}".`,
      errors,
    );
    expect(
      target.automatedChecks.length > 0,
      `Accessibility audit target "${target.sectionId}" has no automated checks.`,
      errors,
    );
    expect(
      target.manualChecks.length > 0,
      `Accessibility audit target "${target.sectionId}" has no manual checks.`,
      errors,
    );
    expect(
      target.reviewRoutes.length > 0,
      `Accessibility audit target "${target.sectionId}" has no review routes.`,
      errors,
    );

    if (target.auditAreas.gpui === "blocked") {
      expect(
        target.blockerNotes.length > 0,
        `Accessibility audit target "${target.sectionId}" blocks GPUI without naming blockers.`,
        errors,
      );
    }

    if (target.auditAreas.gpui === "manual" || target.auditAreas.gpui === "hybrid") {
      expect(
        target.gpuiDeltaNotes.length > 0,
        `Accessibility audit target "${target.sectionId}" marks GPUI as ${target.auditAreas.gpui} without naming GPUI deltas.`,
        errors,
      );
      expect(
        target.blockerNotes.length > 0,
        `Accessibility audit target "${target.sectionId}" marks GPUI as ${target.auditAreas.gpui} without naming remaining blockers.`,
        errors,
      );
    }

    const seenUrls = new Set<string>();
    for (const route of target.reviewRoutes) {
      expect(
        route.state.sectionId === target.sectionId,
        `Accessibility audit target "${target.sectionId}" includes mismatched route section "${route.state.sectionId}".`,
        errors,
      );

      const url = buildPreviewUrl(route.state);
      expect(
        !seenUrls.has(url),
        `Accessibility audit target "${target.sectionId}" has duplicate review URL "${url}".`,
        errors,
      );
      seenUrls.add(url);
    }
  }
}

function validateReleaseOperations(errors: string[]): void {
  const releaseManifest = JSON.parse(fs.readFileSync(releaseManifestPath, "utf8")) as {
    versionPolicy: {
      publicIntentChannel: string;
      internalChannel: string;
      operationsPolicyArtifact?: string;
    };
    packages: Array<{
      name: string;
      path: string;
      kind: string;
      language: string;
      publicIntent: boolean;
      channel: string;
    }>;
  };
  const releaseOperations = JSON.parse(fs.readFileSync(releaseOperationsPath, "utf8")) as {
    changeClasses: Array<{
      id: string;
      requiresReleaseNotes: boolean;
    }>;
    channelRules: Record<
      string,
      {
        enabled: boolean;
        publicIntent: boolean;
      }
    >;
    deprecationWorkflow: {
      stages: string[];
      minimumRemovalGenerationDelay: number;
      minimumRequirements: string[];
    };
    packages: Array<{
      name: string;
      channel: string;
      publicIntent: boolean;
      changeControl: string;
      requiresReleaseNotes: boolean;
      requiresDeprecationRecord: boolean;
      removalGate: string;
    }>;
  };

  expect(
    releaseManifest.versionPolicy.publicIntentChannel === "preview",
    "packages/release-manifest.json must keep preview as the public-intent channel.",
    errors,
  );
  expect(
    releaseManifest.versionPolicy.internalChannel === "internal",
    "packages/release-manifest.json must keep internal as the internal channel.",
    errors,
  );
  expect(
    releaseManifest.versionPolicy.operationsPolicyArtifact === "packages/release-operations.json",
    "packages/release-manifest.json must point at packages/release-operations.json as the operations artifact.",
    errors,
  );

  compareLists(
    "packages/release-operations.json change classes",
    releaseOperations.changeClasses.map((entry) => entry.id).sort(),
    ["additive", "behavioral", "breaking", "docs-only"],
    errors,
  );

  expect(
    releaseOperations.channelRules.preview?.enabled === true,
    "packages/release-operations.json must keep preview channel enabled.",
    errors,
  );
  expect(
    releaseOperations.channelRules.preview?.publicIntent === true,
    "packages/release-operations.json must mark preview as public-intent.",
    errors,
  );
  expect(
    releaseOperations.channelRules.internal?.enabled === true,
    "packages/release-operations.json must keep internal channel enabled.",
    errors,
  );
  expect(
    releaseOperations.channelRules.internal?.publicIntent === false,
    "packages/release-operations.json must mark internal as non-public.",
    errors,
  );
  expect(
    releaseOperations.channelRules.stable?.enabled === false,
    "packages/release-operations.json must keep stable channel disabled in g03.",
    errors,
  );
  expect(
    releaseOperations.deprecationWorkflow.stages.join(",") ===
      "proposed,documented,deprecated,removal-ready,removed",
    "packages/release-operations.json must keep the documented deprecation stage order.",
    errors,
  );
  expect(
    releaseOperations.deprecationWorkflow.minimumRemovalGenerationDelay >= 1,
    "packages/release-operations.json must require at least one generation of deprecation delay for public removals.",
    errors,
  );
  expect(
    releaseOperations.deprecationWorkflow.minimumRequirements.length > 0,
    "packages/release-operations.json must list minimum deprecation requirements.",
    errors,
  );

  compareLists(
    "packages/release-operations.json package inventory",
    releaseOperations.packages.map((entry) => entry.name).sort(),
    releaseManifest.packages.map((entry) => entry.name).sort(),
    errors,
  );

  for (const manifestEntry of releaseManifest.packages) {
    const operationsEntry = releaseOperations.packages.find((entry) => entry.name === manifestEntry.name);
    expect(
      Boolean(operationsEntry),
      `packages/release-operations.json is missing package "${manifestEntry.name}".`,
      errors,
    );

    if (operationsEntry) {
      expect(
        operationsEntry.channel === manifestEntry.channel,
        `Release operations channel mismatch for "${manifestEntry.name}".`,
        errors,
      );
      expect(
        operationsEntry.publicIntent === manifestEntry.publicIntent,
        `Release operations public-intent mismatch for "${manifestEntry.name}".`,
        errors,
      );
      expect(
        operationsEntry.changeControl.trim().length > 0,
        `Release operations changeControl is missing for "${manifestEntry.name}".`,
        errors,
      );
      expect(
        operationsEntry.removalGate.trim().length > 0,
        `Release operations removalGate is missing for "${manifestEntry.name}".`,
        errors,
      );

      if (manifestEntry.publicIntent) {
        expect(
          operationsEntry.requiresReleaseNotes,
          `Public-intent package "${manifestEntry.name}" must require release notes.`,
          errors,
        );
        expect(
          operationsEntry.requiresDeprecationRecord,
          `Public-intent package "${manifestEntry.name}" must require deprecation records.`,
          errors,
        );
      }
    }

    if (manifestEntry.language === "typescript") {
      const packageJsonPath = path.join(repoRoot, manifestEntry.path, "package.json");
      const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, "utf8")) as {
        name?: string;
        version?: string;
        exports?: unknown;
        bin?: string | Record<string, string>;
        poodleRelease?: {
          publicIntent?: boolean;
          channel?: string;
          stability?: string;
        };
      };

      expect(packageJson.name === manifestEntry.name, `${packageJsonPath} name does not match release manifest.`, errors);
      expect(Boolean(packageJson.poodleRelease), `${packageJsonPath} is missing poodleRelease metadata.`, errors);

      if (manifestEntry.publicIntent) {
        const exportTargets: string[] = [];
        const collectExportTargets = (value: unknown): void => {
          if (typeof value === "string") {
            exportTargets.push(value);
          } else if (value && typeof value === "object") {
            Object.values(value).forEach(collectExportTargets);
          }
        };
        collectExportTargets(packageJson.exports);
        if (typeof packageJson.bin === "string") {
          exportTargets.push(packageJson.bin);
        } else if (packageJson.bin) {
          exportTargets.push(...Object.values(packageJson.bin));
        }

        expect(exportTargets.length > 0, `${packageJsonPath} must declare exports or a binary.`, errors);
        for (const target of exportTargets) {
          if (!target.startsWith("./")) continue;
          const wildcardIndex = target.indexOf("*");
          if (wildcardIndex === -1) {
            expect(
              fs.existsSync(path.join(repoRoot, manifestEntry.path, target)),
              `${packageJsonPath} export target ${target} does not exist.`,
              errors,
            );
            continue;
          }

          const prefix = target.slice(0, wildcardIndex);
          const suffix = target.slice(wildcardIndex + 1);
          const directory = path.join(
            repoRoot,
            manifestEntry.path,
            prefix.endsWith("/") ? prefix : path.dirname(prefix),
          );
          const namePrefix = prefix.endsWith("/") ? "" : path.basename(prefix);
          const hasMatch = fs.existsSync(directory) && fs.readdirSync(directory).some(
            (entry) => entry.startsWith(namePrefix) && entry.endsWith(suffix),
          );
          expect(hasMatch, `${packageJsonPath} export target ${target} matches no files.`, errors);
        }
      }

      // Version must be present and pre-1.0 (0.x) per the version policy.
      const version = packageJson.version;
      expect(
        typeof version === "string" && /^0\.\d+\.\d+$/.test(version),
        `${packageJsonPath} version must be present and 0.x semver (got ${String(version)}).`,
        errors,
      );
      // Preview packages on a real version (past the 0.0.0 baseline) require a
      // release note that lists them (operations requiresReleaseNotes).
      if (manifestEntry.channel === "preview" && typeof version === "string" && version !== "0.0.0") {
        const notePath = path.join(repoRoot, "docs", "release-notes", `${version}.md`);
        if (!fs.existsSync(notePath)) {
          errors.push(`${manifestEntry.name} is at ${version} but docs/release-notes/${version}.md is missing.`);
        } else if (!fs.readFileSync(notePath, "utf8").includes(manifestEntry.name)) {
          errors.push(`docs/release-notes/${version}.md must list ${manifestEntry.name}.`);
        }
      }

      if (packageJson.poodleRelease) {
        expect(
          packageJson.poodleRelease.publicIntent === manifestEntry.publicIntent,
          `${packageJsonPath} poodleRelease.publicIntent does not match release manifest.`,
          errors,
        );
        expect(
          packageJson.poodleRelease.channel === manifestEntry.channel,
          `${packageJsonPath} poodleRelease.channel does not match release manifest.`,
          errors,
        );

        if (manifestEntry.channel === "preview") {
          expect(
            packageJson.poodleRelease.stability === "pre-release" ||
              packageJson.poodleRelease.stability === "experimental",
            `${packageJsonPath} preview packages must use stability "pre-release" or "experimental".`,
            errors,
          );
        }
      }
    } else if (manifestEntry.language === "rust") {
      const cargoPath = path.join(repoRoot, manifestEntry.path, "Cargo.toml");
      const cargoSource = fs.readFileSync(cargoPath, "utf8");
      const cargoMetadata = parseCargoPoodleMetadata(cargoSource);
      const cargoVersion = cargoSource.match(/^version\s*=\s*"([^"]+)"/m)?.[1];

      expect(cargoMetadata.name === manifestEntry.name, `${cargoPath} package name does not match release manifest.`, errors);
      expect(
        typeof cargoVersion === "string" && /^0\.\d+\.\d+$/.test(cargoVersion),
        `${cargoPath} version must be present and 0.x semver (got ${String(cargoVersion)}).`,
        errors,
      );
      if (
        manifestEntry.channel === "preview" &&
        typeof cargoVersion === "string" &&
        cargoVersion !== "0.0.0" &&
        cargoReleaseNoteRequired(cargoVersion)
      ) {
        const notePath = path.join(repoRoot, "docs", "release-notes", `${cargoVersion}.md`);
        if (!fs.existsSync(notePath)) {
          errors.push(`${manifestEntry.name} is at ${cargoVersion} but docs/release-notes/${cargoVersion}.md is missing.`);
        } else if (!fs.readFileSync(notePath, "utf8").includes(manifestEntry.name)) {
          errors.push(`docs/release-notes/${cargoVersion}.md must list ${manifestEntry.name}.`);
        }
      }
      expect(
        cargoMetadata.publicIntent === manifestEntry.publicIntent,
        `${cargoPath} public-intent metadata does not match release manifest.`,
        errors,
      );
      expect(
        cargoMetadata.channel === manifestEntry.channel,
        `${cargoPath} channel metadata does not match release manifest.`,
        errors,
      );

      if (manifestEntry.channel === "preview") {
        expect(
          cargoMetadata.stability === "pre-release",
          `${cargoPath} preview packages must use stability "pre-release".`,
          errors,
        );
      }
    }
  }

  // Reverse check: every first-party package must be recorded. Requiring
  // release metadata before checking inventory lets new internal tooling evade
  // the package-classification rule entirely.
  const manifestNames = new Set(releaseManifest.packages.map((p) => p.name));
  const stack = [path.join(repoRoot, "packages")];
  while (stack.length > 0) {
    const dir = stack.pop() as string;
    for (const dirent of fs.readdirSync(dir, { withFileTypes: true })) {
      if (dirent.name === "node_modules" || dirent.name === "target" || dirent.name.startsWith(".")) continue;
      const full = path.join(dir, dirent.name);
      if (dirent.isDirectory()) {
        stack.push(full);
      } else if (dirent.name === "package.json") {
        const pkg = JSON.parse(fs.readFileSync(full, "utf8")) as { name?: string; poodleRelease?: unknown };
        if (pkg.name && !manifestNames.has(pkg.name)) {
          errors.push(`${full} package "${pkg.name}" is missing from packages/release-manifest.json.`);
        }
      } else if (dirent.name === "Cargo.toml") {
        const cargoSource = fs.readFileSync(full, "utf8");
        const cargoName = cargoSource.match(/^name\s*=\s*"([^"]+)"/m)?.[1];
        if (cargoName && !manifestNames.has(cargoName)) {
          errors.push(`${full} package "${cargoName}" is missing from packages/release-manifest.json.`);
        }
      }
    }
  }
}

/**
 * A shared npm/native version does not make an older web-only release into a
 * native release. Use its candidate evidence to preserve that historical
 * boundary; new or malformed evidence remains fail-closed and requires the
 * native packages to appear in the release note.
 */
function cargoReleaseNoteRequired(version: string): boolean {
  const evidencePath = path.join(
    repoRoot,
    "docs",
    "evidence",
    "releases",
    `v${version}-candidate.json`,
  );
  if (!fs.existsSync(evidencePath)) return true;
  try {
    const evidence = JSON.parse(fs.readFileSync(evidencePath, "utf8")) as {
      schema?: unknown;
      version?: unknown;
      native_train_changed?: unknown;
    };
    return !(
      evidence.schema === "poodle.web-candidate-evidence.v1" &&
      evidence.version === version &&
      evidence.native_train_changed === false
    );
  } catch {
    return true;
  }
}

function validateEcosystemAcceptance(errors: string[]): { suiteCount: number; regressionClassCount: number } {
  const ecosystemAcceptance = JSON.parse(fs.readFileSync(ecosystemAcceptancePath, "utf8")) as {
    suites: Array<{
      id: string;
      label: string;
      status: string;
      coveredPackages: string[];
      evidenceArtifacts: string[];
      regressionClasses: string[];
      requiredChecks: string[];
      blockers: string[];
    }>;
    regressionClasses: Array<{
      id: string;
      label: string;
      scope: string;
      primaryEvidence: string[];
    }>;
    ecosystemReadinessGate: string[];
  };
  const requiredSuiteIds = [
    "preview-docs-harness",
    "loophole-foundation-adoption",
    "gpui-target-matrix",
  ];
  const requiredRegressionClassIds = [
    "tokens-and-themes",
    "package-surface-and-contracts",
    "preview-routes-and-docs-build",
    "accessibility-and-keyboard",
    "downstream-boundaries-and-ownership",
    "release-metadata-and-change-control",
    "runtime-specific-gpui-assumptions",
  ];
  const suiteIds = ecosystemAcceptance.suites.map((entry) => entry.id);
  const regressionClassIds = ecosystemAcceptance.regressionClasses.map((entry) => entry.id);
  const regressionClassIdSet = new Set(regressionClassIds);
  const releaseManifest = JSON.parse(fs.readFileSync(releaseManifestPath, "utf8")) as {
    packages: Array<{ name: string }>;
  };
  const knownPackageNames = new Set(releaseManifest.packages.map((entry) => entry.name));
  const suiteIdSet = new Set<string>();
  const regressionIdSet = new Set<string>();

  compareLists(
    "packages/ecosystem-acceptance.json required suites",
    suiteIds.sort(),
    requiredSuiteIds.sort(),
    errors,
  );
  compareLists(
    "packages/ecosystem-acceptance.json regression classes",
    regressionClassIds.sort(),
    requiredRegressionClassIds.sort(),
    errors,
  );

  for (const suite of ecosystemAcceptance.suites) {
    expect(!suiteIdSet.has(suite.id), `Ecosystem acceptance duplicates suite "${suite.id}".`, errors);
    suiteIdSet.add(suite.id);

    expect(suite.label.trim().length > 0, `Ecosystem acceptance suite "${suite.id}" is missing a label.`, errors);
    expect(
      ["baseline", "proof-backed", "matrix-only"].includes(suite.status),
      `Ecosystem acceptance suite "${suite.id}" has unsupported status "${suite.status}".`,
      errors,
    );
    expect(
      suite.coveredPackages.length > 0,
      `Ecosystem acceptance suite "${suite.id}" is missing covered packages.`,
      errors,
    );
    expect(
      suite.evidenceArtifacts.length > 0,
      `Ecosystem acceptance suite "${suite.id}" is missing evidence artifacts.`,
      errors,
    );
    expect(
      suite.regressionClasses.length > 0,
      `Ecosystem acceptance suite "${suite.id}" is missing regression classes.`,
      errors,
    );
    expect(
      suite.requiredChecks.length > 0,
      `Ecosystem acceptance suite "${suite.id}" is missing required checks.`,
      errors,
    );

    for (const packageName of suite.coveredPackages) {
      expect(
        knownPackageNames.has(packageName),
        `Ecosystem acceptance suite "${suite.id}" references unknown package "${packageName}".`,
        errors,
      );
    }

    for (const artifactPath of suite.evidenceArtifacts) {
      expect(
        fs.existsSync(path.join(repoRoot, artifactPath)),
        `Ecosystem acceptance suite "${suite.id}" references missing evidence artifact "${artifactPath}".`,
        errors,
      );
    }

    for (const regressionClassId of suite.regressionClasses) {
      expect(
        regressionClassIdSet.has(regressionClassId),
        `Ecosystem acceptance suite "${suite.id}" references unknown regression class "${regressionClassId}".`,
        errors,
      );
    }

    if (suite.status === "matrix-only") {
      expect(
        suite.blockers.length > 0,
        `Ecosystem acceptance suite "${suite.id}" is matrix-only but does not record blockers.`,
        errors,
      );
    }
  }

  for (const regressionClass of ecosystemAcceptance.regressionClasses) {
    expect(
      !regressionIdSet.has(regressionClass.id),
      `Ecosystem acceptance duplicates regression class "${regressionClass.id}".`,
      errors,
    );
    regressionIdSet.add(regressionClass.id);

    expect(
      regressionClass.label.trim().length > 0,
      `Regression class "${regressionClass.id}" is missing a label.`,
      errors,
    );
    expect(
      regressionClass.scope.trim().length > 0,
      `Regression class "${regressionClass.id}" is missing scope text.`,
      errors,
    );
    expect(
      regressionClass.primaryEvidence.length > 0,
      `Regression class "${regressionClass.id}" is missing primary evidence.`,
      errors,
    );

    for (const evidencePath of regressionClass.primaryEvidence) {
      expect(
        fs.existsSync(path.join(repoRoot, evidencePath)),
        `Regression class "${regressionClass.id}" references missing evidence path "${evidencePath}".`,
        errors,
      );
    }
  }

  expect(
    ecosystemAcceptance.ecosystemReadinessGate.length >= 4,
    "packages/ecosystem-acceptance.json must include an explicit ecosystem readiness gate.",
    errors,
  );

  return {
    suiteCount: ecosystemAcceptance.suites.length,
    regressionClassCount: ecosystemAcceptance.regressionClasses.length,
  };
}

function validateReferenceApps(errors: string[]): { shapeCount: number; laneCount: number } {
  const referenceApps = JSON.parse(fs.readFileSync(referenceAppsPath, "utf8")) as {
    referenceShapes: Array<{
      id: string;
      label: string;
      format: string;
      status: string;
      packageEntryPoints: string[];
      exampleSectionIds: string[];
      onboardingSteps: string[];
      evidencePaths: string[];
      blockers: string[];
    }>;
    onboardingLanes: Array<{
      id: string;
      label: string;
      startPaths: string[];
      primaryReferenceShapeId: string;
      nonGoals: string[];
    }>;
    publicExampleExpectations: string[];
  };
  const requiredReferenceShapeIds = [
    "direct-svelte-consumer",
    "workstation-foundation-consumer",
    "public-example-surface",
  ];
  const requiredOnboardingLaneIds = [
    "evaluate",
    "direct-adoption",
    "workstation-adoption",
  ];
  const shapeIds = referenceApps.referenceShapes.map((entry) => entry.id);
  const laneIds = referenceApps.onboardingLanes.map((entry) => entry.id);
  const shapeIdSet = new Set<string>();
  const laneIdSet = new Set<string>();
  const docsSectionIds = new Set(docsNavigationSections.map((entry) => entry.id));
  const releaseManifest = JSON.parse(fs.readFileSync(releaseManifestPath, "utf8")) as {
    packages: Array<{ name: string }>;
  };
  const knownPackageNames = new Set(releaseManifest.packages.map((entry) => entry.name));

  compareLists(
    "packages/reference-apps.json reference shapes",
    shapeIds.sort(),
    requiredReferenceShapeIds.sort(),
    errors,
  );
  compareLists(
    "packages/reference-apps.json onboarding lanes",
    laneIds.sort(),
    requiredOnboardingLaneIds.sort(),
    errors,
  );

  for (const shape of referenceApps.referenceShapes) {
    expect(!shapeIdSet.has(shape.id), `Reference-apps matrix duplicates shape "${shape.id}".`, errors);
    shapeIdSet.add(shape.id);

    expect(shape.label.trim().length > 0, `Reference shape "${shape.id}" is missing a label.`, errors);
    expect(
      ["reference-app-shape", "public-example-family"].includes(shape.format),
      `Reference shape "${shape.id}" has unsupported format "${shape.format}".`,
      errors,
    );
    expect(
      ["docs-backed", "proof-backed", "preview-backed"].includes(shape.status),
      `Reference shape "${shape.id}" has unsupported status "${shape.status}".`,
      errors,
    );
    expect(shape.packageEntryPoints.length > 0, `Reference shape "${shape.id}" is missing package entry points.`, errors);
    expect(shape.onboardingSteps.length > 0, `Reference shape "${shape.id}" is missing onboarding steps.`, errors);
    expect(shape.evidencePaths.length > 0, `Reference shape "${shape.id}" is missing evidence paths.`, errors);
    expect(shape.blockers.length > 0, `Reference shape "${shape.id}" must record at least one blocker or caution.`, errors);

    for (const packageName of shape.packageEntryPoints) {
      expect(
        knownPackageNames.has(packageName),
        `Reference shape "${shape.id}" references unknown package "${packageName}".`,
        errors,
      );
    }

    for (const sectionId of shape.exampleSectionIds) {
      expect(
        docsSectionIds.has(sectionId),
        `Reference shape "${shape.id}" references unknown example section "${sectionId}".`,
        errors,
      );
    }

    for (const evidencePath of shape.evidencePaths) {
      expect(
        fs.existsSync(path.join(repoRoot, evidencePath)),
        `Reference shape "${shape.id}" references missing evidence path "${evidencePath}".`,
        errors,
      );
    }
  }

  for (const lane of referenceApps.onboardingLanes) {
    expect(!laneIdSet.has(lane.id), `Reference-apps matrix duplicates lane "${lane.id}".`, errors);
    laneIdSet.add(lane.id);

    expect(lane.label.trim().length > 0, `Onboarding lane "${lane.id}" is missing a label.`, errors);
    expect(lane.startPaths.length > 0, `Onboarding lane "${lane.id}" is missing start paths.`, errors);
    expect(lane.nonGoals.length > 0, `Onboarding lane "${lane.id}" is missing non-goals.`, errors);
    expect(
      shapeIdSet.has(lane.primaryReferenceShapeId) || shapeIds.includes(lane.primaryReferenceShapeId),
      `Onboarding lane "${lane.id}" references unknown reference shape "${lane.primaryReferenceShapeId}".`,
      errors,
    );

    for (const startPath of lane.startPaths) {
      expect(
        fs.existsSync(path.join(repoRoot, startPath)),
        `Onboarding lane "${lane.id}" references missing start path "${startPath}".`,
        errors,
      );
    }
  }

  expect(
    referenceApps.publicExampleExpectations.length >= 3,
    "packages/reference-apps.json must define public example expectations.",
    errors,
  );

  return {
    shapeCount: referenceApps.referenceShapes.length,
    laneCount: referenceApps.onboardingLanes.length,
  };
}

function validateGpuiPreviewBaseline(errors: string[]): { previewSectionCount: number } {
  const audit = JSON.parse(fs.readFileSync(sharedDemoAppAuditPath, "utf8")) as {
    sourceSurface: { docsOnlySectionIds: string[] };
  };
  const previewBaseline = JSON.parse(fs.readFileSync(gpuiPreviewBaselinePath, "utf8")) as {
    generation: string;
    themeRuntime: {
      tokenSource: string;
      themeIds: string[];
      densityModes: string[];
      controlSizes: string[];
      applicationRules: string[];
    };
    previewApp: {
      sectionIds: string[];
      requiredControls: string[];
      shellAreas: string[];
      comparisonSource: string;
      evidenceCapture: string[];
      sideBySideExpectations: string[];
    };
    nonGoals: string[];
  };
  expect(previewBaseline.generation === "g04.002", "packages/gpui/preview-app-baseline.json must target g04.002.", errors);
  expect(
    previewBaseline.themeRuntime.tokenSource === gpuiTokenSource,
    "packages/gpui/preview-app-baseline.json must use poodle-tokens as token source.",
    errors,
  );
  compareLists(
    "packages/gpui/preview-app-baseline.json theme ids",
    [...previewBaseline.themeRuntime.themeIds].sort(),
    ["eclipse", "iceberg", "graphite"],
    errors,
  );
  compareLists(
    "packages/gpui/preview-app-baseline.json density modes",
    [...previewBaseline.themeRuntime.densityModes].sort(),
    ["comfortable", "compact"],
    errors,
  );
  compareLists(
    "packages/gpui/preview-app-baseline.json control sizes",
    [...previewBaseline.themeRuntime.controlSizes].sort(),
    ["lg", "md", "sm"],
    errors,
  );
  compareLists(
    "packages/gpui/preview-app-baseline.json preview sections",
    [...previewBaseline.previewApp.sectionIds].sort(),
    [...audit.sourceSurface.docsOnlySectionIds].sort(),
    errors,
  );
  compareLists(
    "packages/gpui/preview-app-baseline.json required controls",
    [...previewBaseline.previewApp.requiredControls].sort(),
    ["controlSize", "density", "section", "theme"],
    errors,
  );

  expect(
    fs.existsSync(path.join(repoRoot, previewBaseline.previewApp.comparisonSource)),
    `packages/gpui/preview-app-baseline.json references missing comparison source "${previewBaseline.previewApp.comparisonSource}".`,
    errors,
  );
  expect(
    previewBaseline.themeRuntime.applicationRules.length > 0,
    "packages/gpui/preview-app-baseline.json must record theme application rules.",
    errors,
  );
  expect(
    previewBaseline.previewApp.shellAreas.length >= 3,
    "packages/gpui/preview-app-baseline.json must record shell areas.",
    errors,
  );
  expect(
    previewBaseline.previewApp.evidenceCapture.length > 0,
    "packages/gpui/preview-app-baseline.json must record evidence capture expectations.",
    errors,
  );
  expect(
    previewBaseline.previewApp.sideBySideExpectations.length === previewBaseline.previewApp.sectionIds.length,
    "packages/gpui/preview-app-baseline.json must record one side-by-side expectation per preview section.",
    errors,
  );
  expect(
    previewBaseline.nonGoals.length >= 3,
    "packages/gpui/preview-app-baseline.json must record explicit non-goals.",
    errors,
  );

  return {
    previewSectionCount: previewBaseline.previewApp.sectionIds.length,
  };
}

function validateGpuiNativeAccessibilityProof(errors: string[]): {
  gpuiAccessibilityLayerCount: number;
  gpuiAccessibilitySectionCount: number;
} {
  const proof = JSON.parse(fs.readFileSync(gpuiNativeAccessibilityProofPath, "utf8")) as {
    generation: string;
    sourceLedger: string;
    currentPosture: {
      nativeNotApplicable: string[];
      status: string;
      specEvidence: string;
      mountedEvidence: string[];
      assistiveTechnologyEvidence: string;
      limitation: string;
    };
    comparisonSource: string;
    sectionIds: string[];
    layerProof: Array<{
      id: string;
      crateName: string;
      cratePath: string;
      focusEntryStatus: string;
      focusRecoveryStatus: string;
      keyboardTraversalStatus: string;
      stateExposureStatus: string;
      announcementsStatus: string;
      assistiveTechnologyStatus: string;
      evidence: string[];
      remainingBlockers: string[];
    }>;
    sectionProof: Array<{
      sectionId: string;
      gpuiStatus: string;
      focusStatus: string;
      keyboardStatus: string;
      announcementsStatus: string;
      owningLayer: string;
      remainingBlockers: string[];
    }>;
    manualReviewExpectations: string[];
    deltaRegister: string[];
    nonGoals: string[];
  };
  const expectedSectionIds = [
    "browse-suite",
    "command-suite",
    "detail-suite",
    "form-suite",
    "media-suite",
    "notification-suite",
    "picker-suite",
    "table-suite",
    "workspace-suite",
  ];
  const expectedLayerIds = ["composites", "primitives", "workstation"];
  const allowedStatuses = new Set(["explicit", "hybrid", "manual"]);
  const sectionTargets = new Map(
    accessibilityAuditTargets
      .filter((target) => target.auditAreas.gpui !== "not-applicable")
      .map((target) => [target.sectionId, target]),
  );
  expect(
    proof.generation === "g16.001",
    "packages/gpui/native-accessibility-proof.json must target g16.001.",
    errors,
  );
  expect(
    proof.sourceLedger === "docs/evidence/nucleus/parity-evidence-ledger.md",
    "packages/gpui/native-accessibility-proof.json must point at the g16 evidence ledger.",
    errors,
  );
  expect(
    ROSTER_WEB_ONLY_NAMES.every((name) => proof.currentPosture.nativeNotApplicable.includes(name)) &&
      proof.currentPosture.nativeNotApplicable.length === ROSTER_WEB_ONLY_NAMES.length,
    `packages/gpui/native-accessibility-proof.json must keep ${ROSTER_WEB_ONLY_NAMES.join(", ")} as the sole native not-applicable boundary.`,
    errors,
  );
  expect(
    proof.currentPosture.status === "manual" &&
      proof.currentPosture.assistiveTechnologyEvidence === "missing" &&
      proof.currentPosture.limitation.includes("does not prove broad native"),
    "packages/gpui/native-accessibility-proof.json must keep broad native accessibility proof manual and missing.",
    errors,
  );
  expect(
    proof.comparisonSource === "packages/svelte/preview/src/accessibility.ts",
    "packages/gpui/native-accessibility-proof.json must compare against packages/svelte/preview/src/accessibility.ts.",
    errors,
  );
  compareLists(
    "packages/gpui/native-accessibility-proof.json section ids",
    [...proof.sectionIds].sort(),
    [...expectedSectionIds].sort(),
    errors,
  );
  compareLists(
    "packages/gpui/native-accessibility-proof.json layer ids",
    proof.layerProof.map((layer) => layer.id).sort(),
    [...expectedLayerIds].sort(),
    errors,
  );
  expect(
    proof.manualReviewExpectations.length >= 2,
    "packages/gpui/native-accessibility-proof.json must record manual review expectations.",
    errors,
  );
  expect(
    proof.deltaRegister.length >= 2,
    "packages/gpui/native-accessibility-proof.json must record explicit accessibility deltas.",
    errors,
  );
  expect(
    proof.nonGoals.length >= 2,
    "packages/gpui/native-accessibility-proof.json must record explicit non-goals.",
    errors,
  );

  const expectedLayerData = new Map(
    expectedLayerIds.map((id) => [
      id,
      { crateName: gpuiAdapterCrateName, cratePath: gpuiAdapterCratePath },
    ]),
  );

  for (const layer of proof.layerProof) {
    const expected = expectedLayerData.get(layer.id);

    expect(
      Boolean(expected),
      `packages/gpui/native-accessibility-proof.json includes unknown layer "${layer.id}".`,
      errors,
    );
    if (!expected) {
      continue;
    }

    expect(
      layer.crateName === expected.crateName,
      `packages/gpui/native-accessibility-proof.json layer "${layer.id}" must target ${expected.crateName}.`,
      errors,
    );
    expect(
      layer.cratePath === expected.cratePath,
      `packages/gpui/native-accessibility-proof.json layer "${layer.id}" must target ${expected.cratePath}.`,
      errors,
    );
    for (const status of [
      layer.focusEntryStatus,
      layer.focusRecoveryStatus,
      layer.keyboardTraversalStatus,
      layer.stateExposureStatus,
      layer.announcementsStatus,
      layer.assistiveTechnologyStatus,
    ]) {
      expect(
        allowedStatuses.has(status),
        `packages/gpui/native-accessibility-proof.json layer "${layer.id}" uses unsupported status "${status}".`,
        errors,
      );
    }

    expect(
      layer.evidence.length > 0,
      `packages/gpui/native-accessibility-proof.json layer "${layer.id}" must record evidence.`,
      errors,
    );
    expect(
      layer.remainingBlockers.length > 0,
      `packages/gpui/native-accessibility-proof.json layer "${layer.id}" must record remaining blockers.`,
      errors,
    );
    expect(
      fs.existsSync(path.join(repoRoot, layer.cratePath, "README.md")),
      `packages/gpui/native-accessibility-proof.json references missing crate README "${layer.cratePath}/README.md".`,
      errors,
    );
  }

  compareLists(
    "packages/gpui/native-accessibility-proof.json section proof entries",
    proof.sectionProof.map((entry) => entry.sectionId).sort(),
    [...expectedSectionIds].sort(),
    errors,
  );

  for (const section of proof.sectionProof) {
    const accessibilityTarget = sectionTargets.get(section.sectionId);

    expect(
      Boolean(accessibilityTarget),
      `packages/gpui/native-accessibility-proof.json references unknown section "${section.sectionId}".`,
      errors,
    );
    if (!accessibilityTarget) {
      continue;
    }

    expect(
      section.gpuiStatus === accessibilityTarget.auditAreas.gpui,
      `packages/gpui/native-accessibility-proof.json section "${section.sectionId}" GPUI status must match packages/svelte/preview/src/accessibility.ts.`,
      errors,
    );
    expect(
      section.focusStatus === accessibilityTarget.auditAreas.focus,
      `packages/gpui/native-accessibility-proof.json section "${section.sectionId}" focus status must match packages/svelte/preview/src/accessibility.ts.`,
      errors,
    );
    expect(
      section.keyboardStatus === accessibilityTarget.auditAreas.keyboard,
      `packages/gpui/native-accessibility-proof.json section "${section.sectionId}" keyboard status must match packages/svelte/preview/src/accessibility.ts.`,
      errors,
    );
    expect(
      section.announcementsStatus === accessibilityTarget.auditAreas.announcements,
      `packages/gpui/native-accessibility-proof.json section "${section.sectionId}" announcement status must match packages/svelte/preview/src/accessibility.ts.`,
      errors,
    );
    expect(
      section.remainingBlockers.length > 0,
      `packages/gpui/native-accessibility-proof.json section "${section.sectionId}" must record remaining blockers.`,
      errors,
    );

  }

  return {
    gpuiAccessibilityLayerCount: proof.layerProof.length,
    gpuiAccessibilitySectionCount: proof.sectionProof.length,
  };
}

function validateGpuiCrossRuntimeParityReport(errors: string[]): {
  gpuiCrossRuntimeRouteCount: number;
  gpuiCrossRuntimeDeltaCount: number;
} {
  const report = JSON.parse(fs.readFileSync(gpuiCrossRuntimeParityReportPath, "utf8")) as {
    artifact: string;
    generation: string;
    sourceLedger: string;
    runtime: string;
    status: string;
    denominator: {
      notApplicable: string[];
    };
    construction: {
      status: string;
      claim: string;
      evidence: string[];
    };
    mountedBehaviour: {
      status: string;
      scope: string;
      testFile: string;
      namedTests: string[];
    };
    accessibility: {
      status: string;
      scope: string;
      evidence: string[];
    };
    visual: {
      status: string;
      scope: string;
      evidence: string[];
      capturePosture: string;
    };
    knownDeltas: string[];
  };

  const expectEvidence = (label: string, evidence: string[]): void => {
    expect(evidence.length > 0, `packages/gpui/cross-runtime-parity-report.json ${label} must record evidence.`, errors);
    for (const entry of evidence) {
      const [reference, fragment] = entry.split("#");
      if (/^(?:packages|docs|test|scripts)\//.test(reference)) {
        expect(
          fs.existsSync(path.join(repoRoot, reference)),
          `packages/gpui/cross-runtime-parity-report.json ${label} references missing evidence "${reference}".`,
          errors,
        );
        if (fragment !== undefined && fs.existsSync(path.join(repoRoot, reference))) {
          const source = fs.readFileSync(path.join(repoRoot, reference), "utf8");
          expect(
            source.includes(fragment),
            `packages/gpui/cross-runtime-parity-report.json ${label} references unresolved evidence "${entry}".`,
            errors,
          );
        }
      }
    }
  };

  expect(report.artifact === "packages/gpui/cross-runtime-parity-report.json", "GPUI parity report artifact path is stale.", errors);
  expect(report.generation === "g16.001", "packages/gpui/cross-runtime-parity-report.json must target g16.001.", errors);
  expect(report.sourceLedger === "docs/evidence/nucleus/parity-evidence-ledger.md", "GPUI parity report must point at the g16 evidence ledger.", errors);
  expect(report.runtime === "gpui" && report.status === "current", "GPUI parity report must identify the current GPUI posture.", errors);
  expect(
    ROSTER_WEB_ONLY_NAMES.every((name) => report.denominator.notApplicable.includes(name)) &&
      report.denominator.notApplicable.length === ROSTER_WEB_ONLY_NAMES.length,
    `GPUI parity report must keep ${ROSTER_WEB_ONLY_NAMES.join(", ")} as the sole native not-applicable boundary.`,
    errors,
  );

  expect(
    report.construction.status === "focused" &&
      report.construction.claim.includes("headless GPUI specimen probe"),
    "GPUI parity report must claim headless specimen construction.",
    errors,
  );
  expectEvidence("construction", report.construction.evidence);

  expect(
    report.mountedBehaviour.status === "mounted" &&
      report.mountedBehaviour.scope.includes("bounded") &&
      report.mountedBehaviour.scope.includes("not a roster-wide behaviour pass"),
    "GPUI parity report must bound mounted behaviour rather than promote it to roster coverage.",
    errors,
  );
  expect(
    fs.existsSync(path.join(repoRoot, report.mountedBehaviour.testFile)),
    "GPUI parity report mounted behaviour test file is missing.",
    errors,
  );
  expect(
    report.mountedBehaviour.namedTests.length > 0,
    "GPUI parity report must name the bounded mounted regression set.",
    errors,
  );
  const mountedSource = fs.readFileSync(path.join(repoRoot, report.mountedBehaviour.testFile), "utf8");
  for (const testName of report.mountedBehaviour.namedTests) {
    expect(
      mountedSource.includes(testName),
      `GPUI mounted regression is unresolved: ${testName}.`,
      errors,
    );
  }

  expect(
    report.accessibility.status === "manual" &&
      report.accessibility.scope.includes("not proved") &&
      report.accessibility.scope.includes("assistive-technology"),
    "GPUI parity report must keep broad native accessibility proof manual and unproved.",
    errors,
  );
  expectEvidence("accessibility", report.accessibility.evidence);

  expect(
    report.visual.status === "compared" &&
      report.visual.scope.includes("Button-only") &&
      report.visual.scope.includes("18-case"),
    "GPUI parity report must keep visual comparison Button-only.",
    errors,
  );
  expect(
    report.visual.capturePosture.includes("non-activating windowed") &&
      report.visual.capturePosture.includes("absent from default QA/CI"),
    "GPUI parity report must record the operator-approved windowed capture boundary.",
    errors,
  );
  expectEvidence("visual", report.visual.evidence);
  expect(report.knownDeltas.length >= 3, "GPUI parity report must record current known deltas.", errors);

  return {
    gpuiCrossRuntimeRouteCount: PORTABLE_ROUTE_COUNT,
    gpuiCrossRuntimeDeltaCount: report.knownDeltas.length,
  };
}

function validateSharedDemoAppAudit(errors: string[]): {
  demoAuditFindingCount: number;
  demoAuditScreenCount: number;
} {
  const audit = JSON.parse(fs.readFileSync(sharedDemoAppAuditPath, "utf8")) as {
    generation: string;
    sourceSurface: {
      entryPath: string;
      stylePath: string;
      entryLineCount: number;
      styleLineCount: number;
      docsOnlySectionIds: string[];
      sharedTargetSectionIds: string[];
    };
    packageCoverage: {
      artifactsSource: string;
      packages: Array<{
        packageName: string;
        exportCount: number;
        previewedCount: number;
        contractOnlyCount: number;
        priorityMissingExports: string[];
      }>;
    };
    auditFindings: Array<{
      id: string;
      severity: string;
      summary: string;
      evidence: string[];
      impact: string;
    }>;
    targetShapeFreeze: {
      appId: string;
      description: string;
      requiredScreens: Array<{
        id: string;
        goal: string;
      }>;
      requiredRules: string[];
      nonGoals: string[];
    };
  };
  const parityReport = JSON.parse(
    fs.readFileSync(path.join(repoRoot, "packages", "svelte", "preview", "artifacts", "parity-report.json"), "utf8"),
  ) as {
    packageSurfaceCoverage: {
      summary: Array<{
        packageName: string;
        exportCount: number;
        previewedCount: number;
        contractOnlyCount: number;
      }>;
    };
  };
  const docsSectionIds = new Set(docsNavigationSections.map((entry) => entry.id));
  const docsOnlySet = new Set(audit.sourceSurface.docsOnlySectionIds);
  const targetSectionSet = new Set(audit.sourceSurface.sharedTargetSectionIds);

  expect(
    audit.generation === "g09.018",
    "packages/shared-demo-app-audit.json must target g09.018.",
    errors,
  );
  expect(
    audit.sourceSurface.entryPath === "packages/svelte/preview/src/App.svelte",
    "packages/shared-demo-app-audit.json must target packages/svelte/preview/src/App.svelte as the source entry.",
    errors,
  );
  expect(
    audit.sourceSurface.stylePath === "packages/svelte/preview/src/app.css",
    "packages/shared-demo-app-audit.json must target packages/svelte/preview/src/app.css as the source stylesheet.",
    errors,
  );
  expect(
    fs.existsSync(path.join(repoRoot, audit.sourceSurface.entryPath)),
    `packages/shared-demo-app-audit.json references missing source entry "${audit.sourceSurface.entryPath}".`,
    errors,
  );
  expect(
    fs.existsSync(path.join(repoRoot, audit.sourceSurface.stylePath)),
    `packages/shared-demo-app-audit.json references missing source stylesheet "${audit.sourceSurface.stylePath}".`,
    errors,
  );
  expect(
    audit.sourceSurface.entryLineCount >= 2000,
    "packages/shared-demo-app-audit.json must record the current monolithic App.svelte posture honestly.",
    errors,
  );
  expect(
    audit.sourceSurface.styleLineCount >= 1500,
    "packages/shared-demo-app-audit.json must record the current global app.css posture honestly.",
    errors,
  );
  expect(
    audit.packageCoverage.artifactsSource === "packages/svelte/preview/artifacts/parity-report.json",
    "packages/shared-demo-app-audit.json must use packages/svelte/preview/artifacts/parity-report.json as its coverage source.",
    errors,
  );
  expect(
    audit.auditFindings.length >= 5,
    "packages/shared-demo-app-audit.json must record multiple audit findings.",
    errors,
  );
  expect(
    audit.targetShapeFreeze.appId === "shared-demo-app",
    "packages/shared-demo-app-audit.json must freeze the shared-demo-app target.",
    errors,
  );
  expect(
    audit.targetShapeFreeze.requiredScreens.length >= 5,
    "packages/shared-demo-app-audit.json must record the rebuilt demo screen set.",
    errors,
  );
  expect(
    audit.targetShapeFreeze.requiredRules.length >= 3,
    "packages/shared-demo-app-audit.json must record required target-shape rules.",
    errors,
  );
  expect(
    audit.targetShapeFreeze.nonGoals.length >= 2,
    "packages/shared-demo-app-audit.json must record explicit non-goals.",
    errors,
  );

  for (const sectionId of audit.sourceSurface.docsOnlySectionIds) {
    expect(
      docsSectionIds.has(sectionId),
      `packages/shared-demo-app-audit.json docs-only section "${sectionId}" is not a known docs section.`,
      errors,
    );
  }
  for (const sectionId of audit.sourceSurface.sharedTargetSectionIds) {
    expect(
      docsSectionIds.has(sectionId),
      `packages/shared-demo-app-audit.json target section "${sectionId}" is not a known docs section.`,
      errors,
    );
  }
  for (const sectionId of audit.sourceSurface.docsOnlySectionIds) {
    expect(
      !targetSectionSet.has(sectionId),
      `packages/shared-demo-app-audit.json section "${sectionId}" cannot be both docs-only and a shared target section.`,
      errors,
    );
  }
  expect(
    docsOnlySet.has("catalog-hub") && docsOnlySet.has("token-summary-section") && docsOnlySet.has("token-inspector"),
    "packages/shared-demo-app-audit.json must keep catalog-hub, token-summary-section, and token-inspector as docs-only sections.",
    errors,
  );

  const parityCoverage = new Map(
    parityReport.packageSurfaceCoverage.summary.map((entry) => [entry.packageName, entry]),
  );

  for (const packageEntry of audit.packageCoverage.packages) {
    const parityEntry = parityCoverage.get(packageEntry.packageName);
    expect(
      Boolean(parityEntry),
      `packages/shared-demo-app-audit.json references unknown package coverage "${packageEntry.packageName}".`,
      errors,
    );
    if (!parityEntry) {
      continue;
    }
    expect(
      packageEntry.exportCount === parityEntry.exportCount,
      `packages/shared-demo-app-audit.json package "${packageEntry.packageName}" exportCount must match the parity report.`,
      errors,
    );
    expect(
      packageEntry.previewedCount === parityEntry.previewedCount,
      `packages/shared-demo-app-audit.json package "${packageEntry.packageName}" previewedCount must match the parity report.`,
      errors,
    );
    expect(
      packageEntry.contractOnlyCount === parityEntry.contractOnlyCount,
      `packages/shared-demo-app-audit.json package "${packageEntry.packageName}" contractOnlyCount must match the parity report.`,
      errors,
    );
    expect(
      packageEntry.contractOnlyCount === 0 || packageEntry.priorityMissingExports.length > 0,
      `packages/shared-demo-app-audit.json package "${packageEntry.packageName}" must record priority missing exports when contract-only exports remain.`,
      errors,
    );
  }

  for (const finding of audit.auditFindings) {
    expect(finding.id.trim().length > 0, "Shared demo-app audit findings must have ids.", errors);
    expect(
      ["high", "medium", "low"].includes(finding.severity),
      `Shared demo-app audit finding "${finding.id}" has unsupported severity "${finding.severity}".`,
      errors,
    );
    expect(
      finding.summary.trim().length > 0,
      `Shared demo-app audit finding "${finding.id}" must record a summary.`,
      errors,
    );
    expect(
      finding.evidence.length > 0,
      `Shared demo-app audit finding "${finding.id}" must record evidence paths.`,
      errors,
    );
    expect(
      finding.impact.trim().length > 0,
      `Shared demo-app audit finding "${finding.id}" must record impact.`,
      errors,
    );
    for (const evidencePath of finding.evidence) {
      expect(
        fs.existsSync(path.join(repoRoot, evidencePath)),
        `Shared demo-app audit finding "${finding.id}" references missing evidence path "${evidencePath}".`,
        errors,
      );
    }
  }

  const requiredScreenIds = new Set<string>();
  for (const screen of audit.targetShapeFreeze.requiredScreens) {
    expect(screen.id.trim().length > 0, "Shared demo-app target screens must have ids.", errors);
    expect(screen.goal.trim().length > 0, `Shared demo-app target screen "${screen.id}" must record a goal.`, errors);
    expect(
      !requiredScreenIds.has(screen.id),
      `packages/shared-demo-app-audit.json repeats target screen "${screen.id}".`,
      errors,
    );
    requiredScreenIds.add(screen.id);
  }

  return {
    demoAuditFindingCount: audit.auditFindings.length,
    demoAuditScreenCount: audit.targetShapeFreeze.requiredScreens.length,
  };
}

function validateSharedDemoAppContract(errors: string[]): {
  demoContractScreenCount: number;
  demoContractRegionCount: number;
} {
  const audit = JSON.parse(fs.readFileSync(sharedDemoAppAuditPath, "utf8")) as {
    targetShapeFreeze: {
      appId: string;
      requiredScreens: Array<{ id: string }>;
    };
    sourceSurface: {
      docsOnlySectionIds: string[];
      sharedTargetSectionIds: string[];
    };
  };
  const contract = JSON.parse(fs.readFileSync(sharedDemoAppContractPath, "utf8")) as {
    generation: string;
    appId: string;
    dependsOnAudit: string;
    reviewDimensions: string[];
    docsShellBoundary: {
      docsOnlySectionIds: string[];
      rules: string[];
    };
    shellRegions: Array<{
      id: string;
      purpose: string;
    }>;
    screenContracts: Array<{
      id: string;
      title: string;
      sourceSectionIds: string[];
      regionIds: string[];
      componentExpectations: string[];
      stateMatrix: string[];
      interactionCheckpoints: string[];
      comparisonMode: string;
    }>;
    parityChecklist: string[];
    runtimeBindings: {
      svelte: {
        status: string;
        milestone: string;
        implementationRoot: string;
      };
      gpui: {
        status: string;
        milestone: string;
        implementationRoot: string;
      };
    };
    nonGoals: string[];
  };
  const knownDocsSections = new Set(docsNavigationSections.map((entry) => entry.id));
  const auditScreenIds = audit.targetShapeFreeze.requiredScreens.map((screen) => screen.id).sort();
  const auditDocsOnlyIds = [...audit.sourceSurface.docsOnlySectionIds].sort();
  const auditTargetIds = new Set(audit.sourceSurface.sharedTargetSectionIds);
  const contractScreenIds = contract.screenContracts.map((screen) => screen.id).sort();
  const regionIds = new Set<string>();

  expect(
    contract.generation === "g04.013",
    "packages/shared-demo-app-contract.json must target g04.013.",
    errors,
  );
  expect(
    contract.appId === audit.targetShapeFreeze.appId,
    "packages/shared-demo-app-contract.json appId must match the shared demo-app audit.",
    errors,
  );
  expect(
    contract.dependsOnAudit === "packages/shared-demo-app-audit.json",
    "packages/shared-demo-app-contract.json must depend on packages/shared-demo-app-audit.json.",
    errors,
  );
  compareLists(
    "packages/shared-demo-app-contract.json review dimensions",
    [...contract.reviewDimensions].sort(),
    ["controlSize", "density", "theme"],
    errors,
  );
  compareLists(
    "packages/shared-demo-app-contract.json docs-only section ids",
    [...contract.docsShellBoundary.docsOnlySectionIds].sort(),
    auditDocsOnlyIds,
    errors,
  );
  compareLists(
    "packages/shared-demo-app-contract.json screen ids",
    contractScreenIds,
    auditScreenIds,
    errors,
  );
  expect(
    contract.docsShellBoundary.rules.length >= 2,
    "packages/shared-demo-app-contract.json must record docs-shell boundary rules.",
    errors,
  );
  expect(
    contract.shellRegions.length >= 5,
    "packages/shared-demo-app-contract.json must record shell regions.",
    errors,
  );
  expect(
    contract.parityChecklist.length >= 4,
    "packages/shared-demo-app-contract.json must record the parity checklist.",
    errors,
  );
  expect(
    contract.nonGoals.length >= 2,
    "packages/shared-demo-app-contract.json must record explicit non-goals.",
    errors,
  );

  for (const region of contract.shellRegions) {
    expect(region.id.trim().length > 0, "Shared demo-app contract regions must have ids.", errors);
    expect(region.purpose.trim().length > 0, `Shared demo-app contract region "${region.id}" must record a purpose.`, errors);
    expect(
      !regionIds.has(region.id),
      `packages/shared-demo-app-contract.json repeats shell region "${region.id}".`,
      errors,
    );
    regionIds.add(region.id);
  }

  for (const screen of contract.screenContracts) {
    expect(screen.title.trim().length > 0, `Shared demo-app contract screen "${screen.id}" must record a title.`, errors);
    expect(
      screen.sourceSectionIds.length > 0,
      `Shared demo-app contract screen "${screen.id}" must record source sections.`,
      errors,
    );
    expect(
      screen.regionIds.length > 0,
      `Shared demo-app contract screen "${screen.id}" must record shell regions.`,
      errors,
    );
    expect(
      screen.componentExpectations.length > 0,
      `Shared demo-app contract screen "${screen.id}" must record component expectations.`,
      errors,
    );
    expect(
      screen.stateMatrix.length >= 2,
      `Shared demo-app contract screen "${screen.id}" must record a state matrix.`,
      errors,
    );
    expect(
      screen.interactionCheckpoints.length > 0,
      `Shared demo-app contract screen "${screen.id}" must record interaction checkpoints.`,
      errors,
    );
    expect(
      ["direct-parity", "native-adaptation"].includes(screen.comparisonMode),
      `Shared demo-app contract screen "${screen.id}" has unsupported comparison mode "${screen.comparisonMode}".`,
      errors,
    );

    for (const sectionId of screen.sourceSectionIds) {
      expect(
        knownDocsSections.has(sectionId),
        `Shared demo-app contract screen "${screen.id}" references unknown section "${sectionId}".`,
        errors,
      );
      expect(
        auditTargetIds.has(sectionId),
        `Shared demo-app contract screen "${screen.id}" references non-target section "${sectionId}".`,
        errors,
      );
    }
    for (const regionId of screen.regionIds) {
      expect(
        regionIds.has(regionId),
        `Shared demo-app contract screen "${screen.id}" references unknown region "${regionId}".`,
        errors,
      );
    }
  }

  expect(
    contract.runtimeBindings.svelte.status === "rebuilt",
    "packages/shared-demo-app-contract.json must mark the Svelte demo as rebuilt.",
    errors,
  );
  expect(
    contract.runtimeBindings.svelte.milestone === "g04.014",
    "packages/shared-demo-app-contract.json must attach the Svelte demo to g04.014.",
    errors,
  );
  expect(
    contract.runtimeBindings.svelte.implementationRoot === "packages/svelte/preview/src/",
    "packages/shared-demo-app-contract.json must attach the Svelte demo implementation root to packages/svelte/preview/src/.",
    errors,
  );
  expect(
    contract.runtimeBindings.gpui.status === "implementation-required",
    "packages/shared-demo-app-contract.json must mark the GPUI demo as implementation-required.",
    errors,
  );
  expect(
    contract.runtimeBindings.gpui.milestone === "g04.015",
    "packages/shared-demo-app-contract.json must attach the GPUI demo to g04.015.",
    errors,
  );
  expect(
    contract.runtimeBindings.gpui.implementationRoot === "packages/gpui/",
    "packages/shared-demo-app-contract.json must attach the GPUI demo implementation root to packages/gpui/.",
    errors,
  );

  return {
    demoContractScreenCount: contract.screenContracts.length,
    demoContractRegionCount: contract.shellRegions.length,
  };
}

const errors: string[] = [];
const componentContractCount = validateComponentContracts(errors);
validateCurrentArchitectureReferences(errors);
validateContractIndexes(errors);
const operatorGuideCount = validateOperatorGuides(errors);
// Unified package — README surface validation skipped (130+ exports)
validatePackageSurfaceCoverage("packages/svelte/components", "@inflatable-cookie/poodle-svelte", errors);
validateDocsCatalog(errors);
validateParityCoverage(errors);
validateAccessibilityAudit(errors);
validateReleaseOperations(errors);
const ecosystemAcceptanceCounts = validateEcosystemAcceptance(errors);
const referenceAppsCounts = validateReferenceApps(errors);
const gpuiPreviewCounts = validateGpuiPreviewBaseline(errors);
const gpuiNativeAccessibilityCounts = validateGpuiNativeAccessibilityProof(errors);
const gpuiCrossRuntimeRouteCounts = validateGpuiCrossRuntimeParityReport(errors);
const sharedDemoAppAuditCounts = validateSharedDemoAppAudit(errors);
const sharedDemoAppContractCounts = validateSharedDemoAppContract(errors);

// Contract <-> Svelte prop-surface drift: share the standalone gate's complete
// two-direction result, including missing contracts or props tables.
const contractDriftResult = contractPropDrift();
errors.push(...contractDriftErrors(contractDriftResult));

// Contract <-> Svelte callback drift: every callback a component emits must be
// named in its contract. contract-prop-drift skips `on*` props by design, so
// this is the only thing that checks them.
// Self-referential container queries (g13-043).
for (const e of containerQueryDriftErrors()) {
  errors.push(e);
}

const callbackDriftResult = contractCallbackDrift();
for (const f of callbackDriftResult.findings) {
  errors.push(
    f.noSection
      ? `contract callback drift: ${f.slug}.md has no Callbacks/Events section, but the ${f.slug} Svelte component emits: ${f.undocumented.join(", ")}`
      : `contract callback drift: the ${f.slug} Svelte component emits callback(s) its contract does not document: ${f.undocumented.join(", ")}`,
  );
}

// Contract <-> poodle-specs drift: every documented Public Prop must also reach
// the shared spec surface, or neither native target can render it.
const specDriftResult = contractSpecDrift();
for (const f of specDriftResult.findings) {
  errors.push(
    `contract/spec drift: ${f.slug}.md documents prop(s) absent from its poodle-specs Spec: ${f.missing.join(", ")}`,
  );
}

// Focus-ring radius drift (g13.037 R4): a focus outline follows the element's
// own border-radius, so a ring element with no radius anywhere renders square.
// The baseline holds rings that are square by intent, each with a reason.
for (const f of focusRingDriftErrors()) {
  errors.push(f);
}

if (errors.length > 0) {
  throw new Error(errors.join("\n"));
}

console.log(
  `Validated ${componentContractCount} component contracts, ${operatorGuideCount} operator guides, ${docsSections.length} docs sections, ${docsFamilies.length} docs families, ${parityTargets.length} parity targets, ${accessibilityAuditTargets.length} accessibility audit targets, ${ecosystemAcceptanceCounts.suiteCount} ecosystem acceptance suites, ${ecosystemAcceptanceCounts.regressionClassCount} regression classes, ${referenceAppsCounts.shapeCount} reference shapes, ${referenceAppsCounts.laneCount} onboarding lanes, ${gpuiPreviewCounts.previewSectionCount} GPUI preview baseline sections, ${gpuiNativeAccessibilityCounts.gpuiAccessibilityLayerCount} GPUI accessibility-proof layers, ${gpuiNativeAccessibilityCounts.gpuiAccessibilitySectionCount} GPUI accessibility-proof sections, ${gpuiCrossRuntimeRouteCounts.gpuiCrossRuntimeRouteCount} GPUI headless construction routes, ${gpuiCrossRuntimeRouteCounts.gpuiCrossRuntimeDeltaCount} GPUI current known deltas, ${sharedDemoAppAuditCounts.demoAuditFindingCount} shared demo-app audit findings, ${sharedDemoAppAuditCounts.demoAuditScreenCount} shared demo target screens, ${sharedDemoAppContractCounts.demoContractScreenCount} shared demo contract screens, ${sharedDemoAppContractCounts.demoContractRegionCount} shared demo shell regions, ${contractDriftResult.checked} contract<->Svelte prop surfaces, ${callbackDriftResult.checked} contract<->Svelte callback surfaces, and ${specDriftResult.checked} contract<->spec prop surfaces.`,
);
