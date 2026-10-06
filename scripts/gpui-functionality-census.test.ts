import { describe, expect, it } from "bun:test";
import { readFileSync, readdirSync } from "node:fs";
import path from "node:path";
import {
  ADMITTED_VIA,
  AXIS_TEST_SIGNALS,
  CENSUS_AXES,
  GPUI_PREVIEW_CARGO_MANIFEST,
  PLATFORM_LANGUAGE,
  RECEIPT_DIR,
  RECEIPT_SCHEMA,
  admitReceiptTextAxes,
  admitTestAxes,
  CENSUS_JSON_PATH,
  deriveCapabilityManifest,
  expectedTestReceiptContent,
  extractTestBody,
  loadExecutionRecord,
  loadPreviewPackageVersion,
  observedDriver,
  observedRenderer,
  parsePreviewPackageVersion,
  validateCapabilityManifest,
  validateCensusDoc,
  validateExecutionRecord,
  validateReceiptPackageVersion,
  type CensusAxis,
  type CensusDoc,
  type ManifestEntry,
} from "./gpui-functionality-census";

import {
  PORTABLE_ROUTE_COUNT,
  PUBLIC_COMPONENT_COUNT,
  ROSTER_WEB_ONLY_NAMES,
} from "./component-denominator";

const axes = [...CENSUS_AXES];
const repositoryRoot = path.resolve(import.meta.dir, "..");

function manifestEntry(overrides: Partial<ManifestEntry> = {}): ManifestEntry {
  return {
    component: "Button",
    portable: true,
    contract: "docs/contracts/components/button.md",
    substrate: "general-composite",
    required: [...axes],
    notApplicable: [],
    ...overrides,
  };
}

function censusDoc(overrides: Partial<CensusDoc> = {}): CensusDoc {
  const rows = [
    {
      component: "Button",
      portable: true,
      contract: "docs/contracts/components/button.md",
      substrate: "general-composite",
      required: ["semantic", "events", "pointer", "keyboard_focus", "accessibility", "visual"] as CensusAxis[],
      admitted: [
        {
          axis: "semantic" as CensusAxis,
          via: "expected-test" as const,
          ref: "packages/gpui/preview/tests/headless_regressions.rs#a_mounted_button_carries_its_controls_target",
        },
      ],
      missing: ["events", "pointer", "keyboard_focus", "accessibility", "visual"] as CensusAxis[],
      holds: [
        {
          axis: "accessibility" as CensusAxis,
          kind: "A2-platform-hold" as const,
          note: "The live platform-tree proof is blocked by local macOS Accessibility trust; no platform content has been verified for this component.",
          ref: "docs/contracts/003-native-accessibility.md",
        },
      ],
      receipts: ["docs/evidence/gpui/mounted-receipts/Button--a-mounted-button.json"],
      refusals: [],
    },
  ];
  return {
    schema: "poodle.g18-gpui-functionality-census.v1",
    task: "g18.001",
    source_commit: "d8e174fb40b2634b7d00018c721816ffc037d712",
    denominator: {
      public: PUBLIC_COMPONENT_COUNT,
      portable: PORTABLE_ROUTE_COUNT,
      notApplicable: [...ROSTER_WEB_ONLY_NAMES],
    },
    axes: [...axes],
    manifest: [manifestEntry()],
    rows,
    summary: {
      admittedRows: 1,
      fullyAdmittedRows: 0,
      missingTally: { semantic: 0, events: 1, pointer: 1, keyboard_focus: 1, accessibility: 1, visual: 1 },
      holdCount: 1,
      refusalCount: 0,
    },
    groups: [],
    crossRuntime: {
      constructionClaim: "Every portable component route constructs through the headless GPUI specimen probe.",
      mountedScope: "bounded named regression set, not a roster-wide behaviour pass",
      note: "Construction is not functional completion.",
    },
    ...overrides,
  };
}

