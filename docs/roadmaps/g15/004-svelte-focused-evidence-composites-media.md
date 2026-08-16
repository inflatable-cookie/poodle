# g15.004 — Svelte Focused Evidence: Composites & Media

Status: **blocked** — pending orchestrator review of `g15.001`
Depends on: `g15.001`
Governing refs: `release-baseline-roster.md`, `release-gap-register.md`,
`../../contracts/001-working-rules.md`

## Outcome

Close the focused-evidence gap for the 35 composite and media components
measured in `g15.001`. Each component gains focused, owner-local test
evidence that asserts contract behaviour — not an anatomy smoke case.

## Scope

ActionDiscoveryPanel, EditableList, ErrorBoundary, BlockEditor,
CardRadioGroup, CardToggleGroup, CommandPalette, ConfirmAction, DataTable,
DetailSectionGroup, DetailSection, DetailShell, EmbedInput, EmbedPreview,
EmptyState, FilterToolbar, FormLayout, InlineListSection, DebugDialog,
LogList, ListContainer, MarkdownEditor, PageLoading, MediaPicker,
MediaBrowsePanel, MediaPreview, MediaThumbnail, PageHeader, PickerShell,
RelationPicker, SelectionSummary, SidebarNav, MetricTile, ToastStack,
ToastHost

Priority: downstream-used components first (EditableList, BlockEditor,
CardRadioGroup, CardToggleGroup, CommandPalette, ConfirmAction, DataTable,
DetailSectionGroup, DetailSection, EmptyState, FilterToolbar, InlineListSection,
LogList, ListContainer, MarkdownEditor, MediaThumbnail, PageHeader,
RelationPicker, PageLoading — see roster Downstream use column). DebugDialog
and the media workflows may need bounded harness fixtures; do not fold
workflow state into components.

## Goals

- [ ] One focused test file (or named cases in a family test) per component,
      asserting contract semantics for the composite's composition surface.
- [ ] Evidence names exact files and cases; aggregate selectors do not count.
- [ ] No component API, runtime code, specimen, or contract changes to
      produce evidence.

## Acceptance

- [ ] Every scoped component has a named focused test case beyond the anatomy
      smoke.
- [ ] `effigy check:svelte`, `effigy test:components`, `effigy docs:check`
      pass.
- [ ] The register's row for each component flips to evidence-present.

## Stop Conditions

- A test asserts the same anatomy smoke asserts.
- Work expands beyond the scoped component list without a new card.
- A specimen or contract is changed to make a test pass.

## Writable Scope

- focused tests and bounded harness fixtures beside the components
- `PAPERCUTS.md` for newly discovered execution friction

## Validation

- `effigy test:components` (narrow: the touched test files)
- `effigy check:svelte`
- `effigy docs:check`
- `git diff --check`

Never run a `*-windowed` selector, `test:native-visual`, `qa:jetstream`, or
any Jetstream selector.
