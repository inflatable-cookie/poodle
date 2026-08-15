/**
 * Web primitive substrate probes (g14.002).
 *
 * Uses the shared web observer (geometry/channels/states) plus the existing
 * Button adapters for focus/activate/toggle paths. Test-only DOM fixtures
 * cover substrate channels that do not need a public probe component.
 */

import { describe, expect, it } from "vitest";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";

import {
  buttonCases,
  buttonInterface,
  ownedPrimitiveCapabilities,
  popoverCases,
  popoverInterface,
  serializeInterface,
  textInputCases,
  textInputInterface,
  type PrimitiveCapabilityId,
} from "../../../packages/core/src/conformance";
import { installLayoutStub } from "./layout-stub";
import { observeDom, runCase, type RuntimeAdapter } from "./runner";
import { computedStyleOf, parseLength } from "./observer";
import { ReactPopoverAdapter } from "./react-popover-adapter";
import { ReactTextInputAdapter } from "./react-text-input-adapter";
import { SvelteButtonAdapter } from "./svelte-adapter";
import { SveltePopoverAdapter } from "./svelte-popover-adapter";
import { SvelteTextInputAdapter } from "./svelte-text-input-adapter";
import { ReactButtonAdapter } from "./react-adapter";

const OUT_DIR = `${import.meta.dirname}/out`;
const iface = serializeInterface(buttonInterface);
const popoverIface = serializeInterface(popoverInterface);
const textInputIface = serializeInterface(textInputInterface);

const tokensCss = readFileSync(
  `${import.meta.dirname}/../../../packages/core/src/tokens/generated/css/poodle-tokens.css`,
  "utf8",
);
const buttonCss = readFileSync(
  `${import.meta.dirname}/../../../packages/core/src/styles/button.css`,
  "utf8",
);
const popoverCss = readFileSync(
  `${import.meta.dirname}/../../../packages/core/src/styles/popover.css`,
  "utf8",
);
const textInputCss = readFileSync(
  `${import.meta.dirname}/../../../packages/core/src/styles/text-input.css`,
  "utf8",
);
const anchoredCss = readFileSync(
  `${import.meta.dirname}/../../../packages/core/src/styles/anchored-surface.css`,
  "utf8",
);

type ProbeRow = {
  capabilityId: string;
  probeId: string;
  verdict: "pass" | "fail";
  observations: string[];
  fields: Record<string, unknown>;
  reason?: string;
};

function injectRealCss(): void {
  if (document.getElementById("conformance-web-css")) return;
  const style = document.createElement("style");
  style.id = "conformance-web-css";
  style.textContent = `${tokensCss}\n${buttonCss}\n${popoverCss}\n${anchoredCss}\n${textInputCss}\n@keyframes primitive-probe-spin { from { opacity: .98; } to { opacity: .97; } }`;
  document.head.appendChild(style);
}

function pass(
  capabilityId: PrimitiveCapabilityId | string,
  probeId: string,
  fields: Record<string, unknown>,
  observations: string[] = [],
): ProbeRow {
  return { capabilityId, probeId, verdict: "pass", observations, fields };
}

function fail(
  capabilityId: string,
  probeId: string,
  fields: Record<string, unknown>,
  reason: string,
  observations: string[] = [],
): ProbeRow {
  return { capabilityId, probeId, verdict: "fail", observations, fields, reason };
}

/** Substrate DOM fixture — not a public component. */
function mountSurfaceFixture(): HTMLButtonElement {
  const root = document.createElement("button");
  root.id = "primitive-probe-root";
  root.setAttribute("aria-label", "Probe control");
  root.setAttribute("data-has-leading", "true");
  root.setAttribute("data-variant", "primary");
  root.setAttribute("data-tone", "default");
  root.tabIndex = 0;
  root.style.cssText = [
    "height:40px",
    "min-width:80px",
    "padding:8px 12px",
    "border-radius:6px",
    "border:1px solid rgb(89, 102, 122)",
    "background:rgb(46, 56, 71)",
    "color:rgb(242, 245, 250)",
    "opacity:0.98",
    "font-family:system-ui",
    "font-size:14px",
    "font-weight:500",
    "letter-spacing:0.02em",
    "text-align:center",
    "position:relative",
    "overflow:hidden",
    "cursor:pointer",
    "box-shadow:0 2px 4px rgba(0,0,0,0.25)",
    "animation:primitive-probe-spin 2s linear infinite",
  ].join(";");
  root.style.animationName = "primitive-probe-spin";
  root.style.animationDuration = "2s";
  const icon = document.createElement("span");
  icon.setAttribute("data-icon", "star");
  icon.className = "poodle-button__icon";
  icon.textContent = "★";
  const label = document.createElement("span");
  label.className = "poodle-button__label";
  label.textContent = "Probe";
  root.append(icon, label);
  document.body.appendChild(root);
  return root;
}

