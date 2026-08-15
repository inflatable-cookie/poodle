/**
 * Popover portable interface — overlay profile (g14.005).
 *
 * Contract: `docs/contracts/components/popover.md`. The frozen claims:
 * controlled/uncontrolled open, trigger + children composition regions, all
 * 12 `OverlayPlacement` values and numeric `offset`, `dismissOnOutsideInteract`,
 * `initialFocus` strategies, `ariaLabel`/`disabled`/`block`/`surfaceWidth`
 * and the surface width bounds, trigger/surface relationship, close through
 * trigger/Escape/outside/controlled host, focus entry on open, and trigger
 * focus restoration on every close path.
 *
 * `triggerIsInteractive` and `onSurfaceGeometryChange` are documented web-only
 * extensions; they stay beside the web adapters and never enter the portable
 * surface or active-cohort parity claims.
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

export const popoverInterface = defineComponentInterface({
  id: "popover",
  profile: "overlay",
  props: [
    { name: "open", type: { kind: "boolean" }, default: null, nullable: true, controlledBy: "openChange" },
    { name: "defaultOpen", type: { kind: "boolean" }, default: false },
    {
      name: "placement",
      type: {
        kind: "enum",
        values: [
          "top", "top-start", "top-end",
          "bottom", "bottom-start", "bottom-end",
          "left", "left-start", "left-end",
          "right", "right-start", "right-end",
        ],
      },
      default: "bottom-start",
      rustType: "OverlayPlacement",
    },
    { name: "offset", type: { kind: "number" }, default: 8 },
    { name: "dismissOnOutsideInteract", type: { kind: "boolean" }, default: true },
    {
      name: "initialFocus",
      type: { kind: "enum", values: ["first-focusable", "content", "none"] },
      default: "first-focusable",
      rustType: "PopoverInitialFocus",
    },
    { name: "ariaLabel", type: { kind: "string" }, default: null, nullable: true },
    { name: "block", type: { kind: "boolean" }, default: false },
    { name: "disabled", type: { kind: "boolean" }, default: false },
    {
      name: "surfaceWidth",
      type: { kind: "enum", values: ["content", "trigger"] },
      default: "content",
      rustType: "PopoverSurfaceWidth",
    },
    // Portable surface width bounds are authored in rem (the native targets
    // resolve rem; arbitrary CSS lengths are a web-shell extension that does
    // not port). The corpus proves an override case against both runtimes.
    { name: "surfaceMinWidth", type: { kind: "dimension" }, default: null, nullable: true },
    { name: "surfaceMaxWidth", type: { kind: "dimension" }, default: null, nullable: true },
    // Web-only extension: the wrapper observes clicks without adding a second
    // button role or keyboard handler. Native adapters compose the trigger
    // directly and never see this switch.
    { name: "triggerIsInteractive", type: { kind: "boolean" }, default: false, extension: "web" },
  ],
  events: [{ name: "openChange", payload: { open: "boolean" } }],
  regions: [
    { name: "trigger", payload: "node" },
    { name: "children", payload: "node" },
  ],
  parts: [
    { id: "root", resolve: { web: { kind: "self" }, native: { kind: "self" } } },
    {
      id: "trigger",
      role: "button",
      resolve: {
        web: { kind: "class", className: ".poodle-popover__trigger" },
        native: { kind: "id", id: "popover-trigger" },
      },
    },
    {
      id: "surface",
      role: "dialog",
      relativeTo: "trigger",
      resolve: {
        web: { kind: "class", className: ".poodle-popover__surface" },
        native: { kind: "id", id: "popover-surface" },
      },
    },
  ],
  states: [
    {
      name: "open",
      condition: "the surface is mounted and registered on the dismissable layer",
      web: "part-present",
      native: "part-present",
      part: "surface",
    },
  ],
  tokenRoles: [
    { name: "placement", prop: "placement", default: "bottom-start" },
    { name: "surfaceWidth", prop: "surfaceWidth", default: "content" },
  ],
  axes: ["placement", "theme"],
  capabilities: [
    { name: "overlay.intent", required: true },
    { name: "overlay.dismiss", required: true },
    { name: "overlay.layer", required: true },
    { name: "semantic.expanded", required: true },
    { name: "structure.identity", required: true },
    { name: "structure.part-resolution", required: true },
    { name: "semantic.token-roles", required: true },
    { name: "layout.position", required: true },
    { name: "layout.geometry", required: true },
    { name: "focus", required: true },
    { name: "accessibility.projection", required: true },
  ],
});

export type PopoverInterface = typeof popoverInterface;
export type PopoverPortableProps = PortablePropsOf<PopoverInterface>;
export type PopoverPortableEvents = PortableEventsOf<PopoverInterface>;
export type PopoverPartId = PartIdsOf<PopoverInterface>;
export type PopoverEventName = EventNamesOf<PopoverInterface>;
export type PopoverTokenRole = TokenRoleNamesOf<PopoverInterface>;
export type PopoverAxis = AxisNamesOf<PopoverInterface>;

export const POPOVER_DEFAULT_PROPS: PopoverPortableProps = {
  open: null,
  defaultOpen: false,
  placement: "bottom-start",
  offset: 8,
  dismissOnOutsideInteract: true,
  initialFocus: "first-focusable",
  ariaLabel: null,
  block: false,
  disabled: false,
  surfaceWidth: "content",
  surfaceMinWidth: null,
  surfaceMaxWidth: null,
};
