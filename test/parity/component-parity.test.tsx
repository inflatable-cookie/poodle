import { cleanup as cleanupReact, render as renderReact } from "@testing-library/react";
import { cleanup as cleanupSvelte, render as renderSvelte } from "@testing-library/svelte";
import { createElement } from "react";
import { describe, expect, it } from "vitest";

import { COMPONENT_PROPS, PARITY_EXCLUDE } from "../fixtures/component-props";

// Svelte <-> React anatomy parity across EVERY component present in both
// packages. Each side renders with the SAME props and no children (symmetric by
// construction), then the emitted poodle-* class sets are diffed.
//
// The module globs mean new components are gated automatically.

const svelteModules = import.meta.glob("../../packages/svelte/components/src/*.svelte", {
  eager: true,
}) as Record<string, { default: unknown }>;

const reactModules = import.meta.glob("../../packages/react/components/src/*.tsx", {
  eager: true,
}) as Record<string, Record<string, unknown>>;

function basename(file: string, ext: string): string {
  return file.split("/").pop()!.replace(ext, "");
}

const svelteByName = new Map<string, unknown>(
  Object.entries(svelteModules).map(([f, m]) => [basename(f, ".svelte"), m.default]),
);
const reactByName = new Map<string, unknown>(
  Object.entries(reactModules)
    .map(([f, m]) => {
      const name = basename(f, ".tsx");
      return [name, m[name]] as const;
    })
    .filter(([name, comp]) => /^[A-Z]/.test(name) && typeof comp === "function"),
);

// Only components implemented in BOTH packages are parity-gated.
const shared = [...svelteByName.keys()]
  .filter((name) => reactByName.has(name))
  .filter((name) => !(name in PARITY_EXCLUDE))
  .sort();

// Classes that differ by framework idiom, not component anatomy: Svelte context
// providers render a wrapper element; React context emits no DOM node.
const IGNORE = new Set(["poodle-ui-presentation-provider"]);

// Genuine anatomy divergences, held as an explicit baseline so the gate stays
// green while the debt stays visible. Closing one means deleting its entry.
// Currently EMPTY — every shared component matches. Keep it that way: a new
// entry is debt, not a fix.
const KNOWN_DIVERGENCE: Record<string, string[]> = {};

function anatomy(root: ParentNode): string[] {
  const set = new Set<string>();
  for (const el of root.querySelectorAll("*")) {
    for (const c of el.classList) {
      if (c.startsWith("poodle-") && !IGNORE.has(c)) set.add(c);
    }
  }
  return [...set].sort();
}

describe("svelte <-> react anatomy parity", () => {
  it("gates a substantial shared component surface", () => {
    expect(shared.length).toBeGreaterThan(100);
  });

  for (const name of shared) {
    it(`${name} emits matching poodle- anatomy classes`, () => {
      const props = COMPONENT_PROPS[name] ?? {};

      const svContainer = renderSvelte(svelteByName.get(name) as never, { props }).container;
      const svClasses = anatomy(svContainer.parentNode ?? svContainer);
      cleanupSvelte();

      const reContainer = renderReact(createElement(reactByName.get(name) as never, props)).container;
      const reClasses = anatomy(reContainer.parentNode ?? reContainer);
      cleanupReact();

      const allowed = new Set(KNOWN_DIVERGENCE[name] ?? []);
      const svelteOnly = svClasses.filter((x) => !reClasses.includes(x) && !allowed.has(x));
      const reactOnly = reClasses.filter((x) => !svClasses.includes(x) && !allowed.has(x));
      expect({ svelteOnly, reactOnly }).toEqual({ svelteOnly: [], reactOnly: [] });
    });
  }
});
