# CodeEditor implementation hold

Status: held — contract accepted; implementation and release unapproved
Captured: 2026-09-10
Owner: Chatterbox
Source: Desktop contract 034 and g02.058 at `7c7f6f13864512189ba53949fd65534140c57597`

## Accepted planning

- `docs/contracts/components/code-editor.md` owns the reusable seam.
- `Tabs` gains the planned `TabItem.pinned` boundary needed for an immutable
  leading Details tab beside reorderable editor tabs.
- Desktop keeps source semantics, exact-byte drafts, Markdown body/full-file
  projection, diagnostics meaning, persistence, review, saves, and recovery.

## Hold

No implementation task, package release, or consumer pin is authorized by the
intake. Poodle's active-cohort rule requires Svelte, React, shared Rust,
`poodle-render`, and GPUI together; a Svelte-only shipment is not admissible.

Promotion route: operator approves one bounded Northstar task covering the
contract, the Tabs pinning amendment, focused selectors, distribution, and
active-cohort evidence. Release remains a separate approval after merge.
