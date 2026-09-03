# g16.075 — Nucleus Popover M1 Receipt

Status: complete
Date: 2026-09-03
Card: `docs/roadmaps/g16/075-nucleus-popover-m1.md`
Handoff: `/Users/tom/Dev/projects/poodle/docs/handoffs/20260903-081000-g16-075-nucleus-popover-receipt.md`
Governing refs: `docs/roadmaps/g16/nucleus-gpui-parity-programme.md`,
`docs/roadmaps/g16/062-nucleus-parity-receipt-foundation.md`,
`docs/roadmaps/g16/nucleus-parity-manifest.json`,
`docs/roadmaps/g16/parity-evidence-ledger.md`,
`docs/contracts/components/popover.md`,
`docs/contracts/components/surface.md`,
`docs/architecture/002-anchored-overlays.md`
Branch: `feature/g16-075-nucleus-popover-receipt`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-075-nucleus-popover-receipt`
Planning base: `3b8253f98` (`origin/main`)
Worker PR: `https://github.com/inflatable-cookie/poodle/pull/181`

## Outcome

`Popover` now has a validated `M1` execution receipt emitted from the
production GPUI render, GPUI Popover compat adapter
(`node_compat::Popover::from_spec(...).into_element()`), node backend, and
test-platform path in `effigy regressions:native`. The retained regression
`a_nested_popover_paints_without_nesting_deferred_draws` was strengthened —
not replaced — into the executed controlled composition, and still proves
that two nested popover layers paint without a nested deferred draw.

The fixture proves, through `HeadlessDriver` mounted pointer/keyboard input
and host-owned rebuilds: the renderer-owned trigger/surface structure and
contract token profile (elevated fill, 74% alpha border-subtle, surface
radius, overlay elevation plus the inset top highlight, panel padding, dialog
role/name, instance-scoped runtime ids and layers); exact mounted geometry
(bottom-start separation by the authored offset, start alignment, 14rem/24rem
width bounds, exact pinned 20rem width); nested inner-first Escape dismissal;
inner-only outside dismissal from a press inside the outer surface but outside
the inner surface; outer dismissal restoring the outer trigger's backend
focus; sibling isolation with duplicate content (scoped layers, bounds,
callbacks, and traces); outside press on a sibling trigger dismissing the
open layer before scoped activation; disabled trigger inertia (no callback,
no surface, no layer); and the three initial-focus strategies witnessed at the
mounted focus-handle boundary (content focuses the surface, first-focusable
focuses the first focusable descendant, none moves no focus) with close
restoring the matching per-instance trigger. The receipt is emitted only
after the terminal assertion.

The manifest, all 11 existing receipts (`AppHeader`, `Button`, `Dialog`,
`Icon`, `IconButton`, `Menu`, `SegmentedControl`, `SplitView`, `Surface`,
`Tabs`, `Text`), and the new Popover receipt pin the exact runtime source
commit `744006ec4f9de27f46ec8389bac5da907bc20434`. The ledger records 12
mounted Nucleus rows out of 29.

## What landed

- Contracts:
  - `packages/contracts/components/src/popover.rs`: added unit tests for the
    contract defaults (`bottom-start`, offset 8, outside dismissal on,
    first-focusable, enabled, content surface width), the public builder
    surface, controlled `current_open` resolution, surface token recipes
    (elevated fill, border-subtle at 74%, overlay elevation), and the
    14rem/24rem effective width fallbacks and overrides.
  - `packages/contracts/headless/src/popover.rs`: added machine unit tests
    covering open/toggle effects with the declared focus strategy, every
    close axis emitting restore-trigger-focus, inert no-op events, the
    outside-interact guard, and the disabled guard in all directions.
- Renderer:
  - `packages/render/src/popover.rs`: added composition unit tests proving
    the contract surface token profile, open trigger/surface ownership with
    scoped runtime identity and placement roles, closed/disabled/trigger-width
    surface gating, initial-focus surface focusability, authored width-bound
    overrides, and kebab-cased placement roles.
- GPUI Node Backend:
  - `packages/gpui/node-backend/src/interaction.rs`: layer members (elements
    carrying a dismiss layer) observe left pointer presses in the capture
    phase and run the shared outside-dismissal check.
  - `packages/gpui/node-backend/src/layers.rs`: dismissed layer records are
    consumed before their handlers run, so the window host plus layer members
    cannot dismiss one layer twice for the same press; Escape consumes the
    innermost record the same way.
- Headless Regressions:
  - `packages/gpui/preview/tests/headless_regressions.rs`:
    `a_nested_popover_paints_without_nesting_deferred_draws` is now the M1
    fixture across five phases: production renderer composition proof (no
    mount), the mounted nested pair with dismissal order and geometry, mounted
    sibling isolation with duplicate content, mounted disabled inertia and
    exact fixed-width geometry, and the mounted focus-strategy trio with
    scoped restoration and terminal receipt emission.

