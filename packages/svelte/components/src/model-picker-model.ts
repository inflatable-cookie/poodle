// ModelPicker pure model. Renderer-neutral logic: per-model axis resolution,
// defaults, selection normalisation and trigger-summary text. This is the
// canonical TypeScript source; the React shell mirrors it and the Rust spec
// (`poodle-specs::model_picker`) re-implements the same semantics.
//
// No vendor vocabulary lives here: "reasoning", "fast mode" and "context
// window" are host-declared axes, not concepts this file knows about.

import type {
  ModelAxisBinding,
  ModelAxisValue,
  ModelCapabilityAxis,
  ModelOption,
  ModelSelection,
} from "./types";

/** Merge a binding over the shared axis definition. Only the keys the binding
 * sets are overridden, so a provider can swap the level set while inheriting
 * the label, kind and summary behaviour. */
function applyBinding(
  axis: ModelCapabilityAxis,
  binding: ModelAxisBinding,
): ModelCapabilityAxis {
  return {
    ...axis,
    label: binding.label ?? axis.label,
    description: binding.description ?? axis.description,
    options: binding.options ?? axis.options,
    control: binding.control ?? axis.control,
    defaultValue: binding.defaultValue ?? axis.defaultValue,
    onLabel: binding.onLabel ?? axis.onLabel,
    offLabel: binding.offLabel ?? axis.offLabel,
    showInSummary: binding.showInSummary ?? axis.showInSummary,
    disabled: binding.disabled ?? axis.disabled,
  };
}

/** The axes a given model exposes, already merged with its bindings, in the
 * order the model declares them. A model with no `axes` declaration inherits
 * every axis in declaration order — the single-provider case. References to
 * undeclared keys are dropped. */
export function axesForModel(
  axes: ModelCapabilityAxis[],
  model: ModelOption | undefined,
): ModelCapabilityAxis[] {
  if (!model?.axes) return axes;
  const byKey = new Map(axes.map((axis) => [axis.key, axis]));
  const resolved: ModelCapabilityAxis[] = [];
  for (const ref of model.axes) {
    const key = typeof ref === "string" ? ref : ref.key;
    const base = byKey.get(key);
    if (!base) continue;
    resolved.push(typeof ref === "string" ? base : applyBinding(base, ref));
  }
  return resolved;
}

/** Axes that apply to the currently selected model. */
export function applicableAxes(
  models: ModelOption[],
  axes: ModelCapabilityAxis[],
  model: string,
): ModelCapabilityAxis[] {
  if (!model) return [];
  return axesForModel(
    axes,
    models.find((option) => option.value === model),
  );
}

/** Whether a value is representable on this axis. */
export function axisAccepts(axis: ModelCapabilityAxis, value: ModelAxisValue): boolean {
  if (axis.kind === "toggle") return typeof value === "boolean";
  return typeof value === "string" && (axis.options ?? []).some((option) => option.value === value);
}

/** Declared default, else the first enabled option (`select`) or `false`. */
export function axisDefaultValue(axis: ModelCapabilityAxis): ModelAxisValue {
  if (axis.defaultValue !== undefined && axisAccepts(axis, axis.defaultValue)) {
    return axis.defaultValue;
  }
  if (axis.kind === "toggle") return false;
  return (axis.options ?? []).find((option) => !option.disabled)?.value ?? "";
}

/** The effective value for an axis: the held value when representable on *this
 * model's* version of the axis, else the resolved default. A level that only
 * exists on another provider therefore falls back rather than sticking. */
export function axisValue(axis: ModelCapabilityAxis, selection: ModelSelection): ModelAxisValue {
  const held = selection.axes?.[axis.key];
  if (held !== undefined && axisAccepts(axis, held)) return held;
  return axisDefaultValue(axis);
}

/** Drop axes the selected model does not expose, fill the rest with defaults.
 * Every emitted selection passes through here, so another provider's axis value
 * never leaks to the host. */
export function resolveSelection(
  models: ModelOption[],
  axes: ModelCapabilityAxis[],
  selection: ModelSelection,
): ModelSelection {
  const resolved: Record<string, ModelAxisValue> = {};
  for (const axis of applicableAxes(models, axes, selection.model)) {
    resolved[axis.key] = axisValue(axis, selection);
  }
  return { model: selection.model, axes: resolved };
}

/** This axis' contribution to the trigger summary, or null when it has none. */
export function axisSummaryFragment(
  axis: ModelCapabilityAxis,
  value: ModelAxisValue,
): string | null {
  if (axis.showInSummary === false) return null;
  if (axis.kind === "toggle") {
    if (typeof value !== "boolean") return null;
    return (value ? axis.onLabel : axis.offLabel) ?? null;
  }
  return (axis.options ?? []).find((option) => option.value === value)?.label ?? null;
}

/** Applicable axis fragments joined with " · ", in the model's axis order. */
export function summaryText(
  models: ModelOption[],
  axes: ModelCapabilityAxis[],
  selection: ModelSelection,
): string {
  if (!selection.model) return "";
  return applicableAxes(models, axes, selection.model)
    .map((axis) => axisSummaryFragment(axis, axisValue(axis, selection)))
    .filter((fragment): fragment is string => fragment !== null && fragment !== "")
    .join(" · ");
}

/** Trigger label: the selected option's label, the raw value when the host holds
 * a model outside the current list, else the placeholder. */
export function modelLabel(
  models: ModelOption[],
  selection: ModelSelection,
  placeholder: string,
): string {
  if (!selection.model) return placeholder;
  return models.find((model) => model.value === selection.model)?.label ?? selection.model;
}

/** The group heading to emit before `models[index]`, when it opens a new run. */
export function groupHeadingFor(models: ModelOption[], index: number): string | null {
  const group = models[index]?.group;
  if (!group) return null;
  return models[index - 1]?.group === group ? null : group;
}

/** The selection a fresh, uncontrolled picker starts on: the first enabled
 * model plus every axis it exposes at that axis' default. */
export function initialSelection(
  models: ModelOption[],
  axes: ModelCapabilityAxis[],
): ModelSelection {
  const model = models.find((option) => !option.disabled)?.value ?? "";
  return resolveSelection(models, axes, { model, axes: {} });
}

/** Above this many options a `select` axis renders as a vertical list instead of
 * a SegmentedControl — a 6- or 7-level effort scale cannot read as segments in
 * the rail's width. */
export const SEGMENTED_AXIS_MAX_OPTIONS = 3;

/** Which control a `select` axis renders as. `axis.control` overrides the
 * option-count rule. */
export function axisControlKind(axis: ModelCapabilityAxis): "segmented" | "list" {
  if (axis.kind === "toggle") return "segmented";
  if (axis.control === "segmented" || axis.control === "list") return axis.control;
  return (axis.options ?? []).length > SEGMENTED_AXIS_MAX_OPTIONS ? "list" : "segmented";
}
