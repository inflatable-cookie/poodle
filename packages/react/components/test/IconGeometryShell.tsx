import { createElement, useEffect, useRef, useState, type SVGProps } from "react";

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
import { realize } from "../../../core/src/icons/geometry-realize";
import type { MotionPolicy } from "@inflatable-cookie/poodle-core";

export interface IconGeometryShellProps {
  owner?: string;
  pairId: string;
  target?: GeometryEndpoint;
  policy?: MotionPolicy;
  progress?: number | null;
  initial?: boolean;
}

type PathSnapshot = { d: string };

function snapshotFrame(runtime: IconGeometryRuntime): PathSnapshot[] {
  return realize({
    name: "frame",
    frame: currentIconGeometryFrame(runtime),
  }).paths;
}

const REACT_ATTRIBUTE_NAMES: Record<string, string> = {
  class: "className",
  "stroke-width": "strokeWidth",
  "stroke-linecap": "strokeLinecap",
  "stroke-linejoin": "strokeLinejoin",
};

function reactSvgProps(attributes: Readonly<Record<string, string>>): SVGProps<SVGSVGElement> {
  return Object.fromEntries(
    Object.entries(attributes).map(([name, value]) => [REACT_ATTRIBUTE_NAMES[name] ?? name, value]),
  ) as SVGProps<SVGSVGElement>;
}

const SVG_PROPS = reactSvgProps(realize({ name: "frame", frame: null }).svgAttributes);

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
    SVG_PROPS,
    paths.map((contour, index) => createElement("path", { key: index, d: contour.d })),
  );
}
