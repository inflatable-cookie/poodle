import { readFileSync } from "node:fs";

import { describe, expect, test } from "bun:test";

/**
 * Pill appearance style evidence (g15.036). The paired web component tests can
 * only assert data attributes; they passed while a self-referential
 * `--poodle-pill-fill` custom-property cycle made `appearance="subtle"`
 * compute to a fully transparent background. This suite models the pill.css
 * cascade for the root element (and the optional dot), resolves the `var()`
 * chain with cycle detection, and pins the contract recipes for tint
 * preservation, theme-surface solid recipes (tones + custom accent), and
 * distinct subtle/badge treatments.
 */

const css = readFileSync(new URL("../src/styles/pill.css", import.meta.url), "utf8")
  .replace(/\/\*[\s\S]*?\*\//g, "");

type Declarations = Record<string, string>;

interface Rule {
  /** Attribute conditions on the pill root, e.g. { "data-tone": "info" }. */
  conditions: Record<string, string>;
  /** Which element the rule styles. */
  target: "root" | "dot";
  declarations: Declarations;
  /** Class + attribute count across the selector compounds. */
  specificity: number;
  order: number;
}

const ROOT = /^\.poodle-pill((?:\[data-[\w-]+="[^"]*"\])*)$/;
const DOT = /^\.poodle-pill((?:\[data-[\w-]+="[^"]*"\])*) \.poodle-pill__dot$/;
const CONDITIONS = /\[data-([\w-]+)="([^"]*)"\]/g;

function parseSelector(selector: string): Pick<Rule, "conditions" | "target" | "specificity"> | null {
  for (const [pattern, target] of [[ROOT, "root"], [DOT, "dot"]] as const) {
    const match = pattern.exec(selector.trim());
    if (!match) continue;
    const conditions: Record<string, string> = {};
    for (const condition of match[1].matchAll(CONDITIONS)) {
      conditions[`data-${condition[1]}`] = condition[2];
    }
    return { conditions, target, specificity: 1 + match[1].matchAll(CONDITIONS).toArray().length };
  }
  return null;
}

function parseRules(source: string): Rule[] {
  const rules: Rule[] = [];
  const blocks = source.matchAll(/([^{}]+)\{([^{}]*)\}/g);
  let order = 0;
  for (const block of blocks) {
    const declarations: Declarations = {};
    for (const declaration of block[2].matchAll(/(--[\w-]+|font-weight|letter-spacing|text-transform|opacity)\s*:\s*([^;]+);/g)) {
      declarations[declaration[1]] = declaration[2].trim().replace(/\s+/g, " ");
    }
    for (const selector of block[1].split(",")) {
      const parsed = parseSelector(selector);
      if (parsed && Object.keys(declarations).length > 0) {
        rules.push({ ...parsed, declarations, order: order++ });
      }
    }
  }
  return rules;
}

const rules = parseRules(css);

function matches(rule: Rule, attrs: Record<string, string>): boolean {
  return Object.entries(rule.conditions).every(([name, value]) => attrs[name] === value);
}

/** Computes the winning declarations for an element, cascade order applied.
 *  The dot inherits the root's custom properties, then its own rules win. */
function cascade(target: Rule["target"], attrs: Record<string, string>): Declarations {
  const applicable = rules
    .filter((rule) => rule.target === target && matches(rule, attrs))
    .sort((a, b) => a.specificity - b.specificity || a.order - b.order);
  const computed: Declarations =
    target === "dot" ? { ...cascade("root", attrs) } : {};
  for (const rule of applicable) {
    Object.assign(computed, rule.declarations);
  }
  return computed;
}

/** Resolves var() references against the computed custom properties. */
function resolveVars(value: string, props: Declarations, stack: string[] = []): string {
  let result = "";
  let i = 0;
  while (i < value.length) {
    const start = value.indexOf("var(", i);
    if (start === -1) return result + value.slice(i);
    result += value.slice(i, start);
    let depth = 1;
    let j = start + 4;
    while (j < value.length && depth > 0) {
      if (value[j] === "(") depth++;
      if (value[j] === ")") depth--;
      j++;
    }
    const inner = value.slice(start + 4, j - 1);
    let split = -1;
    let innerDepth = 0;
    for (let k = 0; k < inner.length; k++) {
      if (inner[k] === "(") innerDepth++;
      if (inner[k] === ")") innerDepth--;
      if (inner[k] === "," && innerDepth === 0) {
        split = k;
        break;
      }
    }
    const name = (split === -1 ? inner : inner.slice(0, split)).trim();
    const fallback = split === -1 ? null : inner.slice(split + 1).trim();
    if (stack.includes(name)) {
      throw new Error(`custom property cycle: ${[...stack, name].join(" -> ")}`);
    }
    if (name in props) {
      // Component-owned variable: substitute through the cascade.
      result += resolveVars(props[name], props, [...stack, name]);
    } else if (name.startsWith("--poodle-recipe-")) {
      // Optional recipe override hook: absent in these tests, so the authored
      // fallback recipe applies.
      result += fallback !== null ? resolveVars(fallback, props, stack) : "";
    } else {
      // Theme token or inline component property: provided outside this
      // stylesheet, so the whole expression stays literal.
      result += value.slice(start, j);
    }
    i = j;
  }
  return result;
}

function resolved(target: Rule["target"], attrs: Record<string, string>, name: string): string {
  const props = cascade(target, attrs);
  const value = props[name];
  if (value === undefined) throw new Error(`${name} is not set for ${JSON.stringify(attrs)}`);
  return resolveVars(value, props);
}

const TONES = ["neutral", "info", "success", "warning", "danger"];
const APPEARANCES = ["tint", "solid", "subtle", "badge"];

describe("pill.css appearance recipes", () => {
  test("no rule assigns a custom property in terms of itself", () => {
    for (const rule of rules) {
      for (const [name, value] of Object.entries(rule.declarations)) {
        expect(value.includes(`var(${name})`)).toBe(false);
      }
    }
  });

  test("every tone x appearance combination resolves without a cycle", () => {
    for (const tone of TONES) {
      for (const appearance of APPEARANCES) {
        for (const accent of [null, "custom"]) {
          const attrs: Record<string, string> = { "data-tone": tone, "data-appearance": appearance };
          if (accent) attrs["data-accent"] = accent;
          for (const name of ["--poodle-pill-fill", "--poodle-pill-border", "--poodle-pill-text"]) {
            const value = resolved("root", attrs, name);
            // Every component-owned variable must resolve through the cascade;
            // only --poodle-pill-accent stays literal (the component sets it
            // inline when a custom accent is provided).
            const leftovers = [...value.matchAll(/var\((--poodle-pill-[\w-]+)/g)].map((m) => m[1]);
            const allowed = attrs["data-accent"] === "custom" ? ["--poodle-pill-accent"] : [];
            for (const leftover of new Set(leftovers)) {
              expect(allowed).toContain(leftover);
            }
          }
        }
      }
    }
  });

  test("default and explicit tint preserve the ordinary tone-tinted shell", () => {
    const explicitTint = cascade("root", { "data-tone": "neutral", "data-appearance": "tint" });
    const implicit = cascade("root", { "data-tone": "neutral" });
    expect(explicitTint).toEqual(implicit);

    expect(resolved("root", { "data-tone": "neutral", "data-appearance": "tint" }, "--poodle-pill-fill"))
      .toBe("color-mix(in srgb, var(--poodle-color-background-surface) 90%, transparent)");
    expect(resolved("root", { "data-tone": "neutral", "data-appearance": "tint" }, "--poodle-pill-border"))
      .toBe("color-mix(in srgb, var(--poodle-color-border-subtle) 82%, transparent)");
    expect(resolved("root", { "data-tone": "neutral", "data-appearance": "tint" }, "--poodle-pill-text"))
      .toBe("var(--poodle-color-text-secondary)");

    expect(resolved("root", { "data-tone": "success", "data-appearance": "tint" }, "--poodle-pill-fill"))
      .toBe("color-mix(in srgb, var(--poodle-color-status-success, #22c55e) 14%, var(--poodle-color-background-surface))");
    expect(resolved("root", { "data-tone": "success", "data-appearance": "tint" }, "--poodle-pill-text"))
      .toBe("var(--poodle-color-text-primary)");
  });

  test("subtle halves the tint fill and leaves border and text alone", () => {
    const tint = cascade("root", { "data-tone": "success", "data-appearance": "tint" });
    const subtleAttrs = { "data-tone": "success", "data-appearance": "subtle" };
    const subtle = cascade("root", subtleAttrs);

    expect(resolved("root", subtleAttrs, "--poodle-pill-fill")).toBe(
      "color-mix(in srgb, color-mix(in srgb, var(--poodle-color-status-success, #22c55e) 14%, var(--poodle-color-background-surface)) 50%, transparent)",
    );
    expect(subtle["--poodle-pill-border"]).toBe(tint["--poodle-pill-border"]);
    expect(subtle["--poodle-pill-text"]).toBe(tint["--poodle-pill-text"]);
    expect(resolved("root", subtleAttrs, "--poodle-pill-fill"))
      .not.toBe(resolved("root", { "data-tone": "success", "data-appearance": "tint" }, "--poodle-pill-fill"));
  });

  test("solid mixes every tone into the theme surface with primary foreground", () => {
    const toneBases: Record<string, string> = {
      info: "var(--poodle-color-status-info, #3b82f6)",
      success: "var(--poodle-color-status-success, #22c55e)",
      warning: "var(--poodle-color-status-warning, #f59e0b)",
      danger: "var(--poodle-color-status-danger, #ef4444)",
    };
    for (const [tone, base] of Object.entries(toneBases)) {
      const attrs = { "data-tone": tone, "data-appearance": "solid" };
      expect(resolved("root", attrs, "--poodle-pill-fill"))
        .toBe(`color-mix(in srgb, ${base} 40%, var(--poodle-color-background-surface))`);
      expect(resolved("root", attrs, "--poodle-pill-border")).toBe(base);
      expect(resolved("root", attrs, "--poodle-pill-text")).toBe("var(--poodle-color-text-primary)");
    }

    const neutral = { "data-tone": "neutral", "data-appearance": "solid" };
    expect(resolved("root", neutral, "--poodle-pill-fill")).toBe(
      "color-mix(in srgb, var(--poodle-color-text-secondary) 50%, var(--poodle-color-background-surface))",
    );
    expect(resolved("root", neutral, "--poodle-pill-border")).toBe("var(--poodle-color-border-strong)");
    expect(resolved("root", neutral, "--poodle-pill-text")).toBe("var(--poodle-color-text-primary)");
  });

  test("solid with a custom accent uses the accent as the tone base", () => {
    const attrs = { "data-tone": "neutral", "data-appearance": "solid", "data-accent": "custom" };
    expect(resolved("root", attrs, "--poodle-pill-fill")).toBe(
      "color-mix(in srgb, var(--poodle-pill-accent) 40%, var(--poodle-color-background-surface))",
    );
    expect(resolved("root", attrs, "--poodle-pill-border")).toBe("var(--poodle-pill-accent)");
    expect(resolved("root", attrs, "--poodle-pill-text")).toBe("var(--poodle-color-text-primary)");
  });

  test("solid paints the dot with the primary foreground; tint keeps tone dots", () => {
    expect(resolved("dot", { "data-tone": "neutral", "data-appearance": "solid" }, "--poodle-pill-dot-fill"))
      .toBe("var(--poodle-color-text-primary)");
    expect(resolved("dot", { "data-tone": "info", "data-appearance": "tint" }, "--poodle-pill-dot-fill"))
      .toBe("var(--poodle-color-status-info, #3b82f6)");
  });

  test("badge keeps its typography and its own distinct recipes", () => {
    const badge = cascade("root", { "data-tone": "success", "data-appearance": "badge" });
    expect(badge["font-weight"]).toBe("700");
    expect(badge["letter-spacing"]).toBe("0.04em");
    expect(badge["text-transform"]).toBe("uppercase");
    expect(resolved("root", { "data-tone": "success", "data-appearance": "badge" }, "--poodle-pill-fill"))
      .toBe("color-mix(in srgb, var(--poodle-color-status-success, #22c55e) 18%, transparent)");

    const neutralBadge = { "data-tone": "neutral", "data-appearance": "badge" };
    expect(resolved("root", neutralBadge, "--poodle-pill-fill"))
      .toBe("color-mix(in srgb, var(--poodle-surface) 96%, var(--poodle-color-text-primary))");
    expect(resolved("root", neutralBadge, "--poodle-pill-text")).toBe("var(--poodle-color-text-secondary)");
  });
});
