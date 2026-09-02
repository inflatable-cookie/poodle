# g16.047 — Toast Same-Id Update Convention

Status: implemented — awaiting orchestrator review
Date: 2026-09-02
Card: `docs/roadmaps/g16/047-toast-same-id-update-convention.md`
Handoff: `docs/handoffs/20260902-004202-g16-047-toast-same-id.md`
Governing refs: `docs/contracts/components/toast-host.md`,
`docs/contracts/components/toast-stack.md`,
`docs/specs/015-loading-empty-error-notification-and-remediation-rules.md`,
`docs/contracts/003-native-accessibility.md`,
`docs/architecture/012-semantic-motion-policy.md`
Branch: `feature/g16-047-toast-same-id`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-047-toast-same-id`
Planning base: `7f59ae42f4917c675968819eb23a5e41dc90013c` (ancestor)
Live `origin/main` at dispatch: `c1a527898e7425853359bd72b7113a8cf38b8d97`
Rebased onto live `origin/main`: `ccb2ebd5bec2627b28f3cf589d4e4297780438ba`
(includes merged g16.053 PR #150 and g16.055 PR #151)

## Outcome

Host-owned same-id pending-to-settled updates are now a documented convention,
not a lifecycle field. One live row per id. Sticky rows own no clock.
Become-sticky cancels a running clock; become-non-sticky starts exactly the
current configured delay. `6000` and `["danger"]` stay default fixtures.
Settlement keeps the visual row and phase, announces once, restores action
focus through dismiss → next → previous → entered-from, and maps native
danger to `NodeRole::Alert` without a GPUI AT claim.

Toast still does not own operations, promises, progress, retry, or a second
creation API.

## What landed

- Contracts: uniqueness, configured timer table, discrete announcement,
  action-focus fallback, spec 015 durable-failure rule, native Alert/ListItem.
- Core: `uniqueToastInputs`, repaired `reconcileToastTimers` (`delayMs` on
  the plan), `moveToastFocusFromRemovedAction`, same-id `nextToastVisuals`
  proof.
- ToastHost Svelte/React: unique snapshots, `plan.delayMs`, config changes
  re-run the plan without resetting a running non-sticky clock. Stable
  module-level `stickyTones` default so React does not loop.
- ToastStack Svelte/React: `data-toast-id`, action-label focus keep, action
  removal fallback, pending-to-settled tests with no percent in copy.
- Shared Rust: danger `NodeRole::Alert`, other tones `ListItem`, crate test
  plus mounted GPUI `mounted_toast_danger_uses_alert_role`.
- Specimens: same-id settle on Svelte, React, and GPUI ToastHost and
  ToastStack paths.

## Falsification

Committed proofs first. Plants used a clean index and `git checkout --`.

| Row | Plant | Result |
| --- | --- | --- |
| Become-sticky | clear only departed ids | `plan.clear` empty instead of `["job"]` |
| Custom stickyTones become-sticky | same plant | same empty clear |
| Duplicate last-fields | same plant | danger duplicate did not clear |
| Custom delay 2500 | `delayMs` hardcoded to 6000 | expected 2500, received 6000 |
| Disabled / negative expiry | drop `autoDismissMs <= 0` guard | start included `job` / `a` |
| Same-id phase | always enter | expected settled, received enter |
| Focus fallback | skip action-removal restore | focus landed on `body` |
| Action removal after focus left | preserve stale `focusedActionId`, no stack-leave clear | expected Outside, received Dismiss Publish |
| Native Alert | always ListItem | expected Alert, received ListItem |
| API-zero | add `createToastPromise` | source scan matched the helper |

Restored sources reran green.

## Validation

Focused, then required boards, including a rerun after rebasing onto
`ccb2ebd5bec2627b28f3cf589d4e4297780438ba`:

- `bun run --cwd packages/core test test/toast.test.ts test/motion-runtime.test.ts`
  — 26 pass, 0 fail
- `bunx vitest run` ToastHost, ToastStack, motion-families (Svelte + React),
  and SSR ToastStack — 7 files, 62 pass
- `cargo test --manifest-path packages/render/Cargo.toml danger_uses_alert_and_other_tones`
  — pass
- `cargo test --manifest-path packages/gpui/preview/Cargo.toml --test headless_regressions mounted_toast_danger_uses_alert_role`
  — pass
- `effigy ci:web` — pass (svelte-check 0 errors; existing warnings only)
- `effigy ci:rust` — pass
- `effigy ci:native` — pass, including handler/event/adapter/presentation
  drift, render/node-backend, GPUI, Jetstream adapter, headless regressions,
  specimen probe, consumer-identity, and window-capture smoke
- `effigy docs:check` — pass; 176 evidence rows, 0 recipe/surface drift
- `effigy qa` — pass, including `audit:security` after the g16.053 matcher
  repair
- `git diff --check origin/main...HEAD` — pass

## Review repair (PR #152)

React cleared the action-focus latch only on a later action focus, so same-id
action removal stole focus back onto dismiss after the user had already moved
to an external control. The latch now tracks current action ownership and
clears when focus leaves the action or stack; unmount-while-focused still
restores. Paired Svelte already used live `document.activeElement` and did not
steal. Plant: preserve stale `focusedActionId` / skip stack-leave clear →
activeElement was Dismiss Publish, expected Outside.

## Unresolved

- Consumer stores still append; upsert adoption is consumer-owned.
- GPUI still has no assistive-technology mapping. Alert is node metadata.
- Jetstream remains deferred. Shared render role change flows through the
  existing node tree; no Jetstream behavior was edited.
