import { describe, expect, it } from "vitest";

import {
  canonicalizePropName,
  compareComponentProps,
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

  it("refuses to load a baseline entry that lacks a reason string", () => {
    // Invariant: Baseline is reasoned
    // Counterexample: baseline entry without reason string throws
    const invalidBaselines = [
      {
        "test-slug": {
          reactOnly: ["defaultValue"],
        } as unknown as BaselineEntry,
      },
      {
        "test-slug": {
          reason: "",
          reactOnly: ["defaultValue"],
        },
      },
      {
        "test-slug": {
          reason: "   ",
          reactOnly: ["defaultValue"],
        },
      },
      {
        "test-slug": {
          reason: 123,
        } as unknown as BaselineEntry,
      },
    ];

    for (const invalid of invalidBaselines) {
      expect(() => validateBaseline(invalid)).toThrow(
        /must have a non-empty reason string/i,
      );
    }
  });

  it("accepts a baseline entry that carries a valid reason string", () => {
    const validBaseline = {
      "dock-region": {
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
});
