/**
 * Popover overlay/focus corpus (spec 066, g14.005).
 *
 * Covers the frozen contract claims with the smallest non-overlapping case
 * set: closed default, uncontrolled pointer/keyboard open, controlled
 * ownership, disabled inertness, the three initialFocus strategies, every
 * close path (trigger, Escape, outside, controlled host) with trigger focus
 * restoration, the outside guard, the nested dismiss-stack contract, the
 * placement families and start/end rule, offset and trigger-width bounds,
 * and semantics/token evidence.
 *
 * Fixture conventions (hosts interpret, generic code never reads):
 * - the `children` region may contain `<button>label</button>` markup so the
 *   focus-first strategy has a real focusable target in the content;
 * - `host.nested` declares a second popover instance the host composes inside
 *   the content (the nested dismiss-stack proof);
 * - web hosts give the trigger a fixed anchor box so placement resolves
 *   without a layout engine; the shared web layout stub supplies the boxes.
 */

import {
  actionDismiss,
  actionKey,
  actionPointer,
  actionPress,
  componentCase,
  expectEvents,
  expectPart,
  serializeCases,
  type PortablePropsOf,
} from "./define";
import { popoverInterface, type PopoverInterface } from "./popover";

type I = PopoverInterface;
type FixtureProps = Partial<PortablePropsOf<I>>;

function fixture(
  props: FixtureProps = {},
  regions: Record<string, string> = {},
  host?: Record<string, unknown>,
) {
  return {
    props: { ariaLabel: "Quick settings", ...props },
    regions: { trigger: "Open popover", children: "Quick settings panel", ...regions },
    ...(host ? { host } : {}),
  };
}

const openPath = (props: FixtureProps = {}) => [
  actionPress<I>("trigger", "pointer"),
  expectEvents<I>(["openChange"]),
  expectPart<I>("root", { states: { open: true } }),
  expectPart<I>("trigger", { expanded: true, controls: "surface", parent: "root" }),
  expectPart<I>("surface", { present: true, role: "dialog", name: "Quick settings", overlay: true }),
] as const;

