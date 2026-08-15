/**
 * TextInput conformance case corpus (spec 066, g14.006). One authored
 * corpus drives Svelte, React, and GPUI. Assertions stay within fields
 * every active runtime can observe: value, selection, role/name, focus,
 * token roles, event order, affix/adornment structure, and IME commit.
 */

import {
  actionCompose,
  actionFocus,
  actionInsert,
  actionKey,
  actionPress,
  actionSelect,
  componentCase,
  expectEvents,
  expectPart,
  serializeCases,
  type PortablePropsOf,
} from "./define";
import { textInputInterface, type TextInputInterface } from "./text-input";

type I = TextInputInterface;
type FixtureProps = Partial<PortablePropsOf<I>>;

function displayCase(
  id: string,
  caption: string,
  group: string,
  props: FixtureProps,
  extra: Parameters<typeof expectPart<I>>[1] = {},
): ReturnType<typeof componentCase<I>> {
  const value = props.value ?? props.defaultValue ?? "";
  const disabled = props.disabled === true;
  return componentCase(textInputInterface, {
    id: `text-input/${id}`,
    fixture: { props, regions: {} },
    specimen: { group, caption, captureId: `text-input/${id}`, axes: ["theme"] },
    steps: [
      expectPart("root", {
        tokenRoles: {
          size: String(props.size ?? "md"),
          density: String(props.density ?? "default"),
          validation: String(props.validationState ?? "none"),
          type: String(props.type ?? "text"),
        },
        states: {
          disabled,
          prefixPresent: Boolean(props.prefix),
          suffixPresent: Boolean(props.suffix),
        },
      }),
      expectPart("control", {
        role: "textbox",
        name: props.ariaLabel ?? undefined,
        focusable: !disabled,
        value: value || "",
        ...extra,
      }),
      expectEvents([]),
    ],
  });
}

