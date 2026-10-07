/** Test-support realization of the private icon-geometry SVG contract. */

import type { GeometryFrameBuffer } from "./geometry";
import {
  activateIconGeometry,
  createIconGeometryRuntime,
  currentIconGeometryFrame,
  sampleIconGeometry,
  setIconGeometryPolicy,
  teardownIconGeometry,
  type GeometryEndpoint,
} from "./geometry-runtime";

export const ICON_GEOMETRY_REALIZE_STATE_NAMES = [
  "endpoint-from",
  "endpoint-to",
  "midpoint",
  "reverse-midpoint",
  "frozen",
  "interruption",
  "teardown",
] as const;

export type IconGeometryRealizeStateName = (typeof ICON_GEOMETRY_REALIZE_STATE_NAMES)[number];
export type IconGeometryRealizeDirection = "forward" | "reverse";

type NamedState = {
  name: IconGeometryRealizeStateName;
  pairId: string;
  direction?: IconGeometryRealizeDirection;
};

type FrameState = {
  name: "frame";
  frame: GeometryFrameBuffer | null;
};

export type IconGeometryRealizeState = NamedState | FrameState;

export type IconGeometryRealization = {
  svgAttributes: Readonly<Record<string, string>>;
  paths: { d: string }[];
};

const SVG_ATTRIBUTES = Object.freeze({
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
});

const OWNER = "icon-geometry-realize";

/**
 * Resolve a named fixture state or serialize a live runtime frame to the SVG
 * attributes painted by the private Svelte and React shells.
 */
export function realize(state: IconGeometryRealizeState): IconGeometryRealization {
  return {
    svgAttributes: SVG_ATTRIBUTES,
    paths: pathsForFrame(state.name === "frame" ? state.frame : frameForNamedState(state)),
  };
}

function frameForNamedState(state: NamedState): GeometryFrameBuffer | null {
  const runtime = createIconGeometryRuntime("full");
  const direction = state.direction ?? "forward";
  const target: GeometryEndpoint = direction === "forward" ? "to" : "from";
  const initial: GeometryEndpoint = target === "to" ? "from" : "to";
  const intent = (endpoint: GeometryEndpoint, initialState: boolean) => ({
    owner: OWNER,
    pairId: state.pairId,
    target: endpoint,
    initial: initialState,
  });

  switch (state.name) {
    case "endpoint-from":
      activateIconGeometry(runtime, intent("from", true));
      break;
    case "endpoint-to":
      activateIconGeometry(runtime, intent("to", true));
      break;
    case "midpoint": {
      activateIconGeometry(runtime, intent(initial, true));
      const first = activateIconGeometry(runtime, intent(target, false));
      sampleIconGeometry(runtime, first.key, 0.5);
      break;
    }
    case "reverse-midpoint":
    case "interruption": {
      activateIconGeometry(runtime, intent(initial, true));
      const first = activateIconGeometry(runtime, intent(target, false));
      sampleIconGeometry(runtime, first.key, 0.5);
      const reverse = activateIconGeometry(runtime, intent(initial, false));
      sampleIconGeometry(runtime, reverse.key, 0.5);
      break;
    }
    case "frozen": {
      activateIconGeometry(runtime, intent(initial, true));
      activateIconGeometry(runtime, intent(target, false));
      setIconGeometryPolicy(runtime, "frozen");
      break;
    }
    case "teardown": {
      activateIconGeometry(runtime, intent(initial, true));
      activateIconGeometry(runtime, intent(target, false));
      teardownIconGeometry(runtime);
      break;
    }
  }

  return currentIconGeometryFrame(runtime);
}

function pathsForFrame(frame: GeometryFrameBuffer | null): { d: string }[] {
  if (!frame) return [];
  return frame.contours.map((contour) => ({
    d: contourPath(contour.closed, contour.points, contour.count),
  }));
}

function contourPath(
  closed: boolean,
  points: readonly (readonly [number, number])[],
  count: number,
): string {
  if (count === 0) return "";
  const commands: string[] = [];
  for (let index = 0; index < count; index += 1) {
    const point = points[index]!;
    commands.push(`${index === 0 ? "M" : "L"}${point[0] / 10_000} ${point[1] / 10_000}`);
  }
  if (closed) commands.push("Z");
  return commands.join(" ");
}
