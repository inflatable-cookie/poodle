import { describe, expect, test } from "bun:test";

import { buttonInterface } from "../src/conformance/button";
import { componentCase } from "../src/conformance/define";
import type { GeometryExpectation } from "../src/conformance/define";

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
});
