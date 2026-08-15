import { describe, expect, it } from "vitest";

import { allComponents, webOnlyComponents } from "../src/component-registry";
import {
  componentsBySection,
  isFamilyDisclosed,
  matchesCatalogueSearch,
} from "../src/catalogue-nav";
import manifest from "../../../codegen/fixtures/preview-catalogue.json";

const FIXED_AGENT = [
  "agent-chat-input",
  "agent-transcript",
  "agent-message",
  "agent-question",
  "agent-question-record",
  "agent-plan",
  "agent-plan-record",
  "agent-subagent",
  "tool-call-group",
  "tool-call",
  "changed-files",
];
const FIXED_MODEL = [
  "model-picker",
  "model-catalogue-editor",
  "model-connection-card",
  "model-connection-picker",
  "model-connection-setup",
];
const FIXED_AUDIO = [
  "knob",
  "fader",
  "audio-switch",
  "keyboard",
  "envelope-editor",
  "xy-pad",
  "mod-matrix-grid",
  "audio-meter",
  "gain-reduction-meter",
  "waveform-display",
  "value-readout",
];
const FIXED_ACCOUNT = [
  "licence-status",
  "licence-activation",
  "licence-seats",
  "update-status",
  "update-center",
];
const FIXED_SHELL = [
  "app-header",
  "page-header",
  "status-bar",
  "dock-region",
  "toolbar",
  "action-discovery-panel",
  "message-center",
  "history-center",
  "detail-section",
  "detail-section-group",
  "detail-shell",
  "settings-shell",
];

describe("preview catalogue audit", () => {
  // Web-only entries are catalogued by the web previews but deliberately absent
  // from the canonical manifest, which is the portable inventory and also feeds
  // the GPUI/Jetstream catalogues (spec 068 / g14.024: `MeterSurface` has no
  // native counterpart). They are audited here as their own closed set so the
  // canonical invariant stays exact rather than loosened to "superset".
  const webOnlySlugs = webOnlyComponents.map((component) => component.slug);
  const canonicalEntries = allComponents.filter(
    (component) => !webOnlySlugs.includes(component.slug),
  );

  it("classifies every canonical entry once", () => {
    expect(canonicalEntries).toHaveLength(manifest.components.length);
    const slugs = canonicalEntries.map((component) => component.slug);
    expect(new Set(slugs).size).toBe(slugs.length);
    expect(slugs).toEqual(
      manifest.components.map((component) => component.slug),
    );
  });

  it("keeps the web-only supplement out of the canonical manifest", () => {
    expect(webOnlySlugs).toEqual(["meter-surface"]);
    for (const slug of webOnlySlugs) {
      expect(manifest.components.find((entry) => entry.slug === slug)).toBeUndefined();
      expect(allComponents.find((component) => component.slug === slug)).toBeDefined();
    }
    const slugs = allComponents.map((component) => component.slug);
    expect(new Set(slugs).size).toBe(slugs.length);
  });

  it("keeps generated runtime entries on the same section/family/kind", () => {
    for (const component of canonicalEntries) {
      const canonical = manifest.components.find(
        (entry) => entry.slug === component.slug,
      );
      expect(canonical).toBeDefined();
      expect(component.section).toBe(canonical?.section);
      expect(component.family).toBe(canonical?.family);
      expect(component.kind).toBe(canonical?.kind);
    }
  });

  it("places the motivating suites in one family each", () => {
    expect(
      FIXED_AGENT.every(
        (slug) =>
          allComponents.find((c) => c.slug === slug)?.family === "agent-tools",
      ),
    ).toBe(true);
    expect(
      FIXED_MODEL.every(
        (slug) =>
          allComponents.find((c) => c.slug === slug)?.family ===
          "model-connections",
      ),
    ).toBe(true);
    expect(
      FIXED_AUDIO.every(
        (slug) =>
          allComponents.find((c) => c.slug === slug)?.family === "audio-music",
      ),
    ).toBe(true);
    expect(
      FIXED_ACCOUNT.every(
        (slug) =>
          allComponents.find((c) => c.slug === slug)?.family ===
          "account-lifecycle",
      ),
    ).toBe(true);
    expect(
      FIXED_SHELL.every(
        (slug) =>
          allComponents.find((c) => c.slug === slug)?.family ===
          "application-shell",
      ),
    ).toBe(true);
  });

  it("groups by the declared section/family order", () => {
    const sections = componentsBySection(allComponents);
    expect(sections.map((section) => section.id)).toEqual([
      "foundations",
      "composition",
      "systems",
    ]);
    expect(
      sections.flatMap((section) =>
        section.families.map((family) => family.id),
      ),
    ).toEqual(manifest.families.map((family) => family.id));
  });

  it("discloses only the active family by default", () => {
    const disclosure = new Map<string, boolean>();
    expect(
      isFamilyDisclosed(
        "actions-selection",
        undefined,
        disclosure,
        allComponents,
      ),
    ).toBe(false);
    expect(
      isFamilyDisclosed(
        "actions-selection",
        "button",
        disclosure,
        allComponents,
      ),
    ).toBe(true);
    expect(
      isFamilyDisclosed("agent-tools", "button", disclosure, allComponents),
    ).toBe(false);
    expect(
      isFamilyDisclosed("agent-tools", "agent-plan", disclosure, allComponents),
    ).toBe(true);
    disclosure.set("agent-tools", false);
    expect(
      isFamilyDisclosed("agent-tools", "agent-plan", disclosure, allComponents),
    ).toBe(false);
    disclosure.set("actions-selection", true);
    expect(
      isFamilyDisclosed(
        "actions-selection",
        undefined,
        disclosure,
        allComponents,
      ),
    ).toBe(true);
  });

  it("matches search across name, description, family, and kind", () => {
    const button = allComponents.find(
      (component) => component.slug === "button",
    );
    expect(button).toBeDefined();
    expect(matchesCatalogueSearch(button!, "button")).toBe(true);
    expect(matchesCatalogueSearch(button!, "actions")).toBe(true);
    expect(matchesCatalogueSearch(button!, "control")).toBe(true);
    expect(matchesCatalogueSearch(button!, "xyzyz")).toBe(false);
  });

  it("preserves stable component hrefs", () => {
    for (const component of allComponents) {
      expect(`#components/${component.slug}`).toMatch(
        /^#components\/[a-z0-9-]+$/,
      );
    }
  });
});
