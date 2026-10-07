import { describe, expect, test } from "bun:test";

import {
  ICON_GEOMETRY_REALIZE_STATE_NAMES,
  realize,
  type IconGeometryRealizeStateName,
} from "../src/icons/geometry-realize.ts";

const PAIR = "chevron-left-to-chevron-right";

describe("icon geometry SVG realization", () => {
  test("realizes every named state", () => {
    expect(ICON_GEOMETRY_REALIZE_STATE_NAMES).toEqual([
      "endpoint-from",
      "endpoint-to",
      "midpoint",
      "reverse-midpoint",
      "frozen",
      "interruption",
      "teardown",
    ]);

    const states: Record<IconGeometryRealizeStateName, ReturnType<typeof realize>> =
      Object.fromEntries(
        ICON_GEOMETRY_REALIZE_STATE_NAMES.map((name) => [
          name,
          realize({ name, pairId: PAIR }),
        ]),
      ) as Record<IconGeometryRealizeStateName, ReturnType<typeof realize>>;

    expect(states["endpoint-from"].paths.length).toBeGreaterThan(0);
    expect(states["endpoint-to"].paths).not.toEqual(states["endpoint-from"].paths);
    expect(states.midpoint.paths).not.toEqual(states["endpoint-from"].paths);
    expect(states["reverse-midpoint"].paths).not.toEqual(states.midpoint.paths);
    expect(states.frozen.paths).toEqual(states["endpoint-to"].paths);
    // GPUI's existing interruption capture is a midpoint reversal sequence.
    expect(states.interruption.paths).toEqual(states["reverse-midpoint"].paths);
    expect(states.teardown.paths).toEqual([]);
  });

  test("uses the fixed SVG contract for named states and live frames", () => {
    const expectedAttributes = {
      class: "poodle-icon-geometry",
      "data-poodle-icon-geometry": "",
      "data-size": "md",
      xmlns: "http://www.w3.org/2000/svg",
      width: "24",
      height: "24",
      viewBox: "0 0 24 24",
      fill: "none",
      stroke: "currentColor",
      "stroke-width": "2",
      "stroke-linecap": "round",
      "stroke-linejoin": "round",
      role: "presentation",
      "aria-hidden": "true",
    };

    expect(realize({ name: "endpoint-from", pairId: PAIR }).svgAttributes).toEqual(
      expectedAttributes,
    );
    expect(realize({ name: "frame", frame: null })).toEqual({
      svgAttributes: expectedAttributes,
      paths: [],
    });
  });

  test("direction selects the corresponding reverse-midpoint sequence", () => {
    expect(realize({ name: "midpoint", pairId: PAIR, direction: "reverse" }).paths).toEqual(
      realize({ name: "midpoint", pairId: PAIR, direction: "forward" }).paths,
    );
    expect(
      realize({ name: "reverse-midpoint", pairId: PAIR, direction: "reverse" }).paths,
    ).not.toEqual(realize({ name: "reverse-midpoint", pairId: PAIR, direction: "forward" }).paths);
  });
});
