# g15.002 — Svelte Focused Evidence: Foundation Display & Shell Primitives

Status: **blocked** — pending orchestrator review of `g15.001`
Depends on: `g15.001`
Governing refs: `release-baseline-roster.md`, `release-gap-register.md`,
`../../contracts/001-working-rules.md`

## Outcome

Close the focused-evidence gap for the 29 foundation display and shell
primitives measured in `g15.001`. Each component gains focused, owner-local
test evidence that asserts contract behaviour — not an anatomy smoke case.
First executable tranche of the measured runway: it opens the only open
Svelte-denominator surface and unlocks the release-certification card.

## Scope

Accordion, AlertDialog, Avatar, Box, Breadcrumbs, BulkActionBar, Callout,
Card, Code, CollapseToggle, Collapsible, DetailItem, Eyebrow, Grid, HoverCard,
ListCardCounter, ListGrid, MetaBar, MetaItem, NavCard, Pill, Progress, Rating,
Region, Skeleton, Spacer, Stack, Stepper, Spinner

Priority: components with downstream use first (Breadcrumbs, BulkActionBar,
Callout, Card, Code, CollapseToggle, DetailItem, Grid, ListCardCounter,
ListGrid, MetaBar, MetaItem, NavCard, Pill, Progress, Spinner — see roster
Downstream use column).

## Goals

- [ ] One focused test file (or named cases in a family test) per component,
      asserting the contract's observable semantics: props table, states,
      keyboard behaviour, accessibility projection, token use.
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
