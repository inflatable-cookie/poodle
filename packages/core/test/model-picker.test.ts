import { describe, expect, it } from "bun:test";

import {
  axesForModel,
  axisControlKind,
  initialSelection,
  resolveSelection,
  summaryText,
  type ModelCapabilityAxis,
  type ModelOption,
} from "../src/model-picker";

const axes: ModelCapabilityAxis[] = [
  {
    key: "effort",
    label: "Effort",
    kind: "select",
    options: [
      { value: "low", label: "Low" },
      { value: "high", label: "High" },
    ],
    defaultValue: "low",
  },
  {
    key: "fast",
    label: "Fast mode",
    kind: "toggle",
    onLabel: "Fast",
    offLabel: "Standard",
  },
];

const models: ModelOption[] = [
  { value: "pro", label: "Pro", group: "Frontier", axes: ["effort", "fast"] },
  {
    value: "other",
    label: "Other",
    group: "Frontier",
    axes: [
      {
        key: "effort",
        options: [
          { value: "minimal", label: "Minimal" },
          { value: "deep", label: "Deep" },
        ],
        defaultValue: "minimal",
      },
    ],
  },
  { value: "disabled", label: "Disabled", disabled: true },
];

describe("model picker", () => {
  it("merges per-model axis bindings", () => {
    const effort = axes[0];
    const other = models[1];
    if (!effort || !other) throw new Error("missing model picker fixtures");
    expect(axesForModel(axes, other)).toEqual([
      {
        ...effort,
        options: [
          { value: "minimal", label: "Minimal" },
          { value: "deep", label: "Deep" },
        ],
        defaultValue: "minimal",
      },
    ]);
  });

  it("drops unavailable axes and replaces incompatible values", () => {
    expect(
      resolveSelection(models, axes, {
        model: "other",
        axes: { effort: "high", fast: true },
      }),
    ).toEqual({ model: "other", axes: { effort: "minimal" } });
  });

  it("builds summaries in declared axis order", () => {
    expect(
      summaryText(models, axes, {
        model: "pro",
        axes: { effort: "high", fast: true },
      }),
    ).toBe("High · Fast");
  });

  it("starts on the first enabled model with resolved defaults", () => {
    expect(initialSelection(models, axes)).toEqual({
      model: "pro",
      axes: { effort: "low", fast: false },
    });
  });

  it("uses a list only when an automatic select axis exceeds three options", () => {
    const effort = axes[0];
    if (!effort) throw new Error("missing effort axis fixture");
    expect(axisControlKind(effort)).toBe("segmented");
    expect(
      axisControlKind({
        key: "level",
        label: "Level",
        kind: "select",
        options: ["one", "two", "three", "four"].map((value) => ({ value, label: value })),
      }),
    ).toBe("list");
  });
});
