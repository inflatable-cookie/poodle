import { describe, expect, it } from "vitest";

import { allComponents } from "../src/component-registry";
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
  it("classifies every canonical entry once", () => {
    expect(allComponents).toHaveLength(manifest.components.length);
    const slugs = allComponents.map((component) => component.slug);
    expect(new Set(slugs).size).toBe(slugs.length);
    expect(slugs).toEqual(manifest.components.map((component) => component.slug));
  });

  it("keeps generated runtime entries on the same section/family/kind", () => {
    for (const component of allComponents) {
      const canonical = manifest.components.find((entry) => entry.slug === component.slug);
      expect(canonical).toBeDefined();
      expect(component.section).toBe(canonical?.section);
      expect(component.family).toBe(canonical?.family);
      expect(component.kind).toBe(canonical?.kind);
    }
  });

  it("places the motivating suites in one family each", () => {
    expect(FIXED_AGENT.every((slug) => allComponents.find((c) => c.slug === slug)?.family === "agent-tools")).toBe(true);
    expect(FIXED_MODEL.every((slug) => allComponents.find((c) => c.slug === slug)?.family === "model-connections")).toBe(true);
    expect(FIXED_AUDIO.every((slug) => allComponents.find((c) => c.slug === slug)?.family === "audio-music")).toBe(true);
    expect(FIXED_ACCOUNT.every((slug) => allComponents.find((c) => c.slug === slug)?.family === "account-lifecycle")).toBe(true);
    expect(FIXED_SHELL.every((slug) => allComponents.find((c) => c.slug === slug)?.family === "application-shell")).toBe(true);
  });

  it("groups by the declared section/family order", () => {
    const sections = componentsBySection(allComponents);
    expect(sections.map((section) => section.id)).toEqual(["foundations", "composition", "systems"]);
    expect(sections.flatMap((section) => section.families.map((family) => family.id))).toEqual(
      manifest.families.map((family) => family.id),
    );
  });

  it("discloses only the active family by default", () => {
    const expanded = new Set<string>();
    expect(isFamilyDisclosed("actions-selection", undefined, expanded, allComponents)).toBe(false);
    expect(isFamilyDisclosed("actions-selection", "button", expanded, allComponents)).toBe(true);
    expect(isFamilyDisclosed("agent-tools", "button", expanded, allComponents)).toBe(false);
    expect(isFamilyDisclosed("agent-tools", "agent-plan", expanded, allComponents)).toBe(true);
  });

  it("matches search across name, description, family, and kind", () => {
    const button = allComponents.find((component) => component.slug === "button");
    expect(button).toBeDefined();
    expect(matchesCatalogueSearch(button!, "button")).toBe(true);
    expect(matchesCatalogueSearch(button!, "actions")).toBe(true);
    expect(matchesCatalogueSearch(button!, "control")).toBe(true);
    expect(matchesCatalogueSearch(button!, "xyzyz")).toBe(false);
  });

  it("preserves stable component hrefs", () => {
    for (const component of allComponents) {
      expect(`#components/${component.slug}`).toMatch(/^#components\/[a-z0-9-]+$/);
    }
  });
});
