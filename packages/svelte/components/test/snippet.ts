import type { Snippet } from "svelte";

/**
 * Tests pass plain thunks where a component's props type wants a `Snippet`.
 *
 * The Svelte 5 runtime accepts a thunk in that position — every suite using
 * this pattern passes — but the types model a `Snippet` as a branded unique
 * symbol, so a thunk cannot satisfy them. `createRawSnippet` would satisfy the
 * type, but it wraps the value in markup and would change what the existing
 * assertions see.
 *
 * So: one named, documented cast here rather than fifteen anonymous ones
 * scattered through the suites. If Svelte ever accepts thunks in its types,
 * this is the single place to delete.
 */
export function asSnippet<T extends unknown[] = []>(thunk: (...args: T) => unknown): Snippet<T> {
  return thunk as unknown as Snippet<T>;
}
