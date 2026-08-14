/**
 * Button portable interface — the single authority for the portable Button
 * surface (spec 066, architecture 009). Svelte and React import the inferred
 * portable types; `poodle-codegen --conformance` generates the Rust
 * `ButtonSpec` declaration from the serialized form.
 *
 * Web-only HTML attributes (`type`, `form*`, className, style) are marked as
 * platform extensions and are not portable. The `label` prop is the portable
 * text; web implementations render it through the `label` region (children).
 */

import { defineComponentInterface, type ComponentInterface } from "./define";

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
    { id: "root", role: "button", contains: "label" },
    { id: "label", contains: "text" },
    { id: "leadingIcon", contains: "icon" },
    { id: "trailingIcon", contains: "icon" },
    { id: "spinner", contains: "icon" },
    { id: "chevron", contains: "icon" },
  ],
  states: [
    { name: "disabled", condition: "disabled || loading" },
    { name: "loading" },
    { name: "pressed", condition: "toggle mode: pressed ?? defaultPressed" },
    { name: "focusVisible" },
  ],
  capabilities: [
    { name: "activate", required: true },
    { name: "focus", required: true },
    { name: "toggle", required: true },
  ],
});

export type ButtonInterface = typeof buttonInterface;

/** Portable props (extensions excluded), inferred from the interface. */
export type ButtonPortableProps = {
  variant: "primary" | "secondary" | "ghost";
  tone: "default" | "danger" | "success" | "warning";
  size: "xs" | "sm" | "md" | "lg" | "xl" | null;
  sizeRole: "chrome" | "control" | "prominent";
  density: "compact" | "default" | "comfortable" | null;
  disabled: boolean;
  loading: boolean;
  leadingIcon: string | null;
  trailingIcon: string | null;
  chevron: boolean;
  truncate: boolean;
  fit: "default" | "content";
  maxWidth: string | null;
  pressed: boolean | null;
  defaultPressed: boolean | null;
  label: string | null;
  ariaLabel: string | null;
  ariaExpanded: boolean | null;
  describedBy: string | null;
};

export type ButtonPortableEvents = {
  press: () => void;
  pressedChange: (pressed: boolean) => void;
};

export const BUTTON_PART_IDS = ["root", "label", "leadingIcon", "trailingIcon", "spinner", "chevron"] as const;

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
  defaultPressed: false,
  label: null,
  ariaLabel: null,
  ariaExpanded: null,
  describedBy: null,
};

export type ButtonFixtureProps = Partial<ButtonPortableProps>;

export function resolveButtonFixture(fixture: ButtonFixtureProps): ButtonPortableProps {
  return { ...BUTTON_DEFAULT_PROPS, ...fixture };
}
