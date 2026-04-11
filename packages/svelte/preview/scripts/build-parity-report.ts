import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { docsFamilies, docsSections } from "../src/catalog";
import {
  buildPreviewUrl,
  docsNavigationSections,
  packageSurfaceCoverage,
  parityTargets,
  previewHarnessBoundary,
} from "../src/parity";

type HarnessCoverage = (typeof parityTargets)[number]["harnessCoverage"];

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const previewDir = path.resolve(scriptDir, "..");
const artifactPath = path.join(previewDir, "artifacts", "parity-report.json");
const repoRoot = path.resolve(previewDir, "../../..");
const gpuiCrossRuntimeParityPath = path.join(repoRoot, "packages", "gpui", "cross-runtime-parity-report.json");

function countCoverageKind(
  harnessName: keyof HarnessCoverage,
): Record<string, number> {
  return parityTargets.reduce<Record<string, number>>((accumulator, target) => {
    const coverage = target.harnessCoverage[harnessName];
    accumulator[coverage] = (accumulator[coverage] ?? 0) + 1;
    return accumulator;
  }, {});
}

function validateParityState(): void {
  const errors: string[] = [];
  const docsSectionIds = new Set(docsNavigationSections.map((entry) => entry.id));
  const parityTargetIds = new Set(parityTargets.map((entry) => entry.sectionId));
  const docsCatalogSections = docsNavigationSections;
  const docsCatalogSectionIds = new Set(docsCatalogSections.map((entry) => entry.id));
  const seenPackageExports = new Set<string>();

  for (const sectionId of docsSectionIds) {
    if (!parityTargetIds.has(sectionId)) {
      errors.push(`Missing parity target for section "${sectionId}".`);
    }
  }

  for (const target of parityTargets) {
    if (!docsSectionIds.has(target.sectionId)) {
      errors.push(`Parity target "${target.sectionId}" is not a known docs section.`);
    }

    if (target.automatedChecks.length === 0) {
      errors.push(`Parity target "${target.sectionId}" has no automated checks.`);
    }

    if (target.harnessCoverage.visual !== "not-applicable" && target.manualChecks.length === 0) {
      errors.push(`Parity target "${target.sectionId}" needs manual checks for visual review.`);
    }

    if (target.reviewRoutes.length === 0) {
      errors.push(`Parity target "${target.sectionId}" has no review routes.`);
    }

    const routeUrls = new Set<string>();
    for (const route of target.reviewRoutes) {
      if (route.state.sectionId !== target.sectionId) {
        errors.push(
          `Parity target "${target.sectionId}" has a route pointing at "${route.state.sectionId}".`,
        );
      }

      const url = buildPreviewUrl(route.state);
      if (routeUrls.has(url)) {
        errors.push(`Parity target "${target.sectionId}" has duplicate review URL "${url}".`);
      }
      routeUrls.add(url);
    }
  }

  for (const family of docsFamilies) {
    for (const sectionId of family.sectionIds) {
      if (!docsSections.some((entry) => entry.id === sectionId)) {
        errors.push(`Family "${family.id}" references unknown section "${sectionId}".`);
      }
    }
  }

  for (const entry of packageSurfaceCoverage) {
    const exportId = `${entry.packageName}:${entry.exportName}`;

    if (seenPackageExports.has(exportId)) {
      errors.push(`Package surface coverage duplicates "${exportId}".`);
    }
    seenPackageExports.add(exportId);

    if (entry.note.trim().length === 0) {
      errors.push(`Package surface coverage "${exportId}" is missing review guidance.`);
    }

    if (entry.status === "previewed") {
      if (entry.sectionIds.length === 0) {
        errors.push(`Package surface coverage "${exportId}" is previewed without section coverage.`);
      }

      const seenSectionIds = new Set<string>();
      for (const sectionId of entry.sectionIds) {
        if (!docsCatalogSectionIds.has(sectionId)) {
          errors.push(`Package surface coverage "${exportId}" references unknown section "${sectionId}".`);
        }
        if (seenSectionIds.has(sectionId)) {
          errors.push(`Package surface coverage "${exportId}" repeats section "${sectionId}".`);
        }
        seenSectionIds.add(sectionId);
      }
    }

    if (entry.status === "contract-only" && entry.sectionIds.length > 0) {
      errors.push(`Package surface coverage "${exportId}" is contract-only but still references preview sections.`);
    }
  }

  if (errors.length > 0) {
    throw new Error(errors.join("\n"));
  }
}

