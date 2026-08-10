import type { CSSProperties } from "react";
import type { XYPadVisualState } from "@inflatable-cookie/poodle-core";

export function XYPadVisual({ visualState }: { visualState: XYPadVisualState }) {
  return <span className="poodle-xy-pad-visual" data-hover={visualState.hover} data-focus={visualState.focus} data-drag={visualState.drag} data-enabled={visualState.enabled} aria-hidden="true" style={{ "--poodle-xy-pad-x": visualState.xNorm, "--poodle-xy-pad-y": visualState.yNorm } as CSSProperties}>
    <span className="poodle-xy-pad-visual__grid" />
    <span className="poodle-xy-pad-visual__trace-x" />
    <span className="poodle-xy-pad-visual__trace-y" />
    <span className="poodle-xy-pad-visual__thumb" />
  </span>;
}
