/**
 * Button portable interface — the single authority for the portable Button
 * surface (spec 066, architecture 009). Svelte and React import the inferred
 * portable types (`PortablePropsOf`/`PortableEventsOf` of this value);
 * `poodle-codegen --conformance` generates the Rust `ButtonSpec` declaration
 * from the serialized form; the web and native observers resolve parts and
 * states from the declared resolution descriptors. Nothing portable is
 * restated anywhere else.
 *
 * Web-only HTML attributes (`type`, `form*`, className, style) are marked as
 * platform extensions and are not portable. The `label` prop is the portable
 * text; web implementations render it through the `label` region (children).
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

export const buttonInterface = defineComponentInterface({
  id: "button",
  profile: "control",
  props: [
    { name: "variant", type: { kind: "enum", values: ["primary", "secondary", "ghost"] }, default: "secondary", rustType: "ButtonVariant" },
    { name: "tone", type: { kind: "enum", values: ["default", "danger", "success", "warning"] }, default: "default", rustType: "ButtonTone" },
    { name: "size", type: { kind: "enum", values: ["xs", "sm", "md", "lg", "xl"] }, default: "md", nullable: true, rustType: "ControlSize" },
    { name: "sizeRole", type: { kind: "enum", values: ["chrome", "control", "prominent"] }, default: "control", rustType: "SemanticControlSizeRole" },
    { name: "density", type: { kind: "enum", values: ["compact", "default", "comfortable"] }, default: "default", nullable: true, rustType: "ControlDensity" },
    { name: "disabled", type: { kind: "boolean" }, default: false, rustName: "is_disabled" },
    { name: "loading", type: { kind: "boolean" }, default: false, rustName: "is_loading" },
    { name: "leadingIcon", type: { kind: "icon" }, default: null, nullable: true },
    { name: "trailingIcon", type: { kind: "icon" }, default: null, nullable: true },
    { name: "chevron", type: { kind: "boolean" }, default: false },
    { name: "truncate", type: { kind: "boolean" }, default: false },
    { name: "fit", type: { kind: "enum", values: ["default", "content"] }, default: "default", rustEnumName: "ButtonFit" },
    { name: "maxWidth", type: { kind: "dimension" }, default: null, nullable: true },
    { name: "pressed", type: { kind: "boolean" }, default: null, nullable: true, controlledBy: "pressedChange" },
    { name: "defaultPressed", type: { kind: "boolean" }, default: false, nullable: true },
    { name: "label", type: { kind: "string" }, default: null, nullable: true },
    { name: "ariaLabel", type: { kind: "string" }, default: null, nullable: true },
    { name: "ariaExpanded", type: { kind: "boolean" }, default: null, nullable: true },
    { name: "describedBy", type: { kind: "string" }, default: null, nullable: true },
    // Web-only platform extensions.
    { name: "type", type: { kind: "enum", values: ["button", "submit", "reset"] }, default: "button", extension: "web-html" },
    { name: "form", type: { kind: "string" }, default: null, nullable: true, extension: "web-html" },
    { name: "formAction", type: { kind: "string" }, default: null, nullable: true, extension: "web-html" },
    { name: "formNoValidate", type: { kind: "boolean" }, default: false, extension: "web-html" },
    { name: "formTarget", type: { kind: "string" }, default: null, nullable: true, extension: "web-html" },
    { name: "className", type: { kind: "string" }, default: "", extension: "web-styling" },
    { name: "style", type: { kind: "string" }, default: null, nullable: true, extension: "web-styling" },
  ],
  events: [
    { name: "press", payload: {} },
    { name: "pressedChange", payload: { pressed: "boolean" } },
  ],
  regions: [
    { name: "label", payload: "text" },
    { name: "leading", payload: "icon" },
    { name: "trailing", payload: "icon" },
  ],
  parts: [
    {
      id: "root",
      role: "button",
      contains: "label",
      resolve: { web: { kind: "self" }, native: { kind: "self" } },
    },
    {
      id: "label",
      contains: "text",
      resolve: {
        web: { kind: "class", className: ".poodle-button__label" },
        native: { kind: "root-label" },
      },
    },
    {
      id: "leadingIcon",
      contains: "icon",
      resolve: {
        web: { kind: "icon", position: "first", gatedBy: "data-has-leading" },
        native: { kind: "icon-side", side: "leading", except: ["spinner"] },
      },
    },
    {
      id: "trailingIcon",
      contains: "icon",
      resolve: {
        web: { kind: "icon", position: "last", gatedBy: "data-has-trailing" },
        native: { kind: "icon-side", side: "trailing", except: ["chevron-down"] },
      },
    },
    {
      id: "spinner",
      contains: "icon",
      resolve: {
        web: { kind: "class", className: ".poodle-button__spinner" },
        native: { kind: "icon-named", name: "spinner" },
      },
    },
    {
      id: "chevron",
      contains: "icon",
      resolve: {
        web: { kind: "class", className: ".poodle-button__chevron" },
        native: { kind: "icon-named", name: "chevron-down" },
      },
    },
  ],
  states: [
    {
      name: "disabled",
      condition: "disabled || loading",
      web: "disabled-attr",
      native: "interaction-disabled",
    },
    {
      name: "loading",
      web: "data-attr",
      attr: "data-loading",
      native: "part-present",
      part: "spinner",
    },
    {
      name: "pressed",
      condition: "toggle mode: pressed ?? defaultPressed",
      web: "aria-pressed",
      native: "a11y-toggled",
    },
    {
      name: "focused",
      web: "active-element",
      native: "backend-focus",
    },
    {
      name: "focusVisible",
      web: "focus-visible-pseudo",
      native: "focus-with-focus-style",
    },
  ],
  tokenRoles: [
    { name: "variant", prop: "variant" },
    { name: "tone", prop: "tone", default: "default" },
    { name: "size", prop: "size" },
    { name: "density", prop: "density" },
    { name: "fit", prop: "fit", default: "default" },
  ],
  axes: ["size", "density", "theme"],
  capabilities: [
    { name: "activate", required: true },
    { name: "focus", required: true },
    { name: "toggle", required: true },
  ],
});

export type ButtonInterface = typeof buttonInterface;

/** Portable props, mechanically derived from the interface. */
export type ButtonPortableProps = PortablePropsOf<ButtonInterface>;

/** Portable events, mechanically derived from the interface. */
export type ButtonPortableEvents = PortableEventsOf<ButtonInterface>;

export type ButtonPartId = PartIdsOf<ButtonInterface>;
export type ButtonStateName = StateNamesOf<ButtonInterface>;
export type ButtonEventName = EventNamesOf<ButtonInterface>;
export type ButtonTokenRole = TokenRoleNamesOf<ButtonInterface>;
export type ButtonAxis = AxisNamesOf<ButtonInterface>;

export const BUTTON_DEFAULT_PROPS: ButtonPortableProps = {
  variant: "secondary",
  tone: "default",
  size: null,
  sizeRole: "control",
  density: null,
  disabled: false,
  loading: false,
  leadingIcon: null,
  trailingIcon: null,
  chevron: false,
  truncate: false,
  fit: "default",
  maxWidth: null,
  pressed: null,
  defaultPressed: null,
  label: null,
  ariaLabel: null,
  ariaExpanded: null,
  describedBy: null,
};
