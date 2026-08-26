import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

// Reuses the canonical (Svelte-authored) accessibility planning data. Shared
// contracts do not transfer Svelte axe evidence to React; this artifact keeps
// that missing React sweep explicit.
import {
  accessibilityAuditBoundary,
  accessibilityAuditTargets,
  buildAccessibilityAuditUrl,
  type AccessibilityAuditAreaStatus,
} from "../../../svelte/preview/src/accessibility";

type AccessibilityAreaName = keyof AccessibilityAuditAreaStatus;

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const previewDir = path.resolve(scriptDir, "..");
const artifactPath = path.join(previewDir, "artifacts", "accessibility-report.json");

function countAreaStatus(areaName: AccessibilityAreaName): Record<string, number> {
  return accessibilityAuditTargets.reduce<Record<string, number>>((accumulator, target) => {
    const status = target.auditAreas[areaName];
    accumulator[status] = (accumulator[status] ?? 0) + 1;
    return accumulator;
  }, {});
}

const report = {
  artifact: "packages/react/preview/artifacts/accessibility-report.json",
  previewBasePath: "/",
  frameworks: ["@inflatable-cookie/poodle-react", "@inflatable-cookie/poodle-svelte"],
  sharedContractNote:
    "@inflatable-cookie/poodle-react is a reference-faithful re-implementation of the same component contracts as @inflatable-cookie/poodle-svelte. The Svelte preview owns the current axe sweep; Svelte axe evidence does not transfer to React, and no React axe sweep is currently recorded.",
  runtimeEvidence: {
    svelteAxe: "test/a11y/component-a11y.test.ts",
    reactAxe: "missing",
  },
  automatedBoundary: accessibilityAuditBoundary.automated,
  manualBoundary: accessibilityAuditBoundary.manual,
  summary: {
    targetCount: accessibilityAuditTargets.length,
    blockedGpuiTargets: accessibilityAuditTargets.filter((target) => target.auditAreas.gpui === "blocked").length,
    manualGpuiTargets: accessibilityAuditTargets.filter((target) => target.auditAreas.gpui === "manual").length,
    explicitGpuiTargets: accessibilityAuditTargets.filter((target) => target.auditAreas.gpui === "explicit").length,
    semanticsCoverage: countAreaStatus("semantics"),
    focusCoverage: countAreaStatus("focus"),
    keyboardCoverage: countAreaStatus("keyboard"),
    announcementCoverage: countAreaStatus("announcements"),
    gpuiCoverage: countAreaStatus("gpui"),
  },
  targets: accessibilityAuditTargets.map((target) => ({
    sectionId: target.sectionId,
    title: target.title,
    layer: target.layer,
    packageName: target.packageName,
    contractRoot: target.contractRoot,
    summary: target.summary,
    auditAreas: target.auditAreas,
    automatedChecks: target.automatedChecks,
    manualChecks: target.manualChecks,
    gpuiDeltaNotes: target.gpuiDeltaNotes,
    blockerNotes: target.blockerNotes,
    reviewRoutes: target.reviewRoutes.map((route) => ({
      id: route.id,
      label: route.label,
      note: route.note,
      state: route.state,
      url: buildAccessibilityAuditUrl(route),
    })),
  })),
};

fs.mkdirSync(path.dirname(artifactPath), { recursive: true });
fs.writeFileSync(artifactPath, `${JSON.stringify(report, null, 2)}\n`);

console.log(
  `Wrote accessibility report with ${report.summary.targetCount} targets to ${path.relative(process.cwd(), artifactPath)}`,
);
