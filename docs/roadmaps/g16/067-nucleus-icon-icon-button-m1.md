# g16.067 — Nucleus Icon And IconButton M1 Receipts

Status: ready
Type: Nucleus NP-1 mounted receipt child
Opened: 2026-09-03
Depends on: completed `g16.062`, completed `g16.066`
Governing refs: `nucleus-gpui-parity-programme.md`,
`062-nucleus-parity-receipt-foundation.md`,
`nucleus-parity-manifest.json`, `parity-evidence-ledger.md`,
`../../contracts/components/icon.md`,
`../../contracts/components/icon-button.md`

## Goal

Produce validated `M1` receipts for the Nucleus `Icon` and `IconButton` rows
through the production Rust render, Node, GPUI backend, and test-platform
paths. Treat `IconProvider` as fixture setup, not a thirtieth rendered row.

## Fixed Boundary

- One mounted Icon scenario installs the normal icon registry/provider path,
  renders a named icon through `poodle_render`, and proves its resolved glyph,
  token size, tint, and explicit accessible label reach the painted backend.
- The existing mounted IconButton scenario remains the interaction authority.
  It must prove pointer and keyboard activation, toggle state, disabled
  inertness, and the merged 300ms tooltip lifecycle through dispatched input.
- Emit one deterministic `M1` receipt per manifest row only after its named
  mounted test passes inside `effigy regressions:native`.
- Refresh the manifest resolution, all existing receipts, and the generated
  ledger from the exact committed runtime source. A stale Button receipt may
  not be hand-edited or silently discarded.
- `M1` does not imply `A1`, `V1`, Nucleus adoption, or publication.

## Acceptance

- `Icon` has a named mounted test and a validated receipt for
  `nucleus.shell.icon`.
- The Icon fixture uses the ordinary registry/provider setup and production
  render/backend path; a directly constructed GPUI SVG is not evidence.
- `IconButton` has a validated receipt for `nucleus.shell.icon-button` tied to
  its mounted pointer/keyboard test.
- Removing the icon registry entry, bypassing the render path, skipping input
  dispatch, or emitting either receipt without executing its test fails.
- The Nucleus denominator remains 29 plus the non-rendered IconProvider
  prerequisite. Button remains mounted and no other row advances.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Icon uses the production path | paint an SVG directly in the fixture | receipt observation or mounted assertion fails |
| Provider is setup only | emit an IconProvider receipt or row 30 | manifest/ledger validation fails |
| Registry is real | omit the named icon from the installed registry | mounted Icon proof fails before receipt emission |
| IconButton input is dispatched | call its handler directly | production observation or action proof fails |
| Receipt means execution | write either JSON file without running its test | emitter/validator counterexample rejects it |
| Evidence identity is exact | keep the prior Button source SHA | receipt validation fails after source movement |
| Levels stay separate | label either receipt A1 or V1 | schema validation fails |

## Writable Scope

Focused Icon/IconButton shared Rust or GPUI repair only if a mounted
counterexample proves one is required; mounted native fixtures and receipt
emission; Nucleus manifest/receipts/ledger tooling or generated evidence;
this card; one execution log; and new papercuts. Do not edit Nucleus, web
component behavior, public APIs, accessibility authority, visual-lab code,
Jetstream, workflows, versions, release surfaces, or other component rows.

## Validation

Run focused Rust/render/backend tests, the named Icon and IconButton mounted
tests, the real `effigy regressions:native` receipt run,
`effigy check:parity-evidence-ledger`, `effigy ci:rust`, `effigy ci:native`,
`effigy docs:check`, and `git diff --check origin/main...HEAD`. Do not run
windowed or native-visual selectors.

## Stop Conditions

Stop for orchestrator review if Icon requires a new public registration API,
if the fixture cannot observe the production paint path, or if closing these
rows requires Nucleus data, visual judgment, or accessibility claims.

## Continuation

After merge, compile the next small NP-1 receipt child from the new validated
manifest state. Because receipts pin the shared native source identity, later
native receipt cards merge serially even when their implementation scopes are
otherwise independent.
