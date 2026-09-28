import { render } from "@testing-library/svelte";
import type { Component } from "svelte";
import { describe, expect, it } from "vitest";

import { COMPONENT_PROPS, SMOKE_EXCLUDE } from "../../../../test/fixtures/component-props.ts";
import {
  selectPublicSvelteEntries,
  sveltePublicComponentSources,
} from "../../../../test/fixtures/svelte-public-components.ts";

// Anatomy smoke across the public Svelte export surface. The glob loads the
// compiled modules; membership comes from the package barrels through the
// shared selection step, so an internal top-level `.svelte` file is not
// treated as public and an export alias still covers its implementation file.
const modules = import.meta.glob("../src/*.svelte", { eager: true }) as Record<
  string,
  { default: Component<Record<string, unknown>> }
>;

const entries = selectPublicSvelteEntries(modules, sveltePublicComponentSources())
  .filter(([name]) => !(name in SMOKE_EXCLUDE))
  .sort(([a], [b]) => a.localeCompare(b));

describe("svelte component smoke", () => {
  it("covers the public export surface, not every top-level .svelte file", () => {
    const publicSources = sveltePublicComponentSources();
    expect(publicSources.has("Button")).toBe(true);
    expect(publicSources.has("MenuSurface")).toBe(false);
    expect(entries.some(([name]) => name === "Button")).toBe(true);
    expect(entries.some(([name]) => name === "MenuSurface")).toBe(false);
    for (const [name] of entries) expect(publicSources.has(name)).toBe(true);
    expect(entries.length).toBeGreaterThan(120);
  });

  for (const [name, Comp] of entries) {
    it(`${name} mounts and emits a poodle- class`, () => {
      const { container } = render(Comp, { props: COMPONENT_PROPS[name] ?? {} });
      // Overlays (Dialog, Drawer, ToastHost, ...) portal into document.body, so
      // fall back to the document when the render container itself is empty.
      const found =
        container.querySelector('[class*="poodle-"]') ??
        document.body.querySelector('[class*="poodle-"]');
      expect(found, `${name}: no poodle- classed element rendered`).not.toBeNull();
    });
  }
});
