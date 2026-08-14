/** Tabs collection/navigation corpus (spec 066, g14.004). */

import {
  actionFocus,
  actionKey,
  actionPress,
  componentCase,
  expectEvents,
  expectPart,
  serializeCases,
  type PortablePropsOf,
} from "./define";
import { tabsInterface, type TabsInterface, type TabsPortableItem } from "./tabs";

type I = TabsInterface;
type FixtureProps = Partial<PortablePropsOf<I>>;

const items: TabsPortableItem[] = [
  { value: "overview", label: "Overview" },
  { value: "settings", label: "Settings", disabled: true },
  { value: "billing", label: "Billing" },
];

function fixture(props: FixtureProps = {}) {
  return {
    props: {
      items,
      value: "overview",
      ariaLabel: "Account sections",
      ...props,
    },
    regions: { panel: "Selected section" },
  };
}

const defaultAnatomy = [
  expectPart<I>("root", {
    tokenRoles: { variant: "card", orientation: "horizontal", size: "sm", density: "default" },
  }),
  expectPart<I>("list", { role: "tablist", name: "Account sections", orientation: "horizontal" }),
  expectPart<I>("trigger:overview", {
    role: "tab",
    name: "Overview",
    selected: true,
    focusable: true,
    tabbable: true,
    controls: "panel:overview",
    geometry: { paddingLeft: 12, paddingRight: 12, tolerance: 1 },
  }),
  expectPart<I>("trigger:settings", {
    role: "tab",
    name: "Settings",
    selected: false,
    focusable: false,
    tabbable: false,
    controls: "panel:settings",
  }),
  expectPart<I>("trigger:billing", {
    role: "tab",
    name: "Billing",
    selected: false,
    focusable: true,
    tabbable: false,
    controls: "panel:billing",
  }),
  expectPart<I>("panel:overview", {
    role: "tabpanel",
    labelledBy: "trigger:overview",
    focusable: true,
  }),
  expectEvents<I>([]),
] as const;

const cases = [
  componentCase(tabsInterface, {
    id: "tabs/default-anatomy",
    fixture: fixture(),
    specimen: { group: "Basics", caption: "Identified tabs", captureId: "tabs/default-anatomy", axes: ["size", "theme"] },
    steps: defaultAnatomy,
  }),
  componentCase(tabsInterface, {
    id: "tabs/reordered-identity",
    fixture: fixture({ items: [items[2]!, items[1]!, items[0]!] }),
    specimen: { group: "Identity", caption: "Reordered fixture", captureId: "tabs/reordered-identity", axes: [] },
    steps: [
      expectPart("trigger:overview", { name: "Overview", selected: true, controls: "panel:overview" }),
      expectPart("trigger:billing", { name: "Billing", selected: false, controls: "panel:billing" }),
      expectPart("panel:overview", { labelledBy: "trigger:overview" }),
      expectEvents([]),
    ],
  }),
  componentCase(tabsInterface, {
    id: "tabs/automatic-next-skip-disabled",
    fixture: fixture(),
    specimen: { group: "Navigation", caption: "Automatic next", captureId: "tabs/automatic-next-skip-disabled", axes: [] },
    steps: [
      actionFocus("trigger:overview"),
      expectEvents([]),
      actionKey("trigger:overview", "ArrowRight"),
      expectEvents(["valueChange"]),
      expectPart("trigger:billing", { selected: true, focused: true, tabbable: true, controls: "panel:billing" }),
      expectPart("trigger:overview", { selected: false, tabbable: false }),
      expectPart("panel:billing", { labelledBy: "trigger:billing" }),
    ],
  }),
  componentCase(tabsInterface, {
    id: "tabs/automatic-previous-wrap",
    fixture: fixture(),
    specimen: { group: "Navigation", caption: "Previous wraps", captureId: "tabs/automatic-previous-wrap", axes: [] },
    steps: [
      actionFocus("trigger:overview"),
      actionKey("trigger:overview", "ArrowLeft"),
      expectEvents(["valueChange"]),
      expectPart("trigger:billing", { selected: true, focused: true }),
    ],
  }),
  componentCase(tabsInterface, {
    id: "tabs/home-end",
    fixture: fixture({ value: "billing" }),
    specimen: { group: "Navigation", caption: "Home and End", captureId: "tabs/home-end", axes: [] },
    steps: [
      actionFocus("trigger:billing"),
      actionKey("trigger:billing", "Home"),
      expectEvents(["valueChange"]),
      expectPart("trigger:overview", { selected: true, focused: true }),
      actionKey("trigger:overview", "End"),
      expectEvents(["valueChange", "valueChange"]),
      expectPart("trigger:billing", { selected: true, focused: true }),
    ],
  }),
  componentCase(tabsInterface, {
    id: "tabs/vertical-direction",
    fixture: fixture({ orientation: "vertical" }),
    specimen: { group: "Orientation", caption: "Vertical navigation", captureId: "tabs/vertical-direction", axes: ["theme"] },
    steps: [
      expectPart("list", { orientation: "vertical" }),
      actionFocus("trigger:overview"),
      actionKey("trigger:overview", "ArrowDown"),
      expectEvents(["valueChange"]),
      expectPart("trigger:billing", { selected: true, focused: true }),
      actionKey("trigger:billing", "ArrowUp"),
      expectEvents(["valueChange", "valueChange"]),
      expectPart("trigger:overview", { selected: true, focused: true }),
    ],
  }),
  componentCase(tabsInterface, {
    id: "tabs/manual-activation",
    fixture: fixture({ activationMode: "manual" }),
    specimen: { group: "Activation", caption: "Manual activation", captureId: "tabs/manual-activation", axes: [] },
    steps: [
      actionFocus("trigger:overview"),
      actionKey("trigger:overview", "ArrowRight"),
      expectEvents([]),
      expectPart("trigger:billing", { selected: false, focused: true, tabbable: true }),
      expectPart("trigger:overview", { selected: true }),
      actionKey("trigger:billing", "Enter"),
      expectEvents(["valueChange"]),
      expectPart("trigger:billing", { selected: true, focused: true }),
      expectPart("panel:billing", { labelledBy: "trigger:billing" }),
    ],
  }),
  componentCase(tabsInterface, {
    id: "tabs/disabled-inert",
    fixture: fixture(),
    specimen: { group: "States", caption: "Disabled item", captureId: "tabs/disabled-inert", axes: [] },
    steps: [
      actionPress("trigger:settings", "keyboard"),
      expectEvents([]),
      expectPart("trigger:overview", { selected: true }),
      expectPart("trigger:settings", { selected: false, focusable: false, tabbable: false }),
    ],
  }),
  componentCase(tabsInterface, {
    id: "tabs/controlled-press-order",
    fixture: fixture({ variant: "pill", bordered: true }),
    specimen: { group: "Activation", caption: "Controlled press", captureId: "tabs/controlled-press-order", axes: ["density", "theme"] },
    steps: [
      actionPress("trigger:billing", "keyboard"),
      expectEvents(["valueChange"]),
      expectPart("root", { tokenRoles: { variant: "pill", orientation: "horizontal", size: "sm", density: "default" } }),
      expectPart("trigger:billing", { selected: true, controls: "panel:billing" }),
      expectPart("panel:billing", { labelledBy: "trigger:billing" }),
    ],
  }),
];

export const tabsCases = serializeCases("tabs", cases);