function fixtureProbes(root: HTMLElement): ProbeRow[] {
  const observation = observeDom("web-fixture", "primitive-probe", iface, root);
  const rootPart = observation.parts.root;
  const geometry = rootPart.geometry;
  const channels = rootPart.channels;
  const style = computedStyleOf(root);
  const icon = root.querySelector("[data-icon]") as HTMLElement | null;
  const label = root.querySelector(".poodle-button__label") as HTMLElement | null;

  const rows: ProbeRow[] = [];

  rows.push(
    root.id === "primitive-probe-root" && root.children.length === 2
      ? pass("structure.identity", "dom-tree", {
          "node.id": root.id,
          "children.len": root.children.length,
        }, ["parts.present"])
      : fail("structure.identity", "dom-tree", { id: root.id }, "node.id or children"),
  );

  rows.push(
    icon?.getAttribute("data-icon") === "star" && label?.textContent === "Probe"
      ? pass("structure.part-resolution", "dom-parts", {
          "parts.icon": icon.getAttribute("data-icon"),
          "parts.text": label.textContent,
        }, ["parts.present", "parts.text", "parts.icon"])
      : fail("structure.part-resolution", "dom-parts", {}, "parts.text or parts.icon"),
  );

  rows.push(
    style.position === "relative" && style.overflow === "hidden"
      ? pass("layout.intent", "computed-layout", {
          position: style.position,
          overflow: style.overflow,
          display: style.display,
        }, ["parts.geometry"])
      : fail("layout.intent", "computed-layout", { position: style.position }, "layout intent"),
  );

  rows.push(
    geometry.height != null && geometry.minWidth != null && geometry.paddingLeft != null
      ? pass("layout.geometry", "computed-geometry", geometry, ["parts.geometry"])
      : fail("layout.geometry", "computed-geometry", geometry, "parts.geometry"),
  );

  rows.push(
    style.position === "relative"
      ? pass("layout.position", "computed-position", { position: style.position })
      : fail("layout.position", "computed-position", { position: style.position }, "position"),
  );

  rows.push(
    channels.background && channels.borderColor && channels.color && channels.opacity
      ? pass("surface.channels", "computed-channels", channels, ["parts.channels"])
      : fail("surface.channels", "computed-channels", channels, "parts.channels"),
  );

  rows.push(
    (style.boxShadow && style.boxShadow !== "none") || style.cursor === "pointer"
      ? pass("surface.extended", "computed-extended", {
          boxShadow: style.boxShadow,
          cursor: style.cursor,
          visibility: style.visibility,
        })
      : fail("surface.extended", "computed-extended", {
          boxShadow: style.boxShadow,
          cursor: style.cursor,
        }, "surface.extended"),
  );

  // State patches: focus-visible path via :focus-visible after focus().
  root.focus();
  const focusVisible = root.matches(":focus-visible") || document.activeElement === root;
  rows.push(
    focusVisible
      ? pass("surface.state-patches", "focus-style", { focusVisible: true }, ["parts.focusVisible"])
      : fail("surface.state-patches", "focus-style", { focusVisible }, "parts.focusVisible"),
  );

  rows.push(
    root.style.animationName === "primitive-probe-spin"
      ? pass("surface.animation", "dom-animation-declaration", {
          animationName: root.style.animationName,
          animationDuration: root.style.animationDuration,
        })
      : fail(
          "surface.animation",
          "dom-animation-declaration",
          { animationName: root.style.animationName },
          "animationName",
        ),
  );

  rows.push(
    icon?.getAttribute("data-icon") === "star" && label?.textContent === "Probe"
      ? pass("content.text-icon", "dom-content", {
          icon: icon.getAttribute("data-icon"),
          text: label.textContent,
        }, ["parts.text", "parts.icon"])
      : fail("content.text-icon", "dom-content", {}, "parts.text or parts.icon"),
  );

  rows.push(
    parseLength(style.fontSize) === 14 && style.fontWeight !== "400"
      ? pass("content.typography", "computed-typography", {
          fontSize: style.fontSize,
          fontWeight: style.fontWeight,
          letterSpacing: style.letterSpacing,
          textAlign: style.textAlign,
        })
      : fail("content.typography", "computed-typography", {}, "typography"),
  );

  rows.push(
    rootPart.tokenRoles.variant === "primary" && rootPart.tokenRoles.tone === "default"
      ? pass("semantic.token-roles", "data-roles", {
          variant: rootPart.tokenRoles.variant,
          tone: rootPart.tokenRoles.tone,
        }, ["parts.tokenRoles"])
      : fail("semantic.token-roles", "data-roles", {}, "parts.tokenRoles"),
  );

  rows.push(
    rootPart.role === "button" && rootPart.name === "Probe control"
      ? pass("accessibility.projection", "dom-a11y", {
          role: rootPart.role,
          name: rootPart.name,
        }, ["parts.role", "parts.name"])
      : fail("accessibility.projection", "dom-a11y", {}, "parts.role or parts.name"),
  );

  root.remove();
  return rows;
}

