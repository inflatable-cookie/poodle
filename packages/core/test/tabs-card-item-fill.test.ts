import { readFileSync } from "node:fs";

import { describe, expect, test } from "bun:test";

/**
 * Card-item fill cascade (g18.004). happy-dom cannot resolve recipe var()
 * chains at computed-value time, so this suite models the tabs.css cascade
 * for the item wrapper and tab button the same way pill-appearance-styles
 * models pill.css.
 */

const css = readFileSync(new URL("../src/styles/tabs.css", import.meta.url), "utf8").replace(
  /\/\*[\s\S]*?\*\//g,
  "",
);

type Declarations = Record<string, string>;

interface Rule {
  root: Record<string, string>;
  item: Record<string, string>;
  hover: boolean;
  target: "item" | "tab";
  declarations: Declarations;
  specificity: number;
  order: number;
}

const ATTR = /\[([^\]]+)\]/g;
const ITEM =
  /^\.poodle-tabs((?:\[[^\]]+\])*) \.poodle-tabs__item((?:\[[^\]]+\])*)(?::hover)?$/;
const BARE_ITEM = /^\.poodle-tabs__item((?:\[[^\]]+\])*)(?::hover)?$/;
const TAB = /^(?:\.poodle-tabs((?:\[[^\]]+\])*) )\.poodle-tabs__tab$/;
const BARE_TAB = /^\.poodle-tabs__tab$/;

function parseAttrs(chunk: string): Record<string, string> {
  const attrs: Record<string, string> = {};
  for (const match of chunk.matchAll(ATTR)) {
    const [name, raw] = match[1].split("=");
    attrs[name] = (raw ?? "true").replaceAll('"', "");
  }
  return attrs;
}

function parseSelector(selector: string): Omit<Rule, "declarations" | "order"> | null {
  const trimmed = selector.trim();
  if (trimmed.includes(" .poodle-tabs__tab") && trimmed.includes(".poodle-tabs__item")) {
    return null;
  }
  const itemMatch = ITEM.exec(trimmed) ?? BARE_ITEM.exec(trimmed);
  if (itemMatch) {
    const root = itemMatch.length > 2 ? parseAttrs(itemMatch[1] ?? "") : {};
    const item = parseAttrs(itemMatch[itemMatch.length - 1] ?? "");
    const hover = trimmed.endsWith(":hover");
    const specificity =
      (trimmed.includes(".poodle-tabs") ? 1 : 0) +
      1 +
      Object.keys(root).length +
      Object.keys(item).length +
      (hover ? 1 : 0);
    return { root, item, hover, target: "item", specificity };
  }
  const tabMatch = TAB.exec(trimmed) ?? (BARE_TAB.test(trimmed) ? (["", ""] as unknown as RegExpExecArray) : null);
  if (tabMatch) {
    const root = parseAttrs(typeof tabMatch[1] === "string" ? tabMatch[1] : "");
    return {
      root,
      item: {},
      hover: false,
      target: "tab",
      specificity: 1 + Object.keys(root).length,
    };
  }
  return null;
}

function parseRules(source: string): Rule[] {
  const rules: Rule[] = [];
  let order = 0;
  for (const block of source.matchAll(/([^{}]+)\{([^{}]*)\}/g)) {
    const declarations: Declarations = {};
    for (const declaration of block[2].matchAll(/(background|border|border-color|border-radius)\s*:\s*([^;]+);/g)) {
      declarations[declaration[1]] = declaration[2].trim().replace(/\s+/g, " ");
    }
    if (Object.keys(declarations).length === 0) continue;
    for (const selector of block[1].split(",")) {
      const parsed = parseSelector(selector);
      if (parsed) rules.push({ ...parsed, declarations, order: order++ });
    }
  }
  return rules;
}

const rules = parseRules(css);

const CARD_FILL =
  "var(--poodle-recipe-tabs-card-item-fill, var(--poodle-color-background-surface))";
const TINT_FILL =
  "var(--poodle-recipe-tabs-card-selected-tab-fill, color-mix(in srgb, var(--poodle-color-accent-base) 18%, transparent))";