describe("g18.001 census oracles", () => {
  it("keeps the axis set closed", () => {
    expect([...CENSUS_AXES].sort()).toEqual(
      ["accessibility", "events", "keyboard_focus", "pointer", "semantic", "visual"].sort(),
    );
    expect(ADMITTED_VIA.sort()).toEqual(["expected-test", "nucleus-a1", "nucleus-m1", "nucleus-v1"].sort());
    expect(RECEIPT_SCHEMA).toBe("poodle.g18-gpui-mounted-receipt.v1");
  });

  it("denominator oracle: rejects a dropped component row", () => {
    const doc = censusDoc({ rows: [] });
    expect(() => validateCensusDoc(doc)).toThrow(/denominator|rows/i);
  });

  it("denominator oracle: rejects a duplicated component row", () => {
    const doc = censusDoc();
    doc.rows.push({ ...doc.rows[0] });
    expect(() => validateCensusDoc(doc)).toThrow(/duplicate/i);
  });

  it("stale-test oracle: a renamed expected test admits nothing", () => {
    const body = extractTestBody("test/fixture", "does_not_exist_anywhere");
    expect(body).toBeUndefined();
  });

  it("test-body oracle: appended test docs stay outside the previous body", () => {
    const body = extractTestBody(repositoryRoot, "toast_stack_action_removal_hands_focus_on_or_leaves_it_alone");
    expect(body).toBeDefined();
    expect(body!.trimEnd().endsWith("\n}")).toBe(true);
    expect(body).not.toContain("Spinner parity against the Svelte contract");
  });

  it("claim-overreach oracle: a pointer-only body never admits keyboard_focus", () => {
    const body = [
      "run_headless(|cx| {",
      "  let payloads = Arc::new(Mutex::new(Vec::new()));",
      "  let sink = Arc::clone(&payloads);",
      "  let node = poodle_render::button(&spec, &ctx, Some(Arc::new(move |next| sink.lock().unwrap().push(next))));",
      "  let mut driver = HeadlessDriver::new(cx, node);",
      "  driver.pointer_activate();",
      "  assert_eq!(payloads.lock().unwrap().as_slice(), [true]);",
      "});",
    ].join("\n");
    const admission = admitTestAxes(body);
    expect(admission.production).toBe(true);
    expect(admission.axes).toContain("pointer");
    expect(admission.axes).toContain("events");
    expect(admission.axes).not.toContain("keyboard_focus");
    expect(admission.axes).not.toContain("accessibility");
    expect(admission.axes).not.toContain("visual");
  });

  it("backend-bypass oracle: a direct handler call without the mounted backend admits nothing", () => {
    const body = [
      "fn button_handler_unit() {",
      "  let count = counting_handler();",
      "  (count.0)();",
      "  assert_eq!(*count.1.lock().unwrap(), 1);",
      "}",
    ].join("\n");
    const admission = admitTestAxes(body);
    expect(admission.production).toBe(false);
    expect(admission.axes).toEqual([]);
  });

  it("backend-bypass oracle: a renderer unit test without HeadlessDriver admits nothing", () => {
    const body = [
      "fn button_node_shape() {",
      "  let node = poodle_render::button(&spec, &ctx, None);",
      "  assert_eq!(node.id, Some(\"x\".into()));",
      "}",
    ].join("\n");
    const admission = admitTestAxes(body);
    expect(admission.production).toBe(false);
    expect(admission.axes).toEqual([]);
  });

  it("production-mount oracle: imported node-compat components mount through the renderer", () => {
    const source = readFileSync(path.join(repositoryRoot, "packages/gpui/preview/tests/headless_regressions.rs"), "utf8");
    const cases = [
      "gpui_mounted_selection_summary_split_actions_and_accessible_names",
      "gpui_mounted_nav_card_link_button_actions_and_accessibility",
    ];
    for (const name of cases) {
      const body = extractTestBody(repositoryRoot, name);
      expect(body, name).toBeDefined();
      const admission = admitTestAxes(body!, source, name);
      expect(admission.production, name).toBe(true);
      expect(admission.axes, name).toContain("semantic");
      expect(observedRenderer(body!, source, name), name).toMatch(/^node_compat-import:/);
    }
  });

  it("manifest oracle: callbacks and static summaries follow their contract surfaces", () => {
    const manifest = deriveCapabilityManifest(repositoryRoot);
    const pill = manifest.find((entry) => entry.component === "Pill");
    const audioPlayer = manifest.find((entry) => entry.component === "AudioPlayer");
    const appHeader = manifest.find((entry) => entry.component === "AppHeader");
    const paginationSummary = manifest.find((entry) => entry.component === "PaginationSummary");
    const agentQuestionRecord = manifest.find((entry) => entry.component === "AgentQuestionRecord");
    const detailShell = manifest.find((entry) => entry.component === "DetailShell");
    const detailSectionGroup = manifest.find((entry) => entry.component === "DetailSectionGroup");
    const pageHeader = manifest.find((entry) => entry.component === "PageHeader");
    const region = manifest.find((entry) => entry.component === "Region");
    expect(pill?.required).toEqual(axes);
    expect(pill?.notApplicable).toEqual([]);
    expect(paginationSummary?.required).toEqual(["semantic", "accessibility", "visual"]);
    expect(paginationSummary?.notApplicable.map((item) => item.axis)).toEqual([
      "events",
      "pointer",
      "keyboard_focus",
    ]);
    expect(paginationSummary?.notApplicable.every((item) => item.contractRef.includes("pagination-summary.md#"))).toBe(true);
    expect(agentQuestionRecord?.required).toEqual(["semantic", "accessibility", "visual"]);
    expect(agentQuestionRecord?.notApplicable).toEqual([
      {
        axis: "events",
        reason:
          "Contract declares the record has no interactive parts and no inputs, so it has no callbacks or events to prove.",
        contractRef: "docs/contracts/components/agent-question-record.md#2. Read-Only By Construction",
      },
      {
        axis: "pointer",
        reason: "Contract declares the record has no interactive parts, so it has no pointer interaction to prove.",
        contractRef: "docs/contracts/components/agent-question-record.md#2. Read-Only By Construction",
      },
      {
        axis: "keyboard_focus",
        reason: "Contract states nothing inside is focusable, so the record never appears in the tab order.",
        contractRef: "docs/contracts/components/agent-question-record.md#6. Accessibility",
      },
    ]);
    expect(audioPlayer?.required).toEqual([
      "semantic",
      "pointer",
      "keyboard_focus",
      "accessibility",
      "visual",
    ]);
    expect(audioPlayer?.notApplicable.map((item) => item.axis)).toEqual(["events"]);
    expect(audioPlayer?.notApplicable[0]?.contractRef).toBe(
      "docs/contracts/components/audio-player.md#5. Events",
    );
    expect(appHeader?.required).toEqual(["semantic", "pointer", "keyboard_focus", "accessibility", "visual"]);
    expect(appHeader?.notApplicable).toEqual([
      {
        axis: "events",
        reason: "Contract declares no component callbacks or events.",
        contractRef: "docs/contracts/components/app-header.md#6. Events",
      },
    ]);
    expect(detailShell?.required).toEqual(["semantic", "pointer", "keyboard_focus", "accessibility", "visual"]);
    expect(detailShell?.notApplicable.map((item) => item.axis)).toEqual(["events"]);
    expect(pageHeader?.required).toEqual(["semantic", "pointer", "keyboard_focus", "accessibility", "visual"]);
    expect(pageHeader?.notApplicable.map((item) => item.axis)).toEqual(["events"]);
    expect(detailSectionGroup?.required).toEqual(["semantic", "accessibility", "visual"]);
    expect(detailSectionGroup?.notApplicable.map((item) => item.axis)).toEqual([
      "events",
      "pointer",
      "keyboard_focus",
    ]);
    expect(detailSectionGroup?.notApplicable.every((item) => item.contractRef.includes("detail-section-group.md#"))).toBe(true);
    expect(region?.required).toEqual(["semantic", "accessibility", "visual"]);
    expect(region?.notApplicable.map((item) => item.axis)).toEqual([
      "events",
      "pointer",
      "keyboard_focus",
    ]);
    expect(region?.notApplicable.every((item) => item.contractRef.includes("region.md#"))).toBe(true);
  });

  it("manifest oracle: static metadata leaves only its interaction axes not applicable", () => {
    const manifest = deriveCapabilityManifest(repositoryRoot);
    for (const [component, contract] of [
      ["Avatar", "avatar.md"],
      ["MetaBar", "meta-bar.md"],
      ["MetaItem", "meta-item.md"],
      ["AudioMeter", "audio-meter.md"],
      ["GainReductionMeter", "gain-reduction-meter.md"],
      ["ValueReadout", "value-readout.md"],
    ]) {
      const entry = manifest.find((candidate) => candidate.component === component);
      expect(entry?.required).toEqual(["semantic", "accessibility", "visual"]);
      expect(entry?.notApplicable.map((item) => item.axis)).toEqual([
        "events",
        "pointer",
        "keyboard_focus",
      ]);
      expect(entry?.notApplicable.every((item) => item.contractRef.includes(contract))).toBe(true);
      expect(entry?.notApplicable.every((item) => item.reason.length > 0)).toBe(true);
    }
  });

  it("manifest oracle: feedback states keep only their delegated-events axis not applicable", () => {
    const manifest = deriveCapabilityManifest(repositoryRoot);
    for (const [component, contract, required] of [
      ["EmptyState", "empty-state.md", ["semantic", "pointer", "keyboard_focus", "accessibility", "visual"]],
      ["ErrorBoundary", "error-boundary.md", ["semantic", "pointer", "keyboard_focus", "accessibility", "visual"]],
      ["EmbedPreview", "embed-preview.md", ["semantic", "pointer", "keyboard_focus", "accessibility", "visual"]],
      ["InlineListSection", "inline-list-section.md", ["semantic", "pointer", "keyboard_focus", "accessibility", "visual"]],
      ["PasswordRequirements", "password-requirements.md", ["semantic", "accessibility", "visual"]],
    ]) {
      const entry = manifest.find((candidate) => candidate.component === component);
      expect(entry?.required).toEqual(required);
      expect(entry?.notApplicable.map((item) => item.axis)).toEqual(
        component === "PasswordRequirements" ? ["events", "pointer", "keyboard_focus"] : ["events"],
      );
      expect(entry?.notApplicable.every((item) => item.contractRef.includes(contract as string))).toBe(true);
      expect(entry?.notApplicable.every((item) => item.reason.length > 0)).toBe(true);
    }
  });

  it("manifest oracle: workstation and agent composites keep only their contract-declared N/A axes", () => {
    const manifest = deriveCapabilityManifest(repositoryRoot);
    for (const [component, contract, notApplicable] of [
      ["DetailSection", "detail-section.md", ["events"]],
      ["MotionPolicyProvider", "motion-policy-provider.md", ["events", "pointer"]],
      ["UiPresentationProvider", "ui-presentation-provider.md", ["events", "pointer"]],
      ["AgentMessage", "agent-message.md", ["events", "pointer"]],
      ["ScrollShell", "scroll-shell.md", []],
    ] as Array<[string, string, string[]]>) {
      const entry = manifest.find((candidate) => candidate.component === component);
      expect(entry?.notApplicable.map((item) => item.axis)).toEqual(notApplicable);
      expect(entry?.notApplicable.every((item) => item.contractRef.includes(contract))).toBe(true);
      expect(entry?.notApplicable.every((item) => item.reason.length > 0)).toBe(true);
    }
  });

  it("widened-A2 oracle: platform language can never justify not-applicable", () => {
    const entry = manifestEntry({
      component: "Checkbox",
      contract: "docs/contracts/components/checkbox.md",
      required: ["semantic", "events", "pointer", "visual"],
      notApplicable: [
        {
          axis: "keyboard_focus",
          reason: "No native accessibility tree on gpui 0.2.2, so keyboard and focus are not applicable (A2 hold).",
          contractRef: "docs/contracts/003-native-accessibility.md#A2",
        },
        {
          axis: "accessibility",
          reason: "Upstream publication hold makes accessibility not-applicable.",
          contractRef: "docs/contracts/003-native-accessibility.md#Consequences For Planning",
        },
      ],
    });
    expect(() => validateCapabilityManifest([entry])).toThrow(/A2|platform/i);
  });

  it("widened-A2 oracle: the platform detector matches every known phrasing", () => {
    for (const phrase of [
      "blocked on A2",
      "no platform tree yet",
      "waits on accesskit projection",
      "gpui-apple cannot build",
      "upstream has not published",
      "pinned at 0.2.2",
      "publication hold",
      "no assistive-technology proof available",
    ]) {
      expect(PLATFORM_LANGUAGE.test(phrase), phrase).toBe(true);
    }
  });

  it("manifest oracle: required plus not-applicable must partition the closed axes", () => {
    const entry = manifestEntry({ required: ["semantic"] });
    expect(() => validateCapabilityManifest([entry])).toThrow(/partition/i);
  });

  it("overclaim oracle: construction language can never read as functional completion", () => {
    const doc = censusDoc({
      crossRuntime: {
        constructionClaim: "All 175 portable routes construct, so the catalogue is fully functional.",
        mountedScope: "bounded named regression set, not a roster-wide behaviour pass",
        note: "Construction is not functional completion.",
      },
    });
    expect(() => validateCensusDoc(doc)).toThrow(/overclaim|fully functional/i);
  });

  it("overclaim oracle: holds stay off every axis except accessibility", () => {
    const doc = censusDoc();
    doc.rows[0].holds = [
      {
        axis: "keyboard_focus",
        kind: "A2-platform-hold",
        note: "Keyboard held by the platform gate.",
        ref: "docs/contracts/003-native-accessibility.md",
      },
    ];
    expect(() => validateCensusDoc(doc)).toThrow(/hold/i);
  });

  it("receipt-text oracle: pointer-only prose never admits keyboard_focus", () => {
    const axes = admitReceiptTextAxes(
      "mount the control and activate it through mounted pointer input; the callback emits exactly once",
    );
    expect(axes).toContain("semantic");
    expect(axes).toContain("pointer");
    expect(axes).toContain("events");
    expect(axes).not.toContain("keyboard_focus");
  });

  it("signal table covers every closed axis", () => {
    for (const axis of axes) {
      expect(AXIS_TEST_SIGNALS[axis].length).toBeGreaterThan(0);
    }
  });

  it(
    "record-state oracle: evidence identity is pinned, current tree descends from it",
    () => {
      const root = path.resolve(import.meta.dir, "..");
      const record = loadExecutionRecord(root);
      // The census's own generator embeds the record's commit; the checked-in
      // census must carry exactly that identity so the record stays the single
      // evidence source (state, never live HEAD).
      const census = JSON.parse(readFileSync(path.join(root, CENSUS_JSON_PATH), "utf8")) as { source_commit?: string };
      expect(record.source_commit).toMatch(/^[0-9a-f]{40}$/);
      expect(census.source_commit).toBe(record.source_commit);
      // Pin ancestry, lockfile identity, and every admitted test body hash are
      // the record's own oracle; repinning the identity to a newer ancestor
      // stays legal without touching this test. It re-reads and hashes every
      // expected body, so it grows with the expected-test set and needs more
      // than the default 5s budget on a loaded machine.
      validateExecutionRecord(record, root);
    },
    30_000,
  );

  it("evidence-text oracle: receipts store matched body text, not patterns", () => {
    const body = [
      "run_headless(|cx| {",
      "  let node = poodle_render::button(&spec, &ctx, handler);",
      "  let mut driver = HeadlessDriver::new(cx, node);",
      "  driver.pointer_activate();",
      "  assert_eq!(count.lock().unwrap().as_slice(), [1]);",
      "});",
    ].join("\n");
    const admission = admitTestAxes(body);
    for (const fragment of admission.signals.pointer) {
      expect(fragment).not.toContain("/");
      expect(body).toContain(fragment);
    }
    expect(observedDriver(body, "", "")).toBe("HeadlessDriver::new");
    expect(observedRenderer(body, "", "")).toBe("poodle_render::button");
  });

  it("announce-hardening oracle: prose alone without the read idiom admits no accessibility", () => {
    const body = [
      "run_headless(|cx| {",
      "  let node = poodle_render::button(&spec, &ctx, handler);",
      "  let mut driver = HeadlessDriver::new(cx, node);",
      "  driver.pointer_activate();",
      "  // announcement role is carried by the host, asserted elsewhere",
      "  assert_eq!(count.lock().unwrap().as_slice(), [1]);",
      "});",
    ].join("\n");
    const admission = admitTestAxes(body);
    expect(admission.production).toBe(true);
    expect(admission.axes).not.toContain("accessibility");
  });

  it("formatting-tolerance oracle: a chain rustfmt splits across lines claims what the one-line form claims", () => {
    // poodle#072 census ruling (2026-09-30): formatting must never change a
    // census claim. rustfmt wrapped `n.a11y.label` onto separate chain lines
    // and the one-line `a11y\.` matcher dropped a true accessibility
    // admission; the matcher and the claim must be wrap-invariant.
    const mounted = [
      "run_headless(|cx| {",
      "  let node = poodle_render::button(&spec, &ctx, handler);",
      "  let mut driver = HeadlessDriver::new(cx, node);",
      "  driver.pointer_activate();",
    ];
    const oneLine = admitTestAxes(
      [
        ...mounted,
        '  assert!(give_first_id(&mut node, "card-switch", &|n| n.a11y.label.as_deref() == Some("Ready")));',
        "});",
      ].join("\n"),
    );
    const split = admitTestAxes(
      [
        ...mounted,
        '  assert!(give_first_id(&mut node, "card-switch", &|n| n',
        "    .a11y",
        "    .label",
        "    .as_deref()",
        '    == Some("Ready")));',
        "});",
      ].join("\n"),
    );
    expect(oneLine.production).toBe(true);
    expect(split.production).toBe(true);
    expect(oneLine.axes).toContain("accessibility");
    expect(split.axes).toEqual(oneLine.axes);
    expect(split.signals.accessibility.map((fragment) => fragment.replaceAll(/\s+/g, ""))).toEqual(
      oneLine.signals.accessibility.map((fragment) => fragment.replaceAll(/\s+/g, "")),
    );
  });
});

