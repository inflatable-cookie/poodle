// Renderer-neutral ModelPicker types and behavior. Framework packages own the
// visual shell; this module owns axis resolution and selection normalization.

/** An image source for a model row. `alt` defaults to `""` because the visible
 * model label normally supplies the accessible name. */
export type ModelImage = { src: string; alt?: string };

export type ModelAxisKind = "select" | "toggle";

export type ModelAxisOption = {
  value: string;
  label: string;
  description?: string;
  disabled?: boolean;
};

export type ModelAxisValue = string | boolean;

/** Per-model overrides for a shared axis definition. Unset fields inherit from
 * the shared axis with the same key. */
export type ModelAxisBinding = {
  key: string;
  label?: string;
  description?: string;
  options?: ModelAxisOption[];
  control?: "auto" | "segmented" | "list";
  defaultValue?: ModelAxisValue;
  onLabel?: string;
  offLabel?: string;
  showInSummary?: boolean;
  disabled?: boolean;
};

export type ModelAxisRef = string | ModelAxisBinding;

/** One selectable engine or model. Labels, grouping, badges, and capability
 * axes are host vocabulary; Poodle does not assign provider-specific meaning. */
export type ModelOption = {
  value: string;
  label: string;
  description?: string;
  badge?: string;
  icon?: string;
  group?: string;
  disabled?: boolean;
  /** Replaces `icon` when both are set. */
  image?: ModelImage;
  /** Capability axes in display order. Omit to inherit every declared axis. */
  axes?: ModelAxisRef[];
};

/** A host-declared capability axis. Its key becomes the corresponding key in
 * `ModelSelection.axes`. */
export type ModelCapabilityAxis = {
  key: string;
  label: string;
  kind: ModelAxisKind;
  description?: string;
  /** Used by `select` axes. */
  options?: ModelAxisOption[];
  /** `auto` uses a segmented control for up to three options, then a list. */
  control?: "auto" | "segmented" | "list";
  /** Applied when the selected model has no compatible held value. */
  defaultValue?: ModelAxisValue;
  /** Trigger-summary labels for a `toggle` axis. */
  onLabel?: string;
  offLabel?: string;
  /** Defaults to true. */
  showInSummary?: boolean;
  disabled?: boolean;
};

export type ModelSelection = {
  model: string;
  axes: Record<string, ModelAxisValue>;
};

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

export function axisAccepts(axis: ModelCapabilityAxis, value: ModelAxisValue): boolean {
  if (axis.kind === "toggle") return typeof value === "boolean";
  return typeof value === "string" && (axis.options ?? []).some((option) => option.value === value);
}

export function axisDefaultValue(axis: ModelCapabilityAxis): ModelAxisValue {
  if (axis.defaultValue !== undefined && axisAccepts(axis, axis.defaultValue)) {
    return axis.defaultValue;
  }
  if (axis.kind === "toggle") return false;
  return (axis.options ?? []).find((option) => !option.disabled)?.value ?? "";
}

export function axisValue(axis: ModelCapabilityAxis, selection: ModelSelection): ModelAxisValue {
  const held = selection.axes?.[axis.key];
  if (held !== undefined && axisAccepts(axis, held)) return held;
  return axisDefaultValue(axis);
}

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

export function modelLabel(
  models: ModelOption[],
  selection: ModelSelection,
  placeholder: string,
): string {
  if (!selection.model) return placeholder;
  return models.find((model) => model.value === selection.model)?.label ?? selection.model;
}

export function groupHeadingFor(models: ModelOption[], index: number): string | null {
  const group = models[index]?.group;
  if (!group) return null;
  return models[index - 1]?.group === group ? null : group;
}

export function initialSelection(
  models: ModelOption[],
  axes: ModelCapabilityAxis[],
): ModelSelection {
  const model = models.find((option) => !option.disabled)?.value ?? "";
  return resolveSelection(models, axes, { model, axes: {} });
}

export const SEGMENTED_AXIS_MAX_OPTIONS = 3;

export function axisControlKind(axis: ModelCapabilityAxis): "segmented" | "list" {
  if (axis.kind === "toggle") return "segmented";
  if (axis.control === "segmented" || axis.control === "list") return axis.control;
  return (axis.options ?? []).length > SEGMENTED_AXIS_MAX_OPTIONS ? "list" : "segmented";
}
