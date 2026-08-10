import type { CSSProperties } from "react";
import type { KeyboardVisualState } from "@inflatable-cookie/poodle-core";

export function KeyboardVisual({ visualState }: { visualState: KeyboardVisualState }) {
  return <span className="poodle-keyboard-visual" aria-hidden="true" data-enabled={visualState.enabled}>
    {visualState.keys.map((key) => <span key={key.note} className="poodle-keyboard-visual__key" data-kind={key.kind} data-held={key.held} data-external={key.externallyHeld} data-focused={key.focused} style={(visualState.orientation === "horizontal" ? { left: `${key.startNorm * 100}%`, width: `${key.lengthNorm * 100}%`, height: `${key.breadthNorm * 100}%` } : { top: `${key.startNorm * 100}%`, height: `${key.lengthNorm * 100}%`, width: `${key.breadthNorm * 100}%` }) as CSSProperties} />)}
  </span>;
}
