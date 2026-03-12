import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import {
  accessibilityAuditBoundary,
  accessibilityAuditTargets,
  buildAccessibilityAuditUrl,
  type AccessibilityAuditAreaStatus,
} from "../src/accessibility";

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
  generatedAt: new Date().toISOString(),
  artifact: "packages/svelte/preview/artifacts/accessibility-report.json",
  previewBasePath: "/",
  automatedBoundary: accessibilityAuditBoundary.automated,
  manualBoundary: accessibilityAuditBoundary.manual,
  summary: {
    targetCount: accessibilityAuditTargets.length,
    blockedGpuiTargets: accessibilityAuditTargets.filter((target) => target.auditAreas.gpui === "blocked").length,
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
