# g15.003 — Svelte Focused Evidence: Foundation Forms, Inputs & Overlays

Status: **blocked** — pending orchestrator review of `g15.001`
Depends on: `g15.001`
Governing refs: `release-baseline-roster.md`, `release-gap-register.md`,
`../../contracts/001-working-rules.md`

## Outcome

Close the focused-evidence gap for the 26 foundation form, input, and overlay
primitives measured in `g15.001`. Each component gains focused, owner-local
test evidence that asserts contract behaviour — not an anatomy smoke case.

## Scope

AudioPlayer, ColorPicker, Calendar, DatePicker, DateRangePicker,
DateTimePicker, DateTimeRangePicker, DurationInput, EditableLabel, Field,
FieldSet, FormActions, IconButton, Meter, NumberInput, Pagination,
PaginationSummary, PasswordRequirements, Radio, RadioGroup, ResizeHandle,
RangeSlider, SegmentedControl, ScrollShell, Separator, Slider

Priority: downstream-used components first (EditableLabel, Field, FieldSet,
FormActions, Meter, NumberInput, Pagination, PaginationSummary,
PasswordRequirements, SegmentedControl, ScrollShell — see roster Downstream
use column). RangeSlider carries retained native regression evidence
(`a_scrub_reports_change_while_dragging_and_commits_once_at_release`); web
evidence must assert the same contract semantics without borrowing the native
case.

## Goals

- [ ] One focused test file (or named cases in a family test) per component,
      asserting contract semantics: form value flow, validation, focus and
      keyboard operation, overlay dismissal where applicable, token use.
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

- focused tests beside the components
- `PAPERCUTS.md` for newly discovered execution friction

## Validation

- `effigy test:components` (narrow: the touched test files)
- `effigy check:svelte`
- `effigy docs:check`
- `git diff --check`

Never run a `*-windowed` selector, `test:native-visual`, `qa:jetstream`, or
any Jetstream selector.
