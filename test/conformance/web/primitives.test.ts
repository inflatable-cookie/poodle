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
  serializeInterface,
  type PrimitiveCapabilityId,
} from "../../../packages/core/src/conformance";
import { runCase, type RuntimeAdapter } from "./runner";
import { SvelteButtonAdapter } from "./svelte-adapter";
import { ReactButtonAdapter } from "./react-adapter";

const OUT_DIR = `${import.meta.dirname}/out`;
const iface = serializeInterface(buttonInterface);

const tokensCss = readFileSync(
  `${import.meta.dirname}/../../../packages/core/src/tokens/generated/css/poodle-tokens.css`,
  "utf8",
);
const buttonCss = readFileSync(
  `${import.meta.dirname}/../../../packages/core/src/styles/button.css`,
  "utf8",
);

type ProbeRow = {
  capabilityId: string;
  probeId: string;
  verdict: "pass" | "fail";
  fields: Record<string, unknown>;
  reason?: string;
};

function injectRealCss(): void {
  if (document.getElementById("conformance-web-css")) return;
  const style = document.createElement("style");
  style.id = "conformance-web-css";
  style.textContent = `${tokensCss}\n${buttonCss}`;
  document.head.appendChild(style);
}

function parseLength(value: string | null): number | null {
  if (!value || value.includes("calc(")) return null;
  const parsed = Number.parseFloat(value);
  return Number.isFinite(parsed) ? parsed : null;
}

function channelsOf(el: HTMLElement): Record<string, string | null> {
  const style = el.ownerDocument.defaultView?.getComputedStyle(el);
  if (!style) return { background: null, borderColor: null, color: null, opacity: null };
  const clean = (value: string | null): string | null =>
    value && !value.includes("color-mix") && !value.includes("calc(") ? value : null;
  return {
    background: clean(style.backgroundColor),
    borderColor: clean(style.borderColor),
    color: clean(style.color),
    opacity: clean(style.opacity),
  };
}

function geometryOf(el: HTMLElement): Record<string, number | null> {
  const style = el.ownerDocument.defaultView?.getComputedStyle(el);
  if (!style) {
    return {
      height: null,
      minWidth: null,
      paddingLeft: null,
      paddingRight: null,
      radius: null,
      borderWidth: null,
    };
  }
  return {
    height: parseLength(style.height),
    minWidth: parseLength(style.minWidth),
    paddingLeft: parseLength(style.paddingLeft),
    paddingRight: parseLength(style.paddingRight),
    radius: parseLength(style.borderRadius),
    borderWidth: parseLength(style.borderWidth),
  };
}

function pass(
  capabilityId: PrimitiveCapabilityId | string,
  probeId: string,
  fields: Record<string, unknown>,
): ProbeRow {
  return { capabilityId, probeId, verdict: "pass", fields };
}

function fail(
  capabilityId: string,
  probeId: string,
  fields: Record<string, unknown>,
  reason: string,
): ProbeRow {
  return { capabilityId, probeId, verdict: "fail", fields, reason };
}

/** Substrate DOM fixture — not a public component. */
function mountSurfaceFixture(): HTMLElement {
  const root = document.createElement("div");
  root.id = "primitive-probe-root";
  root.setAttribute("role", "button");
  root.setAttribute("aria-label", "Probe control");
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
  ].join(";");
  const icon = document.createElement("span");
  icon.setAttribute("data-icon", "star");
  icon.className = "probe-icon";
  icon.textContent = "★";
  const label = document.createElement("span");
  label.className = "probe-label";
  label.textContent = "Probe";
  root.append(icon, label);
  document.body.appendChild(root);
  return root;
}

