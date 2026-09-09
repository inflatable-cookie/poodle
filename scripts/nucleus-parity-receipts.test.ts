import { createHash } from "node:crypto";
import { mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import { describe, expect, it } from "bun:test";
import {
  deriveV1Receipts,
  loadNucleusManifest,
  loadValidatedV1Bundle,
  NUCLEUS_V1_BUNDLE_DIR,
  NUCLEUS_V1_DIRECTORY_SHA256,
  NUCLEUS_V1_SUMMARY_SHA256,
  NUCLEUS_SCHEMA_PATH,
  nucleusV1ScenarioMap,
  receiptFileStem,
  v1DirectoryHash,
  validateNucleusManifest,
  validateNucleusReceipt,
  validateV1BundleDocument,
  type NucleusManifest,
  type NucleusReceipt,
  type NucleusV1Receipt,
} from "./nucleus-parity-receipts";

const root = path.resolve(import.meta.dir, "..");

function validButtonReceipt(manifest: NucleusManifest): NucleusReceipt {
  return {
    schema: "poodle.g16-nucleus-parity-receipt.v1",
    component: "Button",
    scenario_id: "nucleus.shell.button",
    proof_level: "M1",
    runtime: "gpui-headless",
    command: "effigy regressions:native",
    package: manifest.resolution.package,
    package_version: manifest.resolution.version,
    source_commit: manifest.resolution.source_commit,
    lockfile: manifest.resolution.lockfile,
    lockfile_sha256: manifest.resolution.lockfile_sha256,
    lock_resolution: manifest.resolution.lock_resolution,
    distribution: "workspace",
    production_path_observation: {
      observed: true,
      mount: "HeadlessDriver",
      render_path: "poodle_render -> poodle_gpui_node_backend::to_gpui",
      input_dispatch: "gpui-test-platform-dispatch",
    },
    actions: ["mount Button through HeadlessDriver", "pointer press and release through GPUI dispatch"],
    assertions: ["the mounted Button listener fired exactly once"],
    outcome: "passed",
    artifact_paths: [],
  };
}

describe("g16.062 Nucleus parity receipt contract", () => {
  it("freezes 29 rendered rows plus IconProvider as a non-rendered prerequisite", () => {
    const manifest = loadNucleusManifest(root);
    expect(manifest.components).toHaveLength(29);
    expect(manifest.prerequisites).toEqual([
      expect.objectContaining({ name: "IconProvider", rendered: false }),
    ]);
    expect(manifest.components.map((entry) => entry.name)).toEqual([
      "Icon",
      "Text",
      "Surface",
      "Button",
      "IconButton",
      "AppHeader",
      "SplitView",
      "SegmentedControl",
      "Tabs",
      "Menu",
      "Dialog",
      "Popover",
      "Select",
      "EditableLabel",
      "AgentTranscript",
      "AgentChatInput",
      "AgentPlan",
      "AgentQuestion",
      "ModelPicker",
      "StatusIndicator",
      "RadioGroup",
      "Switch",
      "TextInput",
      "Callout",
      "ConfirmAction",
      "DetailItem",
      "CommandPalette",
      "MessageCenter",
      "ToastHost",
    ]);
  });

  it("keeps the checked-in schema and manifest valid", () => {
    const manifest = loadNucleusManifest(root);
    expect(JSON.parse(readFileSync(path.join(root, NUCLEUS_SCHEMA_PATH), "utf8")).properties.proof_level.enum).toEqual(["M1", "A1", "V1"]);
    expect(() => validateNucleusManifest(manifest, root)).not.toThrow();
  });

  it("accepts a receipt only for an observed mounted M1 execution", () => {
    const manifest = loadNucleusManifest(root);
    expect(() => validateNucleusReceipt(validButtonReceipt(manifest), manifest, root)).not.toThrow();
  });

  it("rejects properties forbidden by the checked-in manifest and receipt schemas", () => {
    const manifest = loadNucleusManifest(root);
    const manifestExtra = { ...manifest, invented: true } as NucleusManifest;
    expect(() => validateNucleusManifest(manifestExtra, root)).toThrow(/manifest has unexpected property invented/);

    const manifestNestedExtra = structuredClone(manifest) as NucleusManifest;
    (manifestNestedExtra.resolution as Record<string, unknown>).invented = true;
    expect(() => validateNucleusManifest(manifestNestedExtra, root)).toThrow(/manifest resolution has unexpected property invented/);

    const receipt = validButtonReceipt(manifest);
    const receiptExtra = { ...receipt, invented: true } as NucleusReceipt;
    expect(() => validateNucleusReceipt(receiptExtra, manifest, root)).toThrow(/receipt has unexpected property invented/);

    const receiptNestedExtra = structuredClone(receipt) as NucleusReceipt;
    (receiptNestedExtra.production_path_observation as Record<string, unknown>).invented = true;
    expect(() => validateNucleusReceipt(receiptNestedExtra, manifest, root)).toThrow(
      /receipt production_path_observation has unexpected property invented/,
    );
  });

  it("requires every nonempty artifact to identify an existing file by SHA-256", () => {
    const manifest = loadNucleusManifest(root);
    const receipt = validButtonReceipt(manifest);
    const artifactPath = "docs/evidence/nucleus/nucleus-parity-receipts/README.md";
    const artifactHash = createHash("sha256").update(readFileSync(path.join(root, artifactPath))).digest("hex");

    expect(() => validateNucleusReceipt({ ...receipt, artifact_paths: [{ path: artifactPath, sha256: artifactHash }] }, manifest, root)).not.toThrow();
    expect(() => validateNucleusReceipt({
      ...receipt,
      artifact_paths: [{ path: artifactPath, sha256: "0".repeat(64) }],
    }, manifest, root)).toThrow(/SHA-256 does not match/);
    expect(() => validateNucleusReceipt({
      ...receipt,
      artifact_paths: [{ path: "does/not/exist.png", sha256: "0".repeat(64) }],
    }, manifest, root)).toThrow(/path does not exist/);
    expect(() => validateNucleusReceipt({
      ...receipt,
      artifact_paths: [{ path: "../outside.png", sha256: "0".repeat(64) }],
    }, manifest, root)).toThrow(/path must be repository-relative/);
    expect(() => validateNucleusReceipt({
      ...receipt,
      artifact_paths: [{ path: artifactPath, sha256: artifactHash, invented: true } as never],
    }, manifest, root)).toThrow(/receipt artifact_paths\[0\] has unexpected property invented/);
  });

  it("rejects wrong commit, runtime, direct-handler, and proof-level substitutions", () => {
    const manifest = loadNucleusManifest(root);
    const base = validButtonReceipt(manifest);
    expect(() => validateNucleusReceipt({ ...base, source_commit: "b".repeat(40) }, manifest, root)).toThrow(/source commit/);
    expect(() => validateNucleusReceipt({ ...base, runtime: "direct-handler" } as NucleusReceipt, manifest, root)).toThrow(/runtime/);
    expect(() => validateNucleusReceipt({
      ...base,
      production_path_observation: { ...base.production_path_observation, observed: false, mount: "direct-handler" },
    } as NucleusReceipt, manifest, root)).toThrow(/observed mounted|HeadlessDriver|production path/);
    expect(() => validateNucleusReceipt({ ...base, proof_level: "V1" } as never, manifest, root)).toThrow(/proof level/);
    expect(() => validateNucleusReceipt({ ...base, proof_level: "A1" } as NucleusReceipt, manifest, root)).toThrow(/accessibility block/);
  });

  it("rejects a cohort with IconProvider promoted to row 30", () => {
    const manifest = loadNucleusManifest(root);
    const invalid = {
      ...manifest,
      rendered_component_count: 30,
      components: [...manifest.components, { ...manifest.prerequisites[0], rendered: true }],
    } as NucleusManifest;
    expect(() => validateNucleusManifest(invalid, root)).toThrow(/29|unique/);
  });

  it("rejects an unmanifested receipt component", () => {
    const manifest = loadNucleusManifest(root);
    expect(() => validateNucleusReceipt({ ...validButtonReceipt(manifest), component: "NotNucleus" }, manifest, root)).toThrow(
      /not a rendered manifest entry/,
    );
  });
});

describe("g16.111 Nucleus A1 paired accessibility receipts", () => {
  const a1Path = "docs/evidence/nucleus/nucleus-parity-receipts/switch--nucleus-settings-switch--a1.json";
  const committedA1 = (): NucleusReceipt => JSON.parse(readFileSync(path.join(root, a1Path), "utf8")) as NucleusReceipt;

  it("accepts the committed Switch A1 receipt with its paired snapshots", () => {
    const manifest = loadNucleusManifest(root);
    const receipt = committedA1();
    expect(receipt.proof_level).toBe("A1");
    expect(receipt.accessibility?.diff).toEqual([]);
    expect(() => validateNucleusReceipt(receipt, manifest, root)).not.toThrow();
  });

  it("rejects an A1 receipt whose snapshots, scenario, or diff were substituted", () => {
    const manifest = loadNucleusManifest(root);
    const base = committedA1();
    const withBlock = (patch: Partial<NonNullable<NucleusReceipt["accessibility"]>>): NucleusReceipt => ({
      ...base,
      accessibility: { ...(base.accessibility as NonNullable<NucleusReceipt["accessibility"]>), ...patch },
    });
    expect(() => validateNucleusReceipt(withBlock({ scenario_sha256: "0".repeat(64) }), manifest, root)).toThrow(/scenario SHA-256/);
    expect(() => validateNucleusReceipt(withBlock({ svelte_snapshot_sha256: "0".repeat(64) }), manifest, root)).toThrow(/svelte snapshot SHA-256/);
    expect(() => validateNucleusReceipt(withBlock({ gpui_snapshot_sha256: "0".repeat(64) }), manifest, root)).toThrow(/gpui snapshot SHA-256/);
    expect(() => validateNucleusReceipt(withBlock({ diff: [{ index: 0, field: "role" }] }), manifest, root)).toThrow(/diff is not empty/);
    expect(() => validateNucleusReceipt(withBlock({ svelte_snapshot_path: "test/nucleus-a11y/snapshots/tabs.svelte.json" }), manifest, root)).toThrow(/does not belong to the scenario row/);
    expect(() => validateNucleusReceipt(withBlock({ web_only_exclusions: [{ attribute: "aria-readonly", reason: "invented" }] }), manifest, root)).toThrow(/web_only_exclusions/);
    expect(() => validateNucleusReceipt({ ...base, accessibility: { ...base.accessibility, invented: true } } as never, manifest, root)).toThrow(/receipt accessibility has unexpected property invented/);
  });

  it("keeps A1 evidence separate from M1 and rejects an unmanifested A1 receipt", () => {
    const manifest = loadNucleusManifest(root);
    const base = committedA1();
    const m1 = validButtonReceipt(manifest);
    expect(() => validateNucleusReceipt({ ...m1, accessibility: base.accessibility }, manifest, root)).toThrow(/M1 receipt carries no accessibility block/);
    expect(() => validateNucleusReceipt({ ...base, component: "Tree", scenario_id: "nucleus.settings.tree" }, manifest, root)).toThrow(/not a rendered manifest entry/);
    expect(receiptFileStem(base)).toBe("switch--nucleus-settings-switch--a1");
    expect(receiptFileStem(m1)).toBe("button--nucleus-shell-button");
  });
});

describe("g17.001 Nucleus V1 visual receipts", () => {
  const bundleSummary = (): unknown => JSON.parse(readFileSync(path.join(root, NUCLEUS_V1_BUNDLE_DIR, "summary.json"), "utf8")) as unknown;

  it("accepts the 29 committed V1 receipts with every covered row and all 160 retained findings", () => {
    const manifest = loadNucleusManifest(root);
    const { bundle, summarySha256 } = loadValidatedV1Bundle(root, manifest);
    expect(bundle.slugs).toHaveLength(29);
    expect(bundle.pairs).toHaveLength(116);
    expect(bundle.findingCount).toBe(160);
    expect(bundle.pairsOk).toBe(48);
    expect(summarySha256).toBe(NUCLEUS_V1_SUMMARY_SHA256);
    const receipts = deriveV1Receipts(bundle, manifest, summarySha256);
    expect(receipts).toHaveLength(29);
    for (const receipt of receipts) {
      expect(() => validateNucleusReceipt(receipt, manifest, root)).not.toThrow();
      expect(receipt.proof_level).toBe("V1");
      expect(receipt.outcome).toBe("compared");
      expect(receipt.fixtures).toHaveLength(2);
      expect(receipt.pairs).toHaveLength(4);
      const committed = readFileSync(path.join(root, `docs/evidence/nucleus/nucleus-parity-receipts/${receiptFileStem(receipt)}.json`), "utf8");
      expect(JSON.stringify(receipt, null, 2)).toBe(committed);
    }
    expect(receipts.reduce((count, receipt) => count + receipt.finding_count, 0)).toBe(160);
    const button = receipts.find((receipt) => receipt.component === "Button") as NucleusV1Receipt;
    expect(receiptFileStem(button)).toBe("button--nucleus-shell-button--v1");
  });

  it("refuses a tampered bundle by its bytes before any receipt is derived", () => {
    const scratch = mkdtempSync(path.join(tmpdir(), "nucleus-v1-tamper-"));
    for (const file of ["summary.json", "report.md"]) {
      writeFileSync(path.join(scratch, file), readFileSync(path.join(root, NUCLEUS_V1_BUNDLE_DIR, file)));
    }
    expect(v1DirectoryHash(scratch, ".").sha256).toBe(NUCLEUS_V1_DIRECTORY_SHA256);
    const tamperedPath = path.join(scratch, "summary.json");
    writeFileSync(tamperedPath, readFileSync(tamperedPath, "utf8").replace("2026-09-08T14-06-48", "2026-09-08T14-06-49"));
    expect(v1DirectoryHash(scratch, ".").sha256).not.toBe(NUCLEUS_V1_DIRECTORY_SHA256);
    const manifest = loadNucleusManifest(root);
    expect(() => validateV1BundleDocument(JSON.parse(readFileSync(tamperedPath, "utf8")) as unknown, nucleusV1ScenarioMap(root), manifest)).toThrow(/run id/);
  });

  it("refuses unknown, duplicate, and mismatched fixture identities", () => {
    const manifest = loadNucleusManifest(root);
    const scenarios = nucleusV1ScenarioMap(root);
    const unknown = structuredClone(bundleSummary()) as { comparisons: Array<{ fixture: string }> };
    unknown.comparisons[0].fixture = "cohort/bogus/initial";
    expect(() => validateV1BundleDocument(unknown, scenarios, manifest)).toThrow(/absent from the bundle captures/);
    const duplicate = structuredClone(bundleSummary()) as { comparisons: unknown[] };
    duplicate.comparisons.push(structuredClone(duplicate.comparisons[0]));
    expect(() => validateV1BundleDocument(duplicate, scenarios, manifest)).toThrow(/duplicates pair verdict/);
    const mismatched = structuredClone(bundleSummary()) as { captures: Array<{ fixture: string }> };
    mismatched.captures[0].fixture = "cohort/bogus/initial";
    expect(() => validateV1BundleDocument(mismatched, scenarios, manifest)).toThrow(/has no Poodle scenario file/);
  });

  it("refuses an import without Lab traceability", () => {
    const manifest = loadNucleusManifest(root);
    const scenarios = nucleusV1ScenarioMap(root);
    const noRun = structuredClone(bundleSummary()) as Record<string, unknown>;
    delete noRun.runId;
    expect(() => validateV1BundleDocument(noRun, scenarios, manifest)).toThrow(/run id/);
    const openBatch = structuredClone(bundleSummary()) as Record<string, unknown>;
    openBatch.closedBatch = false;
    expect(() => validateV1BundleDocument(openBatch, scenarios, manifest)).toThrow(/closed batch/);
    const noSources = structuredClone(bundleSummary()) as Record<string, unknown>;
    delete noSources.sources;
    expect(() => validateV1BundleDocument(noSources, scenarios, manifest)).toThrow(/Poodle pin/);
  });

  it("refuses a dropped finding: verdict counts no longer recompute", () => {
    const manifest = loadNucleusManifest(root);
    const scenarios = nucleusV1ScenarioMap(root);
    const dropped = structuredClone(bundleSummary()) as {
      comparisons: Array<{ channels: Record<string, { status: string; findings: Array<{ detail: string }> }> }>;
    };
    const comparison = dropped.comparisons.find((candidate) => candidate.channels.pixels.findings.length > 0) as {
      channels: Record<string, { status: string; findings: Array<{ detail: string }> }>;
    };
    comparison.channels.pixels.findings.pop();
    expect(() => validateV1BundleDocument(dropped, scenarios, manifest)).toThrow(/verdict findings do not match/);
  });

  it("refuses V1 receipts with substituted bundle identity, outcome, or dropped findings", () => {
    const manifest = loadNucleusManifest(root);
    const { bundle, summarySha256 } = loadValidatedV1Bundle(root, manifest);
    const receipts = deriveV1Receipts(bundle, manifest, summarySha256);
    const base = receipts.find((receipt) => receipt.finding_count > 0) as NucleusV1Receipt;
    expect(() => validateNucleusReceipt({ ...base, lab_bundle: { ...base.lab_bundle, run_id: "2026-09-08T14-06-49" } }, manifest, root)).toThrow(
      /cited run/,
    );
    expect(() => validateNucleusReceipt({ ...base, outcome: "passed" } as NucleusV1Receipt, manifest, root)).toThrow(/never passed/);
    const dropped = structuredClone(base) as NucleusV1Receipt;
    const channel = dropped.pairs.flatMap((pair) => Object.values(pair.channels)).find((verdict) => verdict.findings.length > 0);
    channel?.findings.pop();
    dropped.finding_count -= 1;
    expect(() => validateNucleusReceipt(dropped, manifest, root)).toThrow(/does not match the validated Lab bundle derivation/);
    expect(() => validateNucleusReceipt({ ...base, component: "NotNucleus" }, manifest, root)).toThrow(/not a rendered manifest entry/);
  });
});
