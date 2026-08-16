# g15.002 — Svelte Focused Evidence: Foundation Display & Shell Primitives

Status: **blocked** — pending orchestrator review of `g15.001`
Depends on: `g15.001`
Governing refs: `release-baseline-roster.md`, `release-gap-register.md`,
`../../contracts/001-working-rules.md`

## Outcome

Close the focused-evidence gap for the 29 foundation display and shell
primitives measured in `g15.001`. Each component gains focused, owner-local
test evidence that asserts **load-bearing observable contract behaviour** —
interaction, state, keyboard/accessibility, or value semantics that a change
in the implementation could break without the anatomy changing. Every batch
lands the same contract cases paired on the React side so the mirror evidence
moves with the reference and `g15.006` stays bounded to implementations and
gallery pages.

## Minimum Evidence Threshold

- a named test case (file or family file) per component, asserting at least
  one load-bearing behaviour from the contract: default and controlled states,
  emitted events/callbacks, keyboard operation, accessibility projection
  (role/label/focus), or composed-token output
- "mounts and renders anatomy" does not count; a case that only repeats the
  smoke assertion is not evidence
- not every prop or style axis needs a case — only contract behaviour that is
  observable and breakable
- evidence must name exact files and cases; aggregate selectors do not count

## Execution Plan

- [ ] **Batch A — layout & content primitives (10):** Avatar, Box, Card, Code,
      Eyebrow, Grid, Region, Skeleton, Spacer, Stack
- [ ] **Batch B — disclosure, navigation & feedback (10):** Accordion,
      Breadcrumbs, CollapseToggle, Collapsible, DetailItem, HoverCard,
      NavCard, Progress, Spinner, Stepper
- [ ] **Batch C — selection, chrome & alerting (9):** AlertDialog,
      BulkActionBar, Callout, ListCardCounter, ListGrid, MetaBar, MetaItem,
      Pill, Rating

Priority within each batch: downstream-used components first (per the roster
Downstream use column; Longhorn, underlay, soundcheck and its library, and
acowtancy lead). Run the narrow test round at the end of each batch.

## Goals

- [ ] One focused Svelte test file (or named family cases) per component
      meeting the threshold above.
- [ ] The same contract cases mirrored as focused React tests in the same
      batch (React stays tightly paired through the shared web substrate).
- [ ] Bounded fixes to scoped implementation defects the new tests expose,
      contract-first per the working rules: update the contract before
      changing observable inputs, defaults, states, events, keyboard
      behaviour, accessibility, layout intent, or token use.
- [ ] Record each batch in one August batch log under `docs/logs/2026-08/`.

## Acceptance

- [ ] Every scoped component has a named focused test case beyond the anatomy
      smoke, on both Svelte and React sides.
- [ ] `effigy check:svelte`, `effigy react:build`, `effigy test:components`,
      `effigy docs:check` pass.
- [ ] The register's row for each component flips to evidence-present, and the
      roster's Focused Svelte / Focused React test cells name the case files.
- [ ] No contract changed except as a required consequence of a scoped fix,
      and then only with the fix itself and a contract-first update.

## Stop Conditions

- A test asserts the same anatomy smoke asserts.
- A fix changes observable behaviour without a contract-first update.
- The public contract is ambiguous and cannot be resolved by reading the
  contract and the reference specimen — stop and surface the ambiguity to
  the orchestrator instead of guessing.
- Work expands beyond the scoped component list without a new card.

## Writable Scope

- focused Svelte and React tests beside the components
- bounded fixes to scoped component source and its tests, contract-first
- `release-baseline-roster.md` and `release-gap-register.md` (focused-evidence
  rows only, no status lines)
- one August batch log under `docs/logs/2026-08/`
- `PAPERCUTS.md` for newly discovered execution friction

## Validation

- `effigy test:components` (narrow: the touched test files, per batch)
- `effigy check:svelte`, `effigy react:build`
- `effigy docs:check`
- `git diff --check`

Never run a `*-windowed` selector, `test:native-visual`, `qa:jetstream`, or
any Jetstream selector.
