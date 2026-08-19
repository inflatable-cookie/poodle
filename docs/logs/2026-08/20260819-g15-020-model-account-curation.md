# g15.020 — Model Connections and Account Lifecycle Curation (August batch log)

Date: 2026-08-19
Card: `docs/roadmaps/g15/020-curate-model-connection-licence.md`
Parent: `docs/roadmaps/g15/018-overloaded-examples-curation.md`
Handoff: `docs/handoffs/20260819-001234-g15-020-model-account-curation.md`
Worktree: `/Users/tom/.t3/worktrees/poodle/t3code-cd807cf5`
Branch: `t3code/cd807cf5`
Worker base: `75d40ecd3894fabd9dbcee4a374d79334d5f59ad` (`origin/main` at dispatch)

## Summary

The first family curation child. Six overloaded or long pages come back inside
the outline's 3–6 section budget across Svelte, React and GPUI; two already
bounded licence pages were verified and left alone. No component, contract or
public API changed.

Where a section now holds several instances, they answer one teaching question
and stay visually distinct. Nothing was merged into an unlabelled slab, and
every removed caption is mapped below to retained catalogue coverage or to a
focused test.

## Change class

- **Change class:** none (documentation surfaces only)
- **Packages touched:** `poodle-svelte` preview, `poodle-react` preview,
  `poodle-gpui-preview` — specimen files only
- **Public entry points:** unchanged

## Baseline recount at the worker base

Recounted before any edit. Every count matched the card's remeasured table
exactly, so the eight-page set was neither widened nor narrowed.

| Page | Svelte | React | GPUI | Ruling |
| --- | ---: | ---: | ---: | --- |
| LicenceActivation | 5 | 5 | 5 | verified no-op |
| LicenceSeats | 6 | 6 | 6 | verified no-op |
| LicenceStatus | 10 | 10 | 10 | curated |
| ModelCatalogueEditor | 9 | 9 | 11 | curated and converged |
| ModelConnectionCard | 9 | 9 | 12 | curated and converged |
| ModelConnectionPicker | 8 | 8 | 10 | curated and converged |
| ModelConnectionSetup | 8 | 8 | 8 | curated |
| ModelPicker | 13 | 13 | 9 | curated and converged |

## After

| Page | Svelte | React | GPUI |
| --- | ---: | ---: | ---: |
| LicenceActivation | 5 | 5 | 5 |
| LicenceSeats | 6 | 6 | 6 |
| LicenceStatus | 5 | 5 | 5 |
| ModelCatalogueEditor | 6 | 6 | 6 |
| ModelConnectionCard | 5 | 5 | 5 |
| ModelConnectionPicker | 5 | 5 | 5 |
| ModelConnectionSetup | 6 | 6 | 6 |
| ModelPicker | 5 | 5 | 5 |

Web totals: 68 → 43 captions. GPUI: 71 → 43. All three runtimes now agree on
the ordered section set for every page in the family.

## Final ordered captions

Svelte and React are verbatim identical. GPUI teaches the same ordered set.

**LicenceActivation** (unchanged) — Embedded account activation; External
account activation; Key activation; Pending and disabled; Host copy

**LicenceSeats** (unchanged) — Mixed labels; Unnamed machines; This machine
only; Pending release; Direct release; Empty authority

**LicenceStatus** — Active; In grace; Use window expired; Lease lapsed; Clock
refused

**ModelCatalogueEditor** — Shown and hidden models; Reorder and visibility
controls; Host mark, actions, and row metadata; Loading and pending; Empty
catalogue; Unavailable, error, and session-negotiated

**ModelConnectionCard** — Ready and enabled; Readiness and preference states;
Host mark, badges, actions, and closed accessory; Open details with catalogue;
Narrow summary wrapping

**ModelConnectionPicker** — Grouped catalogue; Search results; Catalogue states
and host lock; Host provider marks and footer; Narrow layout

