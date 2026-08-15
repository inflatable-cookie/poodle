import { popoverInterface } from "../src/conformance/popover.ts";
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

  test("rejects non-rem surface width bounds (the portable dimension subset)", () => {
    expect(() =>
      componentCase(popoverInterface, {
        id: "popover/planted-invalid-dimension",
        fixture: {
          props: { surfaceMinWidth: "320px", ariaLabel: "Quick settings" },
          regions: { trigger: "Open popover", children: "Panel" },
        },
        specimen: { group: "Validation", caption: "Invalid dimension", axes: [], captureId: "popover/planted-invalid-dimension" },
        steps: [{ kind: "expectPart", part: "root", expect: {} }],
      }),
    ).toThrow("not a portable rem length");
    expect(() =>
      componentCase(popoverInterface, {
        id: "popover/planted-invalid-dimension-2",
        fixture: {
          props: { surfaceMinWidth: "min(24rem, 90vw)" },
          regions: { trigger: "Open popover", children: "Panel" },
        },
        specimen: { group: "Validation", caption: "Invalid dimension", axes: [], captureId: "popover/planted-invalid-dimension-2" },
        steps: [{ kind: "expectPart", part: "root", expect: {} }],
      }),
    ).toThrow("not a portable rem length");
    expect(() =>
      componentCase(popoverInterface, {
        id: "popover/planted-overflow-dimension",
        fixture: {
          props: { surfaceMinWidth: `${"9".repeat(100)}rem` },
          regions: { trigger: "Open popover", children: "Panel" },
        },
        specimen: { group: "Validation", caption: "Overflow dimension", axes: [], captureId: "popover/planted-overflow-dimension" },
        steps: [{ kind: "expectPart", part: "root", expect: {} }],
      }),
    ).toThrow("not a portable rem length");
  });

  test("accepts rem surface width bounds", () => {
    expect(() =>
      componentCase(popoverInterface, {
        id: "popover/planted-valid-dimension",
        fixture: {
          props: { surfaceMinWidth: "20rem", surfaceMaxWidth: "20.5rem" },
          regions: { trigger: "Open popover", children: "Panel" },
        },
        specimen: { group: "Validation", caption: "Valid dimension", axes: [], captureId: "popover/planted-valid-dimension" },
        steps: [{ kind: "expectPart", part: "root", expect: {} }],
      }),
    ).not.toThrow();
  });

  test("keeps unconstrained dimensions available to other component contracts", () => {
    expect(() =>
      componentCase(buttonInterface, {
        id: "button/css-dimension",
        fixture: { props: { maxWidth: "min(20rem, 90vw)" }, regions: { label: "Run" } },
        specimen: { group: "Validation", caption: "CSS dimension", axes: [], captureId: "button/css-dimension" },
        steps: [{ kind: "expectPart", part: "root", expect: {} }],
      }),
    ).not.toThrow();
  });
});