describe("g18.031 census release provenance", () => {
  const root = path.resolve(import.meta.dir, "..");

  it("live-manifest law: preview Cargo.toml is the receipt version authority", () => {
    const live = loadPreviewPackageVersion(root);
    const manifest = readFileSync(path.join(root, GPUI_PREVIEW_CARGO_MANIFEST), "utf8");
    expect(parsePreviewPackageVersion(manifest)).toBe(live);
    const receiptDir = path.join(root, RECEIPT_DIR);
    const files = readdirSync(receiptDir);
    expect(files.length).toBeGreaterThan(0);
    for (const file of files) {
      const receipt = JSON.parse(readFileSync(path.join(receiptDir, file), "utf8")) as {
        package_version?: string;
      };
      expect(receipt.package_version, file).toBe(live);
    }
  });

  it("planted-manifest law: a preview bump flows into emitted receipt content", () => {
    const planted = '[package]\nname = "poodle-gpui-preview"\nversion = "0.4.0"\n';
    const version = parsePreviewPackageVersion(planted);
    expect(version).toBe("0.4.0");
    const content = expectedTestReceiptContent({
      component: "Button",
      test: "a_mounted_button_carries_its_controls_target",
      command: "effigy regressions:native",
      axes: ["semantic"],
      signals: { semantic: ["assert"], events: [], pointer: [], keyboard_focus: [], accessibility: [], visual: [] },
      driver: "HeadlessDriver::new",
      renderer: "poodle_render::button",
      packageVersion: version,
      sourceCommit: "d8e174fb40b2634b7d00018c721816ffc037d712",
      lockfileSha256: "0".repeat(64),
      runId: "planted-run",
      bodySha256: "0".repeat(64),
    });
    const receipt = JSON.parse(content) as { schema?: string; package_version?: string };
    expect(receipt.schema).toBe(RECEIPT_SCHEMA);
    expect(receipt.package_version).toBe("0.4.0");
  });

  it("fail-closed law: missing, duplicate, and malformed versions are rejected", () => {
    expect(() => parsePreviewPackageVersion('[package]\nname = "poodle-gpui-preview"\n')).toThrow(/one version/i);
    expect(() =>
      parsePreviewPackageVersion('[package]\nversion = "0.3.0"\nversion = "0.4.0"\n'),
    ).toThrow(/one version/i);
    expect(() => parsePreviewPackageVersion("[package]\nversion = 0.4\n")).toThrow(/quoted semver/i);
    expect(() => parsePreviewPackageVersion('[package]\nversion = "release"\n')).toThrow(/quoted semver/i);
    expect(() =>
      parsePreviewPackageVersion('[package]\nversion = "0.3.0"\n\n[package]\nversion = "0.4.0"\n'),
    ).toThrow(/one \[package\]/i);
  });

  it("stale-receipt law: a receipt version that disagrees with the live manifest is rejected", () => {
    expect(validateReceiptPackageVersion("0.3.0", "0.3.0", "Button")).toBe("0.3.0");
    expect(() => validateReceiptPackageVersion("0.2.0", "0.3.0", "Button")).toThrow(/preview manifest/i);
    expect(() => validateReceiptPackageVersion(undefined, "0.3.0", "Button")).toThrow(/preview manifest/i);
  });
});