**ModelConnectionSetup** — Choose a connection; Configure: API key;
Auto-detected local route; OAuth in progress; Local endpoint; Validation and
pending

**ModelPicker** — Cross-provider default; Axis control forms; Variants and
emphasis; What the trigger shows; Nothing selected, and disabled

## The two verified no-op pages

Both were recounted in all three runtimes and their caption text compared
across Svelte, React and GPUI. LicenceActivation is 5/5/5 and LicenceSeats is
6/6/6, with identical ordered captions in every runtime. No cross-runtime drift
was found, so neither page's source was touched. Both are pinned in the new
regression so a later card cannot inflate them silently either.

## Removals and coverage disposition

Every caption removed from the catalogue, and where its behaviour still lives.

### LicenceStatus (10 → 5)

The card required one distinct surface per usability state, with coverage
windows and both trust bases distributed through those five fixtures. The five
removed captions were all variations of `active`; their prop combinations moved
onto the retained fixtures rather than disappearing.

| Removed caption | Disposition |
| --- | --- |
| No coverage expiry | `useUntil: null, updateUntil: null` moved onto **Clock refused** |
| Updates expired | second instance inside **Active** (`useUntil: null, updateUntil: past, attention: informational`) — the only `informational` fixture on the page, kept deliberately |
| Use window only | `useUntil: later, updateUntil: null` moved onto **Lease lapsed** |
| Offline verification | `offlineSignature` is the trust basis on **Active** (both instances) and **Use window expired** |
| Remote verification | `remoteAssertion` is the trust basis on **In grace**, **Lease lapsed** and **Clock refused** |

Contract §9 acceptance re-checked on the curated page: five usability states,
both trust bases, and all four `useUntil`/`updateUntil` null-value pairings —
`(value, value)`, `(null, past)`, `(value, null)`, `(null, null)` — are still
visible. The regression asserts the trust copy and the both-null pairing
directly.

### ModelCatalogueEditor (9 → 6, GPUI 11 → 6)

| Removed caption | Disposition |
| --- | --- |
| Reorder-capable list (web) | was a byte-identical second copy of the default harness; the slot now teaches `isDragEnabled=false` and `showMoveActions=false`, which had no web surface at all |
| Duplicate display labels (web + GPUI) | was a text-only note with no component instance. The sentence moved beside the default caption, and the duplicate `Shared Label` rows are still visible in the fixtures. Identity-by-opaque-id is asserted by `ModelCatalogueEditor.test.ts` → "emits complete shown-id order from move actions" (`model-dup-a` in the emitted order) |
| Custom action (web + GPUI) | folded into **Host mark, actions, and row metadata** beside `leading` and `rowMeta` |
| Reorder-capable list with host content (GPUI) | same fold; the web side gained `leading`/`rowMeta`, which GPUI already had — a convergence gain, not a loss |
| Loading / Unavailable / Empty / Error / Session negotiated (five separate surfaces) | regrouped into three: **Loading and pending**, **Empty catalogue**, **Unavailable, error, and session-negotiated**. All six contract postures still render |
| Pending mutation lock (GPUI) | second instance inside **Loading and pending**; the web side gained it |
| Drag disabled (GPUI) | first instance inside **Reorder and visibility controls**; the web side gained it |

### ModelConnectionCard (9 → 5, GPUI 12 → 5)

| Removed caption | Disposition |
| --- | --- |
| Ready and disabled | second instance inside **Ready and enabled**, where it also carries the two-instances story |
| Two OpenAI instances | same surface — work and personal are stacked there, differing only by instance label and opaque id |
| Checking / Needs attention / Unavailable | three instances inside **Readiness and preference states** |
| Closed UpdateCenter accessory | second instance inside **Host mark, badges, actions, and closed accessory** |
| Host mark, badges, and actions (GPUI) | first instance of that same surface; the web side gained `leading`/`badges`/`actions`, which it did not have |
| Disabled card (GPUI) | instance inside **Readiness and preference states**; the web side gained `isDisabled` |
| Enable switch disabled on its own (GPUI) | instance in the same surface; the web side gained `isEnableDisabled` |

