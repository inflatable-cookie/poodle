// RefSelect pure model. Renderer-neutral logic: query filtering, kind glyphs and
// group-heading runs. This is the canonical TypeScript source; the React shell
// mirrors it and the Rust spec (`poodle-specs::ref_select`) re-implements the
// same semantics.
//
// Poodle knows the shape of a ref, never git itself: no fetching, no parsing, no
// ahead/behind maths.

import type { RefKind, RefOption } from "./types.ts";

/** Default glyph per ref kind. Unknown kinds fall back to the branch glyph — a
 * host inventing its own kind still gets something sensible. */
export function refKindIcon(kind: RefKind | string | undefined): string {
  switch (kind) {
    case "tag":
      return "tag";
    case "commit":
      return "git-commit-horizontal";
    default:
      return "git-branch";
  }
}

/** The glyph an option renders: its own `icon` override, else its kind glyph. */
export function refIcon(option: RefOption): string {
  return option.icon ?? refKindIcon(option.kind);
}

/** Case-insensitive substring match across label, then value, then description —
 * a user typing a sha or a path fragment finds the row. An empty query passes
 * everything. */
export function filterRefs(refs: RefOption[], query: string): RefOption[] {
  const needle = query.trim().toLowerCase();
  if (!needle) return refs;
  return refs.filter((option) => {
    const haystacks = [option.label, option.value, option.description ?? ""];
    return haystacks.some((text) => text.toLowerCase().includes(needle));
  });
}

/** The group heading to emit before `refs[index]`, when it opens a new run.
 * Runs are computed over the *filtered* list, so a heading never survives its
 * last matching row. */
export function groupHeadingFor(refs: RefOption[], index: number): string | null {
  const group = refs[index]?.group;
  if (!group) return null;
  return refs[index - 1]?.group === group ? null : group;
}

/** Trigger label: the selected ref's label, the raw value when the host holds a
 * ref outside the current list, else the placeholder. */
export function refLabel(refs: RefOption[], value: string, placeholder: string): string {
  if (!value) return placeholder;
  return refs.find((option) => option.value === value)?.label ?? value;
}
