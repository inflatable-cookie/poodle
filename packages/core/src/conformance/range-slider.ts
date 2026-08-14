/**
 * RangeSlider portable interface — controlled two-part value profile (g14.003).
 * Svelte/React import the inferred portable types; cases and observers resolve
 * parts from the declared descriptors. No second hand-written type mirror.
 */

import { defineComponentInterface } from "./define";
import type {
  AxisNamesOf,
  EventNamesOf,
  PartIdsOf,
  PortableEventsOf,
  PortablePropsOf,
  StateNamesOf,
  TokenRoleNamesOf,
} from "./define";

export const rangeSliderInterface = defineComponentInterface({
  id: "range-slider",
  profile: "control",
  props: [
    { name: "value", type: { kind: "numberPair" }, default: [0, 100], controlledBy: "valueChange" },
    { name: "min", type: { kind: "number" }, default: 0 },
    { name: "max", type: { kind: "number" }, default: 100 },
    { name: "step", type: { kind: "number" }, default: 1 },
    {
      name: "variant",
      type: { kind: "enum", values: ["standard", "embedded"] },
      default: "standard",
      rustType: "SliderVariant",
    },
    {
      name: "polarity",
      type: { kind: "enum", values: ["unipolar", "bipolar"] },
      default: "unipolar",
      rustType: "SliderPolarity",
    },
    { name: "centerValue", type: { kind: "number" }, default: null, nullable: true },
    {
      name: "orientation",
      type: { kind: "enum", values: ["horizontal", "vertical"] },
      default: "horizontal",
      rustType: "Orientation",
    },
    { name: "disabled", type: { kind: "boolean" }, default: false, rustName: "is_disabled" },
    { name: "ariaLabel", type: { kind: "string" }, default: null, nullable: true },
    { name: "lowerValueText", type: { kind: "string" }, default: null, nullable: true },
    { name: "upperValueText", type: { kind: "string" }, default: null, nullable: true },
    {
      name: "size",
      type: { kind: "enum", values: ["xs", "sm", "md", "lg", "xl"] },
      default: null,
      nullable: true,
      rustType: "ControlSize",
    },
    {
      name: "sizeRole",
      type: { kind: "enum", values: ["chrome", "control", "prominent"] },
      default: "control",
      rustType: "SemanticControlSizeRole",
    },
    {
      name: "density",
      type: { kind: "enum", values: ["compact", "default", "comfortable"] },
      default: null,
      nullable: true,
      rustType: "ControlDensity",
    },
  ],
  events: [
    { name: "valueChange", payload: { value: "numberPair" } },
    { name: "valueCommit", payload: { value: "numberPair" } },
  ],
  regions: [],
  parts: [
    {
      id: "root",
      role: "group",
      resolve: { web: { kind: "self" }, native: { kind: "self" } },
    },
    {
      id: "lower",
      role: "slider",
      resolve: {
        web: {
          kind: "class",
          className:
            ".poodle-range-slider__control--lower, .poodle-range-slider__embedded-control--lower",
        },
        native: { kind: "id", id: "range-slider-lower" },
      },
    },
    {
      id: "upper",
      role: "slider",
      resolve: {
        web: {
          kind: "class",
          className:
            ".poodle-range-slider__control--upper, .poodle-range-slider__embedded-control--upper",
        },
        native: { kind: "id", id: "range-slider-upper" },
      },
    },
  ],
  states: [
    {
      name: "disabled",
      web: "data-attr",
      attr: "data-disabled",
      native: "interaction-disabled",
    },
    {
      name: "lowerFocused",
      web: "part-active-element",
      part: "lower",
      native: "part-backend-focus",
    },
    {
      name: "upperFocused",
      web: "part-active-element",
      part: "upper",
      native: "part-backend-focus",
    },
  ],
  tokenRoles: [
    { name: "size", prop: "size", default: "md" },
    { name: "density", prop: "density", default: "default" },
    { name: "variant", prop: "variant" },
    { name: "polarity", prop: "polarity" },
  ],
  axes: ["size", "density", "theme"],
  capabilities: [
    { name: "focus", required: true },
    { name: "interaction.scrub", required: true },
    { name: "interaction.key", required: true },
    { name: "accessibility.projection", required: true },
  ],
});

export type RangeSliderInterface = typeof rangeSliderInterface;
export type RangeSliderPortableProps = PortablePropsOf<RangeSliderInterface>;
export type RangeSliderPortableEvents = PortableEventsOf<RangeSliderInterface>;
export type RangeSliderPartId = PartIdsOf<RangeSliderInterface>;
export type RangeSliderStateName = StateNamesOf<RangeSliderInterface>;
export type RangeSliderEventName = EventNamesOf<RangeSliderInterface>;
export type RangeSliderTokenRole = TokenRoleNamesOf<RangeSliderInterface>;
export type RangeSliderAxis = AxisNamesOf<RangeSliderInterface>;

type AssertPayload<T extends true> = T;
type ValueChangeParams = Parameters<RangeSliderPortableEvents["valueChange"]>;
type ValueCommitParams = Parameters<RangeSliderPortableEvents["valueCommit"]>;
type _ValueChangeTakesPair = AssertPayload<
  ValueChangeParams extends [[number, number]] ? true : false
>;
type _ValueCommitTakesPair = AssertPayload<
  ValueCommitParams extends [[number, number]] ? true : false
>;

export const RANGE_SLIDER_DEFAULT_PROPS: RangeSliderPortableProps = {
  value: [0, 100],
  min: 0,
  max: 100,
  step: 1,
  variant: "standard",
  polarity: "unipolar",
  centerValue: null,
  orientation: "horizontal",
  disabled: false,
  ariaLabel: null,
  lowerValueText: null,
  upperValueText: null,
  size: null,
  sizeRole: "control",
  density: null,
};