### ModelConnectionPicker (8 → 5, GPUI 10 → 5)

| Removed caption | Disposition |
| --- | --- |
| Availability: available, checking, unavailable, unsupported | was a text-only note. All four postures sit on their matching options in **Grouped catalogue**, and the sentence moved beside that caption |
| Query with results / Query with no results | two instances inside **Search results** |
| Loading / Error / Empty catalogue | three instances inside **Catalogue states and host lock** |
| Disabled (GPUI) | fourth instance in that surface; the web side gained `isDisabled`, which had no web surface and no focused test |
| Host provider marks and footer (GPUI) | kept as its own surface; the web side gained `leading`/`footer` to converge |

### ModelConnectionSetup (8 → 6)

| Removed caption | Disposition |
| --- | --- |
| Auto-detect: found / Auto-detect: missing | two instances inside **Auto-detected local route**, both on the `choose` stage over the interactive option set where `codex-app` is available. Because that option sets `requiresConfiguration: false`, the action row reads Add rather than Continue and carries no Back: the credential step is genuinely skipped, not merely described. Corrected after PR review — the first draft forced both instances onto `configure`, which showed the opposite of the claim |
| Validation failure / Pending submit | two instances inside **Validation and pending** |

### ModelPicker (13 → 5, GPUI 9 → 5)

Per the card, existing contract coverage is consolidated rather than deleted:
the default fixture carries grouping, provider marks, badges, descriptions, a
model-scoped axis, a disabled option and the live serialized selection.

| Removed caption | Disposition |
| --- | --- |
| Different axes per model (one picker per model) | the default fixture's six models expose different axis sets; opening the picker shows each model's own rail. Asserted by `ModelPicker.test.ts` → "summarises the axes the selected model exposes", "drops an axis the model does not reference", "applies a per-model binding" |
| Model marks: registry icon vs arbitrary image | both marks remain in the default fixture (Atlas `icon`, Corvid `image`). Asserted by `ModelPicker.test.ts` → "renders an arbitrary image in place of a default Lucide icon" and "defaults image alt to empty" |
| Rebound axis / Many-level axis / Model with no axes at all | three instances inside **Axis control forms** |
| Emphasis: default vs subdued / Outlined trigger | three instances inside **Variants and emphasis** |
| Summary suppressed / Descriptions hidden | two instances inside **What the trigger shows** — kept on the catalogue because neither has a focused test |
| Models only (no axes declared) | asserted by `ModelPicker.test.ts` → "splits into models \| axes columns only when axes apply", which covers the no-axes-declared single-column surface exactly. Visually identical to the no-axis model already shown in **Axis control forms** |
| No model selected / Disabled | two instances inside **Nothing selected, and disabled** |
| Trigger only (collapsed) (GPUI) | the collapsed trigger is the `default` instance inside **Variants and emphasis** |

## Contract-critical stories kept

- **LicenceStatus** keeps one surface per usability state; both trust bases and
  all four coverage-window pairings survive across the five fixtures.
- **ModelConnectionSetup** visibly proves that a route with no required
  configuration skips the credential step: both instances sit on `choose` with
  the available `codex-app` option, whose `requiresConfiguration: false` makes
  the action Add instead of Continue and emits no configure stage at all. The
  regression asserts the stage, the action labels, the enabled/disabled Add,
  and the absence of any configuration surface, in both web runtimes and in the
  GPUI source.
- **ModelPicker** consolidates rather than deletes: contract §14's required
  specimen coverage is either on the curated page or in a named focused test,
  itemised above.

## New evidence

`test/parity/g15-020-model-account-specimens.test.tsx` (54 assertions, runs
under `effigy test:parity`). For this exact eight-page set it pins:

