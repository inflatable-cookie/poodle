# g16.065 — Tabs Native Tooltip Parity

Status: complete — merged in PR #172 at `718d6f082`
Date: 2026-09-03
Card: `docs/roadmaps/g16/065-tabs-native-tooltip-parity.md`
Handoff: `docs/handoffs/20260902-225241-g16-065-tabs-tooltips.md`
Governing refs: `docs/contracts/001-working-rules.md`,
`docs/contracts/components/tabs.md`
Branch: `fix/g16-065-tabs-native-tooltips`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-065-tabs-native-tooltips-resume`
Planning base: `d0e07ed65` (`origin/main` after g16.066 / PR #171)
PR: https://github.com/inflatable-cookie/poodle/pull/172

## Outcome

`showTooltips` has one public meaning across Svelte, React, the Rust spec,
the renderer, and mounted GPUI. False stays inert after 300ms on hover and
keyboard focus. True, and every vertical strip, projects the trimmed tab
label onto `Node.tooltip`. Horizontal `showTooltips=true` schedules on
keyboard focus and paints at 300ms, matching native `Node.tooltip`. Disabled
tabs never pending/visible on web, including when the live pending or
visible target becomes disabled; native still projects the label and the
g16.066 backend refuses the timer. Web tooltip state latches tab value, so
removing Search cannot paint Git at the vacated index. The merged g16.066
backend owns delay and dismiss. No new Node field.

## What landed

- Spec: `TabsSpec::with_shows_tooltips`.
- Renderer: `tab_tooltip_text` from `wire_collection_semantics` (card, pill,
  block). Vertical strips project when the flag is false. Empty labels are
  omitted. Disabled tabs keep the label; the backend does not start a timer.
- Mounted proof: compact/`sm` chrome fixture (`Explorer`/`Search`/`Git`/
  `Terminal`), no Nucleus data. False inert at 300ms; true silent at 299ms
  then `Search` at 300ms; leave, blur-while-hovered, Escape, disabled Git,
  removal-while-pending, and teardown leave no late paint.
- Tooltip machine: `FOCUS_ENTER` matches `POINTER_ENTER`. Adapters must not
  send either ENTER for a disabled item.
- Web: `scheduleTooltip` returns after dismiss when `hasTooltips` is false or
  the live item is disabled. `onFocus` schedules whenever `hasTooltips`, not
  only when vertical. Paint requires the live item is not disabled. The
  pending or visible target becoming disabled or removed cancels the timer and
  value identity before paint. Reorder keeps the tooltip on that tab.
- Paired Svelte/React `TabsTooltips` suites with fake timers, including
  disabled never pending/visible, horizontal 299/300ms keyboard focus,
  rerender disablement while Search is pending or visible, and rerender
  removal of pending/visible Search (no late Git tooltip).

## Falsification

Green proofs first. Plants restored after each row.

| Row | Plant | Result |
| --- | --- | --- |
| Dropped projection | `node.tooltip = None` | `show_tooltips_true_projects_each_tab_label` expected `Some("Explorer")`, got `None` |
| Label-only projection | ignore `shows_tooltips` / vertical | `show_tooltips_false_does_not_project_hover_text` saw a tooltip |
| Skip delay | `advance_clock(0)` after the 299ms None row | mounted test expected visible `Search`, got none |
| Skip hide | omit leave hover | mounted test expected none, got `Search` |
| Disabled still paints | omit live disabled gate in `scheduleTooltip` | paired disabled proof kept Search visible after entering Git |
| Horizontal focus silent | keep `onFocus` behind `isVertical` | paired 300ms keyboard proof expected `Search`, got none |
| Disable while pending | keep timer after Search is disabled | paired pending rerender rematerialized `Search` on re-enable |
| Disable while visible | paint-gate only | paired visible rerender rematerialized `Search` on re-enable |
| Remove while pending | index-latched timer | paired pending removal painted `Git` at 300ms |
| Remove while visible | stale index after Search leaves | paired visible removal showed `Git` |

## Validation

Focused:

- `bun run --cwd packages/core test test/tabs.test.ts` — 24 pass
- `cargo test --manifest-path packages/contracts/components/Cargo.toml shows_tooltips` — 1 pass
- renderer `tabs::` — 22 pass, including 5 tooltip projection tests
- Svelte/React `TabsTooltips` plus Tabs/controlled-focus/roving/subject files — 92 pass
- `tabs_show_tooltips_delay_and_hide_through_mounted_gpui` — pass

Boards (this exact-head repair):

- `effigy ci:web` — pass
- `effigy docs:check` — pass
- `git diff --check origin/main...HEAD` — pass
- `effigy ci:rust` / `ci:native` — not re-run; no SOURCE_PATHS or Rust change

## Limits

- No Nucleus source, windowed/native-visual run, Jetstream, workflow, or shared
  g16 front-door change.
- Nucleus Tabs stays a later card. The Button receipt pin is the same SOURCE_PATHS
  refresh g16.066 used; it is not a Tabs M1 receipt. This exact-head repair did
  not retouch `packages/render` / GPUI / contracts, so the receipt stays at
  `5a7a8f2a0`.
