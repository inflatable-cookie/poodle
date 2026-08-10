import type { AudioControlVisualState } from "@inflatable-cookie/poodle-core";

export function ValueVisual({ visualState, text, kind }: { visualState: AudioControlVisualState; text: string; kind: "readout" | "drag-number" }) {
  return <span className={kind === "readout" ? "poodle-value-readout-visual" : "poodle-drag-number-field-visual"} data-hover={visualState.hover} data-focus={visualState.focus} data-drag={visualState.drag} data-enabled={visualState.enabled} aria-hidden="true">{text}</span>;
}