- the final ordered caption list per page, per runtime
- Svelte/React caption equality
- the 3–6 section budget, and non-blank captions
- no size/density matrix leaking back into `Examples`
- the two no-op pages at 5 and 6
- GPUI's ordered captions, read from each specimen's `group`/`section` helper
- LicenceStatus's retained contract stories: five surfaces, six instances, both
  trust bases, and exactly one both-windows-unbounded fixture
- ModelConnectionSetup's direct-add story: `data-stage="choose"`, action labels
  `["Cancel", "Add connection"]`, Add enabled when detected and disabled when
  not, and no `__configuration` surface — asserted in Svelte and React
- GPUI's matching seeds: the detected pair built from `interactive_options()`
  on `Choose`, and the open-details card seeded through
  `card_is_open(CARD_LIVE_ID, true)`

Each of the three assertions above was verified to fail against the pre-review
source before being accepted as green.

The GPUI assertion is source-structural. `g15.026` owns the headless page probe
and was not built here; until it lands, the declared caption order is the
narrowest deterministic native evidence available.

## Commands and results

| Command | Result |
| --- | --- |
| `effigy tasks` | selector inventory read |
| `effigy doctor` | 3 errors, 1 warning — recorded baseline, unchanged |
| `bunx vitest run --project parity test/parity/g15-020-model-account-specimens.test.tsx` | 54 passed |
| `effigy test:parity` | 5 files, 313 passed |
| `effigy check:svelte` | 0 errors (install-smoke, components, preview) |
| `effigy react:build` | built |
| `effigy check:gpui` | 326 + 19 passed, 0 failed |
| `effigy docs:check` | exit 0 |
| `git diff --check origin/main...HEAD` | clean |

Windowed, native-visual, conformance, Jetstream and release selectors were not
run, per the card.

### Doctor baseline (recorded, not absorbed)

`scan.generated-in-src` (30), `scan.god-files` (38),
`scan.stale-suppressions` (17), `scan.comment-ratio` (2). Present at the worker
base and unchanged by this card.

## One step outside the specimen files

`packages/gpui/preview/src/app_state.rs` — `card_is_open(&self, id)` gained a
`default` parameter, mirroring the `card_is_enabled(&self, id, default)` that
already sat beside it. Seeding the open-details example from the specimen alone
left `card_is_open` dead, and suppressing that would have traded a real warning
for a stale suppression. One call site, one signature, no behaviour change for
any other page. Flagged here because it is preview infrastructure rather than a
specimen file.

## Unresolved findings

- **ModelConnectionCard readiness domain is incomplete on the catalogue.** The
  contract defines six readiness values; the page shows four (`ready`,
  `checking`, `attention`, `unavailable`). `unknown` and `error` had no surface
  before this card and still have none. Adding them was outside the card's
  outline; worth a line on a later child or a focused test.
- **GPUI evidence is structural only** until `g15.026` supplies the headless
  page probe. Caption order is proven; rendered native layout is not.
- **`effigy bootstrap:deps` fails in a second worktree** on a Cargo lockfile
  collision. Recorded in `PAPERCUTS.md`; it cost nothing here because the bun
  half completes first and per-crate `cargo build` still resolves.

## Review rounds

Round 1 (PR #42) requested two blocking changes, both accepted as correct:

1. The `Auto-detected local route` section forced the no-configuration route
   onto `configure`, so the page taught the opposite of the contract story it
   claimed. Fixed in all three runtimes and pinned by new assertions.
2. GPUI's `Open details with catalogue` example seeded from an empty host map
   and therefore started closed, while Svelte and React seeded it open — the
   caption-only regression could not see the difference. Fixed and pinned.

Both were invisible to the first regression because it asserted caption text
only. The lesson is recorded in the evidence list above: a section that claims
a behaviour needs an assertion on that behaviour, not on its heading.

## Live operator review

Open until the orchestrator records it. The six changed routes are
`licence-status`, `model-catalogue-editor`, `model-connection-card`,
`model-connection-picker`, `model-connection-setup` and `model-picker`. The two
no-op licence routes need no review.
