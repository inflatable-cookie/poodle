import { render } from "@testing-library/react";
import { createElement } from "react";
import { describe, expect, it } from "vitest";

import { COMPONENT_PROPS, SMOKE_EXCLUDE, SMOKE_EXCLUDE_REACT } from "../../../../test/fixtures/component-props";

// Anatomy smoke across EVERY React component. The module glob means new
// components are covered automatically — coverage cannot silently regress.
// Mirrors the Svelte sweep so both implementations are held to the same floor.
const modules = import.meta.glob("../src/*.tsx", { eager: true }) as Record<
  string,
  Record<string, unknown>
>;

const entries = Object.entries(modules)
  .map(([file, mod]) => {
    const name = file.split("/").pop()!.replace(".tsx", "");
    return [name, mod[name]] as const;
  })
  // Only PascalCase modules that actually export a component of the same name
  // (skips helper modules like portal.tsx / presentation.tsx).
  .filter(([name, comp]) => /^[A-Z]/.test(name) && typeof comp === "function")
  .filter(([name]) => !(name in SMOKE_EXCLUDE) && !(name in SMOKE_EXCLUDE_REACT))
  .sort(([a], [b]) => a.localeCompare(b));

describe("react component smoke", () => {
  it("covers every component in the package", () => {
    expect(entries.length).toBeGreaterThan(120);
  });

  for (const [name, Comp] of entries) {
    it(`${name} mounts and emits a poodle- class`, () => {
      const { container } = render(
        createElement(Comp as never, COMPONENT_PROPS[name] ?? {}),
      );
      // Overlays portal into document.body, so fall back to the document when
      // the render container itself is empty.
      const found =
        container.querySelector('[class*="poodle-"]') ??
        document.body.querySelector('[class*="poodle-"]');
      expect(found, `${name}: no poodle- classed element rendered`).not.toBeNull();
    });
  }
});
