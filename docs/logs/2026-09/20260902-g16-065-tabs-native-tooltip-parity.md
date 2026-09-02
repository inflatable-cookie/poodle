# g16.065 — Tabs Native Tooltip Parity

Status: in review — awaiting orchestrator merge
Date: 2026-09-02
Card: `docs/roadmaps/g16/065-tabs-native-tooltip-parity.md`
Handoff: `docs/handoffs/20260902-225241-g16-065-tabs-tooltips.md`
Governing refs: `docs/contracts/001-working-rules.md`,
`docs/contracts/components/tabs.md`
Branch: `fix/g16-065-tabs-native-tooltips`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-065-tabs-native-tooltips`

## Outcome

`showTooltips` now has one public meaning across Svelte, React, the Rust spec,
the renderer, and mounted GPUI: false is inert after the full delay; true (and
every vertical strip) exposes the hovered tab's trimmed label after a delay
and hides it when the hover target is gone.

Native uses the existing `Node.tooltip` → GPUI `.tooltip()` house path. No new
Node field, public prop, Nucleus source, or preview-only state. Web overlay
timing (300ms, blur, Escape) is unchanged. GPUI 0.2.2 hardcodes 500ms and is
hover-only.

## What landed

- Spec: `TabsSpec::with_shows_tooltips`.
- Renderer: `tab_tooltip_text` projects onto `Node.tooltip` from
  `wire_collection_semantics` (card, pill, and block). Vertical strips project
  even when the flag is false. Empty labels are omitted. Disabled tabs keep
  the label, matching web.
- GPUI: frame-scoped `painted_tooltip_text()` recorded from the tooltip view
  and cleared in `overlay_frame_begin`.
- Mounted proof: compact/`sm` chrome fixture (`Explorer`/`Search`/`Git`/
  `Terminal` icons), no Nucleus data. False inert at 500ms; true silent at
  300ms then `Search` at 500ms; leave, removal-while-pending, and teardown
  leave no late paint.
- Paired Svelte/React `TabsTooltips` suites with fake timers (not in
  `Tabs.test.*`, which stub `requestAnimationFrame` as sync).
- Contract §10 GPUI Notes and Known Deltas record the 500ms and hover-only
  deltas.

## Falsification

Green proofs first. Plants restored after each row.

| Row | Plant | Result |
| --- | --- | --- |
| Dropped projection | `node.tooltip = None` | `show_tooltips_true_projects_each_tab_label` expected `Some("Explorer")`, got `None` |
| Label-only projection | ignore `shows_tooltips` / vertical | `show_tooltips_false_does_not_project_hover_text` saw a tooltip |
| Skip delay | `wait_ms(0)` after the 300ms None row | mounted test expected `Some("Search")`, got `None` |
| Skip hide | omit leave `pointer_hover` | mounted test expected `None`, got `Some("Search")` |

Restored sources reran green for the renderer plants. Hide/delay plants were
restored in the mounted test before the native board.

## Validation

Focused:

- `bun run --cwd packages/core test test/tabs.test.ts` — 23 pass
- `cargo test --manifest-path packages/contracts/components/Cargo.toml shows_tooltips` — 1 pass
- renderer tooltip projection tests — 5 pass
- Svelte `TabsTooltips` + existing Tabs/controlled-focus files — 34 pass
- React `TabsTooltips` + existing Tabs/controlled-focus files — 34 pass
- `tabs_show_tooltips_delay_and_hide_through_mounted_gpui` — pass after restoring delay/hide plants

Boards:

- `effigy ci:web` — pass after warming `react:package` dist (first cold run failed three `react-preview` suites; papercut recorded)
- `effigy ci:rust` — pass
- `effigy ci:native` — pass, including 175 headless regressions
- `effigy docs:check` — pass
- `git diff --check` on the lane files — pass

## Limits

- Native delay is GPUI 500ms, not web 300ms.
- Native show/hide is hover-owned. Web still dismisses on blur and Escape.
- Disabled tabs still project labels (web wraps disabled items). Removal and
  teardown cancel a pending show.
- No Nucleus edit, windowed/native-visual run, Jetstream, workflow, or shared
  g16 front-door change.
