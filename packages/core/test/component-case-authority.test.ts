import { describe, expect, test } from "bun:test";

import { buttonInterface } from "../src/conformance/button";
import { componentCase } from "../src/conformance/define";
import type { GeometryExpectation } from "../src/conformance/define";
import { tabsInterface } from "../src/conformance/tabs";

describe("component case authority", () => {
  test("rejects geometry without an assertion-local tolerance", () => {
    expect(() =>
      componentCase(buttonInterface, {
        id: "button/missing-geometry-tolerance",
        fixture: { props: {}, regions: { label: "Run" } },
        specimen: {
          group: "Validation",
          caption: "Missing tolerance",
          axes: ["theme"],
          captureId: "button/missing-geometry-tolerance",
        },
        steps: [
          {
            kind: "expectPart",
            part: "root",
            expect: {
              geometry: { height: 36 } as unknown as GeometryExpectation,
            },
          },
        ],
      }),
    ).toThrow("geometry tolerance must be a finite non-negative number");
  });

  const tabsCase = (items: unknown[], part = "trigger:overview") => ({
    id: "tabs/planted-invalid-collection",
    fixture: { props: { items, value: "overview" }, regions: { panel: "Panel" } },
    specimen: { group: "Validation", caption: "Invalid collection", axes: [], captureId: "tabs/planted-invalid-collection" },
    steps: [{ kind: "action", name: "press", part, input: "keyboard" }],
  });

  test("rejects duplicate repeated-part keys", () => {
    expect(() => componentCase(tabsInterface, tabsCase([
      { value: "overview", label: "Overview" },
      { value: "overview", label: "Duplicate" },
    ]) as never)).toThrow("duplicate key 'overview'");
  });

  test("rejects repeated parts outside the fixture collection", () => {
    expect(() => componentCase(tabsInterface, tabsCase([
      { value: "overview", label: "Overview" },
    ], "trigger:missing") as never)).toThrow("unknown part 'trigger:missing'");
  });
});
