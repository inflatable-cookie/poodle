/**
 * Recipe-variable inventory generator.
 *
 * Scans the Svelte component sources for component-local CSS custom
 * properties (--poodle-<component>-...) and classifies each as an
 * appearance variable (public recipe surface) or a metric variable
 * (internal sizing/spacing, not part of the recipe contract).
 *
 * Output: packages/svelte/preview/artifacts/recipe-inventory.json
 * See docs/architecture/007-appearance-recipe-contract.md.
 */

import { readdirSync, readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const SRC = join(import.meta.dir, "..", "..", "components", "src");
const OUT_DIR = join(import.meta.dir, "..", "artifacts");
const OUT = join(OUT_DIR, "recipe-inventory.json");

// Token/system prefixes that are not component recipe variables.
const SYSTEM_PREFIXES = [
  "color", "space", "size", "typography", "radius", "border", "state",
  "elevation", "overlay", "treatment", "motion", "focus",
];

// Appearance suffix heuristics: these participate in the public recipe
// surface. Everything else is a metric/internal variable.
const APPEARANCE_PATTERN =
  /(fill|border|text|shadow|tone|ring|color|background|glow|tint|opacity|accent)(-(hover|active|focus|selected|disabled|checked|open|error|warning|success|danger))?$/;

const perComponent = new Map<string, { recipe: Set<string>; candidates: Set<string>; metric: Set<string> }>();

function componentKey(variable: string): string | null {
  const body = variable.slice("--poodle-".length);
  const [head] = body.split("-");

  if (head && SYSTEM_PREFIXES.includes(head)) {
    return null;
  }

  return body;
}

for (const file of readdirSync(SRC)) {
  // Components keep styles either inline (<style>) or in an extracted,
  // co-located .css file (god-file decomposition); scan both.
  if (!file.endsWith(".svelte") && !file.endsWith(".css")) continue;

  const source = readFileSync(join(SRC, file), "utf8");
  const kebab = file
    .replace(/\.(svelte|css)$/, "")
    .replace(/([a-z])([A-Z])/g, "$1-$2")
    .toLowerCase();

  const entry =
    perComponent.get(kebab) ?? { recipe: new Set<string>(), candidates: new Set<string>(), metric: new Set<string>() };

  for (const match of source.matchAll(/--poodle-recipe-[a-z0-9-]+/g)) {
    entry.recipe.add(match[0]);
  }

  // Appearance vars whose every definition resolves through a recipe hook
  // are internal resolution variables (architecture 007), not candidates.
  const hookedDefinitions = new Set<string>();
  for (const match of source.matchAll(/(--poodle-[a-z0-9-]+)\s*:\s*var\(--poodle-recipe-/g)) {
    hookedDefinitions.add(match[1]);
  }
  const bareDefinitions = new Set<string>();
  const propChannel = new Set<string>();
  for (const match of source.matchAll(/style:(--poodle-[a-z0-9-]+)=/g)) {
    propChannel.add(match[1]);
  }
  for (const match of source.matchAll(/(--poodle-[a-z0-9-]+)\s*:(?!\s*var\(--poodle-recipe-)\s*([^;\n]*)/g)) {
    // Template-literal definitions (`--x: ${prop}`) are the per-instance
    // prop channel, not a missing hook.
    if (match[2]?.includes("${")) {
      propChannel.add(match[1]);
      continue;
    }
    bareDefinitions.add(match[1]);
  }

  for (const match of source.matchAll(/--poodle-[a-z0-9-]+/g)) {
    const variable = match[0];

    if (variable.startsWith("--poodle-recipe-")) continue;

    const body = componentKey(variable);

    if (!body || !body.startsWith(kebab)) continue;

    if (!APPEARANCE_PATTERN.test(variable)) {
      entry.metric.add(variable);
    } else if (
      bareDefinitions.has(variable) ||
      (!hookedDefinitions.has(variable) && !propChannel.has(variable))
    ) {
      entry.candidates.add(variable);
    }
  }

  if (entry.recipe.size > 0 || entry.candidates.size > 0 || entry.metric.size > 0) {
    perComponent.set(kebab, entry);
  }
}

const inventory = Object.fromEntries(
  [...perComponent.entries()]
    .sort(([a], [b]) => a.localeCompare(b))
    .map(([component, sets]) => [
      component,
      {
        recipe: [...sets.recipe].sort(),
        candidates: [...sets.candidates].sort(),
        metric: [...sets.metric].sort(),
      },
    ]),
);

const summary = {
  components: Object.keys(inventory).length,
  recipeHooks: Object.values(inventory).reduce((total, entry) => total + entry.recipe.length, 0),
  hookCandidates: Object.values(inventory).reduce((total, entry) => total + entry.candidates.length, 0),
  metricVariables: Object.values(inventory).reduce((total, entry) => total + entry.metric.length, 0),
};

mkdirSync(OUT_DIR, { recursive: true });
writeFileSync(OUT, JSON.stringify({ summary, inventory }, null, 2) + "\n");

console.log(
  `recipe-inventory: ${summary.components} components, ${summary.recipeHooks} recipe hooks, ${summary.hookCandidates} hook candidates, ${summary.metricVariables} metric vars`,
);
