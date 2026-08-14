/**
 * Button conformance case corpus (spec 066, g14.001). One authored corpus
 * drives all four runtimes: tests and specimen projections. The serialized
 * form (`button-cases.json`) is consumed by the Rust runners and native
 * specimen pages.
 *
 * Required coverage from g14.001:
 *   - default labelled button
 *   - every contract variant and tone
 *   - disabled and loading
 *   - leading/trailing icon regions
 *   - press by pointer and keyboard
 *   - focus-visible state
 *   - theme, density, and control-size specimen axes
 */

import {
  actionFocus,
  actionPress,
  componentCase,
  expectEvents,
  expectPart,
  serializeCases,
  type ComponentCase,
} from "./define";

const DEFAULT_GEOMETRY = {
  height: 36,
  minWidth: 80,
  paddingLeft: 12,
  paddingRight: 12,
  radius: 6,
  borderWidth: 1,
  tolerance: 1,
} as const;

/** A display case: id, label text, fixture, specimen placement. */
function displayCase(
  id: string,
  caption: string,
  group: string,
  label: string,
  props: Record<string, boolean | string | number | null>,
  axes: readonly string[] = ["theme", "density", "size"],
  geometry: object | null = DEFAULT_GEOMETRY,
): ComponentCase {
  const disabled = props.disabled === true || props.loading === true;
  const steps: ComponentCase["steps"] = [
    expectPart("root", {
      role: "button",
      name: label,
      focusable: !disabled,
      states: { disabled, loading: props.loading === true },
      tokenRoles: {
        variant: String(props.variant ?? "secondary"),
        tone: String(props.tone ?? "default"),
        size: String(props.size ?? "md"),
        density: String(props.density ?? "default"),
      },
      ...(geometry ? { geometry } : {}),
    }),
    expectPart("label", { present: true, text: label }),
    expectEvents([]),
  ];
  return componentCase({
    id: `button/${id}`,
    fixture: { props, regions: { label } },
    specimen: { group, caption, captureId: `button/${id}`, axes },
    steps,
  });
}

function iconCase(
  id: string,
  caption: string,
  leading: string | null,
  trailing: string | null,
  chevron = false,
): ComponentCase {
  const label = "Save";
  const regions: Record<string, string> = { label };
  if (leading) regions.leading = leading;
  if (trailing) regions.trailing = trailing;
  return componentCase({
    id: `button/${id}`,
    fixture: { props: { chevron: chevron || null }, regions },
    specimen: { group: "Icons", caption, captureId: `button/${id}`, axes: ["size"] },
    steps: [
      expectPart("root", { role: "button", name: label, focusable: true, states: { disabled: false } }),
      expectPart("label", { present: true, text: label }),
      expectPart("leadingIcon", { present: Boolean(leading), icon: leading ?? undefined }),
      expectPart("trailingIcon", { present: Boolean(trailing), icon: trailing ?? undefined }),
      expectPart("chevron", { present: chevron, icon: chevron ? "chevron-down" : undefined }),
      expectEvents([]),
    ],
  });
}

const cases: ComponentCase[] = [
  displayCase("default", "Default", "Basics", "Run", {}),

  // Every contract variant.
  displayCase("primary", "Primary", "Variants", "Run", { variant: "primary" }),
  displayCase("secondary", "Secondary", "Variants", "Run", { variant: "secondary" }),
  displayCase("ghost", "Ghost", "Variants", "Run", { variant: "ghost" }),

  // Every contract tone (secondary is the default variant).
  displayCase("tone-danger", "Danger", "Tones", "Delete", { tone: "danger" }),
  displayCase("tone-success", "Success", "Tones", "Save", { tone: "success" }),
  displayCase("tone-warning", "Warning", "Tones", "Sync", { tone: "warning" }),

  // Disabled and loading. Loading shows a spinner in the leading slot, which
  // shrinks the left padding — its geometry is spinner-dependent, so only the
  // disabled case asserts bounds.
  displayCase("disabled", "Disabled", "States", "Run", { disabled: true }, ["theme"]),
  displayCase("loading", "Loading", "States", "Save", { loading: true }, ["theme"], null),

  // Leading/trailing icon regions.
  iconCase("leading-icon", "Leading", "plus", null),
  iconCase("trailing-icon", "Trailing", null, "check"),
  iconCase("both-icons", "Both", "plus", "check"),
  iconCase("chevron", "Chevron", null, null, true),

  // Size and density axes (display projection covers the enum values).
  displayCase("size-lg", "Large", "Size", "Run", { size: "lg" }, ["density", "theme"], null),
  displayCase("density-compact", "Compact", "Density", "Run", { density: "compact" }, ["size", "theme"], null),

  // Behaviour: press by pointer.
  componentCase({
    id: "button/press-pointer",
    fixture: { props: {}, regions: { label: "Press" } },
    specimen: { group: "Behaviour", caption: "Press by pointer", captureId: "button/press-pointer", axes: [] },
    steps: [
      expectPart("root", { role: "button", name: "Press", focusable: true, states: { disabled: false } }),
      expectEvents([]),
      { kind: "action", name: "press", part: "root", input: "pointer" },
      expectEvents(["press"]),
    ],
  }),

  // Behaviour: press by keyboard.
  componentCase({
    id: "button/press-keyboard",
    fixture: { props: {}, regions: { label: "Press" } },
    specimen: { group: "Behaviour", caption: "Press by keyboard", captureId: "button/press-keyboard", axes: [] },
    steps: [
      actionFocus("root"),
      expectPart("root", { states: { focused: true } }),
      expectEvents([]),
      actionPress("root", "keyboard"),
      expectEvents(["press"]),
    ],
  }),

  // Behaviour: controlled toggle press.
  componentCase({
    id: "button/toggle",
    fixture: { props: { pressed: false }, regions: { label: "Mute" } },
    specimen: { group: "Behaviour", caption: "Controlled toggle", captureId: "button/toggle", axes: [] },
    steps: [
      expectPart("root", { role: "button", name: "Mute", states: { pressed: false } }),
      expectEvents([]),
      actionPress("root", "pointer"),
      expectEvents(["pressedChange", "press"]),
      expectPart("root", { states: { pressed: true } }),
    ],
  }),

  // Focus-visible state.
  componentCase({
    id: "button/focus-visible",
    fixture: { props: {}, regions: { label: "Focus" } },
    specimen: { group: "Behaviour", caption: "Focus-visible", captureId: "button/focus-visible", axes: [] },
    steps: [
      actionFocus("root"),
      expectPart("root", { role: "button", name: "Focus", states: { focused: true, focusVisible: true } }),
      expectEvents([]),
    ],
  }),
];

export const buttonCases = serializeCases("button", cases);

export function buttonCaseList(): readonly ComponentCase[] {
  return cases;
}
