import { render } from "@testing-library/svelte";
import axe from "axe-core";
import { describe, expect, it } from "vitest";

import { A11Y_BASELINE, A11Y_EXCLUDE, COMPONENT_PROPS } from "../fixtures/component-props";

// Runtime accessibility sweep over EVERY Svelte component (the parity authority).
// Contracts mandate roles, ARIA and keyboard behaviour, but the accessibility
// report only records self-declared audit *status* — nothing executed the rules.
// This runs axe-core against each rendered component.
//
// Page-level rules are disabled: a component renders as an isolated fragment, so
// landmark/document rules ("must be in a region", "needs an <h1>", "needs a
// <title>") are not meaningful here and would be pure noise.
const PAGE_LEVEL_RULES = [
  "region",
  "page-has-heading-one",
  "landmark-one-main",
  "landmark-unique",
  "landmark-complementary-is-top-level",
  "html-has-lang",
  "html-lang-valid",
  "document-title",
  "bypass",
];

const modules = import.meta.glob("../../packages/svelte/components/src/*.svelte", {
  eager: true,
}) as Record<string, { default: unknown }>;

const entries = Object.entries(modules)
  .map(([file, mod]) => [file.split("/").pop()!.replace(".svelte", ""), mod.default] as const)
  .filter(([name]) => !(name in A11Y_EXCLUDE))
  .sort(([a], [b]) => a.localeCompare(b));

describe("component accessibility (axe)", () => {
  it("sweeps the whole component surface", () => {
    expect(entries.length).toBeGreaterThan(120);
  });

  for (const [name, Comp] of entries) {
    it(`${name} has no axe violations`, async () => {
      const { container } = render(Comp as never, { props: COMPONENT_PROPS[name] ?? {} });
      // Overlays portal into document.body, so scan the document.
      const results = await axe.run(document.body, {
        resultTypes: ["violations"],
        rules: Object.fromEntries(PAGE_LEVEL_RULES.map((id) => [id, { enabled: false }])),
      });
      const allowed = new Set(A11Y_BASELINE[name] ?? []);
      const violations = results.violations
        .filter((v) => !allowed.has(v.id))
        .map((v) => `${v.id} (${v.nodes.length}x): ${v.help}`);
      // A11Y_REPORT=1 lists findings instead of failing — used to triage the
      // sweep without fighting the assertion output.
      if (process.env.A11Y_REPORT === "1") {
        if (violations.length > 0) process.stdout.write(`AXE|${name}|${violations.join(" ;; ")}\n`);
        return;
      }
      expect(violations, `${name} axe violations`).toEqual([]);
      void container;
    });
  }
});
