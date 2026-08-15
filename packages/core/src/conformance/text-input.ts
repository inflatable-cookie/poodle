/**
 * TextInput portable interface — input profile (g14.006).
 *
 * Contract: `docs/contracts/components/text-input.md`. Portable claims:
 * controlled/uncontrolled value, typing, selection, validation chrome,
 * search clear, focus, submit/cancel, disabled/read-only, affixes,
 * adornment icons, and event order.
 *
 * Web-only HTML attributes (`autocomplete`, `spellcheck`, `autofocus`,
 * `autocapitalize`, `autocorrect`, `enterKeyHint`, `list`, `name`, `pattern`,
 * `inputMode`, `debounce`, `describedBy`) and the imperative `focus()` method
 * stay beside the web adapters. Native caret props (`selectionStart`,
 * `selectionEnd`, `isFocused`) are native extensions: generated into the
 * Rust spec so hosts can store caret state, never into PortablePropsOf.
 * Portable selection actions and observations use UTF-16 code-unit offsets;
 * the GPUI adapter translates them to its scalar-indexed editor state.
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

export const textInputInterface = defineComponentInterface({
  id: "text-input",
  profile: "input",
  props: [
    {
      name: "value",
      type: { kind: "string" },
      default: null,
      nullable: true,
      controlledBy: "valueChange",
    },
    { name: "defaultValue", type: { kind: "string" }, default: "" },
    { name: "placeholder", type: { kind: "string" }, default: null, nullable: true },
    { name: "disabled", type: { kind: "boolean" }, default: false, rustName: "is_disabled" },
    { name: "readOnly", type: { kind: "boolean" }, default: false, rustName: "is_read_only" },
    { name: "required", type: { kind: "boolean" }, default: false, rustName: "is_required" },
    {
      name: "validationState",
      type: { kind: "enum", values: ["none", "invalid", "valid", "pending"] },
      default: "none",
      rustType: "ValidationState",
    },
    {
      name: "showValidationStatus",
      type: { kind: "boolean" },
      default: true,
      rustName: "shows_validation_status",
    },
    { name: "ariaLabel", type: { kind: "string" }, default: null, nullable: true },
    { name: "prefix", type: { kind: "string" }, default: null, nullable: true },
    { name: "suffix", type: { kind: "string" }, default: null, nullable: true },
    {
      name: "maxLength",
      type: { kind: "number" },
      default: null,
      nullable: true,
      rustType: "usize",
    },
    { name: "showCharCount", type: { kind: "boolean" }, default: false },
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
    { name: "type", type: { kind: "string" }, default: "text", rustName: "input_type" },
    {
      name: "rows",
      type: { kind: "number" },
      default: null,
      nullable: true,
      rustType: "u16",
    },
    { name: "resize", type: { kind: "string" }, default: "vertical" },
    { name: "source", type: { kind: "string" }, default: null, nullable: true },
    { name: "showClearButton", type: { kind: "boolean" }, default: true },
    { name: "leadingIcon", type: { kind: "icon" }, default: null, nullable: true },
    { name: "trailingIcon", type: { kind: "icon" }, default: null, nullable: true },
    { name: "id", type: { kind: "string" }, default: null, nullable: true },
    // Native caret: the host stores scalar-character offsets; web reads its
    // UTF-16 DOM selection. The conformance adapter translates at the portable
    // action/observation boundary. Generated into the Rust spec, excluded from
    // PortablePropsOf.
    {
      name: "selectionStart",
      type: { kind: "number" },
      default: 0,
      rustType: "usize",
      extension: "native-caret",
    },
    {
      name: "selectionEnd",
      type: { kind: "number" },
      default: 0,
      rustType: "usize",
      extension: "native-caret",
    },
    {
      name: "isFocused",
      type: { kind: "boolean" },
      default: false,
      extension: "native-caret",
    },
    // Residual native-compat fields existing specimens still call. Not in
    // the portable TS surface or the case corpus.
    { name: "name", type: { kind: "string" }, default: null, nullable: true, extension: "native" },
    {
      name: "autocomplete",
      type: { kind: "string" },
      default: null,
      nullable: true,
      extension: "native",
    },
    {
      name: "pattern",
      type: { kind: "string" },
      default: null,
      nullable: true,
      extension: "native",
    },
    {
      name: "inputMode",
      type: { kind: "string" },
      default: null,
      nullable: true,
      rustName: "input_mode",
      extension: "native",
    },
    {
      name: "debounceMs",
      type: { kind: "number" },
      default: 0,
      rustType: "u32",
      rustName: "debounce_ms",
      extension: "native",
    },
    {
      name: "descriptionId",
      type: { kind: "string" },
      default: null,
      nullable: true,
      rustName: "description_id",
      extension: "native",
    },
    {
      name: "errorMessageId",
      type: { kind: "string" },
      default: null,
      nullable: true,
      rustName: "error_message_id",
      extension: "native",
    },
    {
      name: "submitEnabled",
      type: { kind: "boolean" },
      default: false,
      rustName: "submit_enabled",
      extension: "native",
    },
    {
      name: "cancelEnabled",
      type: { kind: "boolean" },
      default: false,
      rustName: "cancel_enabled",
      extension: "native",
    },
    // Web-only HTML attributes — never generated into the Rust spec.
    { name: "spellcheck", type: { kind: "boolean" }, default: null, nullable: true, extension: "web-html" },
    { name: "autofocus", type: { kind: "boolean" }, default: false, extension: "web-html" },
    { name: "list", type: { kind: "string" }, default: null, nullable: true, extension: "web-html" },
    { name: "debounce", type: { kind: "number" }, default: null, nullable: true, extension: "web-html" },
    { name: "describedBy", type: { kind: "string" }, default: null, nullable: true, extension: "web-html" },
  ],
  events: [
    { name: "valueChange", payload: { value: "string" } },
    { name: "submit", payload: { value: "string" } },
    { name: "cancel", payload: {} },
    { name: "clear", payload: {} },
  ],
  regions: [
    { name: "leading", payload: "icon" },
    { name: "trailing", payload: "icon" },
  ],
  parts: [
    {
      id: "root",
      resolve: { web: { kind: "self" }, native: { kind: "self" } },
    },
    {
      id: "control",
      role: "textbox",
      contains: "text",
      resolve: {
        web: { kind: "class", className: ".poodle-text-input__control" },
        native: { kind: "self" },
      },
    },
    {
      id: "prefix",
      contains: "text",
      resolve: {
        web: { kind: "class", className: ".poodle-text-input__affix--prefix" },
        native: { kind: "id", id: "text-input-prefix" },
      },
    },
    {
      id: "suffix",
      contains: "text",
      resolve: {
        web: { kind: "class", className: ".poodle-text-input__affix--suffix" },
        native: { kind: "id", id: "text-input-suffix" },
      },
    },
    {
      id: "leading",
      contains: "icon",
      resolve: {
        web: {
          kind: "icon",
          position: "first",
          gatedBy: "data-has-leading",
          selector: ".poodle-text-input__affordance--leading",
          attribute: "data-icon",
        },
        native: { kind: "id", id: "text-input-leading" },
      },
    },
    {
      id: "trailing",
      contains: "icon",
      resolve: {
        web: {
          kind: "icon",
          position: "first",
          gatedBy: "data-has-trailing",
          selector: ".poodle-text-input__affordance--trailing",
          attribute: "data-icon",
        },
        native: { kind: "id", id: "text-input-trailing" },
      },
    },
    {
      id: "clear",
      role: "button",
      resolve: {
        web: { kind: "class", className: ".poodle-text-input__clear" },
        native: { kind: "id", id: "text-input-clear" },
      },
    },
    {
      id: "validationIndicator",
      contains: "icon",
      resolve: {
        web: { kind: "class", className: ".poodle-text-input__validation-indicator", attribute: "data-icon" },
        native: { kind: "id", id: "text-input-validation" },
      },
    },
    {
      id: "charCount",
      contains: "text",
      resolve: {
        web: { kind: "class", className: ".poodle-text-input__char-count" },
        native: { kind: "id", id: "text-input-char-count" },
      },
    },
  ],
  states: [
    {
      name: "disabled",
      web: "part-disabled-attr",
      part: "control",
      native: "part-interaction-disabled",
    },
    {
      name: "focused",
      web: "part-active-element",
      part: "control",
      native: "part-backend-focus",
    },
    {
      name: "prefixPresent",
      web: "part-present",
      part: "prefix",
      native: "part-present",
    },
    {
      name: "suffixPresent",
      web: "part-present",
      part: "suffix",
      native: "part-present",
    },
    {
      name: "clearPresent",
      web: "part-present",
      part: "clear",
      native: "part-present",
    },
    {
      name: "indicatorPresent",
      web: "part-present",
      part: "validationIndicator",
      native: "part-present",
    },
  ],
  tokenRoles: [
    { name: "size", prop: "size", default: "md" },
    { name: "density", prop: "density", default: "default" },
    { name: "validation", prop: "validationState", default: "none" },
    { name: "type", prop: "type", default: "text" },
  ],
  axes: ["size", "density", "theme"],
  capabilities: [
    { name: "input.value", required: true },
    { name: "input.editing", required: true },
    { name: "input.ime", required: true },
    { name: "focus", required: true },
    { name: "accessibility.projection", required: true },
  ],
});

export type TextInputInterface = typeof textInputInterface;
export type TextInputPortableProps = PortablePropsOf<TextInputInterface>;
export type TextInputPortableEvents = PortableEventsOf<TextInputInterface>;
export type TextInputPartId = PartIdsOf<TextInputInterface>;
export type TextInputStateName = StateNamesOf<TextInputInterface>;
export type TextInputEventName = EventNamesOf<TextInputInterface>;
export type TextInputTokenRole = TokenRoleNamesOf<TextInputInterface>;
export type TextInputAxis = AxisNamesOf<TextInputInterface>;

export const TEXT_INPUT_DEFAULT_PROPS: TextInputPortableProps = {
  value: null,
  defaultValue: "",
  placeholder: null,
  disabled: false,
  readOnly: false,
  required: false,
  validationState: "none",
  showValidationStatus: true,
  ariaLabel: null,
  prefix: null,
  suffix: null,
  maxLength: null,
  showCharCount: false,
  size: null,
  sizeRole: "control",
  density: null,
  type: "text",
  rows: null,
  resize: "vertical",
  source: null,
  showClearButton: true,
  leadingIcon: null,
  trailingIcon: null,
  id: null,
};
