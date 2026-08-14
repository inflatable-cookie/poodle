/**
 * RangeSlider conformance case corpus (spec 066, g14.003). One authored
 * corpus drives Svelte, React, and GPUI. Assertions stay within fields every
 * active runtime can observe: thumb identity, values, roles, names, focus,
 * token roles, event order, and bounded geometry on non-interactive display
 * cases.
 */

import {
  actionFocus,
  actionKey,
  actionScrub,
  componentCase,
  expectEvents,
  expectPart,
  serializeCases,
  type PortablePropsOf,
} from "./define";
import { rangeSliderInterface } from "./range-slider";
import type { RangeSliderInterface } from "./range-slider";

type I = RangeSliderInterface;
type FixtureProps = Partial<PortablePropsOf<I>>;

function thumbName(ariaLabel: string | null | undefined, which: "minimum" | "maximum"): string {
  if (ariaLabel) return `${ariaLabel} ${which}`;
  return which === "minimum" ? "Minimum value" : "Maximum value";
}

function displayCase(
  id: string,
  caption: string,
  group: string,
  props: FixtureProps,
  axes: readonly ("size" | "density" | "theme")[] = ["theme"],
): ReturnType<typeof componentCase<I>> {
  const value = props.value ?? [0, 100];
  const disabled = props.disabled === true;
  const ariaLabel = props.ariaLabel ?? null;
  return componentCase(rangeSliderInterface, {
    id: `range-slider/${id}`,
    fixture: { props, regions: {} },
    specimen: { group, caption, captureId: `range-slider/${id}`, axes },
    steps: [
      expectPart("root", {
        role: "group",
        focusable: false,
        states: { disabled },
        tokenRoles: {
          size: String(props.size ?? "md"),
          density: String(props.density ?? "default"),
          variant: String(props.variant ?? "standard"),
          polarity: String(props.polarity ?? "unipolar"),
        },
        value,
      }),
      expectPart("lower", {
        role: "slider",
        name: thumbName(ariaLabel, "minimum"),
        focusable: !disabled,
        value: value[0],
      }),
      expectPart("upper", {
        role: "slider",
        name: thumbName(ariaLabel, "maximum"),
        focusable: !disabled,
        value: value[1],
      }),
      expectEvents([]),
    ],
  });
}

const cases = [
  displayCase("default", "Default", "Basics", {
    value: [20, 80],
    ariaLabel: "Price range",
  }),

  displayCase("with-step", "With step", "Basics", {
    value: [25, 45],
    min: 18,
    max: 65,
    step: 5,
    ariaLabel: "Age range",
  }),

  displayCase("disabled", "Disabled", "States", {
    value: [30, 70],
    disabled: true,
    ariaLabel: "Disabled range",
  }),

  displayCase(
    "vertical",
    "Vertical",
    "Orientation",
    {
      value: [20, 80],
      orientation: "vertical",
      ariaLabel: "Vertical range",
    },
    ["theme"],
  ),

  displayCase(
    "embedded-unipolar",
    "Embedded unipolar",
    "Variants",
    {
      value: [0.2, 0.75],
      min: 0,
      max: 1,
      step: 0.01,
      variant: "embedded",
      polarity: "unipolar",
      ariaLabel: "Unipolar modulation range",
    },
    ["theme"],
  ),

  displayCase(
    "size-lg",
    "Large",
    "Size",
    { value: [20, 80], size: "lg", ariaLabel: "Large range" },
    ["density", "theme"],
  ),

  // Controlled pointer scrub: press nearer the upper thumb, keep gesture.
  componentCase(rangeSliderInterface, {
    id: "range-slider/scrub-upper",
    fixture: {
      props: { value: [20, 80], ariaLabel: "Scrub range" },
      regions: {},
    },
    specimen: {
      group: "Behaviour",
      caption: "Scrub upper thumb",
      captureId: "range-slider/scrub-upper",
      axes: [],
    },
    steps: [
      expectPart("lower", { value: 20 }),
      expectPart("upper", { value: 80 }),
      expectEvents([]),
      actionScrub("root", 0.9, "press"),
      expectEvents(["valueChange"]),
      expectPart("root", { value: [20, 90] }),
      expectPart("upper", { value: 90 }),
      actionScrub("root", 0.9, "release"),
      expectEvents(["valueChange", "valueCommit"]),
    ],
  }),

  // Crossing policy: drag lower past upper; clamp, keep thumb identity.
  componentCase(rangeSliderInterface, {
    id: "range-slider/crossing-clamp",
    fixture: {
      props: { value: [20, 80], ariaLabel: "Crossing range" },
      regions: {},
    },
    specimen: {
      group: "Behaviour",
      caption: "Crossing clamp",
      captureId: "range-slider/crossing-clamp",
      axes: [],
    },
    steps: [
      expectEvents([]),
      actionScrub("root", 0.1, "press"),
      expectEvents(["valueChange"]),
      actionScrub("root", 0.95, "drag"),
      expectEvents(["valueChange", "valueChange"]),
      expectPart("root", { value: [80, 80] }),
      expectPart("lower", { value: 80 }),
      expectPart("upper", { value: 80 }),
      actionScrub("root", 0.95, "release"),
      expectEvents(["valueChange", "valueChange", "valueCommit"]),
    ],
  }),

  // Keyboard step on the lower thumb.
  componentCase(rangeSliderInterface, {
    id: "range-slider/keyboard-step",
    fixture: {
      props: { value: [20, 80], step: 5, ariaLabel: "Keyboard range" },
      regions: {},
    },
    specimen: {
      group: "Behaviour",
      caption: "Keyboard step",
      captureId: "range-slider/keyboard-step",
      axes: [],
    },
    steps: [
      actionFocus("lower"),
      expectPart("root", { states: { lowerFocused: true, upperFocused: false } }),
      expectEvents([]),
      actionKey("lower", "ArrowRight"),
      expectEvents(["valueChange", "valueCommit"]),
      expectPart("lower", { value: 25 }),
      expectPart("root", { value: [25, 80] }),
    ],
  }),

  // Disabled is inert under scrub.
  componentCase(rangeSliderInterface, {
    id: "range-slider/disabled-inert",
    fixture: {
      props: { value: [30, 70], disabled: true, ariaLabel: "Inert range" },
      regions: {},
    },
    specimen: {
      group: "Behaviour",
      caption: "Disabled inert",
      captureId: "range-slider/disabled-inert",
      axes: [],
    },
    steps: [
      expectPart("root", { states: { disabled: true }, value: [30, 70] }),
      expectEvents([]),
      actionScrub("root", 0.9, "press"),
      expectEvents([]),
      expectPart("root", { value: [30, 70] }),
    ],
  }),
];

export const rangeSliderCases = serializeCases("range-slider", cases);
