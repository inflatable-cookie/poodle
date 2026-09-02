import { createElement, useEffect, useRef, useState } from "react";

import {
  activateIconGeometry,
  createIconGeometryRuntime,
  currentIconGeometryFrame,
  sampleIconGeometry,
  setIconGeometryPolicy,
  startIconGeometryFrameLoop,
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

type PathSnapshot = { closed: boolean; d: string };

function snapshotFrame(runtime: IconGeometryRuntime): PathSnapshot[] {
  const current = currentIconGeometryFrame(runtime);
  if (!current) return [];
  return current.contours.map((contour) => ({
    closed: contour.closed,
    d: contourPath(contour.closed, contour.points, contour.count),
  }));
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
  const [paths, setPaths] = useState<PathSnapshot[]>(() => {
    setIconGeometryPolicy(runtime, policy);
    const decision = activateIconGeometry(runtime, { owner, pairId, target, initial });
    if (progress !== null) {
      sampleIconGeometry(runtime, decision.key, progress);
    }
    return snapshotFrame(runtime);
  });

  useEffect(() => {
    setIconGeometryPolicy(runtime, policy);
    const decision = activateIconGeometry(runtime, { owner, pairId, target, initial });
    if (progress !== null) {
      sampleIconGeometry(runtime, decision.key, progress);
    }
    setPaths(snapshotFrame(runtime));
    if (progress !== null || !decision.liveClock || typeof requestAnimationFrame !== "function") {
      return undefined;
    }
    return startIconGeometryFrameLoop(runtime, decision.key, () => {
      setPaths(snapshotFrame(runtime));
    });
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
    paths.map((contour, index) => createElement("path", { key: index, d: contour.d })),
  );
}