validateParityState();

const gpuiCrossRuntimeParity = JSON.parse(fs.readFileSync(gpuiCrossRuntimeParityPath, "utf8")) as {
  generation: string;
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
    sideBySideReview: boolean;
    parityMode: string;
    intentionalDeltaIds: string[];
  }>;
  deltaRegister: Array<{
    id: string;
    status: string;
    sectionIds: string[];
  }>;
  acceptanceHarness: {
    suiteId: string;
    status: string;
    coveredPackages: string[];
    evidenceArtifacts: string[];
  };
};

const report = {
  generatedAt: new Date().toISOString(),
  artifact: "packages/svelte/preview/artifacts/parity-report.json",
  previewBasePath: "/",
  automatedBoundary: previewHarnessBoundary.automated,
  manualBoundary: previewHarnessBoundary.manual,
  summary: {
    sectionCount: docsNavigationSections.length,
    familyCount: docsFamilies.length,
    contractCoverage: countCoverageKind("contract"),
    visualCoverage: countCoverageKind("visual"),
    interactionCoverage: countCoverageKind("interaction"),
  },
  crossRuntime: {
    generation: gpuiCrossRuntimeParity.generation,
    summary: gpuiCrossRuntimeParity.summary,
    sideBySideSections: gpuiCrossRuntimeParity.sectionReports
      .filter((entry) => entry.sideBySideReview)
      .map((entry) => ({
        sectionId: entry.sectionId,
        parityMode: entry.parityMode,
        deltaIds: entry.intentionalDeltaIds,
      })),
    deltaRegister: gpuiCrossRuntimeParity.deltaRegister.map((entry) => ({
      id: entry.id,
      status: entry.status,
      sectionIds: entry.sectionIds,
    })),
    acceptanceHarness: gpuiCrossRuntimeParity.acceptanceHarness,
  },
  packageSurfaceCoverage: {
    summary: ["@poodle/svelte", "@poodle/svelte"].map((packageName) => {
      const entries = packageSurfaceCoverage.filter((entry) => entry.packageName === packageName);

      return {
        packageName,
        exportCount: entries.length,
        componentCount: entries.filter((entry) => entry.kind === "component").length,
        helperCount: entries.filter((entry) => entry.kind === "helper").length,
        previewedCount: entries.filter((entry) => entry.status === "previewed").length,
        contractOnlyCount: entries.filter((entry) => entry.status === "contract-only").length,
      };
    }),
    exports: packageSurfaceCoverage.map((entry) => ({
      packageName: entry.packageName,
      exportName: entry.exportName,
      kind: entry.kind,
      status: entry.status,
      note: entry.note,
      sectionIds: entry.sectionIds,
      sectionTitles: entry.sectionIds.map(
        (sectionId) => docsNavigationSections.find((section) => section.id === sectionId)?.title ?? sectionId,
      ),
    })),
  },
  families: docsFamilies.map((family) => ({
    id: family.id,
    title: family.title,
    sectionCount: family.sectionIds.length,
    sectionIds: family.sectionIds,
  })),
  targets: parityTargets.map((target) => ({
    sectionId: target.sectionId,
    title: target.title,
    layer: target.layer,
    packageName: target.packageName,
    contractRoot: target.contractRoot,
    summary: target.summary,
    harnessCoverage: target.harnessCoverage,
    automatedChecks: target.automatedChecks,
    manualChecks: target.manualChecks,
    reviewRoutes: target.reviewRoutes.map((route) => ({
      id: route.id,
      label: route.label,
      note: route.note,
      state: route.state,
      url: buildPreviewUrl(route.state),
    })),
  })),
};

fs.mkdirSync(path.dirname(artifactPath), { recursive: true });
fs.writeFileSync(artifactPath, `${JSON.stringify(report, null, 2)}\n`);

console.log(
  `Wrote parity report with ${report.summary.sectionCount} sections to ${path.relative(process.cwd(), artifactPath)}`,
);
