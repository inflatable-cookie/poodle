import { render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import { COMPONENT_PROPS, SMOKE_EXCLUDE } from "../../../../test/fixtures/component-props.ts";

// Anatomy smoke across EVERY Svelte component. The module glob means new
// components are covered automatically — coverage cannot silently regress.
// Each component must mount, emit a poodle-* class (proof the Spec/token wiring
// resolved), and log no console.error (guarded in test/vitest.setup.ts).
const modules = import.meta.glob("../src/*.svelte", { eager: true }) as Record<
  string,
  { default: unknown }
>;

const entries = Object.entries(modules)
  .map(([file, mod]) => [file.split("/").pop()!.replace(".svelte", ""), mod.default] as const)
  .filter(([name]) => !(name in SMOKE_EXCLUDE))
  .sort(([a], [b]) => a.localeCompare(b));

describe("svelte component smoke", () => {
  it("covers every component in the package", () => {
    expect(entries.length).toBeGreaterThan(120);
  });

  for (const [name, Comp] of entries) {
    it(`${name} mounts and emits a poodle- class`, () => {
      const { container } = render(Comp as never, { props: COMPONENT_PROPS[name] ?? {} });
      // Overlays (Dialog, Drawer, ToastHost, ...) portal into document.body, so
      // fall back to the document when the render container itself is empty.
      const found =
        container.querySelector('[class*="poodle-"]') ??
        document.body.querySelector('[class*="poodle-"]');
      expect(found, `${name}: no poodle- classed element rendered`).not.toBeNull();
    });
  }
});