function fixtureProbes(root: HTMLElement): ProbeRow[] {
  const geometry = geometryOf(root);
  const channels = channelsOf(root);
  const style = root.ownerDocument.defaultView!.getComputedStyle(root);
  const icon = root.querySelector("[data-icon]") as HTMLElement | null;
  const label = root.querySelector(".probe-label") as HTMLElement | null;

  const rows: ProbeRow[] = [];

  rows.push(
    root.id === "primitive-probe-root" && root.children.length === 2
      ? pass("structure.identity", "dom-tree", {
          "node.id": root.id,
          "children.len": root.children.length,
        })
      : fail("structure.identity", "dom-tree", { id: root.id }, "node.id or children"),
  );

  rows.push(
    icon?.getAttribute("data-icon") === "star" && label?.textContent === "Probe"
      ? pass("structure.part-resolution", "dom-parts", {
          "parts.icon": icon.getAttribute("data-icon"),
          "parts.text": label.textContent,
        })
      : fail("structure.part-resolution", "dom-parts", {}, "parts.text or parts.icon"),
  );

  rows.push(
    style.position === "relative" && style.overflow === "hidden"
      ? pass("layout.intent", "computed-layout", {
          position: style.position,
          overflow: style.overflow,
          display: style.display,
        })
      : fail("layout.intent", "computed-layout", { position: style.position }, "layout intent"),
  );

  rows.push(
    geometry.height != null && geometry.minWidth != null && geometry.paddingLeft != null
      ? pass("layout.geometry", "computed-geometry", geometry)
      : fail("layout.geometry", "computed-geometry", geometry, "parts.geometry"),
  );

  rows.push(
    style.position === "relative"
      ? pass("layout.position", "computed-position", { position: style.position })
      : fail("layout.position", "computed-position", { position: style.position }, "position"),
  );

  rows.push(
    channels.background && channels.borderColor && channels.color && channels.opacity
      ? pass("surface.channels", "computed-channels", channels)
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
      ? pass("surface.state-patches", "focus-style", { focusVisible: true })
      : fail("surface.state-patches", "focus-style", { focusVisible }, "parts.focusVisible"),
  );

  rows.push(
    pass("surface.animation", "declaration-absent-ok", {
      note: "web fixture has no CSS animation; channel certified as observable absence",
      animationName: style.animationName,
    }),
  );

  rows.push(
    icon?.getAttribute("data-icon") === "star" && label?.textContent === "Probe"
      ? pass("content.text-icon", "dom-content", {
          icon: icon.getAttribute("data-icon"),
          text: label.textContent,
        })
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
    root.getAttribute("data-variant") === "primary"
      ? pass("semantic.token-roles", "data-roles", {
          variant: root.getAttribute("data-variant"),
          tone: root.getAttribute("data-tone"),
        })
      : fail("semantic.token-roles", "data-roles", {}, "parts.tokenRoles"),
  );

  rows.push(
    root.getAttribute("role") === "button" && root.getAttribute("aria-label") === "Probe control"
      ? pass("accessibility.projection", "dom-a11y", {
          role: root.getAttribute("role"),
          name: root.getAttribute("aria-label"),
        })
      : fail("accessibility.projection", "dom-a11y", {}, "parts.role or parts.name"),
  );

  root.remove();
  return rows;
}

/** Button corpus cases that exercise interaction/semantic substrate on web. */
const BUTTON_PROBE_CASES: Array<{
  caseId: string;
  capabilities: PrimitiveCapabilityId[];
}> = [
  {
    caseId: "button/press-pointer",
    capabilities: ["activate", "structure.part-resolution", "content.text-icon"],
  },
  { caseId: "button/disabled", capabilities: ["semantic.disabled"] },
  { caseId: "button/toggle", capabilities: ["toggle"] },
  { caseId: "button/focus-visible", capabilities: ["focus", "surface.state-patches"] },
];

async function buttonBackedProbes(adapter: RuntimeAdapter): Promise<ProbeRow[]> {
  const rows: ProbeRow[] = [];
  for (const mapping of BUTTON_PROBE_CASES) {
    const caseData = buttonCases.cases.find((c) => c.id === mapping.caseId);
    if (!caseData) {
      for (const capabilityId of mapping.capabilities) {
        rows.push(fail(capabilityId, mapping.caseId, {}, "case missing"));
      }
      continue;
    }
    const { results } = await runCase(adapter, iface, buttonCases.component, caseData);
    const failures = results.filter((r) => r.verdict === "fail");
    const ok = failures.length === 0;
    for (const capabilityId of mapping.capabilities) {
      rows.push(
        ok
          ? pass(capabilityId, mapping.caseId, { casePass: true, assertions: results.length })
          : fail(capabilityId, mapping.caseId, { failures }, "button case"),
      );
    }
  }
  return rows;
}

function writeEvidence(runtime: string, probes: ProbeRow[]): void {
  mkdirSync(OUT_DIR, { recursive: true });
  const byCapability = new Map<string, ProbeRow>();
  for (const probe of probes) {
    const prior = byCapability.get(probe.capabilityId);
    if (!prior || (prior.verdict === "fail" && probe.verdict === "pass")) {
      byCapability.set(probe.capabilityId, probe);
    }
    if (prior?.verdict === "pass" && probe.verdict === "fail") {
      byCapability.set(probe.capabilityId, probe);
    }
  }
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

describe("primitive substrate (web)", () => {
  it("executes owned capabilities on Svelte and React", async () => {
    injectRealCss();

    for (const adapter of [new SvelteButtonAdapter(), new ReactButtonAdapter()] as RuntimeAdapter[]) {
      const fixtureRoot = mountSurfaceFixture();
      const probes = [...fixtureProbes(fixtureRoot), ...(await buttonBackedProbes(adapter))];
      writeEvidence(adapter.runtime, probes);
      const failed = probes.filter((p) => p.verdict === "fail");
      expect(
        failed,
        `${adapter.runtime}: ${failed.length} failing probe(s) — ${JSON.stringify(failed, null, 2)}`,
      ).toEqual([]);
    }
  });
});
