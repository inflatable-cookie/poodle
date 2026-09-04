# g16.093 — Nucleus MessageCenter M1 Receipt

Status: review — runtime and 29/29 cohort emitted
Date: 2026-09-04
Card: `docs/roadmaps/g16/093-nucleus-message-center-m1.md`
Handoff: `/Users/tom/Dev/projects/poodle/docs/handoffs/20260903-223100-g16-093-nucleus-message-center-m1.md`
Branch: `feature/g16-093-nucleus-message-center-receipt`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-093-nucleus-message-center-receipt`
Planning base: `5020cab3b`
Preparation-accepted head: `79d46311599aa3a5aae370d9dbaf5e93936e06cd`
Finalization base: `3dc0c736108c53392a4e93f85beec0d1508b5573`
g16.092 merge: `17534f484665bbbdd93e2ec70bec521318201941`
Rebased preparation head: `b6330aa50721f77d69e79f4fb179306b1704f5ab`
Runtime source: `f0774a7d15a195cc6b8506c5da68db99807e5376`
PR: `#198`

## Outcome

`MessageCenter` now has one validated `M1` receipt through the production Rust
compatibility adapter, shared renderer, real IconButton, Popover, Button, Icon,
TimeAgo, EmptyState, Progress, and StatusIndicator composition, Node backend,
and mounted GPUI test platform. The retained
`message_center_composition_open_progress_and_identity_through_mounted_backend`
fixture emits only after its terminal teardown, identity, focus, callback,
scroll, and backend-state assertions.

All 29 cohort receipts and the manifest pin runtime source
`f0774a7d15a195cc6b8506c5da68db99807e5376` and preview lock digest
`c86c2d11c36c9fcf9326bae438ee6acc3bcedacbaf01ac017a298c1bd3c2a34c`. The
generated Nucleus ledger advances only MessageCenter: 29/29 mounted. The full
evidence ledger records 29 mounted and 146 missing GPUI behaviour cells. M1
does not infer A1 or V1.

## Production proof

- Caller-scoped `node_compat::MessageCenter::from_spec(...).into_element()`
  instances reach `HeadlessDriver` through the production adapter, renderer,
  Node backend, and GPUI element path.
- Open/dismiss run the production Popover machine with `initialFocus=content`.
  Opening focuses the labelled dialog; Escape restores the matching trigger.
  The fixture does not call `request_focus`.
- Duplicate open hosts register distinct live layers. Innermost Escape and an
  outside press on the spared sibling dismiss only that layer. Duplicate item
  ids never cross callbacks.
- Selectable rows keep `radius.control`, surface 72% hover fill, and the inset
  accent focus ring.
- Overflowing lists scroll under mounted wheel input. Post-scroll list size
  stays at the 24rem cap; the current list host stays inside the current
  surface; a later row enters the current host.
- Live rows stay inert. Progress is a host projection. Controlled refusal
  leaves the dialog unmounted. Terminal empty hosts clear surfaces, items,
  paint, and focus identity. MessageCenter owns no timeout clock.

## Committed falsification and repair

Range-diff preserves the accepted preparation series across the rebase onto
`3dc0c736108c53392a4e93f85beec0d1508b5573`:

- Production identity counterexample `8e8b3618c` maps exactly to `5dd84d2b5`;
  its repair `7b323cedf` maps exactly to `1c64b034e`.
- Focus/layers/tokens/scroll counterexample `b4f95ce54` maps exactly to
  `6cf53b4f4`; its repair `79d463115` maps exactly to `b6330aa50`.

Runtime commit `f0774a7d1` re-fetches list, surface, and list-host bounds after
mounted wheel input and emits the terminal receipt. It does not change the
accepted production proof beyond that containment tightening.

## Receipt identity

| Field | Value |
| --- | --- |
| File | `docs/roadmaps/g16/nucleus-parity-receipts/messagecenter--nucleus-attention-message-center.json` |
| Component | `MessageCenter` |
| Scenario | `nucleus.attention.message-center` |
| Proof level | `M1` |
| Runtime | `gpui-headless` |
| Command | `effigy regressions:native` |
| Source commit | `f0774a7d15a195cc6b8506c5da68db99807e5376` |
| Lock digest | `c86c2d11c36c9fcf9326bae438ee6acc3bcedacbaf01ac017a298c1bd3c2a34c` |
| Outcome | `passed` |

## Validation

- Focused MessageCenter contract tests 4/4; renderer 6/6; GPUI adapter 134/134;
  named mounted proof 1/1.
- `effigy regressions:native` — 203/203 passed; all 29 receipts emitted from
  runtime source `f0774a7d15a195cc6b8506c5da68db99807e5376` with the g16.092
  preview lock digest
  `c86c2d11c36c9fcf9326bae438ee6acc3bcedacbaf01ac017a298c1bd3c2a34c`.
- `effigy test:nucleus-parity-receipts` — 8 passed.
- `effigy test:parity-evidence-ledger` — 6 passed.
- `effigy check:parity-evidence-ledger` — 176 rows validated after generation.
- `effigy ci:rust` — clean.
- `effigy ci:native` — clean. Dual-dependency consumer compiles; tinyvec
  resolves 1.13.0 with std; capture smoke passes. No windowed or native-visual
  selector ran.
- `effigy docs:check` — clean.
- `git diff --check` — clean.

## Limits

- MessageCenter M1 only. No other Nucleus row changes meaning; the cohort is
  29/29 mounted at this runtime source.
- No A1, V1, Nucleus, web, g16 front-door, workflow, release, version, or
  native-visual change.
- No windowed or native-visual selector ran.
- No merge and no next card.
