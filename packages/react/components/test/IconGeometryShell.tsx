import { createElement, useEffect, useMemo, useRef } from "react";

import {
  activateIconGeometry,
  createIconGeometryRuntime,
  currentIconGeometryFrame,
  sampleIconGeometry,
  setIconGeometryPolicy,
  teardownIconGeometry,
  type GeometryEndpoint,
  type IconGeometryRuntime,
} from "../../../core/src/icons/geometry-runtime";
import type { MotionPolicy } from "@inflatable-cookie/poodle-core";

export interface IconGeometryShellProps {
  owner?: string;
  pairId: string;
  target?: GeometryEndpoint;
  policy?: MotionPolicy;
  progress?: number | null;
  initial?: boolean;
}

function contourPath(
  closed: boolean,
  points: readonly (readonly [number, number])[],
): string {
  if (points.length === 0) return "";
  const commands = points.map(([x, y], index) => `${index === 0 ? "M" : "L"}${x / 10_000} ${y / 10_000}`);
  if (closed) commands.push("Z");
  return commands.join(" ");
}

export function IconGeometryShell({
  owner = "icon-geometry-shell",
  pairId,
  target = "from",
  policy = "full",
  progress = null,
  initial = false,
}: IconGeometryShellProps) {
  const runtimeRef = useRef<IconGeometryRuntime | null>(null);
  if (runtimeRef.current === null) {
    runtimeRef.current = createIconGeometryRuntime(policy);
  }
  const runtime = runtimeRef.current;

  const frame = useMemo(() => {
    setIconGeometryPolicy(runtime, policy);
    const decision = activateIconGeometry(runtime, { owner, pairId, target, initial });
    if (progress !== null) {
      sampleIconGeometry(runtime, decision.key, progress);
    }
    return currentIconGeometryFrame(runtime);
  }, [runtime, owner, pairId, target, policy, progress, initial]);

  useEffect(() => {
    return () => {
      teardownIconGeometry(runtime);
    };
  }, [runtime]);

  return createElement(
    "svg",
    {
      className: "poodle-icon-geometry",
      "data-poodle-icon-geometry": "",
      "data-size": "md",
      xmlns: "http://www.w3.org/2000/svg",
      width: 24,
      height: 24,
      viewBox: "0 0 24 24",
      fill: "none",
      stroke: "currentColor",
      strokeWidth: 2,
      strokeLinecap: "round",
      strokeLinejoin: "round",
      role: "presentation",
      "aria-hidden": true,
    },
    (frame?.contours ?? []).map((contour, index) =>
      createElement("path", { key: index, d: contourPath(contour.closed, contour.points) }),
    ),
  );
}