const cases = [
  displayCase("default", "Default", "Basics", {
    ariaLabel: "Name",
    placeholder: "Jane Doe",
  }),

  displayCase("controlled-value", "Controlled value", "Basics", {
    value: "hello",
    ariaLabel: "Greeting",
  }),

  displayCase(
    "disabled",
    "Disabled",
    "States",
    { value: "locked", disabled: true, ariaLabel: "Disabled field" },
  ),

  displayCase(
    "invalid",
    "Invalid",
    "States",
    { value: "nope", validationState: "invalid", ariaLabel: "Email" },
    {},
  ),

  componentCase(textInputInterface, {
    id: "text-input/invalid-indicator",
    fixture: {
      props: {
        value: "nope",
        validationState: "invalid",
        showValidationStatus: true,
        ariaLabel: "Email",
      },
      regions: {},
    },
    specimen: {
      group: "States",
      caption: "Invalid indicator",
      captureId: "text-input/invalid-indicator",
      axes: ["theme"],
    },
    steps: [
      expectPart("root", {
        tokenRoles: { validation: "invalid" },
        states: { indicatorPresent: true },
      }),
      expectPart("validationIndicator", { present: true }),
      expectEvents([]),
    ],
  }),

  componentCase(textInputInterface, {
    id: "text-input/affixes",
    fixture: {
      props: { value: "12", prefix: "$", suffix: "kg", ariaLabel: "Price" },
      regions: {},
    },
    specimen: {
      group: "Adornment",
      caption: "Prefix and suffix",
      captureId: "text-input/affixes",
      axes: ["theme"],
    },
    steps: [
      expectPart("root", { states: { prefixPresent: true, suffixPresent: true } }),
      expectPart("prefix", { present: true, text: "$" }),
      expectPart("suffix", { present: true, text: "kg" }),
      expectPart("control", { value: "12" }),
      expectEvents([]),
    ],
  }),

  componentCase(textInputInterface, {
    id: "text-input/adornments",
    fixture: {
      props: { value: "query", ariaLabel: "Search field", leadingIcon: "search", trailingIcon: "x" },
      regions: { leading: "search", trailing: "x" },
    },
    specimen: {
      group: "Adornment",
      caption: "Leading and trailing icons",
      captureId: "text-input/adornments",
      axes: ["theme"],
    },
    steps: [
      expectPart("leading", { present: true, icon: "search" }),
      expectPart("trailing", { present: true, icon: "x" }),
      expectPart("control", { value: "query" }),
      expectEvents([]),
    ],
  }),

  componentCase(textInputInterface, {
    id: "text-input/char-count",
    fixture: {
      props: { value: "abc", maxLength: 10, showCharCount: true, ariaLabel: "Limited" },
      regions: {},
    },
    specimen: {
      group: "Adornment",
      caption: "Character count",
      captureId: "text-input/char-count",
      axes: ["theme"],
    },
    steps: [
      expectPart("charCount", { present: true, text: "3/10" }),
      expectPart("control", { value: "abc" }),
      expectEvents([]),
    ],
  }),

  // Typing through the real editing path.
  componentCase(textInputInterface, {
    id: "text-input/type",
    fixture: {
      props: { value: "", ariaLabel: "Type field" },
      regions: {},
    },
    specimen: {
      group: "Behaviour",
      caption: "Type",
      captureId: "text-input/type",
      axes: [],
    },
    steps: [
      expectPart("control", { value: "" }),
      expectEvents([]),
      actionFocus("control"),
      actionInsert("control", "hello"),
      expectEvents(["valueChange"]),
      expectPart("control", { value: "hello", selectionStart: 5, selectionEnd: 5 }),
    ],
  }),

  // Controlled ownership: one valueChange per insert, host stores the value.
  componentCase(textInputInterface, {
    id: "text-input/controlled-type",
    fixture: {
      props: { value: "ab", ariaLabel: "Controlled type" },
      regions: {},
    },
    specimen: {
      group: "Behaviour",
      caption: "Controlled type",
      captureId: "text-input/controlled-type",
      axes: [],
    },
    steps: [
      expectPart("control", { value: "ab" }),
      actionFocus("control"),
      actionSelect("control", 2, 2),
      actionInsert("control", "c"),
      expectEvents(["valueChange"]),
      expectPart("control", { value: "abc" }),
    ],
  }),

  // Selection replaces on insert.
  componentCase(textInputInterface, {
    id: "text-input/selection-replace",
    fixture: {
      props: { value: "hello", ariaLabel: "Select field" },
      regions: {},
    },
    specimen: {
      group: "Behaviour",
      caption: "Selection replace",
      captureId: "text-input/selection-replace",
      axes: [],
    },
    steps: [
      actionFocus("control"),
      actionSelect("control", 0, 5),
      expectPart("control", { value: "hello", selectionStart: 0, selectionEnd: 5 }),
      actionInsert("control", "y"),
      expectEvents(["valueChange"]),
      expectPart("control", { value: "y", selectionStart: 1, selectionEnd: 1 }),
    ],
  }),

  componentCase(textInputInterface, {
    id: "text-input/disabled-inert",
    fixture: {
      props: { value: "stay", disabled: true, ariaLabel: "Inert field" },
      regions: {},
    },
    specimen: {
      group: "Behaviour",
      caption: "Disabled inert",
      captureId: "text-input/disabled-inert",
      axes: [],
    },
    steps: [
      expectPart("control", { value: "stay", focusable: false }),
      expectEvents([]),
      actionInsert("control", "x"),
      expectEvents([]),
      expectPart("control", { value: "stay" }),
    ],
  }),

  componentCase(textInputInterface, {
    id: "text-input/readonly-inert",
    fixture: {
      props: { value: "stay", readOnly: true, ariaLabel: "Read-only field" },
      regions: {},
    },
    specimen: {
      group: "Behaviour",
      caption: "Read-only inert",
      captureId: "text-input/readonly-inert",
      axes: [],
    },
    steps: [
      expectPart("control", { value: "stay", focusable: true }),
      expectEvents([]),
      actionInsert("control", "x"),
      expectEvents([]),
      expectPart("control", { value: "stay" }),
    ],
  }),

  componentCase(textInputInterface, {
    id: "text-input/focus",
    fixture: {
      props: { value: "hi", ariaLabel: "Focus field" },
      regions: {},
    },
    specimen: {
      group: "Behaviour",
      caption: "Focus",
      captureId: "text-input/focus",
      axes: [],
    },
    steps: [
      expectPart("control", { focused: false }),
      actionFocus("control"),
      expectPart("control", { focused: true }),
      expectPart("root", { states: { focused: true } }),
      expectEvents([]),
    ],
  }),

  componentCase(textInputInterface, {
    id: "text-input/submit",
    fixture: {
      props: { value: "done", ariaLabel: "Submit field" },
      regions: {},
    },
    specimen: {
      group: "Behaviour",
      caption: "Submit",
      captureId: "text-input/submit",
      axes: [],
    },
    steps: [
      actionFocus("control"),
      expectEvents([]),
      actionKey("control", "Enter"),
      expectEvents(["submit"]),
      expectPart("control", { value: "done" }),
    ],
  }),

  componentCase(textInputInterface, {
    id: "text-input/cancel",
    fixture: {
      props: { value: "draft", ariaLabel: "Cancel field" },
      regions: {},
    },
    specimen: {
      group: "Behaviour",
      caption: "Cancel",
      captureId: "text-input/cancel",
      axes: [],
    },
    steps: [
      actionFocus("control"),
      expectEvents([]),
      actionKey("control", "Escape"),
      expectEvents(["cancel"]),
    ],
  }),

  componentCase(textInputInterface, {
    id: "text-input/type-then-submit",
    fixture: {
      props: { value: "", ariaLabel: "Order field" },
      regions: {},
    },
    specimen: {
      group: "Behaviour",
      caption: "Type then submit",
      captureId: "text-input/type-then-submit",
      axes: [],
    },
    steps: [
      actionFocus("control"),
      actionInsert("control", "ok"),
      actionKey("control", "Enter"),
      expectEvents(["valueChange", "submit"]),
      expectPart("control", { value: "ok" }),
    ],
  }),

  componentCase(textInputInterface, {
    id: "text-input/search-clear",
    fixture: {
      props: {
        value: "query",
        type: "search",
        showClearButton: true,
        ariaLabel: "Search",
      },
      regions: {},
    },
    specimen: {
      group: "Behaviour",
      caption: "Search clear",
      captureId: "text-input/search-clear",
      axes: [],
    },
    steps: [
      expectPart("root", { states: { clearPresent: true }, tokenRoles: { type: "search" } }),
      expectPart("clear", { present: true, role: "button" }),
      expectEvents([]),
      actionPress("clear"),
      expectEvents(["valueChange", "clear"]),
      expectPart("control", { value: "" }),
    ],
  }),

  componentCase(textInputInterface, {
    id: "text-input/ime-commit",
    fixture: {
      props: { value: "", ariaLabel: "IME field" },
      regions: {},
    },
    specimen: {
      group: "Behaviour",
      caption: "IME commit",
      captureId: "text-input/ime-commit",
      axes: [],
    },
    steps: [
      actionFocus("control"),
      expectEvents([]),
      actionCompose("control", "ñ", "start"),
      expectEvents([]),
      expectPart("control", { value: "" }),
      actionCompose("control", "ñ", "update"),
      expectEvents([]),
      expectPart("control", { value: "" }),
      actionCompose("control", "ñ", "commit"),
      expectEvents(["valueChange"]),
      expectPart("control", { value: "ñ" }),
    ],
  }),
];

export const textInputCases = serializeCases("text-input", cases);
