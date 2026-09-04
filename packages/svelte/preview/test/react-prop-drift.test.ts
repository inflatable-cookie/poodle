import { describe, expect, it } from "vitest";

import {
  canonicalizePropName,
  compareComponentProps,
  parsePropsFromBraceBody,
  parseReactPropsFromSource,
  parseSvelteProps,
  validateBaseline,
  type BaselineEntry,
} from "../scripts/react-prop-drift.ts";

describe("react-prop-drift review oracle invariants", () => {
  it("fails when a React-only prop is planted that the contract does not document", () => {
    // Invariant: React-only prop fails
    // Counterexample: plant defaultValue?: number on a React shell whose contract lacks it
    const finding = compareComponentProps(
      "test-component",
      "TestComponent",
      new Set(["value", "min", "max"]), // Svelte props
      new Set(["value", "min", "max", "defaultValue"]), // React props (planted defaultValue)
      new Set(), // snippets
      new Set(["value", "min", "max"]), // contract props (lacks defaultValue)
    );

    expect(finding).not.toBeNull();
    expect(finding!.slug).toBe("test-component");
    expect(finding!.reactOnly).toEqual(["defaultValue"]);
    expect(finding!.svelteOnly).toEqual([]);
  });

  it("fails when a documented Svelte prop is missing from the React shell", () => {
    // Invariant: Missing React port fails
    // Counterexample: remove one documented Svelte prop (e.g. formenctype) from React
    const finding = compareComponentProps(
      "button",
      "Button",
      new Set(["variant", "tone", "formenctype"]), // Svelte props
      new Set(["variant", "tone"]), // React props (missing formenctype)
      new Set(), // snippets
      new Set(["variant", "tone", "formenctype"]), // contract props
    );

    expect(finding).not.toBeNull();
    expect(finding!.slug).toBe("button");
    expect(finding!.svelteOnly).toEqual(["formenctype"]);
    expect(finding!.reactOnly).toEqual([]);
  });

  it("treats DOM attribute casing differences as the same prop, not drift", () => {
    // Invariant: Attribute casing is not drift
    // Svelte lowercase (autocomplete, spellcheck, autocapitalize, autocorrect, formaction)
    // vs React camelCase (autoComplete, spellCheck, autoCapitalize, autoCorrect, formAction)
    const svelteAttrs = [
      "autocomplete",
      "spellcheck",
      "autocapitalize",
      "autocorrect",
      "formaction",
      "formnovalidate",
      "formtarget",
      "tabindex",
      "readonly",
    ];
    const reactAttrs = [
      "autoComplete",
      "spellCheck",
      "autoCapitalize",
      "autoCorrect",
      "formAction",
      "formNoValidate",
      "formTarget",
      "tabIndex",
      "readOnly",
    ];

    const finding = compareComponentProps(
      "text-input",
      "TextInput",
      new Set(svelteAttrs),
      new Set(reactAttrs),
      new Set(),
      new Set(svelteAttrs),
    );

    expect(finding).toBeNull();

    for (let i = 0; i < svelteAttrs.length; i++) {
      expect(canonicalizePropName(svelteAttrs[i])).toBe(
        canonicalizePropName(reactAttrs[i]),
      );
    }
  });

  it("treats class and className as the same DOM prop, not drift", () => {
    const finding = compareComponentProps(
      "stack",
      "Stack",
      new Set(["direction", "gap", "class"]),
      new Set(["direction", "gap", "className"]),
      new Set(),
      new Set(["direction", "gap", "class"]),
    );

    expect(finding).toBeNull();
  });

  it("excludes snippets and children from prop drift", () => {
    // Invariant: Snippets and children are not drift
    // Svelte children: Snippet vs React children: ReactNode
    const finding = compareComponentProps(
      "dialog",
      "Dialog",
      new Set(["open", "children", "header", "footer"]),
      new Set(["open", "children", "header", "footer"]),
      new Set(["children", "header", "footer"]), // Svelte snippetProps
      new Set(["open"]), // contract public props (slots documented separately)
    );

    expect(finding).toBeNull();
  });

  it("refuses to load a baseline entry that lacks a reason string or kind", () => {
    // Invariant: Baseline is reasoned and kind-tagged
    // Counterexample: baseline entry without reason string or invalid kind throws
    const invalidBaselines = [
      {
        "test-slug": {
          reactOnly: ["defaultValue"],
        } as unknown as BaselineEntry,
      },
      {
        "test-slug": {
          kind: "framework-idiom",
          reason: "",
          reactOnly: ["defaultValue"],
        },
      },
      {
        "test-slug": {
          kind: "framework-idiom",
          reason: "   ",
          reactOnly: ["defaultValue"],
        },
      },
      {
        "test-slug": {
          kind: "unknown-kind" as unknown as BaselineEntry["kind"],
          reason: "some reason",
          reactOnly: ["defaultValue"],
        },
      },
      {
        "test-slug": {
          kind: "framework-idiom",
          reason: 123,
        } as unknown as BaselineEntry,
      },
    ];

    for (const invalid of invalidBaselines) {
      expect(() => validateBaseline(invalid)).toThrow();
    }
  });

  it("accepts a baseline entry that carries a valid reason string and kind", () => {
    const validBaseline = {
      "dock-region": {
        kind: "needs-decision" as const,
        reason:
          "showTabs is spec-surface-pending in contract-spec-drift (g13.014)",
        svelteOnly: ["showTabs"],
      },
    };

    expect(() => validateBaseline(validBaseline)).not.toThrow();

    const finding = compareComponentProps(
      "dock-region",
      "DockRegion",
      new Set(["showTabs"]),
      new Set(),
      new Set(),
      new Set(),
      validBaseline["dock-region"],
    );

    expect(finding).toBeNull();
  });

  it("ratchet: refuses a pending-port baseline entry whose reason names no card", () => {
    // Invariant: Ratchet holds — pending-port must name the card clearing it
    const entryWithoutCard = {
      button: {
        kind: "pending-port" as const,
        reason: "missing port to React without card name",
        svelteOnly: ["style"],
      },
    };

    expect(() => validateBaseline(entryWithoutCard)).toThrow(
      /must name the card that will clear it/i,
    );
  });

  it("ratchet: accepts a pending-port baseline entry whose reason names a card", () => {
    const entryWithCard = {
      button: {
        kind: "pending-port" as const,
        reason: "pending port to React in g16.099",
        svelteOnly: ["style"],
      },
    };

    expect(() => validateBaseline(entryWithCard)).not.toThrow();
  });

  it("ratchet: detects stale baseline entries when a prop no longer drifts", () => {
    // Invariant: Ratchet holds — baselined prop that no longer drifts is flagged as stale
    const finding = compareComponentProps(
      "button",
      "Button",
      new Set(["variant", "tone", "style"]),
      new Set(["variant", "tone", "style"]), // style ported to React!
      new Set(),
      new Set(["variant", "tone", "style"]),
      {
        kind: "pending-port",
        reason: "pending port to React in g16.099",
        svelteOnly: ["style"], // stale: style is no longer Svelte-only
      },
    );

    expect(finding).not.toBeNull();
    expect(finding!.staleBaseline).toBeDefined();
    expect(finding!.staleBaseline!.svelteOnly).toEqual(["style"]);
  });

  it("detects conflicting static literal defaults for documented props", () => {
    const finding = compareComponentProps(
      "slider",
      "Slider",
      new Set(["min", "max", "step"]),
      new Set(["min", "max", "step"]),
      new Set(),
      new Set(["min", "max", "step"]),
      undefined,
      new Map([
        ["min", "0"],
        ["max", "100"],
        ["step", "1"],
      ]),
      new Map([
        ["min", "0"],
        ["max", "100"],
        ["step", "5"], // Planted conflicting default
      ]),
    );

    expect(finding).not.toBeNull();
    expect(finding!.defaultDrift).toBeDefined();
    expect(finding!.defaultDrift).toEqual([
      { prop: "step", svelteDefault: "1", reactDefault: "5" },
    ]);
  });

  it("parsers: extracts props and defaults from Svelte runes source", () => {
    const svelteFixture = `
<script lang="ts">
  import type { Snippet } from "svelte";
  interface Props {
    value?: number;
    step?: number;
    disabled?: boolean;
    class?: string;
    onValueChange?: (val: number) => void;
    children?: Snippet;
  }

  let {
    value = $bindable(0),
    step = 1,
    disabled = false,
    class: className = "",
    onValueChange,
    children,
  }: Props = $props();
</script>
`;
    const { props, defaults } = parseSvelteProps(svelteFixture);
    expect(props).toEqual(
      new Set(["value", "step", "disabled", "class", "onValueChange", "children"]),
    );
    expect(defaults.get("value")).toBe("0");
    expect(defaults.get("step")).toBe("1");
    expect(defaults.get("disabled")).toBe("false");
    expect(defaults.get("class")).toBe('""');
  });

  it("parsers: extracts props and defaults from React TypeScript source", () => {
    const reactFixture = `
import type { ReactNode } from "react";

export interface TestBoxProps {
  size?: "sm" | "md";
  disabled?: boolean;
  className?: string;
  onValueChange?: (val: number) => void;
  children?: ReactNode;
}

export function TestBox({
  size = "md",
  disabled = false,
  className = "",
  onValueChange,
  children,
}: TestBoxProps) {
  return <div>{children}</div>;
}
`;
    const { props, defaults } = parseReactPropsFromSource(reactFixture, "TestBox");
    // children is slot plumbing and excluded
    expect(props).toEqual(
      new Set(["size", "disabled", "className", "onValueChange"]),
    );
    expect(defaults.get("size")).toBe('"md"');
    expect(defaults.get("disabled")).toBe("false");
    expect(defaults.get("className")).toBe('""');
  });

  it("parsers: parses brace body declarations correctly", () => {
    const braceBody = `
      name?: string;
      age: number;
      tags?: string[];
      // comment line
      /* block comment */
      isActive?: boolean;
    `;
    const parsed = parsePropsFromBraceBody(braceBody);
    expect(parsed.get("name")).toBe("string");
    expect(parsed.get("age")).toBe("number");
    expect(parsed.get("tags")).toBe("string[]");
    expect(parsed.get("isActive")).toBe("boolean");
  });
});
