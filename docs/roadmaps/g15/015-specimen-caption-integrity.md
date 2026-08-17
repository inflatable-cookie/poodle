# g15.015 — Specimen Caption Integrity

Status: **planned** — orchestrator review required before dispatch
Depends on: `g15.011` (audit)
Governing refs: `specimen-catalogue-audit.md`, `specimen-plan-outline.md`,
`../../contracts/001-working-rules.md`

## Outcome

Every example on a Svelte catalogue page shows the caption its author wrote,
and the gate that would have caught the failure runs.

`g15.011` found nine agent-surface pages where all 52 example captions render
blank. `SpecimenGroup` accepts `label`; those pages pass `title` and
`description`. Svelte drops unknown props silently, so the authored copy — good
copy, describing what each example teaches — never reaches the page.

The cause is systemic, not local: `check:svelte` type-checks
`packages/svelte/install-smoke` and `packages/svelte/components`, and never
`packages/svelte/preview`. Running `svelte-check` there by hand reports the 52
errors directly.

## Scope

- the nine pages listed under **D — captions do not render** in the audit
- `SpecimenGroup` in both web previews
- the Svelte type-check gate's scope

## Goals

- [ ] All 52 captions render, with the authored wording.
- [ ] `SpecimenGroup` carries an optional `description`, so the explanatory
      sentences those pages already contain reach the reader instead of being
      deleted. React matches.
- [ ] `packages/svelte/preview` is inside a type-check gate.
- [ ] The 348 `readonly never[]` errors from the generated catalogue type are
      resolved or explicitly suppressed with a reason — a gate nobody can pass
      is not a gate.

## Acceptance

- [ ] A live sweep of the nine pages reports zero blank captions.
- [ ] `svelte-check` over `packages/svelte/preview` reports zero errors.
- [ ] The gate runs in `ci:web`; a reintroduced `title=` prop fails it.
- [ ] No component public API, contract, or semantic change.

## Stop Conditions

- The fix becomes a rewrite of the nine pages' examples. This card restores
  captions; content curation is `g15.018`.
- Suppressing the type errors instead of fixing the scope hole.

## Writable Scope

- the nine specimen files, `SpecimenGroup` (Svelte + React)
- the generated catalogue type or its suppression
- `tasks/effigy.tasks.toml` gate scope — **operator approval required**
- one batch log

## Validation

- focused preview tests, `effigy check:svelte`, `effigy react:build`,
  `effigy docs:check`, `git diff --check`
