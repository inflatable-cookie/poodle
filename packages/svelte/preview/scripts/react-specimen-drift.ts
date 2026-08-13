/**
 * Every component that claims a standalone specimen must have a React one
 * registered in the React gallery's `specimen-map.ts`.
 *
 * `specimen-map.ts` already asserts this — but it does so by throwing at
 * module load, which only happens in a browser. `ci:web` *builds* the React
 * preview and never runs it, so on 2026-08-13 the gallery shipped fatally
 * broken on main for two components (`update-center`, `update-status`) with
 * every gate green: the whole preview rendered zero characters. This gate
 * moves that check to build time.
 *
 * The React registry already applies the Svelte-only and embedded-only
 * exclusions, so this imports it rather than re-deriving them — the runtime
 * guard and this gate must not be able to disagree.
 */
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, resolve } from "node:path";

import { allComponents } from "../../../react/preview/src/gallery/registry";

const here = dirname(fileURLToPath(import.meta.url));
const mapPath = resolve(here, "../../../react/preview/src/gallery/specimen-map.ts");
const source = readFileSync(mapPath, "utf8");

// Keys of the `specimenMap` object literal: `"slug": SomeSpecimen,`
const registered = new Set(
  [...source.matchAll(/^\s*"([a-z0-9-]+)":\s*\w+,/gm)].map((match) => match[1]),
);

const missing = allComponents
  .filter((component) => component.hasSpecimen && !registered.has(component.slug))
  .map((component) => component.slug);

const unknown = [...registered].filter(
  (slug) => !allComponents.some((component) => component.slug === slug),
);

if (missing.length > 0 || unknown.length > 0) {
  if (missing.length > 0) {
    console.error(`Missing React specimens: ${missing.join(", ")}`);
  }
  if (unknown.length > 0) {
    console.error(`Unknown React specimens: ${unknown.join(", ")}`);
  }
  console.error(
    "\nThe React gallery throws at module load for these, which makes the whole\n" +
      "preview render nothing. Add the specimen and register it in\n" +
      "packages/react/preview/src/gallery/specimen-map.ts.",
  );
  process.exit(1);
}

console.log(
  `react-specimen-drift: ${registered.size} React specimens registered, ` +
    `all ${allComponents.filter((c) => c.hasSpecimen).length} claimed specimens present.`,
);
