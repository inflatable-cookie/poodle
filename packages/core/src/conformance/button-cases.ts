/**
 * Button conformance case corpus (spec 066, g14.001). One authored corpus
 * drives the three active runtimes (Svelte, React, GPUI): tests and specimen
 * projections. `componentCase(buttonInterface, ...)` binds every fixture
 * prop, region, part, state, event, and axis to the interface at authoring
 * time; the serialized form is validated again by the serializer and the
 * Rust codegen. Unknown names are errors, never ignored.
 *
 * Assertions are restricted to what every active runtime can observe for
 * real: geometry on non-icon cases (web reads computed style from the real
 * CSS, native reads node style), icon *presence* (names are recorded, not
 * asserted — the web DOM does not carry icon names), states, token roles,
 * and event order.
 */

import {
  actionFocus,
  actionPress,
  componentCase,
  expectEvents,
  expectPart,
  serializeCases,
  type CaseStep,
  type GeometryExpectation,
  type PortablePropsOf,
  type RegionNamesOf,
} from "./define";
import { buttonInterface } from "./button";
import type { ButtonInterface } from "./button";

type I = ButtonInterface;
type FixtureProps = Partial<PortablePropsOf<I>>;
type FixtureRegions = Partial<Record<RegionNamesOf<I>, string>>;

const DEFAULT_GEOMETRY = {
  // borderWidth is excluded: web sets it through the `border` shorthand,
  // which happy-dom does not decompose into a longhand. The other five
  // fields resolve on both sides.
  height: 36,
  minWidth: 80,
  paddingLeft: 12,
  paddingRight: 12,
  radius: 6,
  tolerance: 1,
} as const;

/** A display case: id, label text, fixture, specimen placement. */
function displayCase(
  id: string,
  caption: string,
  group: string,
  label: string,
  props: FixtureProps,
  axes: readonly ("size" | "density" | "theme")[] = ["theme", "density", "size"],
  geometry: GeometryExpectation | null = DEFAULT_GEOMETRY,
): ReturnType<typeof componentCase<I>> {
  const disabled = props.disabled === true || props.loading === true;
  const steps: CaseStep<I>[] = [
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
  return componentCase(buttonInterface, {
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
): ReturnType<typeof componentCase<I>> {
  const label = "Save";
  const regions: FixtureRegions = { label };
  if (leading) regions.leading = leading;
  if (trailing) regions.trailing = trailing;
  return componentCase(buttonInterface, {
    id: `button/${id}`,
    fixture: { props: chevron ? { chevron: true } : {}, regions },
    specimen: { group: "Icons", caption, captureId: `button/${id}`, axes: ["size"] },
    steps: [
      expectPart("root", {
        role: "button",
        name: label,
        focusable: true,
        states: { disabled: false },
      }),
      expectPart("label", { present: true, text: label }),
      expectPart("leadingIcon", { present: Boolean(leading), icon: leading ?? undefined }),
      expectPart("trailingIcon", { present: Boolean(trailing), icon: trailing ?? undefined }),
      expectPart("chevron", { present: chevron, icon: chevron ? "chevron-down" : undefined }),
      expectEvents([]),
    ],
  });
}

const cases = [
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
  // The loading case also pins the spinner part's icon identity.
  componentCase(buttonInterface, {
    id: "button/loading",
    fixture: { props: { loading: true }, regions: { label: "Save" } },
    specimen: { group: "States", caption: "Loading", captureId: "button/loading", axes: ["theme"] },
    steps: [
      expectPart("root", {
        role: "button",
        name: "Save",
        focusable: false,
        states: { disabled: true, loading: true },
        tokenRoles: {
          variant: "secondary",
          tone: "default",
          size: "md",
          density: "default",
        },
      }),
      expectPart("label", { present: true, text: "Save" }),
      expectPart("spinner", { present: true, icon: "spinner" }),
      expectEvents([]),
    ],
  }),

  // Leading/trailing icon regions (presence asserted; names recorded).
  iconCase("leading-icon", "Leading", "plus", null),
  iconCase("trailing-icon", "Trailing", null, "check"),
  iconCase("both-icons", "Both", "plus", "check"),
  iconCase("chevron", "Chevron", null, null, true),

  // Size and density axes (display projection covers the enum values).
  displayCase("size-lg", "Large", "Size", "Run", { size: "lg" }, ["density", "theme"], null),
  displayCase("density-compact", "Compact", "Density", "Run", { density: "compact" }, ["size", "theme"], null),

  // Behaviour: press by pointer.
  componentCase(buttonInterface, {
    id: "button/press-pointer",
    fixture: { props: {}, regions: { label: "Press" } },
    specimen: { group: "Behaviour", caption: "Press by pointer", captureId: "button/press-pointer", axes: [] },
    steps: [
      expectPart("root", { role: "button", name: "Press", focusable: true, states: { disabled: false } }),
      expectEvents([]),
      actionPress("root", "pointer"),
      expectEvents(["press"]),
    ],
  }),

  // Behaviour: press by keyboard.
  componentCase(buttonInterface, {
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
  componentCase(buttonInterface, {
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

  // Behaviour: explicit defaultPressed=false enters toggle mode. Absence
  // and explicit false must not collapse — this case pins the
  // cross-language nullable shape (web `null !== false`, Rust
  // `None != Some(false)`).
  componentCase(buttonInterface, {
    id: "button/default-pressed-toggle",
    fixture: { props: { defaultPressed: false }, regions: { label: "Mute" } },
    specimen: { group: "Behaviour", caption: "Default-pressed toggle", captureId: "button/default-pressed-toggle", axes: [] },
    steps: [
      expectPart("root", { role: "button", name: "Mute", states: { pressed: false } }),
      expectEvents([]),
      actionPress("root", "pointer"),
      expectEvents(["pressedChange", "press"]),
      expectPart("root", { states: { pressed: true } }),
    ],
  }),

  // Focus-visible state.
  componentCase(buttonInterface, {
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
