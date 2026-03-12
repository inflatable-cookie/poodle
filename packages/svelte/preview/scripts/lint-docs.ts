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

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const previewDir = path.resolve(scriptDir, "..");
const repoRoot = path.resolve(previewDir, "../../..");
const contractsDir = path.join(repoRoot, "docs", "contracts");
const releaseManifestPath = path.join(repoRoot, "packages", "release-manifest.json");
const releaseOperationsPath = path.join(repoRoot, "packages", "release-operations.json");
const ecosystemAcceptancePath = path.join(repoRoot, "packages", "ecosystem-acceptance.json");
const referenceAppsPath = path.join(repoRoot, "packages", "reference-apps.json");
const g03CloseoutPath = path.join(repoRoot, "packages", "g03-closeout.json");

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

function parseCargoPugMetadata(source: string): {
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
    ...collectMarkdownFiles(path.join(contractsDir, "foundation")).filter((file) => !file.endsWith("README.md")),
    ...collectMarkdownFiles(path.join(contractsDir, "composites")).filter((file) => !file.endsWith("README.md")),
    ...collectMarkdownFiles(path.join(contractsDir, "workstation")).filter((file) => !file.endsWith("README.md")),
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
    expect(markdown.includes("## 1. Purpose"), `${relativePath} is missing a purpose section.`, errors);
    expect(
      /^##\s+\d+\.\s+Accessibility$/m.test(markdown),
      `${relativePath} is missing an accessibility section.`,
      errors,
    );
    expect(
      /^##(?:\s+\d+\.)?\s+Next Task$/m.test(markdown),
      `${relativePath} is missing a next-task section.`,
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

function validateContractIndexes(errors: string[]): void {
  const foundationContracts = collectMarkdownFiles(path.join(contractsDir, "foundation"))
    .map((file) => path.basename(file));
  const compositeContracts = collectMarkdownFiles(path.join(contractsDir, "composites"))
    .map((file) => path.basename(file));
  const workstationContracts = collectMarkdownFiles(path.join(contractsDir, "workstation"))
    .map((file) => path.basename(file));

  compareLists(
    "docs/contracts/foundation/README.md current contracts",
    parseBulletList(
      fs.readFileSync(path.join(contractsDir, "foundation", "README.md"), "utf8"),
      "## Current Contracts",
    ),
    foundationContracts.filter((file) => file !== "README.md"),
    errors,
  );

  compareLists(
    "docs/contracts/composites/README.md current contracts",
    parseBulletList(
      fs.readFileSync(path.join(contractsDir, "composites", "README.md"), "utf8"),
      "## Current Contracts",
    ),
    compositeContracts.filter((file) => file !== "README.md"),
    errors,
  );

  compareLists(
    "docs/contracts/workstation/README.md current contracts",
    parseBulletList(
      fs.readFileSync(path.join(contractsDir, "workstation", "README.md"), "utf8"),
      "## Current Contracts",
    ),
    workstationContracts.filter((file) => file !== "README.md"),
    errors,
  );

  compareLists(
    "docs/contracts/README.md current seed contracts",
    parseBulletList(
      fs.readFileSync(path.join(contractsDir, "README.md"), "utf8"),
      "## Current Seed Contracts",
    ),
    [
      "template/component-contract-template.md",
      ...foundationContracts.map((file) => `foundation/${file}`),
      ...compositeContracts.map((file) => `composites/${file}`),
      ...workstationContracts.map((file) => `workstation/${file}`),
    ],
    errors,
  );
}

function validateSveltePackageSurface(
  packagePath: string,
  contractsLayer: "foundation" | "composites" | "workstation",
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
    collectMarkdownFiles(path.join(contractsDir, contractsLayer))
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
  packageName: "@pug/svelte-primitives" | "@pug/svelte-composites" | "@pug/svelte-workstation",
  errors: string[],
): void {
  const indexSource = fs.readFileSync(path.join(repoRoot, packagePath, "src", "index.ts"), "utf8");
  const componentExports = parseDefaultComponentExports(indexSource);
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
        pugRelease?: {
          publicIntent?: boolean;
          channel?: string;
          stability?: string;
        };
      };

      expect(packageJson.name === manifestEntry.name, `${packageJsonPath} name does not match release manifest.`, errors);
      expect(Boolean(packageJson.pugRelease), `${packageJsonPath} is missing pugRelease metadata.`, errors);

      if (packageJson.pugRelease) {
        expect(
          packageJson.pugRelease.publicIntent === manifestEntry.publicIntent,
          `${packageJsonPath} pugRelease.publicIntent does not match release manifest.`,
          errors,
        );
        expect(
          packageJson.pugRelease.channel === manifestEntry.channel,
          `${packageJsonPath} pugRelease.channel does not match release manifest.`,
          errors,
        );

        if (manifestEntry.channel === "preview") {
          expect(
            packageJson.pugRelease.stability === "pre-release",
            `${packageJsonPath} preview packages must use stability "pre-release".`,
            errors,
          );
        }
      }
    } else if (manifestEntry.language === "rust") {
      const cargoPath = path.join(repoRoot, manifestEntry.path, "Cargo.toml");
      const cargoMetadata = parseCargoPugMetadata(fs.readFileSync(cargoPath, "utf8"));

      expect(cargoMetadata.name === manifestEntry.name, `${cargoPath} package name does not match release manifest.`, errors);
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
    "underlay-bridge-adoption",
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
    "underlay-bridge-consumer",
    "workstation-foundation-consumer",
    "public-example-surface",
  ];
  const requiredOnboardingLaneIds = [
    "evaluate",
    "direct-adoption",
    "bridge-adoption",
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

function validateG03Closeout(errors: string[]): { stableSurfaceCount: number; carryForwardCount: number } {
  const closeout = JSON.parse(fs.readFileSync(g03CloseoutPath, "utf8")) as {
    generation: string;
    status: string;
    completedMilestones: string[];
    stableSurfaces: Array<{
      id: string;
      label: string;
      evidencePaths: string[];
    }>;
    carryForwardGaps: Array<{
      id: string;
      summary: string;
      nextProgramReason: string;
    }>;
    nextProgramPosture: {
      status: string;
      guidance: string[];
    };
  };
  const expectedMilestones = Array.from({ length: 14 }, (_, index) => `g03.${String(index + 1).padStart(3, "0")}`);
  const requiredStableSurfaceIds = [
    "tokens-and-artifacts",
    "contract-backed-svelte-surface",
    "docs-and-preview-evidence",
    "adoption-boundaries",
    "operations-and-adoption-guidance",
  ];
  const requiredCarryForwardIds = [
    "gpui-component-parity",
    "published-docs-platform",
    "downstream-runnable-reference-apps",
    "deeper-automation",
  ];
  const stableSurfaceIds = closeout.stableSurfaces.map((entry) => entry.id);
  const carryForwardIds = closeout.carryForwardGaps.map((entry) => entry.id);
  const stableSurfaceIdSet = new Set<string>();
  const carryForwardIdSet = new Set<string>();

  expect(closeout.generation === "g03", "packages/g03-closeout.json must target generation g03.", errors);
  expect(closeout.status === "completed", "packages/g03-closeout.json must mark g03 as completed.", errors);

  compareLists(
    "packages/g03-closeout.json completed milestones",
    [...closeout.completedMilestones].sort(),
    [...expectedMilestones].sort(),
    errors,
  );
  compareLists(
    "packages/g03-closeout.json stable surfaces",
    [...stableSurfaceIds].sort(),
    [...requiredStableSurfaceIds].sort(),
    errors,
  );
  compareLists(
    "packages/g03-closeout.json carry-forward gaps",
    [...carryForwardIds].sort(),
    [...requiredCarryForwardIds].sort(),
    errors,
  );

  for (const surface of closeout.stableSurfaces) {
    expect(!stableSurfaceIdSet.has(surface.id), `g03 closeout duplicates stable surface "${surface.id}".`, errors);
    stableSurfaceIdSet.add(surface.id);
    expect(surface.label.trim().length > 0, `g03 closeout stable surface "${surface.id}" is missing a label.`, errors);
    expect(surface.evidencePaths.length > 0, `g03 closeout stable surface "${surface.id}" is missing evidence paths.`, errors);

    for (const evidencePath of surface.evidencePaths) {
      expect(
        fs.existsSync(path.join(repoRoot, evidencePath)),
        `g03 closeout stable surface "${surface.id}" references missing evidence path "${evidencePath}".`,
        errors,
      );
    }
  }

  for (const gap of closeout.carryForwardGaps) {
    expect(!carryForwardIdSet.has(gap.id), `g03 closeout duplicates carry-forward gap "${gap.id}".`, errors);
    carryForwardIdSet.add(gap.id);
    expect(gap.summary.trim().length > 0, `g03 closeout gap "${gap.id}" is missing summary text.`, errors);
    expect(
      gap.nextProgramReason.trim().length > 0,
      `g03 closeout gap "${gap.id}" is missing next-program rationale.`,
      errors,
    );
  }

  expect(
    closeout.nextProgramPosture.status === "not-yet-opened",
    "packages/g03-closeout.json must keep next-program posture as not-yet-opened.",
    errors,
  );
  expect(
    closeout.nextProgramPosture.guidance.length >= 3,
    "packages/g03-closeout.json must include explicit next-program guidance.",
    errors,
  );

  return {
    stableSurfaceCount: closeout.stableSurfaces.length,
    carryForwardCount: closeout.carryForwardGaps.length,
  };
}

const errors: string[] = [];
const componentContractCount = validateComponentContracts(errors);
validateContractIndexes(errors);
validateSveltePackageSurface("packages/svelte/primitives", "foundation", "@pug/svelte-primitives", errors);
validateSveltePackageSurface("packages/svelte/composites", "composites", "@pug/svelte-composites", errors);
validateSveltePackageSurface("packages/svelte/workstation", "workstation", "@pug/svelte-workstation", errors);
validatePackageSurfaceCoverage("packages/svelte/primitives", "@pug/svelte-primitives", errors);
validatePackageSurfaceCoverage("packages/svelte/composites", "@pug/svelte-composites", errors);
validatePackageSurfaceCoverage("packages/svelte/workstation", "@pug/svelte-workstation", errors);
validateDocsCatalog(errors);
validateParityCoverage(errors);
validateAccessibilityAudit(errors);
validateReleaseOperations(errors);
const ecosystemAcceptanceCounts = validateEcosystemAcceptance(errors);
const referenceAppsCounts = validateReferenceApps(errors);
const g03CloseoutCounts = validateG03Closeout(errors);

if (errors.length > 0) {
  throw new Error(errors.join("\n"));
}

console.log(
  `Validated ${componentContractCount} component contracts, ${docsSections.length} docs sections, ${docsFamilies.length} docs families, ${parityTargets.length} parity targets, ${accessibilityAuditTargets.length} accessibility audit targets, ${ecosystemAcceptanceCounts.suiteCount} ecosystem acceptance suites, ${ecosystemAcceptanceCounts.regressionClassCount} regression classes, ${referenceAppsCounts.shapeCount} reference shapes, ${referenceAppsCounts.laneCount} onboarding lanes, ${g03CloseoutCounts.stableSurfaceCount} closeout surfaces, and ${g03CloseoutCounts.carryForwardCount} carry-forward gaps.`,
);
