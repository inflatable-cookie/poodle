import { describe, expect, it } from "bun:test";
import {
  ADMITTED_VIA,
  AXIS_TEST_SIGNALS,
  CENSUS_AXES,
  PLATFORM_LANGUAGE,
  RECEIPT_SCHEMA,
  admitReceiptTextAxes,
  admitTestAxes,
  extractTestBody,
  validateCapabilityManifest,
  validateCensusDoc,
  type CensusAxis,
  type CensusDoc,
  type ManifestEntry,
} from "./gpui-functionality-census";

const axes = [...CENSUS_AXES];

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
          note: "Assistive-technology projection waits on the gpui-apple publication gate.",
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
    denominator: { public: 176, portable: 175, notApplicable: ["MeterSurface"] },
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
      mountedScope: "bounded named regression set, not a 175-component behaviour pass",
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
    expect(() => validateCensusDoc(doc)).toThrow(/denominator|176 rows/i);
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
        mountedScope: "bounded named regression set, not a 175-component behaviour pass",
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
});