const SOLID_FILL = "var(--poodle-recipe-tabs-active-solid-fill, var(--poodle-color-accent-base))";
const TAB_FILL = "var(--poodle-recipe-tabs-tab-fill, transparent)";

function matches(
  rule: Rule,
  query: { variant: string; selected: boolean; fill: string; hover?: boolean },
): boolean {
  const root: Record<string, string> = {
    "data-variant": query.variant,
    "data-active-fill": query.fill,
  };
  const item: Record<string, string> = query.selected ? { "data-selected": "true" } : {};
  if (!Object.entries(rule.root).every(([name, value]) => root[name] === value)) return false;
  if (!Object.entries(rule.item).every(([name, value]) => item[name] === value)) return false;
  if (rule.hover && query.hover !== true) return false;
  return true;
}

function cascade(
  target: Rule["target"],
  query: { variant: string; selected: boolean; fill: string; hover?: boolean },
): Declarations {
  const applicable = rules
    .filter((rule) => rule.target === target && matches(rule, query))
    .sort((a, b) => a.specificity - b.specificity || a.order - b.order);
  const computed: Declarations = {};
  for (const rule of applicable) Object.assign(computed, rule.declarations);
  return computed;
}

describe("tabs.css card item fill", () => {
  test("the card-item recipe hook is scoped to card items only", () => {
    const hooked = rules.filter((rule) =>
      Object.values(rule.declarations).some((value) => value.includes("--poodle-recipe-tabs-card-item-fill")),
    );
    expect(hooked.length).toBeGreaterThan(0);
    for (const rule of hooked) {
      expect(rule.target).toBe("item");
      expect(rule.root["data-variant"]).toBe("card");
    }
  });

  test("inactive and selected card items keep a card-shaped fill on the wrapper", () => {
    const inactive = cascade("item", { variant: "card", selected: false, fill: "tint" });
    expect(inactive.background).toBe(CARD_FILL);
    expect(inactive.border).toBeUndefined();
    expect(inactive["border-color"]).toBeUndefined();

    const selected = cascade("item", { variant: "card", selected: true, fill: "tint" });
    expect(selected.background).toBe(TINT_FILL);
    expect(selected.border).toBeUndefined();
  });

  test("closable and disabled cards still take the item-wrapper fill", () => {
    // Disabled and closable are item-wrapper states in markup, not extra CSS
    // axes: the same card item rule paints every wrapper, including those
    // that enclose a close button or a disabled tab.
    const disabled = cascade("item", { variant: "card", selected: false, fill: "tint" });
    expect(disabled.background).toBe(CARD_FILL);
    const tab = cascade("tab", { variant: "card", selected: false, fill: "tint" });
    expect(tab.background).toBe(TAB_FILL);
  });

  test("tint and solid replace the base surface on the selected card", () => {
    const tint = cascade("item", { variant: "card", selected: true, fill: "tint" });
    const solid = cascade("item", { variant: "card", selected: true, fill: "solid" });
    const inactive = cascade("item", { variant: "card", selected: false, fill: "solid" });
    expect(tint.background).toBe(TINT_FILL);
    expect(solid.background).toBe(SOLID_FILL);
    expect(inactive.background).toBe(CARD_FILL);
  });

  test("activeFill none keeps the card surface and leaves pill/block unfilled", () => {
    const card = cascade("item", { variant: "card", selected: true, fill: "none" });
    const cardHover = cascade("item", { variant: "card", selected: true, fill: "none", hover: true });
    const pill = cascade("item", { variant: "pill", selected: true, fill: "none" });
    const block = cascade("item", { variant: "block", selected: true, fill: "none" });
    const pillIdle = cascade("item", { variant: "pill", selected: false, fill: "none" });
    const blockIdle = cascade("item", { variant: "block", selected: false, fill: "none" });

    expect(card.background).toBe(CARD_FILL);
    expect(cardHover.background).toBe(CARD_FILL);
    expect(pill.background).toBe("transparent");
    expect(block.background).toBe("transparent");
    expect(pillIdle.background).toBeUndefined();
    expect(blockIdle.background).toBeUndefined();
  });
});