/** Button corpus cases that exercise interaction/semantic substrate on web. */
const BUTTON_PROBE_CASES: Array<{
  caseId: string;
  capabilities: Array<{ id: PrimitiveCapabilityId; observations: string[] }>;
}> = [
  {
    caseId: "button/press-pointer",
    capabilities: [
      { id: "activate", observations: ["trace"] },
      { id: "structure.part-resolution", observations: ["parts.present", "parts.text", "parts.icon"] },
      { id: "content.text-icon", observations: ["parts.text", "parts.icon"] },
    ],
  },
  { caseId: "button/disabled", capabilities: [{ id: "semantic.disabled", observations: ["parts.states"] }] },
  { caseId: "button/toggle", capabilities: [{ id: "toggle", observations: ["parts.states"] }] },
  {
    caseId: "button/focus-visible",
    capabilities: [
      { id: "focus", observations: ["parts.focusable", "parts.focused"] },
      { id: "surface.state-patches", observations: ["parts.focusVisible"] },
    ],
  },
];

async function buttonBackedProbes(adapter: RuntimeAdapter): Promise<ProbeRow[]> {
  const rows: ProbeRow[] = [];
  for (const mapping of BUTTON_PROBE_CASES) {
    const caseData = buttonCases.cases.find((c) => c.id === mapping.caseId);
    if (!caseData) {
      for (const capability of mapping.capabilities) {
        rows.push(fail(capability.id, mapping.caseId, {}, "case missing", capability.observations));
      }
      continue;
    }
    const { results } = await runCase(adapter, iface, buttonCases.component, caseData);
    const failures = results.filter((r) => r.verdict === "fail");
    const ok = failures.length === 0;
    for (const capability of mapping.capabilities) {
      rows.push(
        ok
          ? pass(capability.id, mapping.caseId, { casePass: true, assertions: results.length }, capability.observations)
          : fail(capability.id, mapping.caseId, { failures }, "button case", capability.observations),
      );
    }
  }
  return rows;
}

function writeEvidence(runtime: string, probes: ProbeRow[]): void {
  mkdirSync(OUT_DIR, { recursive: true });
  // Prefer fail if any probe for a capability failed.
  const merged = new Map<string, ProbeRow>();
  for (const probe of probes) {
    const prior = merged.get(probe.capabilityId);
    if (!prior) {
      merged.set(probe.capabilityId, probe);
      continue;
    }
    if (probe.verdict === "fail") merged.set(probe.capabilityId, probe);
  }

  const ownedIds = new Set(ownedPrimitiveCapabilities().map((row) => row.id));
  const reportProbes = [...merged.values()].filter((row) => ownedIds.has(row.capabilityId as PrimitiveCapabilityId));

  // Ensure every owned capability has a row (missing → fail).
  for (const row of ownedPrimitiveCapabilities()) {
    if (![...reportProbes].some((p) => p.capabilityId === row.id)) {
      reportProbes.push(
        fail(row.id, "missing", {}, `capability=${row.id} runtime=${runtime} probe=missing field=evidence`),
      );
    }
  }

  writeFileSync(
    `${OUT_DIR}/primitive-${runtime}.json`,
    `${JSON.stringify(
      {
        schema: "primitive-probe-evidence.v1",
        runtime,
        probes: reportProbes,
      },
      null,
      2,
    )}\n`,
  );
}

/** Popover corpus cases that exercise the overlay rows on web (g14.005):
 * mounted overlay/layer evidence, the expanded projection, and real
 * dismissal traces through the document-level routes. */
const POPOVER_PROBE_CASES: Array<{
  caseId: string;
  capabilities: Array<{ id: PrimitiveCapabilityId; observations: string[] }>;
}> = [
  {
    caseId: "popover/semantics-tokens",
    capabilities: [
      { id: "overlay.intent", observations: ["parts.overlay"] },
      { id: "semantic.expanded", observations: ["parts.expanded"] },
      { id: "overlay.layer", observations: ["parts.layerCount"] },
    ],
  },
  {
    caseId: "popover/escape",
    capabilities: [{ id: "overlay.dismiss", observations: ["trace"] }],
  },
];

