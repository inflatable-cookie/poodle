# g16.110 — `gpui-unofficial` Feasibility Spike

Status: ready
Type: disposable feasibility spike — throwaway branch, report only, no merge
of production changes
Opened: 2026-09-05
Depends on: none
Governing refs: `../../contracts/003-native-accessibility.md`,
`../../release-notes/0.2.2.md` (the crate-identity defect and its gate),
`test/consumer-dual-dependency/run.ts` (`drift:gpui-consumer-identity`),
`deny.toml` (`allow-git = []`, licence allow list),
`docs/roadmaps/g12/015-native-accessibility-options.md`,
`../../research/gpui-cratesio-nonactivating-capture.md`,
Poodle triage `20260901-233708-*` (native pair findings)
Operator decision: 2026-09-05 — run the spike; adopting the upstream
republish is the intended route to GPUI accessibility rather than building
an AccessKit adapter; work around licence issues but never vendor the
library
Dispatch manifest: `../dispatch.md`

## Facts (verified 2026-09-05)

- `gpui-unofficial` is an automated, unmodified republish of upstream
  `crates/gpui` at every Zed release tag (Apache-2.0; weekly cadence; latest
  stable 1.18.1 on 2026-09-04; 1.19.0-pre on 2026-09-02). Its sibling crates
  are suffixed `-gpui-unofficial`.
- Upstream gpui gained AccessKit on 2026-05-27 (PR #56065); every republished
  version from Zed v1.8 onward carries it. Upstream also split into `gpui`,
  `gpui_platform`, and per-OS backend crates since 0.2.2.
- `ztracing`, `zlog`, and `ztracing_macro` were GPL-3.0-or-later through
  v1.18.1 and were relicensed Apache-2.0 upstream on 2026-09-01 (#63573).
  The republished 1.19.0-pre already carries the Apache label; the first
  stable tag with it will be v1.19.0.
- Nothing outside Poodle ships Poodle GPUI today; Longhorn's GPUI prototypes
  are the only other portfolio GPUI code.

## Goal

Measure, on a throwaway branch, what it costs to move Poodle's native pair
from crates.io `gpui = "0.2.2"` to `gpui-unofficial` at the first
licence-clean version, and prove whether AccessKit attributes reach the GPUI
backend from the `poodle-node` accessibility record. Produce a report and a
recommendation. Merge nothing to `main` except the report.

## Fixed Boundary

- Branch `spike/gpui-unofficial`; never rebased onto or merged into `main`.
  The deliverable is `docs/logs/2026-09/<date>-g16-110-gpui-unofficial-spike.md`
  plus a papercut list, committed on `main` through an ordinary docs PR.
- Target version: `gpui-unofficial = "1.19.0-pre"` (first Apache-clean
  republish). If a stable `1.19.x` publishes during the spike, use it and
  say so. Do not use `1.18.x` or older for any licence claim.
- Replace `gpui` with `gpui-unofficial` (and its platform/backend siblings
  as upstream now requires) in `packages/gpui/adapter`, `node-backend`,
  `preview`, and `packages/jetstream/*` only where they compile against
  gpui. Rename imports as needed. Record every API delta by category:
  crate split, renamed items, removed items, behaviour changes in
  headless/test-support, window capture, focus, and text input.
- Do not vendor, fork, `[patch]`, or git-source any crate. If a crate cannot
  be resolved from crates.io, that is a finding, not a workaround.
- Licence: run `cargo deny check licenses` on the spike graph. The expected
  result is green at 1.19.0-pre with no `deny.toml` change; if any crate
  still resolves GPL, name it and stop that path (no exceptions added).
- AccessKit reachability: from the existing `poodle-node` accessibility
  record (role, label, states, values, relationships), project onto the
  upstream element attributes (`role`, `aria_label`, numeric values, actions)
  for three components with distinct semantics (Checkbox, Slider, Tabs) and
  prove through a headless test that the projected attributes appear in
  gpui's accessibility tree, plus one `on_a11y_action` round trip. Three
  components, not the cohort.
- Validation on the spike branch: `effigy check:gpui`, `effigy gpui:test`,
  `effigy regressions:native`, `effigy probe:gpui-specimens`,
  `effigy drift:gpui-consumer-identity` adapted to declare
  `gpui-unofficial` on the consumer side (record whether the gate can be
  re-pointed without weakening), `cargo deny`. Never run windowed
  selectors.
- Time box: two working days of worker time. Report what is done at the box.

## Report Contents

1. Compile delta: files touched, line counts, and the categorised API
   changes; anything that needed a design decision rather than a rename.
2. Test results on the spike branch against the same selectors on `main`.
3. Licence result per crate; confirmation that no GPL crate resolves at the
   target version.
4. AccessKit proof: the three-component attribute projection and the action
   round trip, with test names.
5. Portfolio identity: what Longhorn's GPUI prototypes and a future Nucleus
   shell must declare; how `drift:gpui-consumer-identity` is re-pointed.
6. Continuity: how Poodle would run the republish `xtask` itself if the
   upstream pipeline stopped (read its `xtask`; do not run it).
7. Recommendation: adopt at `1.19.0`, adopt later, or reject, with the
   estimated size of the real migration card.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Nothing vendored or git-sourced | any `[patch]`, path, or git dep in the spike graph | `cargo metadata` shows crates.io sources only |
| Licence claim is real | a GPL crate resolves at the target | `cargo deny` transcript in the report |
| AccessKit proof is executed | attributes asserted from source, not the tree | headless test reads gpui's accessibility tree |
| Main is untouched | spike code on `main` | only the report PR touches `main` |
| Time box respected | open-ended porting | report at two days, with remaining work listed |

## Owned Paths

The spike branch (any file under `packages/gpui/`, `packages/jetstream/`,
`packages/render/` as needed, `deny.toml` read-only), the report under
`docs/logs/2026-09/`, root `PAPERCUTS.md` (append only). Nothing else on
`main`.

## Stop Conditions

Stop when the platform split requires a change to `poodle-node` or
`poodle-render` vocabulary (report the change, do not make it), when a
crate cannot be resolved from crates.io, or at the time box. Escalation
owner: Chatterbox.

## Continuation

A green report becomes the migration card: `gpui-unofficial = "1.19.x"`
across the native pair, gate re-pointing, Longhorn prototype alignment, and
the A2 accessibility programme on upstream's AccessKit. A red report
reopens the fork-free macOS adapter as the fallback.
