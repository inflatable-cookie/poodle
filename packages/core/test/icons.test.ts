import { describe, expect, test } from "bun:test";

import {
  createIconSet,
  defaultLucideIconSet,
  search,
  x,
} from "../src/icons";
import { renderIconSetModule, selectIconSet } from "../src/icons/build.mjs";

describe("icon catalogue boundary", () => {
  test("the default Lucide set contains only Poodle's component dependencies", () => {
    expect(Object.keys(defaultLucideIconSet)).toHaveLength(54);
    expect(defaultLucideIconSet.search).toBe(search);
    expect(defaultLucideIconSet.x).toBe(x);
    expect("biohazard" in defaultLucideIconSet).toBe(false);
  });

  test("createIconSet types selected named imports without copying them", () => {
    const icons = { search, x };
    expect(createIconSet(icons)).toBe(icons);
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
