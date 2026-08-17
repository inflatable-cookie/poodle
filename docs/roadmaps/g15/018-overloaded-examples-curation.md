# g15.018 — Overloaded Examples Curation

Status: **planned** — orchestrator review required before dispatch
Depends on: `g15.011` (audit and its three approved pilots)
Governing refs: `specimen-catalogue-audit.md`, `specimen-plan-outline.md`
(Section order, Section budget)

## Outcome

The pages that show a reader everything become pages that teach a reader
something.

`g15.011` proved the shape on Button, RangeSlider, and Tabs. This card applies
the same treatment to the remaining pages the audit graded overloaded — the
ones listed under **Examples overloaded** — plus the long pages in the 7–9
band where the length is a prop cross-product rather than a set of distinct
things worth seeing.

`Tabs` before the pilot carried 18 example groups and over seventy tab
controls on one page. `ListCard` carries 20. That is a reference dump, and it
is also why nobody notices when one of the examples stops working.

## Scope

- the pages listed under **Examples overloaded** in the audit
- the 7–9 band pages whose length comes from a cross-product
- no component, contract, or API change

## Goals

- [ ] Each page reaches the outline's 3–6 section budget, or states why it
      cannot.
- [ ] The first example answers "what is this and how do I normally use it?"
- [ ] Variants are distinct forms, not a prop cross-product. One tone row on
      one variant replaces a variant × tone grid.
- [ ] Removed examples are checked against the contract first: anything the
      contract covers and the page was the only evidence for either stays or
      is recorded as a coverage note, not silently dropped.
- [ ] Svelte and React stay identical; GPUI teaches the same set.

## Acceptance

- [ ] No catalogue page exceeds 9 captioned examples.
- [ ] Every page's first example is a realistic default use.
- [ ] A named list of what was removed from each page, and why.
- [ ] Contract coverage is unchanged or its gaps are recorded.

## Stop Conditions

- Curation removes the only evidence for a contract behaviour without
  recording it.
- The card becomes a second attempt at an exhaustive reference view. An
  exhaustive view may be considered later; it never replaces `Examples`.
- One PR tries to redo the whole catalogue. Split by family.

## Writable Scope

- the specimen files named in the audit, across Svelte, React, and GPUI
- one batch log per tranche

## Validation

- focused preview tests, `effigy check:svelte`, `effigy react:build`,
  `effigy check:gpui`, `effigy docs:check`, `git diff --check`
