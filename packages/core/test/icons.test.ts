import { describe, expect, test } from "bun:test";

import {
  createIconSet,
  defaultLucideIconSet,
  resolveIconNodes,
  search,
  x,
} from "../src/icons";
import { renderIconSetModule, selectIconSet } from "../src/icons/build.mjs";

describe("icon catalogue boundary", () => {
  test("the default Lucide set contains only Poodle's component dependencies", () => {
    // The count is a deliberate ceiling: the default set is what every consumer
    // ships, so it grows only when a component genuinely needs an icon. Moved
    // to 106 by SegmentedControl's audio-waveform and piano specimen options,
    // then to 108 by g15.053's Breadcrumbs item icons — the canonical `house`
    // plus its `home` alias, which the accessible icon-only root crumb needs.
    // All four are component-owned, not catalogue creep.
    expect(Object.keys(defaultLucideIconSet)).toHaveLength(108);
    expect(defaultLucideIconSet.search).toBe(search);
    expect(defaultLucideIconSet.x).toBe(x);
    expect("biohazard" in defaultLucideIconSet).toBe(false);
  });

  test("createIconSet types selected named imports without copying them", () => {
    const icons = { search, x };
    expect(createIconSet(icons)).toBe(icons);
  });

  test("resolves every legacy alias through an operator icon set", () => {
    const expectedAliases = {
      "alert-circle": "circle-alert",
      "alert-triangle": "triangle-alert",
      "check-square": "square-check",
      "check-circle": "circle-check",
      "circle-help": "circle-question-mark",
      edit: "pencil",
      "file-question": "file-question-mark",
      filter: "list-filter",
      "more-horizontal": "ellipsis",
      "more-vertical": "ellipsis-vertical",
      "help-circle": "circle-question-mark",
      "pause-circle": "circle-pause",
      spinner: "loader-circle",
      unlock: "lock-open",
      "x-circle": "circle-x",
    };

    for (const [legacyName, canonicalName] of Object.entries(expectedAliases)) {
      const nodes = [["path", { d: canonicalName }]] as never;
      expect(resolveIconNodes(legacyName, { [canonicalName]: nodes })).toBe(nodes);
      expect(resolveIconNodes(legacyName)).toBe(
        defaultLucideIconSet[canonicalName],
      );
    }
  });

  test("keeps direct nodes and operator icons ahead of scoped defaults", () => {
    const direct = [["path", { d: "direct" }]] as never;
    const override = [["path", { d: "override" }]] as never;

    expect(resolveIconNodes(direct)).toBe(direct);
    expect(resolveIconNodes("search", { search: override })).toBe(override);
    expect(resolveIconNodes("search")).toBe(search);
    expect(resolveIconNodes(null)).toEqual([]);
  });

  test("the build helper extracts and serializes only requested catalogue names", () => {
    const selected = selectIconSet(
      { search, x, biohazard: [["path", { d: "unused" }]] },
      ["x", "search", "search"],
    );
    expect(Object.keys(selected)).toEqual(["search", "x"]);
    const generated = renderIconSetModule(selected);
    expect(generated).not.toContain("biohazard");
    expect(generated).not.toContain("@inflatable-cookie/poodle-core");
    expect(generated).toContain('export const search = icons["search"];');
  });

  test("the build helper rejects unknown names", () => {
    expect(() => selectIconSet({ search }, ["missing"])).toThrow("missing");
  });
});