## Focused repair (committed counterexample first)

The strengthened fixture first failed at the mounted nested phase: a pointer
press inside the outer popover surface but outside the nested inner surface
never dismissed the inner layer. gpui 0.2.2 does not route pointer events over
deferred overlay content to window-level (root) listeners, so the window host
alone cannot run outside dismissal for presses inside an enclosing overlay.
The counterexample was committed (red), then repaired on the existing
dismiss-stack machinery: every element carrying a dismiss layer now observes
left presses in the capture phase (where activation and click-synthesis
handlers cannot suppress the dismissal) and dismissed layer records are
consumed before their handlers run so the host and member listeners never
dismiss one layer twice. No new public API, overlay machine, or app-owned
focus policy was introduced.

## Review oracle falsification

| Invariant | Smallest counterexample | Required proof / Observed failure |
| --- | --- | --- |
| Production adapter owns execution | mount `poodle_render::popover` directly | the fixture mounts `node_compat::Popover::from_spec(...).into_element()` through `HeadlessDriver::new_element_in_box`; a renderer-only Node fixture would carry no adapter-path observation |
| Nested draw remains valid | defer the inner overlay independently | two scoped layers paint through `draw_frame` without a deferred-draw panic |
| Layer identity is scoped | reuse one layer/runtime id for outer and inner | inner dismissal leaves the outer surface mounted; layer ids `popover-layer:nested` / `popover-layer:inner` stay separate |
| Sibling identity is isolated | compose duplicate-valued sibling instances without scopes | Escape closes only the innermost sibling; callbacks, bounds, and focus targets stay per-scope |
| Input is mounted | call activation/dismiss handlers directly | every claim is driven through `pointer_activate_id`, `pointer_press`/`release`, or `dispatch_key` |
| Controlled ownership is real | record callback without rebuilding supplied open state | surface presence tracks host open state at every frame boundary |
| Dismissal order is exact | let Escape/outside close both nested layers | inner-first Escape trace with the retained outer surface; inner-only outside dismissal from an outer-surface press |
| Disabled trigger is inert | bind activation while disabled | `locked:open` absent, no trace, no surface, no layer |
| Focus strategy is exact | focus the surface for first-focusable or move focus for none | content lands on the surface handle; first-focusable lands on the first focusable descendant; none leaves the trigger |
| Focus restoration is scoped | restore the wrong trigger after nested close | each closed instance regains its own trigger handle, never the sibling's |
| Geometry and tokens are exact | alter placement offset, width, border alpha, padding, or elevation | token/role structure asserted at the renderer node; mounted gap equals the authored offset and mounted width equals the pinned 20rem bound |
| Receipt is terminal | fail a final nested/sibling assertion | no Popover receipt is emitted unless every mounted phase completes |
| Evidence identity is exact | retain the g16.074 source SHA | receipts and manifest pin `744006ec4`; receipt validation and `currentSourceMatchesReceipt` pass |
| Levels stay separate | label the receipt A1 or V1 | schema validation requires `proof_level` M1 and rejects A1/V1 labels |

## Validation

Focused:
- `cargo test --manifest-path packages/contracts/components/Cargo.toml --lib popover::` — 7 passed
- `cargo test --manifest-path packages/contracts/headless/Cargo.toml --lib popover::` — 6 passed
- `cargo test --manifest-path packages/render/Cargo.toml popover::` — 6 passed
- `cargo test --manifest-path packages/gpui/preview/Cargo.toml --test headless_regressions a_nested_popover_paints_without_nesting_deferred_draws` — passed (all five phases)
- `bun test scripts/nucleus-parity-receipts.test.ts` — 8 passed
- `bun test scripts/parity-evidence-ledger.test.ts` — 6 passed
- `bun scripts/parity-evidence-ledger.ts --write` then `bun scripts/parity-evidence-ledger.ts` — 176 component evidence rows validated

Required boards:
- `effigy regressions:native` — 187 passed (all 12 receipts emitted at runtime commit `744006ec4`)
- `effigy check:parity-evidence-ledger` — passed
- `effigy ci:rust` — passed
- `effigy ci:native` — passed
- `effigy docs:check` — passed
- `git diff --check origin/main...HEAD` — clean

No windowed or native-visual selectors were run.

## Limits

- `M1` proves the mounted production render/adapter/backend path: renderer
  metadata and tokens, exact mounted geometry, controlled rebuilds, nested
  inner-first dismissal, sibling isolation, disabled inertia, and the
  initial-focus strategies only at the mounted focus-handle boundary. It does
  not claim `A1` (accessibility tree semantics), `V1` (pixel comparison),
  browser portal or collision parity, or Nucleus adoption.
- Merge remains orchestrator-owned.
