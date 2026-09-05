import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import path from "node:path";
import { describe, expect, it } from "bun:test";
import {
  loadNucleusManifest,
  NUCLEUS_SCHEMA_PATH,
  receiptFileStem,
  validateNucleusManifest,
  validateNucleusReceipt,
  type NucleusManifest,
  type NucleusReceipt,
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
    expect(JSON.parse(readFileSync(path.join(root, NUCLEUS_SCHEMA_PATH), "utf8")).properties.proof_level.enum).toEqual(["M1", "A1"]);
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
    const artifactPath = "docs/roadmaps/g16/nucleus-parity-receipts/README.md";
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
  const a1Path = "docs/roadmaps/g16/nucleus-parity-receipts/switch--nucleus-settings-switch--a1.json";
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