async function popoverBackedProbes(adapter: RuntimeAdapter): Promise<ProbeRow[]> {
  const rows: ProbeRow[] = [];
  for (const mapping of POPOVER_PROBE_CASES) {
    const caseData = popoverCases.cases.find((c) => c.id === mapping.caseId);
    if (!caseData) {
      for (const capability of mapping.capabilities) {
        rows.push(fail(capability.id, mapping.caseId, {}, "case missing", capability.observations));
      }
      continue;
    }
    const { results } = await runCase(adapter, popoverIface, popoverCases.component, caseData);
    const failures = results.filter((r) => r.verdict === "fail");
    const ok = failures.length === 0;
    for (const capability of mapping.capabilities) {
      rows.push(
        ok
          ? pass(capability.id, mapping.caseId, { casePass: true, assertions: results.length }, capability.observations)
          : fail(capability.id, mapping.caseId, { failures }, "popover case", capability.observations),
      );
    }
  }
  return rows;
}

/** TextInput corpus cases that exercise the input rows on web (g14.006). */
const TEXT_INPUT_PROBE_CASES: Array<{
  caseId: string;
  capabilities: Array<{ id: PrimitiveCapabilityId; observations: string[] }>;
}> = [
  {
    caseId: "text-input/controlled-value",
    capabilities: [{ id: "input.value", observations: ["parts.value", "parts.selection"] }],
  },
  {
    caseId: "text-input/type",
    capabilities: [{ id: "input.editing", observations: ["trace"] }],
  },
  {
    caseId: "text-input/ime-commit",
    capabilities: [{ id: "input.ime", observations: ["trace", "parts.value"] }],
  },
];

async function textInputBackedProbes(adapter: RuntimeAdapter): Promise<ProbeRow[]> {
  const rows: ProbeRow[] = [];
  for (const mapping of TEXT_INPUT_PROBE_CASES) {
    const caseData = textInputCases.cases.find((c) => c.id === mapping.caseId);
    if (!caseData) {
      for (const capability of mapping.capabilities) {
        rows.push(fail(capability.id, mapping.caseId, {}, "case missing", capability.observations));
      }
      continue;
    }
    const { results } = await runCase(adapter, textInputIface, textInputCases.component, caseData);
    const failures = results.filter((r) => r.verdict === "fail");
    const ok = failures.length === 0;
    for (const capability of mapping.capabilities) {
      rows.push(
        ok
          ? pass(capability.id, mapping.caseId, { casePass: true, assertions: results.length }, capability.observations)
          : fail(capability.id, mapping.caseId, { failures }, "text-input case", capability.observations),
      );
    }
  }
  return rows;
}

describe("primitive substrate (web)", () => {
  it("executes owned capabilities on Svelte and React", async () => {
    injectRealCss();
    installLayoutStub();
    // The anchor box the web placement leg resolves against (see the popover
    // conformance test); the overlay probes mount the real popover.
    const anchorStyle = document.createElement("style");
    anchorStyle.textContent = [
      ".poodle-popover { position: absolute !important; top: 40px; left: 24px; width: 96px; height: 32px; }",
      ".poodle-popover__trigger { position: absolute !important; top: 40px; left: 24px; width: 96px; height: 32px; }",
    ].join("\n");
    document.head.appendChild(anchorStyle);

    const pairs: Array<[RuntimeAdapter, RuntimeAdapter]> = [
      [new SvelteButtonAdapter(), new SveltePopoverAdapter()],
      [new ReactButtonAdapter(), new ReactPopoverAdapter()],
    ];
    for (const [buttonAdapter, popoverAdapter] of pairs) {
      const textAdapter =
        buttonAdapter.runtime === "svelte" ? new SvelteTextInputAdapter() : new ReactTextInputAdapter();
      const fixtureRoot = mountSurfaceFixture();
      const probes = [
        ...fixtureProbes(fixtureRoot),
        ...(await buttonBackedProbes(buttonAdapter)),
        ...(await popoverBackedProbes(popoverAdapter)),
        ...(await textInputBackedProbes(textAdapter)),
      ];
      writeEvidence(buttonAdapter.runtime, probes);
      const failed = probes.filter((p) => p.verdict === "fail");
      expect(
        failed,
        `${buttonAdapter.runtime}: ${failed.length} failing probe(s) — ${JSON.stringify(failed, null, 2)}`,
      ).toEqual([]);
    }
  });
});
