# g16.091 — Nucleus ToastHost M1 Receipt

Status: review — runtime repaired; upstream native-board failure recorded
Date: 2026-09-03
Card: `docs/roadmaps/g16/091-nucleus-toast-host-m1.md`
Handoff: `/Users/tom/Dev/projects/poodle/docs/handoffs/20260903-221500-g16-091-nucleus-toast-host-receipt.md`
Branch: `feature/g16-091-nucleus-toast-host-receipt`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-091-nucleus-toast-host-receipt`
Planning base: `4a8dd5018da6feb30f5905a5da195cda03640fc5`
Preparation-accepted head: `5b9d9fa1f8dc5cdf9010c35a0e773986720acbb8`
Finalization base: `420b9a7b1b6ab40f32f3936b5bbc2483a180b0ae`
DetailItem closeout ancestor: `420b9a7b1b6ab40f32f3936b5bbc2483a180b0ae`
Rebased preparation head: `b2ffd421d26af13f38169984040f9c25eed14917`
Runtime source: `0f04083c9dee61d01722104e7403368559f0b590`
PR: `#197`

## Outcome

`ToastHost` now has one validated `M1` receipt through the production Rust
compatibility adapter, shared renderer, real Toast, Button, and Icon
composition, Node backend, and mounted GPUI test platform. The retained
`toast_host_controlled_composition_actions_and_identity_through_mounted_backend`
fixture emits only after its terminal controlled teardown, identity, focus,
callback, and backend-state assertions.

All 28 cohort receipts and the manifest pin runtime source
`0f04083c9dee61d01722104e7403368559f0b590`. The generated Nucleus ledger
advances only ToastHost: 28/29 mounted. The full evidence ledger records 28
mounted and 147 missing GPUI behaviour cells. M1 does not infer A1 or V1.

## Production proof

- Caller-scoped `node_compat::ToastHost::from_spec(...).into_element()`
  instances reach `HeadlessDriver` through the production adapter, renderer,
  Node backend, and GPUI element path.
- All four placements retain exact authored one-rem edges and the 28rem host
  cap. Stack direction, row order, non-overlap, and host, row, action, dismiss,
  and Icon containment are exact; shifted and double-inset stacks are rejected.
- Info, success, warning, and danger preserve exact fill, border, accent,
  elevation, focus, spacing, size, density, and size-role tokens through the
  mounted renderer path.
- Actions use real secondary Button composition. Dismiss controls use real Icon
  composition. Mounted pointer and keyboard input prove live, inert, and
  duplicate-id callback paths without cross-host dispatch.
- Controlled replacement retains same-id focus identity. Explicit removal is
  host-local. Advancing deterministic headless time does not remove controlled
  rows or manufacture callbacks because native rendering owns no timeout clock.
- Terminal empty queues clear both hosts and every row, action, dismiss, Icon,
  focus, ring, and backend identity before further keyboard input.

## Committed falsification and repair

Range-diff preserves the full accepted preparation series across the rebase
onto `420b9a7b1b6ab40f32f3936b5bbc2483a180b0ae`:

- Production identity counterexample `7690aeb40` maps exactly to `0e9fed316`;
  its repair `9b493726c` maps exactly to `e7a3cd43a`.
- Token-spacing counterexample `1118a6d89` maps exactly to `b67e619a4`.
- Placement counterexample `636778932` maps exactly to `4784e0101`.
- Mounted token counterexample `8d27aeb86` maps exactly to `e2ccd643d`.
- Teardown counterexample `17fa8b582` maps exactly to `14b4df3d1`.
- The bounded preparation repair `5b9d9fa1f` maps exactly to `b2ffd421d`.

Runtime commit `740f3cb16` adds terminal receipt emission without changing the
accepted proof. Test-only commit `f089919b8` adds the cross-window focus-sweep
counterexample: before repair it did not complete and was manually stopped.
Repair commit `0f04083c9` keys painted focus identities and their lost-host
sweep by `AnyWindowHandle`; it is the repaired runtime source.

## Receipt identity

| Field | Value |
| --- | --- |
| File | `docs/roadmaps/g16/nucleus-parity-receipts/toasthost--nucleus-attention-toast-host.json` |
| Component | `ToastHost` |
| Scenario | `nucleus.attention.toast-host` |
| Proof level | `M1` |
| Runtime | `gpui-headless` |
| Command | `effigy regressions:native` |
| Source commit | `0f04083c9dee61d01722104e7403368559f0b590` |
| Outcome | `passed` |

## Validation

- Focused Toast tests passed 45/45; renderer Toast tests passed 7/7; Node
  backend passed 51/51; GPUI adapter passed 134/134; focus-filtered mounted
  tests passed 26/26; named mounted ToastHost tests passed 3/3. The new
  cross-window focus-sweep test and the formerly hanging g16.026 drag-isolation
  test each passed alone in 0.00s after the repair.
- `effigy test:nucleus-parity-receipts` — 8 passed.
- `effigy test:parity-evidence-ledger` — 6 passed.
- `effigy check:parity-evidence-ledger` — 176 rows validated after generation.
- `effigy ci:rust` — clean.
- `effigy docs:check` — clean.
- `git diff --check` — clean.
- `effigy regressions:native` completed: 202/202 passed and all 28 receipts
  were emitted from exact runtime source
  `0f04083c9dee61d01722104e7403368559f0b590`.
- `effigy ci:native` is failed, not passed. It completed the drift, build,
  render/backend/adapter, 202-test native-regression, and 9-test specimen-probe
  stages. Its fresh downstream dual-dependency consumer then resolved
  crates.io `tinyvec 1.13.0` and failed inside that dependency with `cannot
  find macro vec`; the negative control consequently failed to reach its
  intended type mismatch. Later consumer and capture stages did not run.
- A direct consumer rerun under the repository minimum Rust 1.95.0 completed
  with the same `tinyvec 1.13.0` compile failure; this is not an active-toolchain
  substitution.

## Limits

- ToastHost M1 only. No other Nucleus row advances.
- No A1, V1, Nucleus, web, g16 front-door, MessageCenter, workflow, release,
  version, or native-visual change.
- MessageCenter remains planning-only.
- No windowed or native-visual selector ran.
- No merge and no next card.
