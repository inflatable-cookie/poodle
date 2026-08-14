/**
 * Tabs portable interface — identified collection profile (g14.004).
 * Repeated parts are addressed as `<part>:<item.value>` so fixture order can
 * change without changing semantic identity.
 */

import { defineComponentInterface } from "./define";
import type {
  AxisNamesOf,
  EventNamesOf,
  PartIdsOf,
  PortableEventsOf,
  PortablePropsOf,
  TokenRoleNamesOf,
} from "./define";

export const tabsInterface = defineComponentInterface({
  id: "tabs",
  profile: "collection",
  props: [
    {
      name: "items",
      type: {
        kind: "collection",
        rustType: "TabDefinition",
        fields: [
          { name: "value", type: { kind: "string" } },
          { name: "label", type: { kind: "string" } },
          { name: "icon", type: { kind: "icon" }, optional: true },
          { name: "disabled", type: { kind: "boolean" }, optional: true },
          { name: "closable", type: { kind: "boolean" }, optional: true },
          { name: "count", type: { kind: "number" }, optional: true },
        ],
      },
      default: [],
      rustName: "tabs",
    },
    { name: "value", type: { kind: "string" }, default: null, nullable: true, controlledBy: "valueChange" },
    {
      name: "variant",
      type: { kind: "enum", values: ["card", "pill", "block"] },
      default: "card",
      rustType: "TabVariant",
    },
    {
      name: "activeEdge",
      type: { kind: "enum", values: ["none", "outline", "underline"] },
      default: "none",
      rustType: "ActiveEdge",
    },
    {
      name: "activeFill",
      type: { kind: "enum", values: ["none", "tint", "solid"] },
      default: "tint",
      rustType: "ActiveFill",
    },
    {
      name: "orientation",
      type: { kind: "enum", values: ["horizontal", "vertical"] },
      default: "horizontal",
      rustType: "Orientation",
    },
    {
      name: "activationMode",
      type: { kind: "enum", values: ["automatic", "manual"] },
      default: "automatic",
      rustType: "TabActivationMode",
    },
    { name: "bordered", type: { kind: "boolean" }, default: false, rustName: "is_bordered" },
    { name: "fullWidth", type: { kind: "boolean" }, default: false, rustName: "is_full_width" },
    { name: "reorderable", type: { kind: "boolean" }, default: false, rustName: "is_reorderable" },
    { name: "ariaLabel", type: { kind: "string" }, default: null, nullable: true },
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
      default: "chrome",
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
  events: [{ name: "valueChange", payload: { value: "string" } }],
  regions: [{ name: "panel", payload: "text" }],
  parts: [
    { id: "root", resolve: { web: { kind: "self" }, native: { kind: "self" } } },
    {
      id: "list",
      role: "tablist",
      resolve: {
        web: { kind: "class", className: ".poodle-tabs__list" },
        native: { kind: "id", id: "tabs-list" },
      },
    },
    {
      id: "trigger",
      role: "tab",
      repeat: { prop: "items", key: "value", webIdPrefix: "poodle-tab-" },
      resolve: {
        web: { kind: "class", className: ".poodle-tabs__tab", keyAttribute: "data-value" },
        native: { kind: "id-template", template: "tabs:{key}" },
      },
    },
    {
      id: "panel",
      role: "tabpanel",
      contains: "text",
      repeat: { prop: "items", key: "value", webIdPrefix: "poodle-tabpanel-" },
      resolve: {
        web: { kind: "class", className: ".poodle-tabs__panel", keyAttribute: "data-value" },
        native: { kind: "id-template", template: "tabs-panel:{key}" },
      },
    },
  ],
  states: [],
  tokenRoles: [
    { name: "variant", prop: "variant", default: "card" },
    { name: "orientation", prop: "orientation", default: "horizontal" },
    // Tabs defaults to the chrome semantic role. At the default presentation
    // scale that resolves to sm in every active runtime.
    { name: "size", prop: "size", default: "sm" },
    { name: "density", prop: "density", default: "default" },
  ],
  axes: ["variant", "orientation", "size", "density", "theme"],
  capabilities: [
    { name: "structure.identity", required: true },
    { name: "structure.part-resolution", required: true },
    { name: "semantic.selected", required: true },
    { name: "focus", required: true },
    { name: "interaction.key", required: true },
    { name: "accessibility.projection", required: true },
  ],
});

export type TabsInterface = typeof tabsInterface;
export type TabsPortableProps = PortablePropsOf<TabsInterface>;
export type TabsPortableItem = TabsPortableProps["items"][number];
export type TabsPortableEvents = PortableEventsOf<TabsInterface>;
export type TabsPartId = PartIdsOf<TabsInterface>;
export type TabsEventName = EventNamesOf<TabsInterface>;
export type TabsTokenRole = TokenRoleNamesOf<TabsInterface>;
export type TabsAxis = AxisNamesOf<TabsInterface>;

export const TABS_DEFAULT_PROPS: TabsPortableProps = {
  items: [],
  value: null,
  variant: "card",
  activeEdge: "none",
  activeFill: "tint",
  orientation: "horizontal",
  activationMode: "automatic",
  bordered: false,
  fullWidth: false,
  reorderable: false,
  ariaLabel: null,
  size: null,
  sizeRole: "chrome",
  density: null,
};
