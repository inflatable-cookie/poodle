# g15.027 — Screen-Clear Human Specimen Review

Status: **complete — not dispatchable**; all six children `g15.028`–`g15.033`
and their routed repairs are accepted
Role: human-teaching completion lane for `g15.011`
Consumes: `g15.011` partial screening baseline
Governing refs: `specimen-catalogue-audit.md`, `specimen-plan-outline.md`,
`../g14/026-human-centred-specimen-catalogue-audit.md`,
`../../contracts/001-working-rules.md`

## Outcome

Every catalogue page that screened clear receives the human judgment the
mechanical audit could not provide. A screen-clear page is not assumed good:
its first example must teach normal use, its variants must be meaningfully
distinct, and its three active-runtime pages must teach the same component.

This parent owns the method and exact partition. Dispatch one child, never this
file.

## Exact Partition

The 56 rows that screened `keep` in the `g15.011` baseline appear exactly once.
Human review may change a row's final grade or disposition; that does not move
the page between child ownership lists. A discovered contract/runtime blocker
is routed separately before the next child continues.

| Child | Family | Pages |
| --- | --- | ---: |
| [`g15.028`](028-review-foundation-controls-entry.md) | Foundation controls and entry | 14 |
| [`g15.029`](029-review-foundation-date-time.md) | Foundation date and time | 7 |
| [`g15.030`](030-review-foundation-layout.md) | Foundation layout | 9 |
| [`g15.031`](031-review-foundation-content-status.md) | Foundation content and status | 9 |
| [`g15.032`](032-review-composition-navigation-overlays.md) | Composition navigation and overlays | 10 |
| [`g15.033`](033-review-composition-forms-data-media.md) | Composition forms, data, and media | 7 |
| **Total** |  | **56** |

## Review Method

For every page, inspect Svelte and React live and consume the headless native
evidence from `g15.026`. Record a short verdict against the carried rubric:

- the first example answers what the component is and how it is normally used;
- variants are meaningfully distinct, not a prop cross-product;
- interactive controls demonstrate the intended gesture and work;
- loading, disabled, empty, error, and narrow states appear only when useful;
- size and density matrices stay in their dedicated panes;
- captions describe user-facing meaning;
- Svelte, React, and GPUI teach the same component and important evidence.

Keep a page unchanged when it passes. If it fails, repair only that page's
specimen presentation inside the child and update its audit row. A contract,
public API, or component-semantic change is a stop condition, not hidden
specimen work.

## Acceptance

- [x] All 56 pages have a recorded human-teaching verdict; none is inferred
      from an A screening grade.
- [x] The six child lists still form one exact, duplicate-free partition of
      the 56 screen-clear rows frozen at the start of this lane.
- [x] Svelte and React structure/copy agree; GPUI teaches the same evidence
      where the active runtime supports it.
- [x] Changed pages are reviewed by the operator in the live Svelte and React
      previews before their child completes. Unchanged pages still carry a
      recorded reviewer verdict.
- [x] The audit records any grade or disposition changes and names the reason.
- [x] No `Conformance` tab, fixture corpus, schema, or generated adapter is
      introduced.

## Closeout

Children `g15.028`–`g15.033` reviewed the exact 56-page partition. PR #63
landed the final seven verdicts and two bounded paired-web repairs; the
operator approved the changed routes before merge. This parent closes with
`g15.011` and unblocks `g15.046`.

## Stop Conditions

- A page needs a contract, public API, or component semantic change.
- Review expands into exhaustive variant enumeration.
- GPUI evidence requires a windowed or screenshot-comparison path.
- A child needs to claim a page outside its exact list.

## Writable Scope

- the exact specimen files named by the dispatched child across Svelte, React,
  and GPUI
- `specimen-catalogue-audit.md` rows for those pages
- one batch log per child

## Validation

- focused preview tests for changed pages
- `effigy catalogue:check`, relevant web/native checks,
  `effigy docs:check`, `git diff --check origin/main...HEAD`
- headless only; never a `*-windowed`, `test:native-visual`, or Jetstream
  selector
