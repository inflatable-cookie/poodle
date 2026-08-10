# Public Docs API Consistency

Poodle remains `strict-ready`. A second executable-example pass found older
Svelte syntax and callback shapes outside the first named-snippet sweep.

## Findings

- Pagination and ToastStack examples used dispatcher events and stale prop
  names.
- The slug and validation examples used legacy reactive labels.
- The Underlay wrapper example used `export let`, `<slot>`, event directives,
  and removed `solid` / `outline` button variants.
- React package metadata still described the complete implementation as a
  pilot, and its Svelte comparison table showed legacy event syntax.
- Public entry-document local links were valid but had no release gate.

## Repaired

- Moved the examples to Svelte 5 runes, callback props, children, and named
  snippets.
- Aligned Pagination, ToastStack, TextInput validation, and Button examples
  with their current public APIs.
- Updated the React package description and framework comparison.
- Expanded docs lint across root entry docs, operator guides, and package
  READMEs. It now rejects retired Svelte syntax and checks local link targets.

## Validated

- `effigy docs:check`
- `effigy ci:web`
- `effigy test:web-pack-install`
- `git diff --check`
