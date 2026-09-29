import { describe, expect, it } from "vitest";

import {
  WEB_ONLY,
  WEB_ONLY_GLOBAL,
  isWebOnly,
} from "../scripts/contract-spec-drift.ts";

describe("contract-spec-drift WEB_ONLY register", () => {
  it("keeps every exemption keyed with a reason, global or slug-scoped", () => {
    const scopes = Object.keys(WEB_ONLY);
    expect(scopes).toContain(WEB_ONLY_GLOBAL);
    expect(scopes.length).toBeGreaterThan(1);

    for (const [scope, props] of Object.entries(WEB_ONLY)) {
      expect(scope === WEB_ONLY_GLOBAL || /^[a-z0-9-]+$/.test(scope)).toBe(true);
      expect(Object.keys(props).length).toBeGreaterThan(0);
      for (const [prop, reason] of Object.entries(props)) {
        expect(prop.length).toBeGreaterThan(0);
        expect(reason.trim().length).toBeGreaterThan(0);
      }
    }
  });

  it("applies a global exemption to every component", () => {
    expect(isWebOnly("button", "className")).toBe(true);
    expect(isWebOnly("text-input", "className")).toBe(true);
    expect(isWebOnly("model-connection-picker", "className")).toBe(true);
  });

  it("does not leak a slug-scoped exemption to other components", () => {
    expect(isWebOnly("model-connection-picker", "defaultValue")).toBe(true);
    expect(isWebOnly("model-connection-setup", "defaultValue")).toBe(true);
    expect(isWebOnly("text-input", "defaultValue")).toBe(false);
    expect(isWebOnly("select", "defaultValue")).toBe(false);
    expect(isWebOnly("button", "defaultValue")).toBe(false);
  });
});
