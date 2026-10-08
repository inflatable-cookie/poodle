// The one generated denominator for public components and portable routes.
//
// `canonicalComponents` is emitted from `packages/codegen/fixtures/preview-catalogue.json`
// by `effigy catalogue:build`; the GPUI and Jetstream `CANONICAL_COMPONENTS`
// registries are emitted from the same fixture. Every gate that needs a
// component or route count reads it here instead of typing the number a second
// time (operator ruling 2026-09-29, working-rules.md §Catalogue Specimens).
//
// A new portable component therefore moves the count by existing in the
// catalogue; nothing in a gate needs a hand bump. `assertPublicBarrelAgrees`
// keeps the agreement check: the Svelte public barrel must still name exactly
// the catalogue components plus its declared web-only and native-deferred rows.

import { canonicalComponents } from "../packages/svelte/preview/src/generated/catalogue/catalogue";

/** Portable catalogue components, in canonical order. */
export const PORTABLE_COMPONENT_NAMES: readonly string[] = canonicalComponents.map(
  (component) => component.displayName,
);

/** Portable catalogue routes, in canonical order. */
export const PORTABLE_ROUTE_SLUGS: readonly string[] = canonicalComponents.map(
  (component) => component.slug,
);

/** Portable-route denominator: the generated catalogue length. */
export const PORTABLE_ROUTE_COUNT: number = PORTABLE_ROUTE_SLUGS.length;

/**
 * Public components outside the portable catalogue.
 *
 * MeterSurface is permanently web-only (spec 068 / g14.024). Listbox is a
 * public web component whose shared Rust and GPUI implementations are deferred
 * to the next lane part; it stays outside the portable catalogue until then.
 * Other `webOnlyComponents` in the preview registry (`CodeEditor`, the
 * rich-text pair, `MarkdownRenderer`) are web-admitted behind dedicated
 * entries and remain outside the public root roster.
 */
export const ROSTER_WEB_ONLY_NAMES: readonly string[] = ["MeterSurface"];
export const ROSTER_WEB_ONLY_ROUTE_SLUGS: readonly string[] = ["meter-surface"];
export const ROSTER_NATIVE_DEFERRED_NAMES: readonly string[] = ["Listbox"];
export const ROSTER_NATIVE_DEFERRED_ROUTE_SLUGS: readonly string[] = ["listbox"];

/** Public component denominator: portable catalogue plus staged public surfaces. */
export const PUBLIC_COMPONENT_NAMES: readonly string[] = [
  ...PORTABLE_COMPONENT_NAMES,
  ...ROSTER_WEB_ONLY_NAMES,
  ...ROSTER_NATIVE_DEFERRED_NAMES,
];

/** Public-component denominator used by the native proofs and the web census. */
export const PUBLIC_COMPONENT_COUNT: number = PUBLIC_COMPONENT_NAMES.length;

/** Web catalogue routes: portable routes plus staged public web routes. */
export const WEB_COMPONENT_ROUTE_SLUGS: readonly string[] = [
  ...PORTABLE_ROUTE_SLUGS,
  ...ROSTER_WEB_ONLY_ROUTE_SLUGS,
  ...ROSTER_NATIVE_DEFERRED_ROUTE_SLUGS,
];

/**
 * Fail when the public Svelte barrel names anything other than the generated
 * denominator. A component planted in the catalogue without the barrel fails
 * as a missing name; one planted in the barrel without the catalogue fails as
 * an unexpected name; a duplicate fails outright. This is the agreement check
 * that keeps the two derivations locked together.
 */
export function assertPublicBarrelAgrees(barrelNames: Iterable<string>): void {
  const names = [...barrelNames];
  const seen = new Set<string>();
  const duplicates: string[] = [];
  for (const name of names) {
    if (seen.has(name)) duplicates.push(name);
    seen.add(name);
  }
  if (duplicates.length > 0) {
    throw new Error(
      `The public Svelte component barrel contains duplicate export(s): ${[...new Set(duplicates)].join(", ")}.`,
    );
  }

  const expected = new Set(PUBLIC_COMPONENT_NAMES);
  const missing = PUBLIC_COMPONENT_NAMES.filter((name) => !seen.has(name));
  const extra = names.filter((name) => !expected.has(name));
  if (missing.length > 0 || extra.length > 0) {
    throw new Error(
      `The public Svelte barrel disagrees with the generated catalogue denominator: ` +
        `missing ${missing.length > 0 ? missing.join(", ") : "none"}; ` +
        `unexpected ${extra.length > 0 ? extra.join(", ") : "none"}.`,
    );
  }
}
