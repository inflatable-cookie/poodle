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
const gpuiParityPriorityPath = path.join(repoRoot, "packages", "gpui", "parity-priority-matrix.json");
const gpuiPreviewBaselinePath = path.join(repoRoot, "packages", "gpui", "preview-app-baseline.json");
const gpuiStructuralBaselinePath = path.join(repoRoot, "packages", "gpui", "structural-primitives-baseline.json");
const gpuiActionFieldBaselinePath = path.join(repoRoot, "packages", "gpui", "action-field-primitives-baseline.json");
const gpuiSelectionFeedbackDateBaselinePath = path.join(repoRoot, "packages", "gpui", "selection-feedback-date-baseline.json");
const gpuiOverlayNavigationMenuBaselinePath = path.join(repoRoot, "packages", "gpui", "overlay-navigation-menu-baseline.json");
const gpuiFormValidationRemediationBaselinePath = path.join(
  repoRoot,
  "packages",
  "gpui",
  "form-validation-remediation-composites-baseline.json",
);
const gpuiDataBrowseDetailPickerMediaBaselinePath = path.join(
  repoRoot,
  "packages",
  "gpui",
  "data-browse-detail-picker-media-baseline.json",
);
const gpuiWorkstationBaselinePath = path.join(
  repoRoot,
  "packages",
  "gpui",
  "workstation-shell-command-layout-baseline.json",
);
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
const workstationContractsPath = path.join(contractsDir, "workstation");
const hasWorkstationContracts = fs.existsSync(workstationContractsPath);
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
    ...collectMarkdownFiles(path.join(contractsDir, "foundation")).filter((file) => !file.endsWith("README.md")),
    ...collectMarkdownFiles(path.join(contractsDir, "composites")).filter((file) => !file.endsWith("README.md")),
    ...(hasWorkstationContracts
      ? collectMarkdownFiles(workstationContractsPath).filter((file) => !file.endsWith("README.md"))
      : []),
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
  const workstationContracts = hasWorkstationContracts
    ? collectMarkdownFiles(workstationContractsPath).map((file) => path.basename(file))
    : [];

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

  if (hasWorkstationContracts) {
    compareLists(
      "docs/contracts/workstation/README.md current contracts",
      parseBulletList(
        fs.readFileSync(path.join(workstationContractsPath, "README.md"), "utf8"),
        "## Current Contracts",
      ),
      workstationContracts.filter((file) => file !== "README.md"),
      errors,
    );
  }

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
  packageName: "@poodle/svelte-primitives" | "@poodle/svelte-composites" | "@poodle/svelte-composites",
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
        poodleRelease?: {
          publicIntent?: boolean;
          channel?: string;
          stability?: string;
        };
      };

      expect(packageJson.name === manifestEntry.name, `${packageJsonPath} name does not match release manifest.`, errors);
      expect(Boolean(packageJson.poodleRelease), `${packageJsonPath} is missing poodleRelease metadata.`, errors);

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
            packageJson.poodleRelease.stability === "pre-release",
            `${packageJsonPath} preview packages must use stability "pre-release".`,
            errors,
          );
        }
      }
    } else if (manifestEntry.language === "rust") {
      const cargoPath = path.join(repoRoot, manifestEntry.path, "Cargo.toml");
      const cargoMetadata = parseCargoPoodleMetadata(fs.readFileSync(cargoPath, "utf8"));

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

function validateGpuiPriorityMatrix(errors: string[]): { waveCount: number; targetCount: number } {
  const matrix = JSON.parse(fs.readFileSync(gpuiParityPriorityPath, "utf8")) as {
    generation: string;
    implementationWaves: Array<{
      id: string;
      label: string;
      goal: string;
      sectionIds: string[];
      packageFocus: string[];
    }>;
    sectionTargets: Array<{
      sectionId: string;
      priority: string;
      parityMode: string;
      gpuiLayer: string;
      sideBySideReview: boolean;
      reasons: string[];
    }>;
    nonGoals: string[];
  };
  const expectedWaveIds = [
    "wave-0-theme-and-preview",
    "wave-1-foundation-primitives",
    "wave-2-product-composites",
    "wave-3-workstation-shell",
  ];
  const expectedSectionIds = ["catalog-hub", ...docsSections.map((entry) => entry.id)];
  const docsSectionIds = new Set(expectedSectionIds);
  const waveIds = matrix.implementationWaves.map((entry) => entry.id);
  const targetSectionIds = matrix.sectionTargets.map((entry) => entry.sectionId);
  const waveIdSet = new Set<string>();
  const targetIdSet = new Set<string>();

  expect(matrix.generation === "g04.001", "packages/gpui/parity-priority-matrix.json must target g04.001.", errors);
  compareLists(
    "packages/gpui/parity-priority-matrix.json waves",
    [...waveIds].sort(),
    [...expectedWaveIds].sort(),
    errors,
  );
  compareLists(
    "packages/gpui/parity-priority-matrix.json section targets",
    [...targetSectionIds].sort(),
    [...expectedSectionIds].sort(),
    errors,
  );

  for (const wave of matrix.implementationWaves) {
    expect(!waveIdSet.has(wave.id), `GPUI priority matrix duplicates wave "${wave.id}".`, errors);
    waveIdSet.add(wave.id);
    expect(wave.label.trim().length > 0, `GPUI priority wave "${wave.id}" is missing a label.`, errors);
    expect(wave.goal.trim().length > 0, `GPUI priority wave "${wave.id}" is missing goal text.`, errors);
    expect(wave.sectionIds.length > 0, `GPUI priority wave "${wave.id}" is missing section coverage.`, errors);
    expect(wave.packageFocus.length > 0, `GPUI priority wave "${wave.id}" is missing package focus.`, errors);

    for (const sectionId of wave.sectionIds) {
      expect(
        docsSectionIds.has(sectionId),
        `GPUI priority wave "${wave.id}" references unknown section "${sectionId}".`,
        errors,
      );
    }
  }

  for (const target of matrix.sectionTargets) {
    expect(!targetIdSet.has(target.sectionId), `GPUI priority matrix duplicates target "${target.sectionId}".`, errors);
    targetIdSet.add(target.sectionId);
    expect(
      docsSectionIds.has(target.sectionId),
      `GPUI priority matrix references unknown section "${target.sectionId}".`,
      errors,
    );
    expect(
      ["highest", "high", "medium", "low"].includes(target.priority),
      `GPUI priority target "${target.sectionId}" has unsupported priority "${target.priority}".`,
      errors,
    );
    expect(
      ["direct-parity", "native-adaptation", "deferred"].includes(target.parityMode),
      `GPUI priority target "${target.sectionId}" has unsupported parity mode "${target.parityMode}".`,
      errors,
    );
    expect(
      target.gpuiLayer.trim().length > 0,
      `GPUI priority target "${target.sectionId}" is missing a GPUI layer.`,
      errors,
    );
    expect(
      target.reasons.length > 0,
      `GPUI priority target "${target.sectionId}" is missing rationale.`,
      errors,
    );
  }

  expect(matrix.nonGoals.length >= 3, "GPUI priority matrix must record explicit non-goals.", errors);

  return {
    waveCount: matrix.implementationWaves.length,
    targetCount: matrix.sectionTargets.length,
  };
}

function validateGpuiPreviewBaseline(errors: string[]): { previewSectionCount: number } {
  const matrix = JSON.parse(fs.readFileSync(gpuiParityPriorityPath, "utf8")) as {
    implementationWaves: Array<{ id: string; sectionIds: string[] }>;
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
  const wave0SectionIds =
    matrix.implementationWaves.find((entry) => entry.id === "wave-0-theme-and-preview")?.sectionIds ?? [];

  expect(previewBaseline.generation === "g04.002", "packages/gpui/preview-app-baseline.json must target g04.002.", errors);
  expect(
    previewBaseline.themeRuntime.tokenSource === gpuiTokenSource,
    "packages/gpui/preview-app-baseline.json must use poodle-tokens as token source.",
    errors,
  );
  compareLists(
    "packages/gpui/preview-app-baseline.json theme ids",
    [...previewBaseline.themeRuntime.themeIds].sort(),
    ["dark", "light", "loophole-studio"],
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
    [...wave0SectionIds].sort(),
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

function validateGpuiStructuralBaseline(errors: string[]): { structuralExportCount: number } {
  const structuralBaseline = JSON.parse(fs.readFileSync(gpuiStructuralBaselinePath, "utf8")) as {
    generation: string;
    crateName: string;
    cratePath: string;
    tokenSource: string;
    contractIds: string[];
    exportNames: string[];
    knownDeltas: string[];
    nonGoals: string[];
  };
  const expectedContractIds = ["box", "grid", "scroll-shell", "separator", "stack", "surface"];
  const expectedExportNames = [
    "BoxSpec",
    "GridSpec",
    "ScrollShellSpec",
    "SeparatorSpec",
    "StackSpec",
    "SurfaceSpec",
  ];
  const crateRoot = path.join(repoRoot, structuralBaseline.cratePath);

  expect(
    structuralBaseline.generation === "g04.003",
    "packages/gpui/structural-primitives-baseline.json must target g04.003.",
    errors,
  );
  expect(
    structuralBaseline.crateName === gpuiAdapterCrateName,
    "packages/gpui/structural-primitives-baseline.json must target the poodle-gpui adapter crate.",
    errors,
  );
  expect(
    structuralBaseline.tokenSource === gpuiTokenSource,
    "packages/gpui/structural-primitives-baseline.json must use poodle-tokens as token source.",
    errors,
  );
  compareLists(
    "packages/gpui/structural-primitives-baseline.json contract coverage",
    [...structuralBaseline.contractIds].sort(),
    [...expectedContractIds].sort(),
    errors,
  );
  compareLists(
    "packages/gpui/structural-primitives-baseline.json export names",
    [...structuralBaseline.exportNames].sort(),
    [...expectedExportNames].sort(),
    errors,
  );
  expect(
    fs.existsSync(path.join(crateRoot, "Cargo.toml")),
    `packages/gpui/structural-primitives-baseline.json references missing crate manifest "${structuralBaseline.cratePath}/Cargo.toml".`,
    errors,
  );
  expect(
    fs.existsSync(path.join(crateRoot, "README.md")),
    `packages/gpui/structural-primitives-baseline.json references missing crate README "${structuralBaseline.cratePath}/README.md".`,
    errors,
  );
  expect(
    fs.existsSync(path.join(crateRoot, "src", "lib.rs")),
    `packages/gpui/structural-primitives-baseline.json references missing crate source "${structuralBaseline.cratePath}/src/lib.rs".`,
    errors,
  );
  expect(
    structuralBaseline.knownDeltas.length >= 2,
    "packages/gpui/structural-primitives-baseline.json must record explicit known deltas.",
    errors,
  );
  expect(
    structuralBaseline.nonGoals.length >= 3,
    "packages/gpui/structural-primitives-baseline.json must record explicit non-goals.",
    errors,
  );

  return {
    structuralExportCount: structuralBaseline.exportNames.length,
  };
}

function validateGpuiActionFieldBaseline(errors: string[]): { actionFieldExportCount: number } {
  const actionFieldBaseline = JSON.parse(fs.readFileSync(gpuiActionFieldBaselinePath, "utf8")) as {
    generation: string;
    crateName: string;
    cratePath: string;
    tokenSource: string;
    contractIds: string[];
    exportNames: string[];
    knownDeltas: string[];
    nonGoals: string[];
  };
  const expectedContractIds = [
    "button",
    "field",
    "form-actions",
    "icon-button",
    "search-field",
    "text-area",
    "text-input",
  ];
  const expectedExportNames = [
    "ButtonSpec",
    "FieldRelationships",
    "FieldSpec",
    "FormActionsSpec",
    "IconButtonSpec",
    "SearchFieldSpec",
    "TextAreaSpec",
    "TextInputSpec",
  ];
  const crateRoot = path.join(repoRoot, actionFieldBaseline.cratePath);
  const libSource = fs.readFileSync(path.join(crateRoot, "src", "lib.rs"), "utf8");

  expect(
    actionFieldBaseline.generation === "g04.004",
    "packages/gpui/action-field-primitives-baseline.json must target g04.004.",
    errors,
  );
  expect(
    actionFieldBaseline.crateName === gpuiAdapterCrateName,
    "packages/gpui/action-field-primitives-baseline.json must target the poodle-gpui adapter crate.",
    errors,
  );
  expect(
    actionFieldBaseline.tokenSource === gpuiTokenSource,
    "packages/gpui/action-field-primitives-baseline.json must use poodle-tokens as token source.",
    errors,
  );
  compareLists(
    "packages/gpui/action-field-primitives-baseline.json contract coverage",
    [...actionFieldBaseline.contractIds].sort(),
    [...expectedContractIds].sort(),
    errors,
  );
  compareLists(
    "packages/gpui/action-field-primitives-baseline.json export names",
    [...actionFieldBaseline.exportNames].sort(),
    [...expectedExportNames].sort(),
    errors,
  );
  expect(
    fs.existsSync(path.join(crateRoot, "README.md")),
    `packages/gpui/action-field-primitives-baseline.json references missing crate README "${actionFieldBaseline.cratePath}/README.md".`,
    errors,
  );
  expect(
    actionFieldBaseline.knownDeltas.length >= 2,
    "packages/gpui/action-field-primitives-baseline.json must record explicit known deltas.",
    errors,
  );
  expect(
    actionFieldBaseline.nonGoals.length >= 3,
    "packages/gpui/action-field-primitives-baseline.json must record explicit non-goals.",
    errors,
  );

  for (const exportName of actionFieldBaseline.exportNames) {
    expect(
      libSource.includes(exportName),
      `packages/gpui/adapter/src/lib.rs must expose GPUI action/field export "${exportName}".`,
      errors,
    );
  }

  return {
    actionFieldExportCount: actionFieldBaseline.exportNames.length,
  };
}

function validateGpuiSelectionFeedbackDateBaseline(errors: string[]): { selectionFeedbackDateExportCount: number } {
  const baseline = JSON.parse(fs.readFileSync(gpuiSelectionFeedbackDateBaselinePath, "utf8")) as {
    generation: string;
    crateName: string;
    cratePath: string;
    tokenSource: string;
    contractIds: string[];
    exportNames: string[];
    knownDeltas: string[];
    nonGoals: string[];
  };
  const expectedContractIds = [
    "badge",
    "calendar",
    "checkbox",
    "date-picker",
    "date-range-picker",
    "date-time-picker",
    "date-time-range-picker",
    "progress",
    "radio-group",
    "range-calendar",
    "segmented-control",
    "select",
    "slider",
    "status-indicator",
    "switch",
    "time-field",
  ];
  const expectedExportNames = [
    "BadgeSpec",
    "CalendarSpec",
    "CheckboxSpec",
    "DatePickerSpec",
    "DateRangePickerSpec",
    "DateTimePickerSpec",
    "DateTimeRangePickerSpec",
    "ProgressSpec",
    "RadioGroupSpec",
    "RangeCalendarSpec",
    "SegmentedControlSpec",
    "SelectSpec",
    "SliderSpec",
    "StatusIndicatorSpec",
    "SwitchSpec",
    "TimeFieldSpec",
  ];
  const crateRoot = path.join(repoRoot, baseline.cratePath);
  const libSource = fs.readFileSync(path.join(crateRoot, "src", "lib.rs"), "utf8");

  expect(
    baseline.generation === "g04.005",
    "packages/gpui/selection-feedback-date-baseline.json must target g04.005.",
    errors,
  );
  expect(
    baseline.crateName === gpuiAdapterCrateName,
    "packages/gpui/selection-feedback-date-baseline.json must target the poodle-gpui adapter crate.",
    errors,
  );
  expect(
    baseline.tokenSource === gpuiTokenSource,
    "packages/gpui/selection-feedback-date-baseline.json must use poodle-tokens as token source.",
    errors,
  );
  compareLists(
    "packages/gpui/selection-feedback-date-baseline.json contract coverage",
    [...baseline.contractIds].sort(),
    [...expectedContractIds].sort(),
    errors,
  );
  compareLists(
    "packages/gpui/selection-feedback-date-baseline.json export names",
    [...baseline.exportNames].sort(),
    [...expectedExportNames].sort(),
    errors,
  );
  expect(
    baseline.knownDeltas.length >= 2,
    "packages/gpui/selection-feedback-date-baseline.json must record explicit known deltas.",
    errors,
  );
  expect(
    baseline.nonGoals.length >= 3,
    "packages/gpui/selection-feedback-date-baseline.json must record explicit non-goals.",
    errors,
  );

  for (const exportName of baseline.exportNames) {
    expect(
      libSource.includes(exportName),
      `packages/gpui/adapter/src/lib.rs must expose GPUI selection/feedback/date export "${exportName}".`,
      errors,
    );
  }

  return {
    selectionFeedbackDateExportCount: baseline.exportNames.length,
  };
}

function validateGpuiOverlayNavigationMenuBaseline(errors: string[]): { overlayNavigationMenuExportCount: number } {
  const baseline = JSON.parse(fs.readFileSync(gpuiOverlayNavigationMenuBaselinePath, "utf8")) as {
    generation: string;
    crateName: string;
    cratePath: string;
    tokenSource: string;
    contractIds: string[];
    exportNames: string[];
    knownDeltas: string[];
    nonGoals: string[];
  };
  const expectedContractIds = [
    "accordion",
    "collapsible",
    "context-menu",
    "dialog",
    "drawer",
    "menu",
    "menubar",
    "navigation-menu",
    "popover",
    "tab-strip",
    "tabs",
    "tooltip",
  ];
  const expectedExportNames = [
    "AccordionSpec",
    "CollapsibleSpec",
    "ContextMenuSpec",
    "DialogSpec",
    "DrawerSpec",
    "MenuSpec",
    "MenubarSpec",
    "NavigationMenuSpec",
    "PopoverSpec",
    "TabStripSpec",
    "TabsSpec",
    "TooltipSpec",
  ];
  const crateRoot = path.join(repoRoot, baseline.cratePath);
  const libSource = fs.readFileSync(path.join(crateRoot, "src", "lib.rs"), "utf8");

  expect(
    baseline.generation === "g04.006",
    "packages/gpui/overlay-navigation-menu-baseline.json must target g04.006.",
    errors,
  );
  expect(
    baseline.crateName === gpuiAdapterCrateName,
    "packages/gpui/overlay-navigation-menu-baseline.json must target the poodle-gpui adapter crate.",
    errors,
  );
  expect(
    baseline.tokenSource === gpuiTokenSource,
    "packages/gpui/overlay-navigation-menu-baseline.json must use poodle-tokens as token source.",
    errors,
  );
  compareLists(
    "packages/gpui/overlay-navigation-menu-baseline.json contract coverage",
    [...baseline.contractIds].sort(),
    [...expectedContractIds].sort(),
    errors,
  );
  compareLists(
    "packages/gpui/overlay-navigation-menu-baseline.json export names",
    [...baseline.exportNames].sort(),
    [...expectedExportNames].sort(),
    errors,
  );
  expect(
    baseline.knownDeltas.length >= 2,
    "packages/gpui/overlay-navigation-menu-baseline.json must record explicit known deltas.",
    errors,
  );
  expect(
    baseline.nonGoals.length >= 3,
    "packages/gpui/overlay-navigation-menu-baseline.json must record explicit non-goals.",
    errors,
  );

  for (const exportName of baseline.exportNames) {
    expect(
      libSource.includes(exportName),
      `packages/gpui/adapter/src/lib.rs must expose GPUI overlay/navigation export "${exportName}".`,
      errors,
    );
  }

  return {
    overlayNavigationMenuExportCount: baseline.exportNames.length,
  };
}

function validateGpuiFormValidationRemediationBaseline(errors: string[]): { gpuiCompositeExportCount: number } {
  const baseline = JSON.parse(fs.readFileSync(gpuiFormValidationRemediationBaselinePath, "utf8")) as {
    generation: string;
    crateName: string;
    cratePath: string;
    tokenSource: string;
    contractIds: string[];
    exportNames: string[];
    knownDeltas: string[];
    nonGoals: string[];
  };
  const expectedContractIds = ["banner", "callout", "field", "form-actions"];
  const expectedExportNames = [
    "FormShellSpec",
    "InlineRemediationSpec",
    "RemediationBannerSpec",
    "ValidationSummarySpec",
  ];
  const crateRoot = path.join(repoRoot, baseline.cratePath);
  const libSource = fs.readFileSync(path.join(crateRoot, "src", "lib.rs"), "utf8");

  expect(
    baseline.generation === "g04.007",
    "packages/gpui/form-validation-remediation-composites-baseline.json must target g04.007.",
    errors,
  );
  expect(
    baseline.crateName === gpuiAdapterCrateName,
    "packages/gpui/form-validation-remediation-composites-baseline.json must target the poodle-gpui adapter crate.",
    errors,
  );
  expect(
    baseline.tokenSource === gpuiTokenSource,
    "packages/gpui/form-validation-remediation-composites-baseline.json must use poodle-tokens as token source.",
    errors,
  );
  compareLists(
    "packages/gpui/form-validation-remediation-composites-baseline.json contract coverage",
    [...baseline.contractIds].sort(),
    [...expectedContractIds].sort(),
    errors,
  );
  compareLists(
    "packages/gpui/form-validation-remediation-composites-baseline.json export names",
    [...baseline.exportNames].sort(),
    [...expectedExportNames].sort(),
    errors,
  );
  expect(
    fs.existsSync(path.join(crateRoot, "README.md")),
    `packages/gpui/form-validation-remediation-composites-baseline.json references missing crate README "${baseline.cratePath}/README.md".`,
    errors,
  );
  expect(
    baseline.knownDeltas.length >= 2,
    "packages/gpui/form-validation-remediation-composites-baseline.json must record explicit known deltas.",
    errors,
  );
  expect(
    baseline.nonGoals.length >= 3,
    "packages/gpui/form-validation-remediation-composites-baseline.json must record explicit non-goals.",
    errors,
  );

  for (const exportName of baseline.exportNames) {
    expect(
      libSource.includes(exportName),
      `packages/gpui/adapter/src/lib.rs must expose GPUI composite export "${exportName}".`,
      errors,
    );
  }

  return {
    gpuiCompositeExportCount: baseline.exportNames.length,
  };
}

function validateGpuiDataBrowseDetailPickerMediaBaseline(errors: string[]): { gpuiDataCompositeExportCount: number } {
  const baseline = JSON.parse(fs.readFileSync(gpuiDataBrowseDetailPickerMediaBaselinePath, "utf8")) as {
    generation: string;
    crateName: string;
    cratePath: string;
    tokenSource: string;
    contractIds: string[];
    exportNames: string[];
    knownDeltas: string[];
    nonGoals: string[];
  };
  const expectedContractIds = [
    "data-table",
    "detail-shell",
    "empty-state",
    "filter-toolbar",
    "media-preview",
    "media-thumbnail",
    "pagination-summary",
    "picker-shell",
    "relation-picker",
    "selection-summary",
  ];
  const expectedExportNames = [
    "DataTableSpec",
    "DetailShellSpec",
    "EmptyStateSpec",
    "FilterToolbarSpec",
    "MediaPreviewSpec",
    "MediaThumbnailSpec",
    "PaginationSummarySpec",
    "PickerShellSpec",
    "RelationPickerSpec",
    "SelectionSummarySpec",
  ];
  const crateRoot = path.join(repoRoot, baseline.cratePath);
  const libSource = fs.readFileSync(path.join(crateRoot, "src", "lib.rs"), "utf8");

  expect(
    baseline.generation === "g04.008",
    "packages/gpui/data-browse-detail-picker-media-baseline.json must target g04.008.",
    errors,
  );
  expect(
    baseline.crateName === gpuiAdapterCrateName,
    "packages/gpui/data-browse-detail-picker-media-baseline.json must target the poodle-gpui adapter crate.",
    errors,
  );
  expect(
    baseline.tokenSource === gpuiTokenSource,
    "packages/gpui/data-browse-detail-picker-media-baseline.json must use poodle-tokens as token source.",
    errors,
  );
  compareLists(
    "packages/gpui/data-browse-detail-picker-media-baseline.json contract coverage",
    [...baseline.contractIds].sort(),
    [...expectedContractIds].sort(),
    errors,
  );
  compareLists(
    "packages/gpui/data-browse-detail-picker-media-baseline.json export names",
    [...baseline.exportNames].sort(),
    [...expectedExportNames].sort(),
    errors,
  );
  expect(
    fs.existsSync(path.join(crateRoot, "README.md")),
    `packages/gpui/data-browse-detail-picker-media-baseline.json references missing crate README "${baseline.cratePath}/README.md".`,
    errors,
  );
  expect(
    baseline.knownDeltas.length >= 2,
    "packages/gpui/data-browse-detail-picker-media-baseline.json must record explicit known deltas.",
    errors,
  );
  expect(
    baseline.nonGoals.length >= 3,
    "packages/gpui/data-browse-detail-picker-media-baseline.json must record explicit non-goals.",
    errors,
  );

  for (const exportName of baseline.exportNames) {
    expect(
      libSource.includes(exportName),
      `packages/gpui/adapter/src/lib.rs must expose GPUI data/browse composite export "${exportName}".`,
      errors,
    );
  }

  return {
    gpuiDataCompositeExportCount: baseline.exportNames.length,
  };
}

function validateGpuiWorkstationBaseline(errors: string[]): { gpuiWorkstationExportCount: number } {
  const baseline = JSON.parse(fs.readFileSync(gpuiWorkstationBaselinePath, "utf8")) as {
    generation: string;
    crateName: string;
    cratePath: string;
    tokenSource: string;
    contractIds: string[];
    exportNames: string[];
    knownDeltas: string[];
    nonGoals: string[];
  };
  const expectedContractIds = [
    "action-discovery-panel",
    "app-header",
    "command-palette",
    "command-palette-shell",
    "dock-region",
    "panel-header",
    "panel-surface",
    "panel-tabs",
    "project-header",
    "shell-status-bar",
    "split-view",
    "surface-tabs",
    "workspace-shell",
  ];
  const expectedExportNames = [
    "ActionDiscoveryPanelSpec",
    "AppHeaderSpec",
    "CommandPaletteShellSpec",
    "CommandPaletteSpec",
    "DockRegionSpec",
    "PanelHeaderSpec",
    "PanelSurfaceSpec",
    "PanelTabsSpec",
    "ProjectHeaderSpec",
    "ShellStatusBarSpec",
    "SplitViewSpec",
    "SurfaceTabsSpec",
    "WorkspaceShellSpec",
  ];
  const crateRoot = path.join(repoRoot, baseline.cratePath);
  const libSource = fs.readFileSync(path.join(crateRoot, "src", "lib.rs"), "utf8");

  expect(
    baseline.generation === "g04.009",
    "packages/gpui/workstation-shell-command-layout-baseline.json must target g04.009.",
    errors,
  );
  expect(
    baseline.crateName === gpuiAdapterCrateName,
    "packages/gpui/workstation-shell-command-layout-baseline.json must target the poodle-gpui adapter crate.",
    errors,
  );
  expect(
    baseline.tokenSource === gpuiTokenSource,
    "packages/gpui/workstation-shell-command-layout-baseline.json must use poodle-tokens as token source.",
    errors,
  );
  compareLists(
    "packages/gpui/workstation-shell-command-layout-baseline.json contract coverage",
    [...baseline.contractIds].sort(),
    [...expectedContractIds].sort(),
    errors,
  );
  compareLists(
    "packages/gpui/workstation-shell-command-layout-baseline.json export names",
    [...baseline.exportNames].sort(),
    [...expectedExportNames].sort(),
    errors,
  );
  expect(
    fs.existsSync(path.join(crateRoot, "README.md")),
    `packages/gpui/workstation-shell-command-layout-baseline.json references missing crate README "${baseline.cratePath}/README.md".`,
    errors,
  );
  expect(
    baseline.knownDeltas.length >= 2,
    "packages/gpui/workstation-shell-command-layout-baseline.json must record explicit known deltas.",
    errors,
  );
  expect(
    baseline.nonGoals.length >= 3,
    "packages/gpui/workstation-shell-command-layout-baseline.json must record explicit non-goals.",
    errors,
  );

  for (const exportName of baseline.exportNames) {
    expect(
      libSource.includes(exportName),
      `packages/gpui/adapter/src/lib.rs must expose GPUI workstation export "${exportName}".`,
      errors,
    );
  }

  return {
    gpuiWorkstationExportCount: baseline.exportNames.length,
  };
}

function validateGpuiNativeAccessibilityProof(errors: string[]): {
  gpuiAccessibilityLayerCount: number;
  gpuiAccessibilitySectionCount: number;
} {
  const proof = JSON.parse(fs.readFileSync(gpuiNativeAccessibilityProofPath, "utf8")) as {
    generation: string;
    comparisonSource: string;
    sectionIds: string[];
    layerProof: Array<{
      id: string;
      crateName: string;
      cratePath: string;
      contractIds: string[];
      exportNames: string[];
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
      sideBySideReview: boolean;
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
  const primitiveExportNames = [
    ...JSON.parse(fs.readFileSync(gpuiStructuralBaselinePath, "utf8")).exportNames,
    ...JSON.parse(fs.readFileSync(gpuiActionFieldBaselinePath, "utf8")).exportNames,
    ...JSON.parse(fs.readFileSync(gpuiSelectionFeedbackDateBaselinePath, "utf8")).exportNames,
    ...JSON.parse(fs.readFileSync(gpuiOverlayNavigationMenuBaselinePath, "utf8")).exportNames,
  ].sort();
  const primitiveContractIds = [
    ...JSON.parse(fs.readFileSync(gpuiStructuralBaselinePath, "utf8")).contractIds,
    ...JSON.parse(fs.readFileSync(gpuiActionFieldBaselinePath, "utf8")).contractIds,
    ...JSON.parse(fs.readFileSync(gpuiSelectionFeedbackDateBaselinePath, "utf8")).contractIds,
    ...JSON.parse(fs.readFileSync(gpuiOverlayNavigationMenuBaselinePath, "utf8")).contractIds,
  ].sort();
  const compositeExportNames = [
    ...JSON.parse(fs.readFileSync(gpuiFormValidationRemediationBaselinePath, "utf8")).exportNames,
    ...JSON.parse(fs.readFileSync(gpuiDataBrowseDetailPickerMediaBaselinePath, "utf8")).exportNames,
  ].sort();
  const compositeContractIds = [
    ...JSON.parse(fs.readFileSync(gpuiFormValidationRemediationBaselinePath, "utf8")).contractIds,
    ...JSON.parse(fs.readFileSync(gpuiDataBrowseDetailPickerMediaBaselinePath, "utf8")).contractIds,
  ].sort();
  const workstationBaseline = JSON.parse(fs.readFileSync(gpuiWorkstationBaselinePath, "utf8")) as {
    exportNames: string[];
    contractIds: string[];
  };
  const sectionTargets = new Map(
    accessibilityAuditTargets
      .filter((target) => target.auditAreas.gpui !== "not-applicable")
      .map((target) => [target.sectionId, target]),
  );
  const gpuiPriorityMatrix = JSON.parse(fs.readFileSync(gpuiParityPriorityPath, "utf8")) as {
    sectionTargets: Array<{
      sectionId: string;
      sideBySideReview: boolean;
    }>;
  };
  const paritySections = new Map(
    gpuiPriorityMatrix.sectionTargets
      .filter((target) => expectedSectionIds.includes(target.sectionId))
      .map((target) => [target.sectionId, target]),
  );

  expect(
    proof.generation === "g04.010",
    "packages/gpui/native-accessibility-proof.json must target g04.010.",
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

  const expectedLayerData = new Map([
    [
      "primitives",
      {
        crateName: gpuiAdapterCrateName,
        cratePath: gpuiAdapterCratePath,
        exportNames: primitiveExportNames,
        contractIds: primitiveContractIds,
      },
    ],
    [
      "composites",
      {
        crateName: gpuiAdapterCrateName,
        cratePath: gpuiAdapterCratePath,
        exportNames: compositeExportNames,
        contractIds: compositeContractIds,
      },
    ],
    [
      "workstation",
      {
        crateName: gpuiAdapterCrateName,
        cratePath: gpuiAdapterCratePath,
      exportNames: [...workstationBaseline.exportNames].sort(),
      contractIds: [...workstationBaseline.contractIds].sort(),
      },
    ],
  ]);

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
    compareLists(
      `packages/gpui/native-accessibility-proof.json ${layer.id} export names`,
      [...layer.exportNames].sort(),
      expected.exportNames,
      errors,
    );
    compareLists(
      `packages/gpui/native-accessibility-proof.json ${layer.id} contract ids`,
      [...layer.contractIds].sort(),
      expected.contractIds,
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
    const parityTarget = paritySections.get(section.sectionId);

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

    if (parityTarget) {
      expect(
        section.sideBySideReview === parityTarget.sideBySideReview,
        `packages/gpui/native-accessibility-proof.json section "${section.sectionId}" side-by-side flag must match the parity registry.`,
        errors,
      );
    }
  }

  return {
    gpuiAccessibilityLayerCount: proof.layerProof.length,
    gpuiAccessibilitySectionCount: proof.sectionProof.length,
  };
}

function validateGpuiCrossRuntimeParityReport(errors: string[]): {
  gpuiCrossRuntimeSectionCount: number;
  gpuiCrossRuntimeDeltaCount: number;
} {
  const report = JSON.parse(fs.readFileSync(gpuiCrossRuntimeParityReportPath, "utf8")) as {
    generation: string;
    comparisonArtifacts: string[];
    acceptanceHarness: {
      suiteId: string;
      status: string;
      coveredPackages: string[];
      evidenceArtifacts: string[];
      requiredChecks: string[];
      blockers: string[];
    };
    summary: {
      sectionCount: number;
      directParityCount: number;
      nativeAdaptationCount: number;
      deferredCount: number;
      sideBySideSectionCount: number;
      manualGpuiProofCount: number;
      blockedGpuiProofCount: number;
    };
    sectionReports: Array<{
      sectionId: string;
      parityMode: string;
      owningLayer: string;
      sideBySideReview: boolean;
      svelteRouteIds: string[];
      gpuiEvidenceArtifacts: string[];
      gpuiStatuses: {
        gpui: string;
        focus: string;
        keyboard: string;
        announcements: string;
      };
      comparisonPosture: string;
      intentionalDeltaIds: string[];
    }>;
    deltaRegister: Array<{
      id: string;
      title: string;
      status: string;
      sectionIds: string[];
      runtimeReason: string;
      followUp: string;
      evidenceArtifacts: string[];
    }>;
    automatedBoundary: string[];
    manualBoundary: string[];
    nonGoals: string[];
  };
  const accessibilityProof = JSON.parse(fs.readFileSync(gpuiNativeAccessibilityProofPath, "utf8")) as {
    sectionProof: Array<{
      sectionId: string;
      gpuiStatus: string;
      focusStatus: string;
      keyboardStatus: string;
      announcementsStatus: string;
      sideBySideReview: boolean;
    }>;
  };
  const gpuiPriorityMatrix = JSON.parse(fs.readFileSync(gpuiParityPriorityPath, "utf8")) as {
    sectionTargets: Array<{
      sectionId: string;
      parityMode: string;
      gpuiLayer: string;
      sideBySideReview: boolean;
    }>;
  };
  const ecosystemAcceptance = JSON.parse(fs.readFileSync(ecosystemAcceptancePath, "utf8")) as {
    suites: Array<{
      id: string;
      status: string;
      coveredPackages: string[];
      evidenceArtifacts: string[];
      requiredChecks: string[];
      blockers: string[];
    }>;
  };
  const parityTargetRoutes = new Map(
    parityTargets.map((target) => [target.sectionId, new Set(target.reviewRoutes.map((route) => route.id))]),
  );
  const accessibilitySections = new Map(
    accessibilityProof.sectionProof.map((entry) => [entry.sectionId, entry]),
  );
  const prioritySections = new Map(
    gpuiPriorityMatrix.sectionTargets.map((entry) => [entry.sectionId, entry]),
  );
  const gpuiAcceptanceSuite = ecosystemAcceptance.suites.find((entry) => entry.id === report.acceptanceHarness.suiteId);
  const allowedDeltaStatuses = new Set(["pending", "allowed", "revisit", "rejected"]);
  const reportSectionIds = report.sectionReports.map((entry) => entry.sectionId).sort();
  const deltaIds = new Set(report.deltaRegister.map((entry) => entry.id));

  expect(
    report.generation === "g09.018",
    "packages/gpui/cross-runtime-parity-report.json must target g09.018.",
    errors,
  );
  expect(
    report.comparisonArtifacts.length >= 3,
    "packages/gpui/cross-runtime-parity-report.json must record comparison artifacts.",
    errors,
  );
  compareLists(
    "packages/gpui/cross-runtime-parity-report.json section ids",
    reportSectionIds,
    Array.from(accessibilitySections.keys()).sort(),
    errors,
  );
  expect(
    report.summary.sectionCount === report.sectionReports.length,
    "packages/gpui/cross-runtime-parity-report.json summary.sectionCount must match sectionReports length.",
    errors,
  );
  expect(
    report.summary.directParityCount + report.summary.nativeAdaptationCount + report.summary.deferredCount ===
      report.summary.sectionCount,
    "packages/gpui/cross-runtime-parity-report.json parity summary counts must add up to the section count.",
    errors,
  );
  expect(
    report.summary.sideBySideSectionCount === report.sectionReports.filter((entry) => entry.sideBySideReview).length,
    "packages/gpui/cross-runtime-parity-report.json sideBySideSectionCount must match the side-by-side section set.",
    errors,
  );
  expect(
    report.summary.manualGpuiProofCount ===
      report.sectionReports.filter((entry) => entry.gpuiStatuses.gpui === "manual").length,
    "packages/gpui/cross-runtime-parity-report.json manualGpuiProofCount must match section proof status.",
    errors,
  );
  expect(
    report.summary.blockedGpuiProofCount ===
      report.sectionReports.filter((entry) => entry.gpuiStatuses.gpui === "blocked").length,
    "packages/gpui/cross-runtime-parity-report.json blockedGpuiProofCount must match section proof status.",
    errors,
  );
  expect(
    report.automatedBoundary.length > 0,
    "packages/gpui/cross-runtime-parity-report.json must record automated boundaries.",
    errors,
  );
  expect(
    report.manualBoundary.length > 0,
    "packages/gpui/cross-runtime-parity-report.json must record manual boundaries.",
    errors,
  );
  expect(
    report.nonGoals.length >= 2,
    "packages/gpui/cross-runtime-parity-report.json must record explicit non-goals.",
    errors,
  );

  for (const artifactPath of report.comparisonArtifacts) {
    expect(
      fs.existsSync(path.join(repoRoot, artifactPath)),
      `packages/gpui/cross-runtime-parity-report.json references missing comparison artifact "${artifactPath}".`,
      errors,
    );
  }

  for (const section of report.sectionReports) {
    const accessibilitySection = accessibilitySections.get(section.sectionId);
    const prioritySection = prioritySections.get(section.sectionId);
    const routeIds = parityTargetRoutes.get(section.sectionId);

    expect(
      Boolean(accessibilitySection),
      `packages/gpui/cross-runtime-parity-report.json references unknown section "${section.sectionId}".`,
      errors,
    );
    expect(
      Boolean(prioritySection),
      `packages/gpui/cross-runtime-parity-report.json section "${section.sectionId}" is missing from the GPUI priority matrix.`,
      errors,
    );
    expect(
      section.comparisonPosture.trim().length > 0,
      `packages/gpui/cross-runtime-parity-report.json section "${section.sectionId}" must record comparison posture.`,
      errors,
    );
    expect(
      section.svelteRouteIds.length > 0,
      `packages/gpui/cross-runtime-parity-report.json section "${section.sectionId}" must record Svelte route ids.`,
      errors,
    );
    expect(
      section.gpuiEvidenceArtifacts.length > 0,
      `packages/gpui/cross-runtime-parity-report.json section "${section.sectionId}" must record GPUI evidence artifacts.`,
      errors,
    );

    if (accessibilitySection) {
      expect(
        section.gpuiStatuses.gpui === accessibilitySection.gpuiStatus,
        `packages/gpui/cross-runtime-parity-report.json section "${section.sectionId}" GPUI status must match the accessibility proof.`,
        errors,
      );
      expect(
        section.gpuiStatuses.focus === accessibilitySection.focusStatus,
        `packages/gpui/cross-runtime-parity-report.json section "${section.sectionId}" focus status must match the accessibility proof.`,
        errors,
      );
      expect(
        section.gpuiStatuses.keyboard === accessibilitySection.keyboardStatus,
        `packages/gpui/cross-runtime-parity-report.json section "${section.sectionId}" keyboard status must match the accessibility proof.`,
        errors,
      );
      expect(
        section.gpuiStatuses.announcements === accessibilitySection.announcementsStatus,
        `packages/gpui/cross-runtime-parity-report.json section "${section.sectionId}" announcement status must match the accessibility proof.`,
        errors,
      );
    }

    if (prioritySection) {
      expect(
        section.parityMode === prioritySection.parityMode,
        `packages/gpui/cross-runtime-parity-report.json section "${section.sectionId}" parity mode must match the GPUI priority matrix.`,
        errors,
      );
      expect(
        section.sideBySideReview === prioritySection.sideBySideReview,
        `packages/gpui/cross-runtime-parity-report.json section "${section.sectionId}" side-by-side flag must match the GPUI priority matrix.`,
        errors,
      );
      expect(
        section.owningLayer === prioritySection.gpuiLayer,
        `packages/gpui/cross-runtime-parity-report.json section "${section.sectionId}" owning layer must match the GPUI priority matrix.`,
        errors,
      );
    }

    for (const routeId of section.svelteRouteIds) {
      expect(
        routeIds?.has(routeId) ?? false,
        `packages/gpui/cross-runtime-parity-report.json section "${section.sectionId}" references unknown Svelte route "${routeId}".`,
        errors,
      );
    }

    for (const artifactPath of section.gpuiEvidenceArtifacts) {
      expect(
        fs.existsSync(path.join(repoRoot, artifactPath)),
        `packages/gpui/cross-runtime-parity-report.json section "${section.sectionId}" references missing GPUI evidence artifact "${artifactPath}".`,
        errors,
      );
    }

    for (const deltaId of section.intentionalDeltaIds) {
      expect(
        deltaIds.has(deltaId),
        `packages/gpui/cross-runtime-parity-report.json section "${section.sectionId}" references unknown delta "${deltaId}".`,
        errors,
      );
    }
  }

  for (const delta of report.deltaRegister) {
    expect(delta.title.trim().length > 0, `Cross-runtime delta "${delta.id}" is missing a title.`, errors);
    expect(
      allowedDeltaStatuses.has(delta.status),
      `Cross-runtime delta "${delta.id}" uses unsupported status "${delta.status}".`,
      errors,
    );
    expect(delta.sectionIds.length > 0, `Cross-runtime delta "${delta.id}" is missing section coverage.`, errors);
    expect(
      delta.runtimeReason.trim().length > 0,
      `Cross-runtime delta "${delta.id}" is missing a runtime reason.`,
      errors,
    );
    expect(
      delta.followUp.trim().length > 0,
      `Cross-runtime delta "${delta.id}" is missing follow-up guidance.`,
      errors,
    );
    expect(
      delta.evidenceArtifacts.length > 0,
      `Cross-runtime delta "${delta.id}" is missing evidence artifacts.`,
      errors,
    );

    for (const sectionId of delta.sectionIds) {
      expect(
        reportSectionIds.includes(sectionId),
        `Cross-runtime delta "${delta.id}" references unknown section "${sectionId}".`,
        errors,
      );
    }

    for (const artifactPath of delta.evidenceArtifacts) {
      expect(
        fs.existsSync(path.join(repoRoot, artifactPath)),
        `Cross-runtime delta "${delta.id}" references missing evidence artifact "${artifactPath}".`,
        errors,
      );
    }
  }

  expect(
    Boolean(gpuiAcceptanceSuite),
    `packages/gpui/cross-runtime-parity-report.json references missing acceptance suite "${report.acceptanceHarness.suiteId}".`,
    errors,
  );

  if (gpuiAcceptanceSuite) {
    expect(
      report.acceptanceHarness.status === gpuiAcceptanceSuite.status,
      "packages/gpui/cross-runtime-parity-report.json acceptance harness status must match packages/ecosystem-acceptance.json.",
      errors,
    );
    compareLists(
      "packages/gpui/cross-runtime-parity-report.json acceptance harness covered packages",
      [...report.acceptanceHarness.coveredPackages].sort(),
      [...gpuiAcceptanceSuite.coveredPackages].sort(),
      errors,
    );
    compareLists(
      "packages/gpui/cross-runtime-parity-report.json acceptance harness evidence artifacts",
      [...report.acceptanceHarness.evidenceArtifacts].sort(),
      [...gpuiAcceptanceSuite.evidenceArtifacts].sort(),
      errors,
    );
    compareLists(
      "packages/gpui/cross-runtime-parity-report.json acceptance harness required checks",
      [...report.acceptanceHarness.requiredChecks].sort(),
      [...gpuiAcceptanceSuite.requiredChecks].sort(),
      errors,
    );
    compareLists(
      "packages/gpui/cross-runtime-parity-report.json acceptance harness blockers",
      [...report.acceptanceHarness.blockers].sort(),
      [...gpuiAcceptanceSuite.blockers].sort(),
      errors,
    );
  }

  return {
    gpuiCrossRuntimeSectionCount: report.sectionReports.length,
    gpuiCrossRuntimeDeltaCount: report.deltaRegister.length,
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
validateContractIndexes(errors);
validateSveltePackageSurface("packages/svelte/primitives", "foundation", "@poodle/svelte-primitives", errors);
validateSveltePackageSurface("packages/svelte/composites", "composites", "@poodle/svelte-composites", errors);
validatePackageSurfaceCoverage("packages/svelte/primitives", "@poodle/svelte-primitives", errors);
validatePackageSurfaceCoverage("packages/svelte/composites", "@poodle/svelte-composites", errors);
validateDocsCatalog(errors);
validateParityCoverage(errors);
validateAccessibilityAudit(errors);
validateReleaseOperations(errors);
const ecosystemAcceptanceCounts = validateEcosystemAcceptance(errors);
const referenceAppsCounts = validateReferenceApps(errors);
const g03CloseoutCounts = validateG03Closeout(errors);
const gpuiPriorityCounts = validateGpuiPriorityMatrix(errors);
const gpuiPreviewCounts = validateGpuiPreviewBaseline(errors);
const gpuiStructuralCounts = validateGpuiStructuralBaseline(errors);
const gpuiActionFieldCounts = validateGpuiActionFieldBaseline(errors);
const gpuiSelectionFeedbackDateCounts = validateGpuiSelectionFeedbackDateBaseline(errors);
const gpuiOverlayNavigationMenuCounts = validateGpuiOverlayNavigationMenuBaseline(errors);
const gpuiFormValidationRemediationCounts = validateGpuiFormValidationRemediationBaseline(errors);
const gpuiDataBrowseDetailPickerMediaCounts = validateGpuiDataBrowseDetailPickerMediaBaseline(errors);
const gpuiWorkstationCounts = validateGpuiWorkstationBaseline(errors);
const gpuiNativeAccessibilityCounts = validateGpuiNativeAccessibilityProof(errors);
const gpuiCrossRuntimeParityCounts = validateGpuiCrossRuntimeParityReport(errors);
const sharedDemoAppAuditCounts = validateSharedDemoAppAudit(errors);
const sharedDemoAppContractCounts = validateSharedDemoAppContract(errors);

if (errors.length > 0) {
  throw new Error(errors.join("\n"));
}

console.log(
  `Validated ${componentContractCount} component contracts, ${docsSections.length} docs sections, ${docsFamilies.length} docs families, ${parityTargets.length} parity targets, ${accessibilityAuditTargets.length} accessibility audit targets, ${ecosystemAcceptanceCounts.suiteCount} ecosystem acceptance suites, ${ecosystemAcceptanceCounts.regressionClassCount} regression classes, ${referenceAppsCounts.shapeCount} reference shapes, ${referenceAppsCounts.laneCount} onboarding lanes, ${g03CloseoutCounts.stableSurfaceCount} closeout surfaces, ${g03CloseoutCounts.carryForwardCount} carry-forward gaps, ${gpuiPriorityCounts.waveCount} GPUI implementation waves, ${gpuiPriorityCounts.targetCount} GPUI section targets, ${gpuiPreviewCounts.previewSectionCount} GPUI preview baseline sections, ${gpuiStructuralCounts.structuralExportCount} GPUI structural exports, ${gpuiActionFieldCounts.actionFieldExportCount} GPUI action or field exports, ${gpuiSelectionFeedbackDateCounts.selectionFeedbackDateExportCount} GPUI selection/feedback/date exports, ${gpuiOverlayNavigationMenuCounts.overlayNavigationMenuExportCount} GPUI overlay/disclosure/navigation/menu exports, ${gpuiFormValidationRemediationCounts.gpuiCompositeExportCount} GPUI form/validation/remediation composite exports, ${gpuiDataBrowseDetailPickerMediaCounts.gpuiDataCompositeExportCount} GPUI data/browse/detail/picker/media composite exports, ${gpuiWorkstationCounts.gpuiWorkstationExportCount} GPUI workstation exports, ${gpuiNativeAccessibilityCounts.gpuiAccessibilityLayerCount} GPUI accessibility-proof layers, ${gpuiNativeAccessibilityCounts.gpuiAccessibilitySectionCount} GPUI accessibility-proof sections, ${gpuiCrossRuntimeParityCounts.gpuiCrossRuntimeSectionCount} GPUI cross-runtime parity sections, ${gpuiCrossRuntimeParityCounts.gpuiCrossRuntimeDeltaCount} GPUI intentional deltas, ${sharedDemoAppAuditCounts.demoAuditFindingCount} shared demo-app audit findings, ${sharedDemoAppAuditCounts.demoAuditScreenCount} shared demo target screens, ${sharedDemoAppContractCounts.demoContractScreenCount} shared demo contract screens, and ${sharedDemoAppContractCounts.demoContractRegionCount} shared demo shell regions.`,
);