const cases = [
  componentCase(popoverInterface, {
    id: "popover/closed-default",
    fixture: fixture(),
    specimen: { group: "Basics", caption: "Closed default", captureId: "popover/closed-default", axes: ["theme"] },
    steps: [
      expectPart<I>("root", {
        states: { open: false },
        tokenRoles: { placement: "bottom-start", surfaceWidth: "content" },
        layerCount: 0,
      }),
      expectPart<I>("trigger", {
        role: "button",
        name: "Open popover",
        focusable: true,
        tabbable: true,
        focused: false,
        expanded: false,
        parent: "root",
      }),
      expectPart<I>("surface", { present: false }),
      expectEvents<I>([]),
    ],
  }),
  componentCase(popoverInterface, {
    id: "popover/uncontrolled-open-pointer",
    fixture: fixture(),
    specimen: { group: "Basics", caption: "Pointer opens", captureId: "popover/uncontrolled-open-pointer", axes: ["theme"] },
    steps: [
      ...openPath(),
      expectPart<I>("root", { layerCount: 1 }),
      expectPart<I>("surface", {
        geometry: { topGap: 8, hStart: 0, tolerance: 1 },
      }),
    ],
  }),
  componentCase(popoverInterface, {
    id: "popover/uncontrolled-open-keyboard",
    fixture: fixture(),
    specimen: { group: "Basics", caption: "Keyboard opens", captureId: "popover/uncontrolled-open-keyboard", axes: [] },
    steps: [
      actionPress<I>("trigger", "keyboard"),
      expectEvents<I>(["openChange"]),
      expectPart<I>("root", { states: { open: true }, layerCount: 1 }),
      expectPart<I>("surface", { present: true, role: "dialog", name: "Quick settings" }),
    ],
  }),
  componentCase(popoverInterface, {
    id: "popover/controlled-ownership",
    fixture: fixture({ open: false }),
    specimen: { group: "Ownership", caption: "Controlled host owns open", captureId: "popover/controlled-ownership", axes: [] },
    steps: [
      expectPart<I>("surface", { present: false }),
      actionPress<I>("trigger", "pointer"),
      expectEvents<I>(["openChange"]),
      expectPart<I>("surface", { present: true }),
      actionPress<I>("trigger", "pointer"),
      expectEvents<I>(["openChange", "openChange"]),
      expectPart<I>("surface", { present: false }),
      expectPart<I>("trigger", { focused: true }),
    ],
  }),
  componentCase(popoverInterface, {
    id: "popover/disabled-inert",
    fixture: fixture({ disabled: true, open: true }),
    specimen: { group: "States", caption: "Disabled stays inert", captureId: "popover/disabled-inert", axes: [] },
    steps: [
      // Programmatic open-direction: the controlled host requests open while
      // disabled — every direction stays inert and the surface never mounts.
      expectPart<I>("root", { states: { open: false } }),
      expectPart<I>("surface", { present: false }),
      expectPart<I>("trigger", { focusable: false, tabbable: false }),
      actionPress<I>("trigger", "pointer"),
      expectEvents<I>([]),
      expectPart<I>("surface", { present: false }),
      actionPress<I>("trigger", "keyboard"),
      expectEvents<I>([]),
      expectPart<I>("surface", { present: false }),
      actionKey<I>("trigger", "Space"),
      expectEvents<I>([]),
      expectPart<I>("surface", { present: false }),
    ],
  }),
  componentCase(popoverInterface, {
    id: "popover/focus-first",
    fixture: fixture({}, { children: "<button>First option</button><button>Second option</button>" }),
    specimen: { group: "Focus", caption: "First focusable", captureId: "popover/focus-first", axes: [] },
    steps: [
      actionPress<I>("trigger", "pointer"),
      expectEvents<I>(["openChange"]),
      expectPart<I>("root", { focusedText: "First option" }),
      expectPart<I>("trigger", { focused: false }),
      expectPart<I>("surface", { focused: false }),
    ],
  }),
  componentCase(popoverInterface, {
    id: "popover/focus-content",
    fixture: fixture({ initialFocus: "content" }),
    specimen: { group: "Focus", caption: "Surface focus", captureId: "popover/focus-content", axes: [] },
    steps: [
      actionPress<I>("trigger", "pointer"),
      expectEvents<I>(["openChange"]),
      expectPart<I>("surface", { focused: true, focusable: true, tabbable: true }),
      expectPart<I>("root", { focusedText: "Quick settings panel" }),
    ],
  }),
  componentCase(popoverInterface, {
    id: "popover/focus-none",
    fixture: fixture({ initialFocus: "none" }),
    specimen: { group: "Focus", caption: "Focus untouched", captureId: "popover/focus-none", axes: [] },
    steps: [
      actionPress<I>("trigger", "pointer"),
      expectEvents<I>(["openChange"]),
      expectPart<I>("trigger", { focused: true }),
      expectPart<I>("surface", { focused: false }),
    ],
  }),
  componentCase(popoverInterface, {
    id: "popover/explicit-close",
    fixture: fixture(),
    specimen: { group: "Dismissal", caption: "Trigger close restores focus", captureId: "popover/explicit-close", axes: [] },
    steps: [
      actionPress<I>("trigger", "pointer"),
      expectEvents<I>(["openChange"]),
      expectPart<I>("surface", { present: true }),
      actionPress<I>("trigger", "pointer"),
      expectEvents<I>(["openChange", "openChange"]),
      expectPart<I>("surface", { present: false }),
      expectPart<I>("trigger", { focused: true }),
    ],
  }),
  componentCase(popoverInterface, {
    id: "popover/escape",
    fixture: fixture(),
    specimen: { group: "Dismissal", caption: "Escape closes", captureId: "popover/escape", axes: [] },
    steps: [
      actionPress<I>("trigger", "pointer"),
      expectEvents<I>(["openChange"]),
      actionDismiss<I>("root"),
      expectEvents<I>(["openChange", "openChange"]),
      expectPart<I>("surface", { present: false }),
      expectPart<I>("trigger", { focused: true }),
    ],
  }),
  componentCase(popoverInterface, {
    id: "popover/outside-interaction",
    fixture: fixture(),
    specimen: { group: "Dismissal", caption: "Outside pointer closes", captureId: "popover/outside-interaction", axes: [] },
    steps: [
      actionPress<I>("trigger", "pointer"),
      expectEvents<I>(["openChange"]),
      actionPointer<I>("root", "outside"),
      expectEvents<I>(["openChange", "openChange"]),
      expectPart<I>("surface", { present: false }),
      expectPart<I>("trigger", { focused: true }),
    ],
  }),
  componentCase(popoverInterface, {
    id: "popover/inside-interaction-stays",
    fixture: fixture(),
    specimen: { group: "Dismissal", caption: "Inside pointer keeps open", captureId: "popover/inside-interaction-stays", axes: [] },
    steps: [
      actionPress<I>("trigger", "pointer"),
      expectEvents<I>(["openChange"]),
      actionPointer<I>("surface", "inside"),
      expectEvents<I>(["openChange"]),
      expectPart<I>("surface", { present: true }),
      expectPart<I>("root", { layerCount: 1 }),
    ],
  }),
  componentCase(popoverInterface, {
    id: "popover/outside-guard",
    fixture: fixture({ dismissOnOutsideInteract: false }),
    specimen: { group: "Dismissal", caption: "Outside guard holds, Escape still closes", captureId: "popover/outside-guard", axes: [] },
    steps: [
      actionPress<I>("trigger", "pointer"),
      expectEvents<I>(["openChange"]),
      actionPointer<I>("root", "outside"),
      expectEvents<I>(["openChange"]),
      expectPart<I>("surface", { present: true }),
      actionDismiss<I>("root"),
      expectEvents<I>(["openChange", "openChange"]),
      expectPart<I>("surface", { present: false }),
    ],
  }),
  componentCase(popoverInterface, {
    id: "popover/nested-escape",
    fixture: fixture({}, {}, { nested: { trigger: "Nested trigger", children: "Nested panel" } }),
    specimen: { group: "Layers", caption: "Escape unwinds innermost first", captureId: "popover/nested-escape", axes: [] },
    steps: [
      expectPart<I>("root", { layerCount: 0 }),
      expectPart<I>("surface", { present: false }),
      actionPress<I>("trigger", "pointer"),
      expectEvents<I>(["openChange"]),
      expectPart<I>("root", { layerCount: 2 }),
      expectPart<I>("surface", { present: true }),
      actionDismiss<I>("root"),
      expectEvents<I>(["openChange", "openChange"]),
      expectPart<I>("root", { layerCount: 1 }),
      expectPart<I>("surface", { present: true }),
      actionDismiss<I>("root"),
      expectEvents<I>(["openChange", "openChange", "openChange"]),
      expectPart<I>("root", { layerCount: 0 }),
      expectPart<I>("surface", { present: false }),
      expectPart<I>("trigger", { focused: true }),
    ],
  }),
  componentCase(popoverInterface, {
    id: "popover/nested-outside-ancestry",
    fixture: fixture({}, {}, { nested: { trigger: "Nested trigger", children: "Nested panel" } }),
    specimen: { group: "Layers", caption: "Inside outer spares ancestry", captureId: "popover/nested-outside-ancestry", axes: [] },
    steps: [
      actionPress<I>("trigger", "pointer"),
      expectEvents<I>(["openChange"]),
      expectPart<I>("root", { layerCount: 2 }),
      actionPointer<I>("surface", "inside"),
      expectEvents<I>(["openChange", "openChange"]),
      expectPart<I>("root", { layerCount: 1 }),
      expectPart<I>("surface", { present: true }),
      actionDismiss<I>("root"),
      expectEvents<I>(["openChange", "openChange", "openChange"]),
      expectPart<I>("root", { layerCount: 0 }),
      expectPart<I>("surface", { present: false }),
    ],
  }),
  componentCase(popoverInterface, {
    id: "popover/placement-top",
    fixture: fixture({ placement: "top-start" }),
    specimen: { group: "Placement", caption: "Top family", captureId: "popover/placement-top", axes: ["theme"] },
    steps: [
      ...openPath(),
      expectPart<I>("root", { tokenRoles: { placement: "top-start" } }),
      expectPart<I>("surface", { geometry: { bottomGap: 8, hStart: 0, tolerance: 1 } }),
    ],
  }),
  componentCase(popoverInterface, {
    id: "popover/placement-right",
    fixture: fixture({ placement: "right-start" }),
    specimen: { group: "Placement", caption: "Right family", captureId: "popover/placement-right", axes: [] },
    steps: [
      ...openPath(),
      expectPart<I>("surface", { geometry: { leftGap: 8, vStart: 0, tolerance: 1 } }),
    ],
  }),
  componentCase(popoverInterface, {
    id: "popover/placement-left",
    fixture: fixture({ placement: "left-start" }),
    specimen: { group: "Placement", caption: "Left family", captureId: "popover/placement-left", axes: [] },
    steps: [
      ...openPath(),
      expectPart<I>("surface", { geometry: { rightGap: 8, vStart: 0, tolerance: 1 } }),
    ],
  }),
  componentCase(popoverInterface, {
    id: "popover/placement-end-rule",
    fixture: fixture({ placement: "bottom-end" }),
    specimen: { group: "Placement", caption: "End alignment", captureId: "popover/placement-end-rule", axes: [] },
    steps: [
      ...openPath(),
      expectPart<I>("surface", { geometry: { topGap: 8, hEnd: 0, tolerance: 1 } }),
    ],
  }),
  componentCase(popoverInterface, {
    id: "popover/offset",
    fixture: fixture({ offset: 12 }),
    specimen: { group: "Placement", caption: "Offset gap", captureId: "popover/offset", axes: [] },
    steps: [
      ...openPath(),
      expectPart<I>("surface", { geometry: { topGap: 12, tolerance: 1 } }),
    ],
  }),
  componentCase(popoverInterface, {
    id: "popover/surface-width-trigger",
    fixture: fixture({ surfaceWidth: "trigger" }),
    specimen: { group: "Placement", caption: "Trigger-width surface", captureId: "popover/surface-width-trigger", axes: [] },
    steps: [
      ...openPath(),
      expectPart<I>("root", { tokenRoles: { surfaceWidth: "trigger" } }),
      expectPart<I>("surface", { geometry: { widthGap: 0, tolerance: 1 } }),
    ],
  }),
  componentCase(popoverInterface, {
    id: "popover/semantics-tokens",
    fixture: fixture({ placement: "bottom-end" }),
    specimen: { group: "Semantics", caption: "Dialog role, name, and tokens", captureId: "popover/semantics-tokens", axes: [] },
    steps: [
      actionPress<I>("trigger", "pointer"),
      expectEvents<I>(["openChange"]),
      expectPart<I>("root", {
        states: { open: true },
        layerCount: 1,
        tokenRoles: { placement: "bottom-end", surfaceWidth: "content" },
      }),
      expectPart<I>("trigger", { role: "button", expanded: true, controls: "surface", parent: "root" }),
      expectPart<I>("surface", { role: "dialog", name: "Quick settings", overlay: true }),
    ],
  }),
];

export const popoverCases = serializeCases("popover", cases);
